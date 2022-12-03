# ssh-proxy

Given a machine behind NAT, this little tool reads your `~/.config/ssh-proxyrc` and expects content
like this there:

```
destination='outer'
```

where `outer` is some machine name.

Then on `outer` you can have ssh config like this in `~/.ssh/config`:

```
Host inner
        NoHostAuthenticationForLocalhost yes
        HostName localhost
        Port 2222
```

This way you can access `inner` from `outer`.
