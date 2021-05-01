cmake_minimum_required(VERSION 3.12)

project(kwik)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

include_directories(${PROJECT_SOURCE_DIR}/include/)

file(GLOB HPP_FILES ${PROJECT_SOURCE_DIR}/include/*.hpp)
file(GLOB CPP_FILES ${PROJECT_SOURCE_DIR}/lib/*.cpp)

add_executable(${PROJECT_NAME}
	${HPP_FILES}
	main.cpp
	${CPP_FILES}
)
