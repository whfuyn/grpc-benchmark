cmake_minimum_required(VERSION 3.5.1)

project(GrpcBenchmark C CXX)

include(./common.cmake)

SET(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O3")
SET(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3")

# Proto file
get_filename_component(example_proto "../proto/example.proto" ABSOLUTE)
get_filename_component(example_proto_path "${example_proto}" DIRECTORY)

# Generated sources
set(example_proto_srcs "${CMAKE_CURRENT_BINARY_DIR}/example.pb.cc")
set(example_proto_hdrs "${CMAKE_CURRENT_BINARY_DIR}/example.pb.h")
set(example_grpc_srcs "${CMAKE_CURRENT_BINARY_DIR}/example.grpc.pb.cc")
set(example_grpc_hdrs "${CMAKE_CURRENT_BINARY_DIR}/example.grpc.pb.h")

add_custom_command(
      OUTPUT "${example_proto_srcs}" "${example_proto_hdrs}" "${example_grpc_srcs}" "${example_grpc_hdrs}"
      COMMAND ${_PROTOBUF_PROTOC}
      ARGS --grpc_out "${CMAKE_CURRENT_BINARY_DIR}"
        --cpp_out "${CMAKE_CURRENT_BINARY_DIR}"
        -I "${example_proto_path}"
        --plugin=protoc-gen-grpc="${_GRPC_CPP_PLUGIN_EXECUTABLE}"
        "${example_proto}"
      DEPENDS "${example_proto}")

# Include generated *.pb.h files
include_directories("${CMAKE_CURRENT_BINARY_DIR}")

# example_grpc_proto
add_library(example_grpc_proto
  ${example_grpc_srcs}
  ${example_grpc_hdrs}
  ${example_proto_srcs}
  ${example_proto_hdrs})

target_link_libraries(example_grpc_proto
  ${_REFLECTION}
  ${_GRPC_GRPCPP}
  ${_PROTOBUF_LIBPROTOBUF})

set(_target "grpc-benchmark")
add_executable(${_target} "${_target}.cc")
target_link_libraries(${_target}
    example_grpc_proto
    ${_REFLECTION}
    ${_GRPC_GRPCPP}
    ${_PROTOBUF_LIBPROTOBUF})
