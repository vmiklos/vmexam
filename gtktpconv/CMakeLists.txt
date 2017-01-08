project(gtktpconv)
cmake_minimum_required(VERSION 2.6)
if (UNIX)
    find_package(PkgConfig REQUIRED)
    pkg_check_modules(GTK3 REQUIRED gtk+-3.0)
elseif(MSVC)
    set(GTK_BUILD "c:/gtk-build/gtk/Win32/release")
    set(GTK3_INCLUDE_DIRS "${GTK_BUILD}/include/gtk-3.0"
                          "${GTK_BUILD}/include/glib-2.0"
                          "${GTK_BUILD}/include/pango-1.0"
                          "${GTK_BUILD}/include/cairo"
                          "${GTK_BUILD}/include/gdk-pixbuf-2.0"
                          "${GTK_BUILD}/include/atk-1.0"
                          "${GTK_BUILD}/lib/glib-2.0/include")
    set(GTK3_LIBRARY_DIRS "${GTK_BUILD}/lib")
    set(GTK3_LIBRARIES gtk-3.0
                       glib-2.0
                       gobject-2.0)
endif()
include_directories(${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})
add_definitions(${GTK3_CFLAGS_OTHER})
add_executable(gtktpconv source/main.cxx source/lib.cxx)
target_link_libraries(gtktpconv ${GTK3_LIBRARIES})

# MSVC supports (a subset of) C++11 without any switches.
if (NOT MSVC)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Werror")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wextra")
endif()

# vim:set shiftwidth=4 softtabstop=4 expandtab:
