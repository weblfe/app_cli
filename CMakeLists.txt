cmake_minimum_required(VERSION 3.23)
project(app_cli C)

set(CMAKE_C_STANDARD 11)

include_directories(src/c)

add_executable(app_cli
        src/c/git.c
        src/c/git.h)
