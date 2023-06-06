#!/bin/bash
set -e
cargo build --release

# clean env
rm -fr test


# case one
mkdir -p test/x
echo "TEST_ONE" > test/x/test.txt
target/release/lnn test/x test/y
echo "TESTING IF TEST_ONE IS IN test/y"
if [[ $(< test/y/test.txt) != "TEST_ONE" ]]; then
    exit 1
fi
echo "TEST_TWO" > test/y/test.txt
echo "TESTING IF TEST_TWO IS IN test/x"
if [[ $(< test/x/test.txt)  != "TEST_TWO" ]]; then
    exit 1
fi

rm -fr test
# CASE TWO
mkdir -p test/x
mkfifo test/x/test
echo "BY DEFAULT LINKING FIFO SHOULD FAIL"
if target/release/lnn test/x test/y; then
    echo "WENT THROUGH ?"
    exit 1
fi

rm -fr test
# CASE THREE
mkdir -p test/x
mkfifo test/x/test
echo "TESTING IF LINKING FIFO SKIPS"
target/release/lnn test/x test/y --skip-unknown
if [[ -f test/y/test ]]; then
    echo "WENT THROUGH ?"
    exit 1
fi

#clean up after us
rm -fr test