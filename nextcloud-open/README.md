# nextcloud-open

If you have a large folder tree synchronized from Nextcloud, then it's annoying that once you're in
`~/Nextcloud/Foo/Bar/Baz Blah`, you need to open
<https://nextcloud.example.com/apps/files/?dir=/Foo/Bar/Baz%20Blah/> mostly manually, by visiting
<https://nextcloud.example.com/apps/files/> and then clicking around.

This tool reads your Nextcloud configuration and allows you to give it a folder, for example:

```console
~/Nextcloud/Foo/Bar/Baz Blah$ nextcloud-open .
```

will open the current local directory in the browser. Then you can e.g. start collaborative editing
for a file in that directory inside the browser.
