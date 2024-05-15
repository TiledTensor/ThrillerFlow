#!/bin/bash

# Run the examples in the examples directory
for file in $(find examples/ -name *.rs); do
    unit=$(basename $file .rs)
    make example EXAMPLE=$unit
done