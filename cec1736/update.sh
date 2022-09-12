#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd2rust -i ../svd/CEC1736_S0_2HW.svd --const_generic
form -i lib.rs -o src
rm lib.rs
cargo fmt
