// 假设 文件名为 libc_lib.dylib
// #[link(name = "c_lib")]
// extern "C" {
//    pub fn my_func(a: i32, b: i32) -> i32;
// }

use libloading::{Library,Symbol};

struct CProject{
    my_c_func:fn(i32,i32)->i32
}

impl CProject {

    fn new(){


    }

}