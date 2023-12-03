// 添加 my_path/file1.rs 这个文件
mod file1;
pub mod file2;

// 引用那个文件中的 mod
pub use file1::mod_sub2; // 加 pub 后，外界就也能调它了

// mod.rs 自己也可以有 mod
pub mod my_mod_sub {
    pub fn func2() {
        println!("hello");
    }
}