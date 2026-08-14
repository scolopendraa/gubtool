const FOLDERS: [(&str, &str, bool); 2] = [
    ("src/sys/resources/asm32/", "sys32", true),
    ("src/sys/resources/asm64/", "sys64", false),
];

fn main() {
    assemble::object::build(&FOLDERS)
}
