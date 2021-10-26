#!/bin/bash
kill "$(<~/pids/guimauve.io-backend)" &&
cd ~/guimauve.io/backend &&
cargo build --release > ~/logs/guimauve.io-backend 2>&1 && (./target/release/guimauve.io-backend > ~/logs/guimauve.io-backend 2>&1 & echo $! > ~/pids/guimauve.io-backend)
