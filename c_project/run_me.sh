mkdir "build"
cd build
cmake ..
make
#sudo make install
cp libc_lib.dylib ../../resources/libc_lib.dylib