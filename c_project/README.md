这里放着一个完整的 C 项目，编译如下：

```shell
mkdir "build"
cd build
cmake ..
make
# sudo make install
# 移动到相关路径
cp libc_lib.dylib ../../resources/libc_lib.dylib
cp libc_lib.so ../../resources/libc_lib.so
```

然后可以用 Rust 调用这些编译好后的动态链接库了