// 添加 my_path/file1.rs 这个文件
mod file1;
mod file2;

// 引用那个文件中的 mod。加 pub 后，外界就也能调它了
pub use file1::mod_sub1;
pub use file2::mod_sub2;
pub use file2::get_file1;

// mod.rs 自己也可以有 mod
pub mod my_mod_sub {
    pub fn func2() {
        println!("hello");
    }
}