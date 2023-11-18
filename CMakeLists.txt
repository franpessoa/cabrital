cmake_minimum_required(VERSION 3.27.7)
project(ContaCabrito CXX)

set(CMAKE_CXX_STANDARD, 20)
include_directories(include)
include_directories(lib)
subdirs(src)