extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("b.cpp")
        .compile("b.o");
}
