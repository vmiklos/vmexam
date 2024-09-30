= gtktpconv

== Windows build notes

=== How to build Gtk3 itself

There doesn't seem to be a ready-to-use installer, so need to build it locally.
Commit fe24a2ca88c9c3f2074dfff682f46d52119f1364 (wing: update so it builds with
meson 0.39.0, 2017-03-11) of <https://github.com/wingtk/gtk-win32/> gives you
build scripts, 32bit VS 2013 can be used for a build like this:

----
python .\build.py build --msys-dir c:\msys32 --vs-install-path "c:\program files (x86)\microsoft visual studio 14.0" --vs-ver=14 gtk3
----

=== How to build the code

One possible way:

----
mkdir workdir
cd workdir
c:\gtk-build\tools\cmake-3.7.2-win64-x64\bin\cmake.exe ..
c:\gtk-build\tools\cmake-3.7.2-win64-x64\bin\cmake --build . --config Release
----

and then pack the result:

----
cd ..
python scripts/pack.py
----

// vim: ft=asciidoc
