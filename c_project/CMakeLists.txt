cmake_minimum_required(VERSION 3.20)
set(CMAKE_C_STANDARD 99)
set(CMAKE_C_FLAGS "-O3")
project(c_lib C)


set(C_LIB_H
        c_code.h
)


set(C_LIB_SRC
        c_code.c
)

# 动态库
ADD_LIBRARY(c_lib SHARED ${C_LIB_H} ${C_LIB_SRC})
# 静态库
# ADD_LIBRARY(c_lib_static STATIC ${C_LIB_H} ${C_LIB_SRC})

INSTALL(FILES ${C_LIB_H} DESTINATION include/c_lib)
# 动态库
INSTALL(TARGETS c_lib LIBRARY DESTINATION lib)
# 静态库
# INSTALL(TARGETS c_lib_static ARCHIVE DESTINATION lib)