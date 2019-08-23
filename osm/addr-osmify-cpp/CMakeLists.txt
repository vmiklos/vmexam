# Copyright 2019 Miklos Vajna. All rights reserved.
# Use of this source code is governed by a BSD-style license that can be
# found in the LICENSE file.

cmake_minimum_required(VERSION 3.4.3)
project(addr_osmify)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

if (NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  message(STATUS "No build type selected, default to Release")
  set(CMAKE_BUILD_TYPE "Release" CACHE STRING "Build type (default Release)" FORCE)
endif()

set(CMAKE_CXX_STANDARD 17)
if(UNIX)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Werror")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wextra")
endif()

option(OSMIFY_IWYU "Run include-what-you-use with the compiler." OFF)
if(OSMIFY_IWYU)
  find_program(IWYU_COMMAND NAMES include-what-you-use iwyu)
  if(NOT IWYU_COMMAND)
    message(FATAL_ERROR "OSMIFY_IWYU is ON but include-what-you-use is not found!")
  endif()
  set(CMAKE_CXX_INCLUDE_WHAT_YOU_USE
    "${IWYU_COMMAND};-Xiwyu;--mapping_file=${CMAKE_SOURCE_DIR}/mapping.imp")
endif()

find_package(Poco REQUIRED Foundation JSON Net NetSSL)

add_executable(addr-osmify addr-osmify.cxx)
target_link_libraries(addr-osmify
    Poco::Foundation
    Poco::JSON
    Poco::Net
    Poco::NetSSL
    )
install(TARGETS addr-osmify DESTINATION bin)

# vim:set shiftwidth=4 softtabstop=4 expandtab:
