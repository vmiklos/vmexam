# Dioxus projects

See their [documentation](https://dioxuslabs.com/learn/). The idea is to write everything in one
typed language (Rust), develop it as a web app on a static site, but have the option to turn this
into an Android or desktop app, too.

## Conditional code

Guard for web-specific code:

```
#[cfg(feature = "web")]
```

Guard for desktop-specific code:

```
#[cfg(feature = "desktop")]
```

## Printf debugging

```
tracing::info!("debug, var is '{}'", var);
```

## Web build, install

This is handled via the `Makefile`, `make run` is debug build, `make install` is the release build.

## Android build, install

Debug build:

```
ANDROID_HOME=~/Android/Sdk/ ANDROID_NDK_HOME=~/Android/Sdk/ndk/current/ dx serve --platform android
```

Release build: first generate a signing key using

```
keytool -genkey -v  -keystore my-release-key.keystore -alias myapp -keyalg RSA -keysize 2048 -validity 10000 # pass: dioxus
```

and then build, sign, and install:

```
ANDROID_HOME=~/Android/Sdk/ ANDROID_NDK_HOME=~/Android/Sdk/ndk/current/ dx build --platform android --release --target aarch64-linux-android
~/Android/Sdk/build-tools/current/apksigner sign --ks my-release-key.keystore target/dx/skel/release/android/app/app/build/outputs/apk/debug/app-debug.apk
adb install -r target/dx/skel/release/android/app/app/build/outputs/apk/debug/app-debug.apk
```
