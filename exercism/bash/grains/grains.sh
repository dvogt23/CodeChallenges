#!/usr/bin/env bash

if [ "$1" = "total" ]; then
    printf "%llu\n" $((2**64-1))
elif [ "$1" -eq 0 ]; then
    echo "Error: invalid input"
    exit 1;
elif [ "$1" -lt 0 ]; then
    echo "Error: invalid input"
    exit 1;
elif [ "$1" -le 64 ]; then
    printf "%llu\n" $((2**($1-1)))
else 
    echo "Error: invalid input"
    exit 1;
fi
