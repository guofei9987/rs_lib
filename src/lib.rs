// 关联 my_mod1.rs，然后才能 use
pub mod my_mod1;
mod my_path;
pub mod c_script;
pub mod c_lib;

pub use crate::my_mod1::mod_in_file::my_func1;
pub use crate::my_path::{my_mod_sub, mod_sub1, mod_sub2, get_file1};

// lib.rs 也可以有函数
pub fn fn_in_lib() -> String {
    let res = "运行 lib.rs 中的 fn_li_lib".to_string();
    println!("my_mod1!");
    my_func1();
    crate::my_mod1::mod_in_file::my_func1();
    res
}