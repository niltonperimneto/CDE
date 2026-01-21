# XmUninstallImage
library call`XmUninstallImage`A pixmap caching function that removes an image from the image cacheXmUninstallImagepixmaps#include <Xm/Xm.h>Boolean`XmUninstallImage`XImage* image
## DESCRIPTION


`XmUninstallImage`removes an image from the image cache.

* **`image`** 

Points to the image structure given to theXmInstallImage()routine

## RETURN


Returns True when successful; returns False if the`image`is
NULL, or if it cannot be found to be uninstalled.
## RELATED


&cdeman.XmInstallImage;, &cdeman.XmGetPixmap;, and &cdeman.XmDestroyPixmap;.