extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-fkeep-inline-functions")
        .file("b.cpp")
        .compile("b.o");
}
