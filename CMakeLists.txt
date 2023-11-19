cmake_minimum_required(VERSION 3.27.7)
project(ContaCabrito CXX)

set(CMAKE_CXX_STANDARD, 20)
#set(CMAKE_TOOLCHAIN_FILE windows_toolchain.cmake)
if(NOT TARGET SPDLOG)
    set(spdlog_DIR ${CMAKE_SOURCE_DIR}/spdlog)
    list(APPEND CMAKE_MODULE_PATH "${spdlog_DIR}/cmake")
    find_package(spdlog REQUIRED)
endif(NOT TARGET SPDLOG)

include_directories(include)
include_directories(lib)
subdirs(src)
