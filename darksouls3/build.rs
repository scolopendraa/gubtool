const FOLDERS: [(&str, &str, bool); 1] = [("src/resources/asm/", "darksouls3", false)];

fn main() {
    assemble::object::build(&FOLDERS)
}
