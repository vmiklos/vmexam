project(hello-gtk)
cmake_minimum_required(VERSION 2.6)
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtk+-3.0)
include_directories(${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})
add_definitions(${GTK3_CFLAGS_OTHER})
add_executable(hello source/main.cxx)
target_link_libraries(hello ${GTK3_LIBRARIES})

# vim:set shiftwidth=4 softtabstop=4 expandtab:
