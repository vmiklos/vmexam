# clang libtooling-based tools

## find-nonconst-methods

Tries to flag non-const member functions which could be const.

Run it in a loop like this:

```
for i in $(cat .git/indented-files2.cache|grep -v /qa/); do ~/git/vmexam/llvm/libtoling/bin/find-nonconst-methods -paths-file .git/indented-files2.cache $i || break; done
```

Obsoleted by loplugin:constmethod.

## readability-avoid-auto

Demo tool to show how AST matching and fixit generation works.

## bugprone-suspicious-xmlsec

Tool that tries to flag misuses of the libxmlsec xmlSecError() API.

Run the self-test with:

```
bin/bugprone-suspicious-xmlsec qa/xmlsec.c --
```

Run it on the actual code with e.g.:

```
WHITELIST_FILE="bugprone-suspicious-xmlsec.whitelist" ~/git/llvm/tools/clang/tools/extra/clang-tidy/tool/run-clang-tidy.py -j1 -clang-tidy-binary ~/git/vmexam/llvm/libtooling/bin/bugprone-suspicious-xmlsec 2>&1 |tee log
grep error: log|sort -u
```

== ast-matcher

Boilerplate to test ast-matchers.

Run the playground with:

```
touch test.cpp && make CCACHE_PREFIX=ast-matcher-wrapper CCACHE_DISABLE=1
```

Run it on online.git code with:

```
make CCACHE_PREFIX=ast-matcher-wrapper CCACHE_DISABLE=1 build-nocheck
```
