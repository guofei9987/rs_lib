// 引用 mod.rs 中的内容
use super::my_mod_sub;
// 引用 file1.rs 中的内容
use super::mod_sub1;

pub mod mod_sub2 {
    pub fn func3() {
        println!("mod_sub2.func3");
    }
}

pub fn get_file1() {
    let file_content = include_str!("../../resources/file1.txt");
    println!("读取文件成功！\n{}",file_content)
}