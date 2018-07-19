// build.rs
//Here is where we build our C code from, we use the crate cc for GCC compiling
extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/fibonacci.c")
        .compile("fibonacci");
}