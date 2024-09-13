# qemu exec client/server

Problem: a (Windows) VM runs some software (Adobe Acrobat, Microsoft Word, etc), but the files are
on the host. Given that the files are shared between the guest and the host, it would be nice to
have a host (Linux) command to open a given file inside the VM.

This is similar to QEMU guest agent's guest-exec, but given that the server is started on login, the
executed process runs within the desktop session of the user, so it is possible to launch graphical
apps as well.

Start the server like this:

````
python exec_server.py
```

Start the client like this:

```
./exec_client.py calc.exe
```

A winword wrapper script is included to give an idea about an app-specific wrapper.
