# LO clang notes

## Member prefixing effort

### Handling the first non-conforming class in a module, with public headers

```
make -C svx -sr -j8 COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-yaml -path-prefix=$PWD/include/svx " FORCE_COMPILE=all 2>&1 |tee ~/rename.yaml
# Touch the header of the relevant class.
make check gb_SUPPRESS_TESTS=y COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=clang-rename-wrapper RENAME_ARGS="-input=$HOME/rename.yaml -force"
clang-apply-replacements -remove-change-desc-files /tmp/rename
make check gb_SUPPRESS_TESTS=y
```

### Find all places pointed out by an ast matcher

```
make -C sw -sr -j8 COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=ast-matcher-wrapper FORCE_COMPILE=all
```

### Handling the first non-conforming class in a module, when the module doesn't have public headers

```
make -C writerfilter -sr -j$(nproc) -O gb_SUPPRESS_TESTS=y FORCE_COMPILE=all COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-path-prefix=$PWD/writerfilter -yaml" 2>&1 |tee ~/rename.yaml
```

### Finding a class with a known name in a module

```
# Touch the header of the relevant class.
make check gb_SUPPRESS_TESTS=y COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-class-name=SwFieldPortion -yaml" 2>&1 |tee ~/rename.yaml
```

(See above for the rest: clang-rename and clang-apply-replacements invocations.)

Building a complete list of non-conforming members for a module:

```
make -k -sr -j8 -O FORCE_COMPILE=all COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-path-prefix=$PWD" 2>&1 |grep :: | tee ~/sw-to-prefix.log
```

### Other examples:

Detect unprefixed members in a whole module:

```
make -sr -j8 COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-class-prefix=Sw" FORCE_COMPILE=all
```

Detect unprefixed members for a class:

```
make -sr -j8 COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-class-name=SdrDragView -yaml" 2>&1 |tee ~/rename.yaml
make build-nocheck COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-class-name=SdrDragView"
```

Detect unprefixed members in a directory:

```
make -sr COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=find-unprefixed-members-wrapper RENAME_ARGS="-path-prefix=$PWD/source/filter/ww8/rtf"
```

Example `rename.csv` for a member function rename:

```
SdrDragStat::GetPointAnz,GetPointCount
```

Rename them:

```
make -sr -j8 COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=rename-wrapper RENAME_ARGS="-csv=$HOME/rename.csv"
make build-nocheck COMPILER_EXTERNAL_TOOL=1 CCACHE_PREFIX=rename-wrapper RENAME_ARGS="-csv=$HOME/rename.csv"
```

Put output into git:

```
for i in $(find . -name "*.new-rename"); do mv -f $i ${i%%.new-rename}; done
```

## online.git commands

Detect:

```
make -j8 CCACHE_PREFIX=find-unprefixed-members-wrapper CCACHE_DISABLE=1 RENAME_ARGS="-poco -class-excluded-prefix=std::,Poco::"
```

Rename:

```
make -j8 CCACHE_PREFIX=rename-wrapper CCACHE_DISABLE=1 RENAME_ARGS="-csv=$HOME/rename.csv"
```

Generate `compile_commands.json` for clang-rename:

```
bear make -j8 CCACHE_DISABLE=1
```

Find with ast matchers:

```
make CCACHE_PREFIX=ast-matcher-wrapper CCACHE_DISABLE=1 build-nocheck
```

## clang-tidy

Online:

```
run-clang-tidy 2>&1 |tee log.clang-tidy
grep error: log.clang-tidy |grep -v misc-non-private
```

Core:

```
~/git/vmexam/libreoffice/clang-tidy.py 2>&1 |tee log.clang-tidy
```

## IWYU

Core:

```
python3.9 bin/find-unneeded-includes $(grep cxx$ .git/indented-files2.cache)
bin/find-unneeded-includes sw/inc/*.hxx
bin/find-unneeded-includes writerfilter/inc/{dmapper,ooxml,rtftok}/*.hxx
```

Online:

```
~/git/include-what-you-use/iwyu_tool.py -p . test/UnitTiffLoad.cpp
```

## Sanitizers

Core:

- See the blog post at <https://vmiklos.hu/blog/libreoffice-asan-setup.html#_update_2019_11_14>

Online:

```
grep FAIL test/*.trs
```

Gives a list of all failures, then `run-wsdunit` can run a single test.

Poco:

```
git clone https://github.com/pocoproject/poco poco-fuzz
cd poco-fuzz/
git checkout -b poco-1.9.1 origin/poco-1.9.1
# This is not enough, we need Crypto and NetSSL_OpenSSL
#./configure --no-samples --no-tests --minimal
./configure --no-samples --no-tests --omit=Zip,Data,Data/SQLite,Data/ODBC,Data/MySQL,MongoDB,PDF,CppParser,PageCompiler
make -j8 CC="$CC" CXX="$CXX"
make -j8 CC="$CC" CXX="$CXX" install INSTALLDIR=$PWD/install
```

## Build time profiling

Core:

```
make check GBUILD_TRACE=$HOME/make-check-trace.json
```
