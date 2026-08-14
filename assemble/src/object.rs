use {
    crate::{AsmFolder, AsmFunction, Relocation},
    object::{Object, ObjectSection, ObjectSymbol, RelocationTarget},
    std::{collections::VecDeque, env, fs, path::PathBuf, process::Command},
};

pub fn build(folders: &[(&'static str, &'static str, bool)]) {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for (path, name, is_32) in folders {
        let folder_out = out_dir.join(name);
        fs::create_dir_all(&folder_out).unwrap();

        let mut functions = Vec::<AsmFunction>::new();

        for file in fs::read_dir(path).unwrap() {
            let file_path = file.unwrap().path();

            let file_stem = file_path.file_stem().unwrap().to_string_lossy();
            let obj = folder_out.join(format!("{file_stem}.o"));

            let mut cmd = Command::new("gcc");
            cmd.arg("-c");
            if *is_32 {
                cmd.arg("-m32");
            }
            cmd.arg(&file_path);
            cmd.arg("-Wa,-msyntax=intel");
            cmd.arg("-Wa,-mnaked-reg");
            cmd.arg("-o");
            cmd.arg(&obj);
            let status = cmd.status().unwrap();
            assert!(status.success(), "failed to assemble {:?}", file_path);

            println!("cargo:rerun-if-changed={}", file_path.display());

            let mut relocations: VecDeque<Relocation> = VecDeque::new();

            let bytes = fs::read(&obj).unwrap();
            let obj_file = object::File::parse(&*bytes).unwrap();

            let section = obj_file.section_by_name(".text").unwrap();
            let text = section.data().unwrap();

            for (offset, reloc) in section.relocations() {
                if let RelocationTarget::Symbol(symbol_index) = reloc.target() {
                    let symbol = obj_file.symbol_by_index(symbol_index).unwrap();
                    relocations
                        .push_back(Relocation::new(symbol.name().unwrap().to_string(), offset));
                }
            }

            functions.push(AsmFunction::new(file_stem.to_string(), text.to_vec(), relocations));
        }

        let folder = AsmFolder::new(functions);
        let encoded = bincode::serialize(&folder).unwrap();
        let out_file = out_dir.join(format!("{name}.bin"));
        fs::write(&out_file, encoded).unwrap();

        println!("cargo:rerun-if-changed={}", path);
    }
}
