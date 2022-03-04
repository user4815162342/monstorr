#!/bin/sh

rm -rf documentation
cargo doc --no-deps --workspace --document-private-items
mv target/doc documentation
