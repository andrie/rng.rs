// build.rs
fn main() {
    cc::Build::new()
        .file("src/runif.c")
        .file("src/rnorm.c")
        .compile("rnorm");
}