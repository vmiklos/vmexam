# send-email

See `src/main.rs` for details, this is a simple email sender that supports sending mail via a
customer server, based on user/pass authentication.

Simple manual test:

```
echo -e "foo\nbar" | cargo run -- -f send-email@example.com -s "send-email test @ $(date -Iseconds)" $USER@example.com
```
