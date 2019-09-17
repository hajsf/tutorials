fn main() {
    cc::Build::new()
        .file("twice.c")    // can be multiple files
        .compile("twice.a");  //  .compile("twice");
}