# cap2exif

Simple tool to read a `captions.txt` in the current directory and set both Exif and XMP caption
inside JPEG files.

## Installation

```
zypper in libgexiv2-devel # provides gexiv2.pc
cargo install --git https://github.com/vmiklos/vmexam cap2exif
```

## Usage

Given captions.txt in the current directory:

```
test.jpg	Foo bar baz
```

When running:

```
cap2exif
```

It'll set captions so that tools like [Geeqie](https://www.geeqie.org/) or
[Memories](https://github.com/pulsejet/memories) can show those captions.
