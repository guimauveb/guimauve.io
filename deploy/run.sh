#!/bin/bash
kill "$(<~/pids/guimauve-io-backend)" &&
kill "$(<~/pids/guimauve-io-frontend)" &&
git pull > ~/logs/guimauve-io-git-pull 2>&1 &
cd ~/guimauve-io/deploy &&
./run-backend.sh &&
./run-frontend.sh
