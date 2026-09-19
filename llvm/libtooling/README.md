# clang libtooling-based tools

## readability-avoid-auto

Demo tool to show how AST matching and fixit generation works.

## ast-matcher

Boilerplate to test ast-matchers.

Run the playground with:

```
touch test.cpp && make CCACHE_PREFIX=ast-matcher-wrapper CCACHE_DISABLE=1
```

Run it on online.git code with:

```
make CCACHE_PREFIX=ast-matcher-wrapper CCACHE_DISABLE=1 build-nocheck
```
