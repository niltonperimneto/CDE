# XmInstallImage
library call`XmInstallImage`A pixmap caching function that adds an image to the image cacheXmInstallImagepixmaps#include <Xm/Xm.h>Boolean`XmInstallImage`XImage* imagechar* image_name
## DESCRIPTION


`XmInstallImage`stores an image in an image cache
that can later be used to generate a pixmap.
Part of the installation process is to extend the
resource converter used to reference these images. The resource converter
is given the image name so that the image can be
referenced in a.Xdefaultsfile.
Since an image can be referenced by a widget
through its pixmap resources, it is up to the application to ensure that the
image is installed before the widget is created.

* **`image`** 

Points to the image structure to be installed.
The installation process does not make a local copy of the image. Therefore,
the application should not destroy the image until it is uninstalled from
the caching functions.
* **`image_name`** 

Specifies a string that the application uses to name the image. After
installation, this name can be used in.Xdefaultsfor referencing
the image. A local copy of the name is created by the image caching
functions.


The image caching functions provide a set of eight preinstalled images.
These names can be used within a.Xdefaultsfile for generating pixmaps
for the resource for which they are provided.`Image Name``Description`backgroundA tile of solid background25_foregroundA tile of 25% foreground, 75% background50_foregroundA tile of 50% foreground, 50% background75_foregroundA tile of 75% foreground, 25% backgroundhorizontalA tile of horizontal lines of the two colorsverticalA tile of vertical lines of the two colorsslant_rightA tile of slanting lines of the two colorsslant_leftA tile of slanting lines of the two colorsmenu_cascadeA tile of an arrow of the foreground colormenu_checkmarkA tile of a checkmark of the foreground colormenu_dashA tile of one horizontal line of the foreground color
## RETURN


Returns True when successful; returns False if NULL`image`, NULL`image_name`, or duplicate`image_name`is used as a parameter
value.
## RELATED


&cdeman.XmUninstallImage;, &cdeman.XmGetPixmap;, and
&cdeman.XmDestroyPixmap;.