#!/bin/bash

# Build the project
cmake -B build -S . -G "Ninja Multi-Config"
cmake --build build --config Debug
