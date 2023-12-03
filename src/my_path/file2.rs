// 引用 mod.rs 中的内容
use super::my_mod_sub;
// 引用 file1.rs 中的内容
use super::mod_sub2;

mod mod3 {
    fn func3() {
        println!("func3");
    }
}