const FOLDERS: [(&str, &str, bool); 1] = [("src/resources/asm/", "eldenring", false)];

fn main() {
    assemble::object::build(&FOLDERS)
}
