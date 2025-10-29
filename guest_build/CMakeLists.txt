cmake_minimum_required(VERSION 3.18)
project(guest-risc0 C)
set(LEAN_RISC0_PATH "$ENV{LEAN_RISC0_PATH}" CACHE STRING "Lean RISC0 runtime path")

file(GLOB_RECURSE SRC_C CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/risc0_ir/*.c")

add_library(Guest STATIC ${SRC_C})

target_include_directories(Guest PUBLIC
  $<BUILD_INTERFACE:${LEAN_RISC0_PATH}/include>
)
target_compile_features(Guest PUBLIC c_std_11)

set(CMAKE_C_FLAGS "-ffunction-sections -fdata-sections -fPIC -march=rv32im -mabi=ilp32 -O3 -ffreestanding -fno-builtin")
