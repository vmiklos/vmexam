#!/bin/bash -ex
#
# Copyright 2026 Miklos Vajna
#
# SPDX-License-Identifier: MIT
#

#
# Add an Android app icon, until DioxusLabs/dioxus #3685 is fixed.
#

CWD=$PWD
NAME=$(basename $PWD)
export ANDROID_HOME=~/Android/Sdk/
export ANDROID_NDK_HOME=~/Android/Sdk/ndk/current/

cd target/dx/$NAME/release/android/app/
./gradlew clean
cp $CWD/assets/icon.xml app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
./gradlew assembleRelease
cd $CWD
~/Android/Sdk/build-tools/current/apksigner sign --ks my-release-key.keystore target/dx/$NAME/release/android/app/app/build/outputs/apk/release/app-release-unsigned.apk
adb install -r target/dx/$NAME/release/android/app/app/build/outputs/apk/release/app-release-unsigned.apk

# vim:set shiftwidth=4 softtabstop=4 expandtab:
