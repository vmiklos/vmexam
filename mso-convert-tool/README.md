# mso-convert-tool

This is the underlying tool used by mso-convert, which runs inside a VM.

Sample usage is meant to be close to how [docto](https://github.com/tobya/DocTo) works:

```
mso-convert-tool --word --from test.docx --output test.doc -t wdFormatDocument97
```

Backends:

- Linux, using `soffice`
