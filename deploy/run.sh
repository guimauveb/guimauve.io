#!/bin/bash
cd ~/guimauve.io/deploy &&
git pull > ~/logs/guimauve.io-git-pull 2>&1 &
./run-backend.sh &&
./run-frontend.sh
