# Minimal rust-based web app server

See this in action at e.g. <https://share.vmiklos.hu/apps/calc>.

Ever had the nostalgia about old PHP apps where you could just throw in a file in a directory on an
Apache server and your app was up and running?

This simple project tries to do almost the same. A sample systemd service is included, which can run
all apps, not just a single one. A minimal HTML output module inspired by <https://www.yattag.org/>
is also included.

A small webapp is just <100 lines of Rust this way.
