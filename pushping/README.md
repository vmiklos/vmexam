# pushping

This is a <https://www.pushbullet.com/> replacement: can run a command and send its exit status to a
Matrix channel.

Config file:

```
access_token='...'
room_url='https://server.example.com:8448/_matrix/client/r0/rooms/!roomhash:example.com'
```

Create the access token using:

```
curl -X POST -d '{"type":"m.login.password", "user":"...", "password":"..."}' "https://server.example.com:8448/_matrix/client/r0/login"
```

The 'pushping' name refers to pushbullet, which provides something similar, but not with your
self-hosted matrix instance.

If you want some more generic way to automatically send messages to matrix rooms, try
[matrix-send](https://github.com/tilosp/matrix-send-rs).
