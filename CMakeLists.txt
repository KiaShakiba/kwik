cmake_minimum_required(VERSION 3.12)

project(kwik)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

find_package(Threads)

file(GLOB CPP_FILES ${PROJECT_SOURCE_DIR}/lib/*.cpp)
file(GLOB TEST_FILES ${PROJECT_SOURCE_DIR}/tests/*.cpp)

add_library(${PROJECT_NAME} ${CPP_FILES})
add_executable(tests ${TEST_FILES})

add_subdirectory(${PROJECT_SOURCE_DIR}/extern/Catch2)

target_include_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/include)
target_link_libraries(${PROJECT_NAME} Threads::Threads)
target_link_libraries(tests ${PROJECT_NAME} Catch2::Catch2WithMain)
