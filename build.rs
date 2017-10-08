extern crate cc;

fn main() {
    cc::Build::new()
        .pic(true)
        .cpp(true)
        .file("b.cpp")
        .compile("b.o");
}
