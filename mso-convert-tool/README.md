# mso-convert-tool

This is the underlying tool used by mso-convert, which runs inside a VM.

Sample usage is meant to be close to how [docto](https://github.com/tobya/DocTo) works:

```
mso-convert-tool --word --from test.docx --output test.doc -t wdFormatDocument97
```

or on Windows:

```
$env:RUST_BACKTRACE=1; cargo run -- --word --from c:\t\test.docx --output c:\t\test.rtf -t wdFormatRTF
```

Backends:

- Linux, using `soffice`
- Windows, using [COM](https://learn.microsoft.com/en-us/office/vba/api/word.documents.open).
