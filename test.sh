#!/bin/sh

cd ~/rust_cli_application_template

RUST_TEST_THREADS=1 cargo test --all

read line
