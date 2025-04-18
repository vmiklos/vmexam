# pcal wrapper with image support

## Dependencies

Build pcal:

```
wget https://kumisystems.dl.sourceforge.net/project/pcal/pcal/pcal-4.11.0/pcal-4.11.0.tgz
tar xvf pcal-4.11.0.tgz
cd pcal-4.11.0/
cat ../../pcal-4.11.0-order.patch |patch -p1
make -j8
cp exec/pcal ~/bin/
```

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam pdfcal
```

## Usage

Place images in images/ as 01.jpg, 02.jpg, ..., then run:

```
pdfcal
```

and the result will be produced as out.pdf. The file is quite large as it doesn't scale down input
photo images. A print optimized version can be produced by running:

```
gs -sDEVICE=pdfwrite -dCompatibilityLevel=1.4 -dPDFSETTINGS=/printer -dNOPAUSE -dQUIET -dBATCH -sOutputFile=out.printer.pdf out.pdf
```

## Workarounds

### Bad JPEG

If an image metadata causes a problem, run:

```
mogrify -strip ./01.jpg
```

to strip the unwanted metadata.

### HEIC

```
heif-convert foo.heic foo.jpg
```

can do a conversion, then pdfcal can consume the image.
