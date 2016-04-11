extern crate gcc;

fn main() {
    gcc::compile_library("libbuse.a", &["lib/buse/buse.c"]);
}
