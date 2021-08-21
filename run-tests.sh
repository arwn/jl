#!/usr/bin/env sh

go build .
for test in tests/*; do
    x=`./jl  $test` || echo "failed: $test"
done
rm jl