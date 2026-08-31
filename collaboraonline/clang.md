# COOL clang notes

## Member prefixing effort

### Handling the first non-conforming class in a module, with public headers

```
make -C editeng -sr -j8 COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-yaml -path-prefix=$PWD/include/editeng " FORCE_COMPILE=all 2>&1 |tee ~/rename.yaml
# Touch the header of the relevant class.
make check gb_SUPPRESS_TESTS=y COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=clang-rename-wrapper RENAME_ARGS="-input=$HOME/rename.yaml -force"
clang-apply-replacements -remove-change-desc-files /tmp/rename
make check gb_SUPPRESS_TESTS=y
```
