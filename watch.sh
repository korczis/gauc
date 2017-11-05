#! /usr/bin/env bash

cargo watch -x 'build' \
    -x 'test --all' \
    -x 'doc --all' \
    -x 'bench --all'
