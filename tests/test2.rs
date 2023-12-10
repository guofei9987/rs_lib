use rs_lib::my_mod1::mod_in_file::my_func1;
use rs_lib;

#[test]
fn test2() {
    rs_lib::my_func2();
    rs_lib::my_func1();
    rs_lib::mod_sub1::func3();
    rs_lib::mod_sub2::func3();
    rs_lib::get_file1();
}

use rs_lib::c_script::{double_input, third_input};

#[test]
fn test3() {
    let input = 4;
    let output = unsafe { double_input(input) };
    let output2: i32 = unsafe { third_input(input) };
    println!("{} * 3 = {}", input, output2);
    println!("{} * 2 = {}", input, output);
}

// use rs_lib::c_lib::my_func;
// #[test]
// fn test4(){
//    let a=unsafe{my_func(3,5)} ;
//     println!("{}",a);
// }