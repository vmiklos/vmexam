#!/bin/bash

time sh -ce "git pull -r
    git clean -x -d -f
    cat ../addr-osmify-js/index.html > app/src/main/assets/index.html
    cat ../addr-osmify-js/bundle.js > app/src/main/assets/bundle.js
    echo 'ndk.dir=$HOME/git/android/Android/Sdk/ndk-bundle' > local.properties
    echo 'sdk.dir=$HOME/git/android/Android/Sdk' >> local.properties
    ./gradlew installDebug" 2>&1 |tee log

# vim:set shiftwidth=4 expandtab:
