# weechat calc

If you're coming to weechat from irssi, then you may miss irssi's `/calc` alias.

Once you build this crate with `cargo build` and you symlink weechat-calc to your PATH, you can
create a `/calc` alias in weechat using:

```
/alias add calc exec -norc weechat-calc
```

Then the usage is something like:

```
/calc 1/3
```

Note that this is really similar to just using [calc](https://github.com/coriolinus/calc), except
that the input expression is also printed (but the underlying library is the same).
