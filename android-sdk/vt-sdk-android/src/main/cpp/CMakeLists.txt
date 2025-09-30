cmake_minimum_required(VERSION 3.18)
project(vtsdk_shim LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(vtsdk_shim SHARED vtsdk_jni.cpp)

find_library(log-lib log)
find_library(dl-lib dl)

target_link_libraries(vtsdk_shim PRIVATE ${log-lib} ${dl-lib})

