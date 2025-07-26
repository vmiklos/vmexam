# cap2exif

Simple tool to read a `captions.txt` in the current directory and set both Exif and XMP caption
inside JPEG files.

## Installation

```
zypper in libgexiv2-devel # provides gexiv2.pc
cargo install --git https://github.com/vmiklos/vmexam cap2exif
```

## Usage

### Writing exif metadata

Given captions.txt in the current directory:

```
test.jpg	Foo bar baz
```

When running:

```
cap2exif
```

Then it'll set captions so that tools like [Geeqie](https://www.geeqie.org/) or
[Memories](https://github.com/pulsejet/memories) can show those captions.

### Renaming files

When the `-r` option is used, the tool names images (.jpg or .JPG extension) based on exif data, so
that multiple DSC0001.jpg can be placed into a single directory, without manual renaming.
