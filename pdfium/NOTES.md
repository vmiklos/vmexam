# pdfium notes

## setup

`zypper in binutils-gold` to have `/usr/bin/ld.gold`

See <https://pdfium.googlesource.com/pdfium/>:

```
git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git
export PATH=`pwd`/depot_tools:"$PATH"
gclient config --unmanaged https://pdfium.googlesource.com/pdfium.git
gclient sync
mkdir workdir
gn gen workdir/debug
gn args workdir/debug
```

argument list:

```
is_debug = true
pdf_enable_xfa = false
pdf_enable_v8 = false
pdf_is_standalone = true    # important - to get pdfium_test
is_component_build = false
pdf_is_complete_lib = true
#system_libjpeg_config = true
#is_asan = true
```

incremental build:

```
. ~/git/vmexam/pdfium/autogen.env
git pull -r
gclient sync
ninja -C workdir/debug
# exclude workdir and other automatically
ctags --c++-kinds=+p --fields=+iaS --extra=+q -R --totals=yes $(git ls-files|grep /|sed 's|/.*||'|sort -u)
# all unit tests are expected to pass locally
workdir/debug/pdfium_unittests
# some embedder tests are only expected to pass on the try bots
workdir/debug/pdfium_embeddertests --gtest_filter=FPDFEditEmbedderTest.AddPaths
```

compile commands:

```
cd workdir/debug
ninja -t compdb cc cxx >../../compile_commands.json
```

IWYU:

```
sed -i 's|-Xclang -debug-info-kind=constructor ||' compile_commands.json
sed -i 's|-Wno-non-c-typedef-for-linkage ||' compile_commands.json
sed -i 's|-Wmax-tokens ||' compile_commands.json
sed -i 's|-Xclang -add-plugin -Xclang find-bad-constructs -Xclang -plugin-arg-find-bad-constructs -Xclang check-ipc ||' compile_commands.json
~/git/include-what-you-use/iwyu_tool.py -p . fpdfsdk/fpdf_signature_embeddertest.cpp
```

vim:

```
:set shiftwidth=2
:set softtabstop=2
:set expandtab
:set textwidth=80
```

sharing a git grep result via a link:

<https://source.chromium.org/search?q=cpdf_annot::subtype::stamp&ss=chromium>

## signature API:

- <https://pdfium-review.googlesource.com/c/pdfium/+/70830> `Add FPDF_GetSignatureCount() API`

```
./testing/tools/fixup_pdf_template.py testing/resources/two_signatures.in
workdir/debug/pdfium_embeddertests --gtest_filter=FPDFSignatureEmbedderTest.*
```

- checklist:
  - wrap commit message at 72 columns
  - `snake_case` for variable names, but kFooBar for constants, see
    <https://google.github.io/styleguide/cppguide.html#General_Naming_Rules>
  - avoid {} on if/else if possible, but put it for all branches if one branch
    needs it, see <https://google.github.io/styleguide/cppguide.html#Conditionals>
  - iwyu before/after on touched files

- pdfiumsig incremental development:

```
touch Makefile && make && (./pdfiumsig good.pdf; ./pdfiumsig 2good.pdf; ./pdfiumsig bad.pdf; ./pdfiumsig reason.pdf; ./pdfiumsig partial-in-between.pdf)
```

## path API

- first goal: expose # of points in a path -> patch is at https://pdfium-review.googlesource.com/c/pdfium/+/13592
- ed4705b4db1405a5abef99ad1b2725eee65fedf8 added similar public api with a test

how to run the tests?

```
workdir/debug/pdfium_embeddertests
```

But has to disable bunch of already failing tests.

### get nth point from path API

- `FPDFPath_GetPoint`, based on `FPDFPage_GetObject`
- impl next to `FPDFPath_CountPoint` -> https://pdfium-review.googlesource.com/c/pdfium/+/14111

## taking clang-format from Chromium into LO

https://chromium.googlesource.com/chromium/buildtools.git/+/master/DEPS -- current is clang-format r302580
sha1 hashes:
- linux64 5349d1954e17f6ccafb6e6663b0f13cdb2bb33c8
- mac 0679b295e2ce2fce7919d1e8d003e497475f24a3
- win c8455d43d052eb79f65d046c6b02c169857b963b

download:
- `download_from_google_storage --no_resume --platform=linux* --no_auth --bucket chromium-clang-format 5349d1954e17f6ccafb6e6663b0f13cdb2bb33c8`
  -> clang-format-r302580-linux64
- `download_from_google_storage --no_resume --no_auth --bucket chromium-clang-format 0679b295e2ce2fce7919d1e8d003e497475f24a3 # mac`
  -> clang-format-r302580-mac
- `download_from_google_storage --no_resume --no_auth --bucket chromium-clang-format c8455d43d052eb79f65d046c6b02c169857b963b # win`
  -> clang-format-r302580-win.exe

https://dev-www.libreoffice.org/bin/ is where we host these binaries

## New APIs

### Added after 3426

- https://pdfium-review.googlesource.com/32770 `FPDFPageObj_GetStrokeWidth()` [2018-05-21]
- https://pdfium-review.googlesource.com/33010 `FPDFPath_GetDrawMode()` [2018-05-30]
- https://pdfium-review.googlesource.com/33670 `FPDFPath_GetMatrix()` [2018-06-04]
- https://pdfium-review.googlesource.com/35434 `FPDFText_GetMatrix()` [was `FPDFTextObj_GetMatrix()`, 2018-06-19]

snip, need to upgrade to 3473 at this point

- https://pdfium-review.googlesource.com/35931 `FPDFTextObj_GetFontSize()` [2018-06-26]
- https://pdfium-review.googlesource.com/36750 `FPDFText_GetTextRenderMode()` [was `FPDFTextObj_GetColor()`, 2018-07-03]
- https://pdfium-review.googlesource.com/37316 `FPDFFormObj_CountObjects()` [2018-07-11]
- https://pdfium-review.googlesource.com/37890 `FPDFFormObj_GetObject()` [2018-07-16]
- https://pdfium-review.googlesource.com/38870 `FPDFText_GetFontName()` [2018-08-01]
- https://pdfium-review.googlesource.com/39530 `FPDFTextObj_GetText()` [2018-08-07]
- https://pdfium-review.googlesource.com/39930 `FPDFFormObj_GetMatrix()` [2018-08-14]

next planned upgrade: around 2018-09-17

Other goals:

- `FPDFImageObj_GetBitmapBgra`
