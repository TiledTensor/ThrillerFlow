#!/bin/bash

function run_examples(){
    for file in $(find $1 -name *.rs); do
        unit=$(basename $file .rs)
        make example EXAMPLE=$unit
    done
}

run_examples "examples/"
run_examples "examples/gemm"

