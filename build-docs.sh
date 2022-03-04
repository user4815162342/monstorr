#!/bin/sh

rm -rf docs/rustdoc
cargo doc --no-deps --workspace --document-private-items
mv target/doc docs/rustdoc
