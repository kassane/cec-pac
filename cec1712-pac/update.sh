#!/usr/bin/env bash
set -x
set -e

rm -rf src
mkdir src
svd2rust -i ../svd/CEC1712H_B2_SX.svd --const_generic
form -i lib.rs -o src
rm lib.rs
cargo fmt
