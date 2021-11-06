#!/bin/bash
cd ~/guimauve-io/frontend &&
TRUNK_BUILD_RELEASE=true
trunk serve > ~/logs/guimauve-io-frontend 2>~/logs/guimauve-io-frontend & echo $! > ~/pids/guimauve-io-frontend
