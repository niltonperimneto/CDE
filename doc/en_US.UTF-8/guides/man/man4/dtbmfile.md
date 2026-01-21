# dtbmfile
special filedtbmfileformat of X11 bitmap format desktop icon files#define`name`_width`width`#define`name`_height`height`#define`name`_x_hot`x`#define`name`_y_hot`y`static unsigned char`name`_bits[] = {0xN,...}
## DESCRIPTION


X Bitmap (XBM) files are used for monochrome icons in the CDE.
XBM icons specify background and foreground
color only.
They would be black and white on a black and white
display.
On a color display, they could be any two colors.
XBM files are not used exclusively for black and white icons.
The CDE icon editor component can be used to create or modify a
bitmap icon.
A bitmap file is an ASCII file, but because the data is
simply a binary representation of the icon, it is not realistic
to edit this file by hand.
X Bitmap files can be included directly
in C source code, so the file data is in the form of a bitmap
structure variable.
X version 11 bitmap file format is the only icon file format
officially supported as part of the X Window System at this time.
## RETURN VALUE


None.
## EXAMPLES


The following is an example of a bitmap file named`julie.bm`:#define julie.bm_width 18
#define julie.bm_height 22
#define julie.bm_x_hot 0
#define julie.bm_y_hot 0
static char julie.bm_bits[] = {
   0xff, 0xff, 0x03, 0x81, 0xc3, 0x02,
   0xf1, 0x98, 0x03, 0x19, 0x18, 0x02,
   0x05, 0x59, 0x02, 0x07, 0xdb, 0x02,
   0x01, 0xdb, 0x02, 0x01, 0xdb, 0x02,
   0x01, 0xdb, 0x02, 0x01, 0xdf, 0x02,
   0x01, 0xff, 0x02, 0x01, 0xfe, 0x02,
   0x01, 0x7c, 0x02, 0x01, 0x38, 0x02,
   0x01, 0x38, 0x02, 0x01, 0x38, 0x02,
   0x01, 0x38, 0x02, 0x01, 0x3f, 0x02,
   0xe1, 0x33, 0x02, 0xb9, 0x31, 0x02,
   0xcf, 0x1c, 0x02, 0xff, 0xff, 0x03};
## APPLICATION USAGE


For a description of icon location, usage, design, etc. within the
CDE, refer to &cdeman.dticonfile;.
## SEE ALSO


`X11`Window`System`documentation, &cdeman.dtpmfile;, &cdeman.dticonfile;, &cdeman.dticon;.