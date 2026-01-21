# XmGetPixmapByDepth
library call`XmGetPixmapByDepth`A pixmap caching function that generates a pixmap, stores it in a pixmap cache, and returns the pixmapXmGetPixmapByDepthpixmaps#include <Xm/Xm.h>Pixmap`XmGetPixmapByDepth`Screen *screenchar *image_namePixelforegroundPixelbackgroundintdepth
## DESCRIPTION


`XmGetPixmapByDepth`uses the parameter data to perform a lookup in the
pixmap cache to see if a pixmap has already been generated that
matches the data. If one is found, a reference count is incremented
and the pixmap is returned. Applications should use`XmDestroyPixmap`when the pixmap is no longer needed.

* **`screen`** 

Specifies the display screen on which the pixmap is to
be drawn
* **`image_name`** 

Specifies the name of the image to be used to
generate the pixmap
* **`foreground`** 

Combines the image with the`foreground`color to create the pixmap
if the image referenced is a bit-per-pixel image
* **`background`** 

Combines the image with the`background`color to create the pixmap
if the image referenced is a bit-per-pixel image
* **`depth`** 

Specifies the depth of the pixmap


If a matching pixmap is not found,`image_name`is used to perform a lookup in
the image cache. If an image is found, it is used to generate the pixmap,
which is then cached and returned.

If an image is not found,`image_name`is used as a filename, and a search is made for
an`X10`or`X11`bitmap file. If it is found, the file is
read, converted into an image, and cached in the image cache. The image
is then used to generate a pixmap, which is cached and returned.

If`image_name`has a leading / (slash), it specifies a full
pathname, and`XmGetPixmapByDepth`opens the file as specified.
Otherwise,`image_name`specifies a filename.
In this case,`XmGetPixmapByDepth`looks for the file along a search
path specified by the`XBMLANGPATH`environment variable or by a
default search path, which varies depending on whether or not the`XAPPLRESDIR`environment variable is set.
The default search path contains a lot of directories.
Therefore,`XmGetPixmapByDepth`will need a relatively
long time to search through all these directories for pixmaps
and bitmaps. Applications that use a lot of pixmaps and bitmaps
will probably run more quickly if`XBMLANGPATH`is set to a short list of directories.
In addition to X bitmap files (XBM), Motif also supports XPM (X
Pixmap) file formats. The`XBMLANGPATH`specifies the path for
both XBM and XPM files. XPM files are described in more detail later
in this reference page.

The`XBMLANGPATH`environment variable specifies a search path
for X bitmap files.
It can contain the substitution field`%B`, where the`image_name`argument to`XmGetPixmapByDepth`is substituted for`%B`.
It can also contain the substitution fields accepted by`XtResolvePathname`.
The substitution field`%T`is always mapped to`bitmaps`, and`%S`is
always mapped to NULL.

If`XBMLANGPATH`is not set, but the environment variable`XAPPLRESDIR`is set, the following pathnames are searched:

%B

$XAPPLRESDIR/%L/bitmaps/%N/%B

$XAPPLRESDIR/%l_%t/bitmaps/%N/%B

$XAPPLRESDIR/%l/bitmaps/%N/%B

$XAPPLRESDIR/bitmaps/%N/%B

$XAPPLRESDIR/%L/bitmaps/%B

$XAPPLRESDIR/%l_%t/bitmaps/%B

$XAPPLRESDIR/%l/bitmaps/%B

$XAPPLRESDIR/bitmaps/%B

$HOME/bitmaps/%B

$HOME/%B

/usr/lib/X11/%L/bitmaps/%N/%B

/usr/lib/X11/%l_%t/bitmaps/%N/%B

/usr/lib/X11/%l/bitmaps/%N/%B

/usr/lib/X11/bitmaps/%N/%B

/usr/lib/X11/%L/bitmaps/%B

/usr/lib/X11/%l_%t/bitmaps/%B

/usr/lib/X11/%l/bitmaps/%B

/usr/lib/X11/bitmaps/%B

/usr/include/X11/bitmaps/%B

If neither`XBMLANGPATH`nor`XAPPLRESDIR`is set, the
following pathnames are searched:

%B

$HOME/%L/bitmaps/%N/%B

$HOME/%l_%t/bitmaps/%N/%B

$HOME/%l/bitmaps/%N/%B

$HOME/bitmaps/%N/%B

$HOME/%L/bitmaps/%B

$HOME/%l_%t/bitmaps/%B

$HOME/%l/bitmaps/%B

$HOME/bitmaps/%B

$HOME/%B

/usr/lib/X11/%L/bitmaps/%N/%B

/usr/lib/X11/%l_%t/bitmaps/%N/%B

/usr/lib/X11/%l/bitmaps/%N/%B

/usr/lib/X11/bitmaps/%N/%B

/usr/lib/X11/%L/bitmaps/%B

/usr/lib/X11/%l_%t/bitmaps/%B

/usr/lib/X11/%l/bitmaps/%B

/usr/lib/X11/bitmaps/%B

/usr/include/X11/bitmaps/%B

These paths are defaults that vendors may change.
For example, a vendor may use different directories for/usr/lib/X11and/usr/include/X11.

The following substitutions are used in these paths:

* **`%B`** 

The image name, from the`image_name`argument
* **`%N`** 

The class name of the application
* **`%L`** 

The display's language string.
This string is influenced by`XtSetLanguageProc`.
The default string is determined by
callingsetlocale(`LC_ALL, NULL`).
* **`%l_%t`** 

The language and territory component of the display's language string
* **`%l`** 

The language component of the display's language string


The contents of the file must conform to the rules for
X11 bitmap files. In other words, Motif can read any X11
conformant bitmap file.

The XPM file format is used for
storing or getting back colored X pixmaps from files. The XPM library
is provided as unsupported with Motif. To build applications without
XPM, use the`NO_XPM`macro.
The following shows both XBM and XPM files, respectively,
for a plaid pattern./* XBM file */
&npzwc;#define plaid_width 22
&npzwc;#define plaid_height 22
&npzwc;#define plaid_x_hot -1
&npzwc;#define plaid_y_hot -1
static char plaid_bits[] = {
   0x75, 0xfd, 0x3f, 0xaa, 0xfa, 0x3e, 0x75, 0xfd, 0x3f, 0xaa, 0xfa, 0x3e,
   0x75, 0xfd, 0x3f, 0xff, 0x57, 0x15, 0x75, 0xfd, 0x3f, 0xaa, 0xfa, 0x3e,
   0x75, 0xfd, 0x3f, 0xaa, 0xfa, 0x3e, 0x75, 0xfd, 0x3f, 0x20, 0xa8, 0x2b,
   0x20, 0x50, 0x15, 0x20, 0xa8, 0x2b, 0x20, 0x50, 0x15, 0x20, 0xa8, 0x2b,
   0xff, 0xff, 0x3f, 0x20, 0xa8, 0x2b, 0x20, 0x50, 0x15, 0x20, 0xa8, 0x2b,
   0x20, 0x50, 0x15, 0x20, 0xa8, 0x2b};/* XPM file */
static char * plaid[] = {
/* plaid pixmap
 * width height ncolors chars_per_pixel */
"22 22 4 2 ",
/* colors */
"   c red       m white  s light_color ",
"Y  c green     m black  s lines_in_mix ",
"+  c yellow    m white  s lines_in_dark ",
"x              m black  s dark_color ",
/* pixels */
"x   x   x x x   x   x x x x x x + x x x x x ",
"  x   x   x   x   x   x x x x x x x x x x x ",
"x   x   x x x   x   x x x x x x + x x x x x ",
"  x   x   x   x   x   x x x x x x x x x x x ",
"x   x   x x x   x   x x x x x x + x x x x x ",
"Y Y Y Y Y x Y Y Y Y Y + x + x + x + x + x + ",
"x   x   x x x   x   x x x x x x + x x x x x ",
"  x   x   x   x   x   x x x x x x x x x x x ",
"x   x   x x x   x   x x x x x x + x x x x x ",
"  x   x   x   x   x   x x x x x x x x x x x ",
"x   x   x x x   x   x x x x x x + x x x x x ",
"          x           x   x   x Y x   x   x ",
"          x             x   x   Y   x   x   ",
"          x           x   x   x Y x   x   x ",
"x x x x x x x x x x x x x x x x x x x x x x ",
"          x           x   x   x Y x   x   x ",
"          x             x   x   Y   x   x   ",
"          x           x   x   x Y x   x   x ",
"          x             x   x   Y   x   x   ",
"          x           x   x   x Y x   x   x "
};
## RETURN


Returns a pixmap when successful; returns`XmUNSPECIFIED_PIXMAP`if the image corresponding to`image_name`cannot be found.
## RELATED


&cdeman.XmDestroyPixmap;,
&cdeman.XmInstallImage;, and
&cdeman.XmUninstallImage;.