extern crate cc;

fn main() {
    cc::Build::new().file("./src/c_scripts/double.c").compile("libdouble.a");
    cc::Build::new().file("./src/c_scripts/third.c").compile("libthird.a");
}