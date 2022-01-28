#!/usr/bin/env sh

go build .
find tests -type f -exec ./jl {} \;
rm jl