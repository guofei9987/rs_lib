# rs_lib
一个 Rust 项目示例，展示：
- 展示如何调用 C 源码。在 build 阶段自动编译 C 代码，并被 Rust 调用。
    - `./src/build.rs` 中定义编译时的行为
    - `./src/c_script.rs` 中定义调用的 C 代码
    - C 源码放在了 `./src/c_scripts` 中
- 展示如何编译一个 C 项目，然后 Rust 调用编译后的动态链接库
    - `./c_project/` 是一个 C 项目，可以使用 `cmake;make;` 编译它
    - 编译后的动态链接库放入 `./resources`
    - 在 `./src/c_lib.rs` 中调用动态链接库
    - 不同的平台应该重新编译并覆盖，否则可能报错
- 展示如何把数据文件一起打包
- 展示一个标准项目的目录结构
- 展示代码子目录


## 测试与运行

编译
```shell
cargo build
```

运行
```shell
cargo test
```


注意事项
1. `./tests/test2.test_c_compile_object` 是测试 rust 调用编译后的二进制文件
    - 在某些操作系统上，它可能报错，这是因为我只针对 MacBook M1 和 Linux AMD 做了编译
    - 你可以运行 `cargo build` 然后在 `./target/build` 中找到正确的编译后二进制文件，然后把它们复制到 `./resources` 里面
