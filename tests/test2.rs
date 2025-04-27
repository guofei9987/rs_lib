use rs_lib::my_mod1::mod_in_file::my_func1;
use rs_lib;


#[test]
fn test_rust_fn() {
    //     测试：rust 代码文件的相互调用
    assert_eq!(rs_lib::fn_in_lib(), "运行 lib.rs 中的 fn_li_lib".to_string());
    assert_eq!(rs_lib::my_func1(), "运行 my_mod1.my_func1".to_string());

    // 下面是子目录中的函数调用
    rs_lib::mod_sub1::func3();
    rs_lib::mod_sub2::func3();
    rs_lib::get_file1();
}


use rs_lib::c_script::{double_input, third_input};

#[test]
fn test_c_script() {
    // 测试：rust 调用 C 源码
    let input = 4;
    let output = unsafe { double_input(input) };
    let output2: i32 = unsafe { third_input(input) };
    println!("{} * 3 = {}", input, output2);
    println!("{} * 2 = {}", input, output);
}

use rs_lib::c_lib::CProject;
#[test]
fn test_c_compile_object() {
    // 测试：调用 C 编译后的文件
    let c_project = CProject::new();
    let z = c_project.call_my_func(3, 5);
    println!("{}", z);
}