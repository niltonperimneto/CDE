# XmDestroyPixmap
library call`XmDestroyPixmap`A pixmap caching function that removes a pixmap from the pixmap cacheXmDestroyPixmappixmaps#include <Xm/Xm.h>Boolean`XmDestroyPixmap`Screen* screenPixmappixmap
## DESCRIPTION


`XmDestroyPixmap`removes pixmaps that are no longer used.
Pixmaps are completely freed only when there is no further reference to them.

* **`screen`** 

Specifies the display screen for which the pixmap was requested
* **`pixmap`** 

Specifies the pixmap to be destroyed

## RETURN


Returns True when successful; returns False if there is no matching screen
and pixmap in the pixmap cache.
## RELATED


&cdeman.XmInstallImage;, &cdeman.XmUninstallImage;, and
&cdeman.XmGetPixmap;.