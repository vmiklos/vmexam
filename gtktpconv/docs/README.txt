= gtktpconv

== Windows build notes

=== How to build Gtk3 itself

There doesn't seem to be a ready-to-use installer, so need to build it locally.
Commit 788356502b4426563202e1ec1e2d71ac1118b0f9 (Python 3.x print on stderr,
2016-12-29) of <https://github.com/wingtk/gtk-win32/> gives you build scripts,
32bit VS 2013 can be used for a build like this:

----
python .\build.py build --msys-dir c:\msys32 --vs-install-path "c:\program files\microsoft visual studio 12.0" gtk3
----

=== How to build the code

One possible way:

----
mkdir workdir
cd workdir
cmake ..
cmake --build . --config Release
----

and then pack the result:

----
cd ..
python windows/pack.py
----

== Android notes

=== How to set up local properties

Write something like to `android/local.properties`:

----
ndk.dir=/home/user/Android/Sdk/ndk-bundle
sdk.dir=/home/user/Android/Sdk
----

=== How to build the code

Plug in your Android device, then:

----
./gradlew installDebug
----

should build and install the code.

// vim: ft=asciidoc
