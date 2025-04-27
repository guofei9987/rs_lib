fn main() {
    cc::Build::new().file("./src/c_scripts/double.c").compile("libdouble.a");
    cc::Build::new().file("./src/c_scripts/third.c").compile("libthird.a");
}

//// OR this：
// fn main() {
//     cc::Build::new()
//         .opt_level(3)
//         .files(["./src/c_scripts/double.c", "./src/c_scripts/third.c"])
//         .compile("libmath.a");
// }