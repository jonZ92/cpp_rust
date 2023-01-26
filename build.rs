
fn main() {
    cc::Build::new()
        .cpp(true)
        .file("print.cpp")
        .shared_flag(true)
        .compile("libprint.a");
}
