use rs_lib::{my_func1, my_func2};

#[test]
fn func_1() { assert!(true, "可以填入报错信息，也可以不填。"); }

#[test]
fn func_2() { assert_eq!(my_func1(), 0, "两个值相等，使用 == 判断的"); }

#[test]
fn func_3() { assert_ne!(2 + 3, 4, "两个值不相等，使用 != 判断的"); }

// 有 panic
// panic 内容要与 expected 一致，才能通过测试
#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 9")]
fn func_4() {
    let v = vec![1, 2, 3];
    let a = v[9];
}

#[test]
fn func_5() -> Result<(), String> {
    if true {
        Ok(())
    } else {
        Err(String::from("如果触发 Err，则测试不通过"))
    }
}

#[test]
#[ignore] // 忽略，不会测试
fn func_6() {}

// debug_assert! 和 debug_assert_eq!，只在调试构建中检查断言