#!/bin/sh

# Use this script to generate the include file for the creature data when stored creatures are added or modified

cargo run -- gen-creatures-rust-array monstorr-data/data/creatures/
