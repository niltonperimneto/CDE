# XmGetScaledPixmap
library call`XmGetScaledPixmap`read a pixmap file and scale it according to pixmap and print resolution#include <Xm/Xm.h>XtEnum`XmGetScaledPixmap`WidgetwidgetStringimage_namePixelforegroundPixelbackgroundintdepthDoublescaling_ratioDESCRIPTIONXmGetScaledPixmapuses itsWidgetargument to look up
for a Print Shell ancestor to get the pixmap resolution
and the default printer resolution information to be used
ifscaling_ratio==0.If scaling is 0, and a valid PrintShell is presentXmGetScaledPixmapapplies a ratio equals to
(printer resolution / default pixmap resolution)
before creating the Pixmap on the widget's Screen.
Otherwise, thescaling_ratiois used in scaling
both dimensions of the image being converted as a Pixmap.XmGetScaledPixmapcompletes theXmGetPixmapByDepthexisting
API by making use of theXmNdefaultPixmapResolutionof the rootingXmPrintShell.
Refer to theXmGetPixmapByDepthdocumentation for details.`widget`Widget used to determine the default pixmap resolution (of the print shell ancestor).`image_name`See XmGetPixmapByDepth for description.`foreground`See XmGetPixmapByDepth for description.`background`See XmGetPixmapByDepth for description.`depth`See XmGetPixmapByDepth for description.`scaling_ratio`Indicate the scaling ratio to be applied, or 0.RETURN VALUEReturns Pixmap or NULL if failed.ERRORS/WARNINGSSame as forXmGetPixmapByDepth.SEE ALSO&cdeman.XmPrintSetup;, &cdeman.XmPrintShell;, &cdeman.XmRedisplayWidget;