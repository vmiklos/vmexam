# qemu exec-client

This can run outside, on a host Linux machine:

- if you symlink the script script to e.g. ~/bin/winword, then it will inject the exe for you
- it'll try to map between the host and guest paths

Config file: ~/.config/qemu-exec-clientrc

```
[qemu-exec-client]
guest-ip = '192.168.x.y'
drive-letter = 'z:'
```

You may want to customize:
- shared directory
- guest port
