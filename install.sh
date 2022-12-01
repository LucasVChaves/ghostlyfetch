#!/bin/bash
#This program needs cargo and rust installed
#It only works in Unix Systems
#I am pretty sure there are better ways to make this work. For now this is fine.
cargo build --release
mv target/release/ghostlyfetch /bin/