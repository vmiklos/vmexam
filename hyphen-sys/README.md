Raw rust bindings for <https://github.com/hunspell/hyphen>.

The motivation for using this over <https://github.com/jfkthame/mapped_hyph>
is to support non-standard hyphenations (e.g. German "Schiffahrt" ->
"Schiff-fahrt" or Hungarian "asszonnyal" -> "asz-szony-nyal").

These are just raw bindings, exposing the full C API as-is, as suggested by
<https://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html>.
