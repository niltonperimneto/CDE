# XmGetPixmap
library call`XmGetPixmap`A pixmap caching function that generates a pixmap, stores it in a pixmap cache, and returns the pixmapXmGetPixmappixmaps#include <Xm/Xm.h>Pixmap`XmGetPixmap`Screen *screenchar *image_namePixelforegroundPixelbackground
## DESCRIPTION


`XmGetPixmap`uses the parameter data to perform a lookup in the
pixmap cache to see if a pixmap has already been generated that
matches the data. If one is found, a reference count is incremented
and the pixmap is returned. Applications should use`XmDestroyPixmap`when the pixmap is no longer needed.

* **`screen`** 

Specifies the display screen on which the pixmap is to
be drawn.
The depth of the pixmap is the default depth for this screen.
* **`image_name`** 

Specifies the name of the image to be used to
generate the pixmap
* **`foreground`** 

Combines the image with the`foreground`color to create the pixmap
if the image referenced is a bit-per-pixel image
* **`background`** 

Combines the image with the`background`color to create the pixmap
if the image referenced is a bit-per-pixel image


If a pixmap is not found,`image_name`is used to perform a lookup in
the image cache. If an image is found, it is used to generate the pixmap,
which is then cached and returned.

If an image is not found, the`image_name`is used as a filename, and a search is made for
an`X10`or`X11`bitmap file. If it is found, the file is
read, converted into an image, and cached in the image cache. The image
is then used to generate a pixmap, which is cached and returned.

If`image_name`has a leading slash (/), it specifies a full
pathname, and`XmGetPixmap`opens the file as specified.
Otherwise,`image_name`specifies a filename.
In this case,`XmGetPixmap`looks for the file along a search path
specified by the`XBMLANGPATH`environment variable or by a default
search path, which varies depending on whether or not the`XAPPLRESDIR`environment variable is set.
The default search path contains a lot of directories.
Therefore,`XmGetPixmap`will need a relatively
long time to search through all these directories for pixmaps
and bitmaps. Applications that use a lot of pixmaps and bitmaps
will probably run more quickly if`XBMLANGPATH`is set to a short list of directories.
In addition to X bitmap files (XBM), Motif also supports XPM (X
Pixmap) file formats. The`XBMLANGPATH`specifies the path for
both XBM and XPM files. Refer to the`XmGetPixmapByDepth`reference
page for further details.

The`XBMLANGPATH`environment variable specifies a search path
for X bitmap files.
It can contain the substitution field`%B`, where the`image_name`argument to`XmGetPixmap`is substituted for`%B`.
It can also contain the substitution fields accepted by`XtResolvePathname`.
The substitution field`%T`is always mapped to`bitmaps`, and`%S`is
always mapped to NULL.

If`XBMLANGPATH`is not set but the environment variable`XAPPLRESDIR`is set, the following pathnames are searched:

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
## RETURN


Returns a pixmap when successful; returns`XmUNSPECIFIED_PIXMAP`if the image corresponding to`image_name`cannot be found.
## RELATED


&cdeman.XmDestroyPixmap;,
&cdeman.XmGetPixmapByDepth;,
&cdeman.XmInstallImage;, and
&cdeman.XmUninstallImage;.