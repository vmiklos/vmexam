#!/bin/bash

time sh -ce "git pull -r
    git clean -x -d -f
    echo 'ndk.dir=$HOME/Android/Sdk/ndk-bundle' > local.properties
    echo 'sdk.dir=$HOME/Android/Sdk' >> local.properties
    ./gradlew installDebug" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
