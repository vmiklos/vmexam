cmake_minimum_required(VERSION 2.8.11)
project(Hello)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

if (MSVC)
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} /WX") # -Werror
elseif(APPLE)
elseif(UNIX)
    find_program(CCACHE "ccache")
    if (CCACHE)
        set_property(GLOBAL PROPERTY RULE_LAUNCH_COMPILE ccache)
    endif (CCACHE)

    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -D_GLIBCXX_DEBUG")
endif()

if (NOT MSVC)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -Werror")
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -Wall")
    set(CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -Wextra")
endif()

add_executable(hello source/hello.cxx)
install(TARGETS hello DESTINATION bin)

# vim:set shiftwidth=4 softtabstop=4 expandtab:
