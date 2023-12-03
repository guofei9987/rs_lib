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