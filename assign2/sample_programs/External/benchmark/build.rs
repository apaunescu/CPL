// build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/fibonacci.c")
        .compile("fibonacci");
}