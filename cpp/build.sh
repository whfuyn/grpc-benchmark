#!/bin/bash
mkdir -p cmake/build && cd cmake/build && cmake ../.. && make && cd ../.. && cp cmake/build/grpc-benchmark grpc-benchmark
