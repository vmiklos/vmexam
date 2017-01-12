cmake_minimum_required(VERSION 2.6)

# Building just this subdir.
if (CMAKE_SOURCE_DIR STREQUAL CMAKE_CURRENT_SOURCE_DIR)
    project(gtktpconv-core)
    if (UNIX)
        set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
    endif()
    set(BUILD_SHARED_LIBS ON)
endif()

# tpconv library.
add_library(tpconv source/lib.cxx)
target_include_directories(tpconv PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include")

# vim:set shiftwidth=4 softtabstop=4 expandtab:
