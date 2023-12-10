// 假设 文件名为 libc_lib.dylib
// #[link(name = "c_lib")]
// extern "C" {
//    pub fn my_func(a: i32, b: i32) -> i32;
// }

use libloading::{Library,Symbol};

pub struct CProject {
    lib: Library,
    my_func: unsafe extern "C" fn(i32, i32) -> i32,
}

impl CProject {
    pub fn new() -> Self {
        let lib_path = if cfg!(target_os = "windows") {
            "./resources/c_lib.dll"
        } else if cfg!(target_os = "macos") {
            "./resources/libc_lib.dylib"
        } else if cfg!(target_os = "linux") {
            "./resources/libc_lib.so"
        } else {
            panic!("Unsupported OS");
        };
        let lib = unsafe { Library::new(lib_path).expect("Failed to load library") };

        // 获取函数符号，并将其转换为函数指针
        let my_func: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            unsafe { lib.get(b"my_func").expect("Failed to find symbol") };
        let my_func_ptr = *unsafe { my_func.into_raw() };

        Self {
            lib,
            my_func: my_func_ptr, // 存储函数指针
        }
    }

    pub fn call_my_func(&self, x: i32, y: i32) -> i32 {
        // 直接使用存储的函数指针调用函数
        unsafe { (self.my_func)(x, y) }
    }
}