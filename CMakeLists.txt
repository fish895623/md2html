cmake_minimum_required(VERSION 3.28)

project(md2html CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(LIBRARY_FILES hello.cpp hello.hpp)
set(TEST_FILES hello_test.cpp)

add_library(md2html_lib STATIC ${LIBRARY_FILES})
add_executable(${PROJECT_NAME} main.cpp)
target_link_libraries(${PROJECT_NAME} md2html_lib)

include(gtest.cmake)

enable_testing()

add_executable(${PROJECT_NAME}_TEST ${TEST_FILES} ${LIBRARY_FILES})
target_link_libraries(${PROJECT_NAME}_TEST GTest::gtest_main md2html_lib)

include(GoogleTest)
gtest_discover_tests(${PROJECT_NAME}_TEST)
