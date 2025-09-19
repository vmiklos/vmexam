# qemu exec server

This can potentially auto-start inside a Windows VM:

- go to C:/Users/you/AppData/Roaming/Microsoft/Windows/Start Menu/Programs/Startup

- create an exec-server.lnk file in Explorer pointing to e.g.
  `c:/path/to/exec-server.exe`

You may want to customize the port number below.

Needless to say, run this only in a VM that is only visible to your local host, not to any outside
network, since it allows executing arbitrary commands, remotely, without auth.
