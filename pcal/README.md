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

Install Python dependencies:

```
zypper in python3-Pillow python3-PyPDF2 python3-img2pdf
```

(Or the equivalent of your linux package manager.)

## Usage

Place images in images/ as 01.jpg, 02.jpg, ..., then run:

```
./pdfcal.py
```

and the result will be produced as out.pdf.

If an image metadata causes a problem, run:

```
mogrify -strip ./01.jpg
```

to strip the unwanted metadata.
