# XmGetColors
library call`XmGetColors`A function that generates foreground, select, and shadow colorsXmGetColorsColor functionsXmGetColors#include <Xm/Xm.h>void`XmGetColors`Screen* screenColormapcolormapPixelbackgroundPixel* foregroundPixel* top_shadowPixel* bottom_shadowPixel* select
## DESCRIPTION


`XmGetColors`takes a screen, a colormap, and a background pixel,
and returns pixel values for foreground, select, and shadow colors.

* **`screen`** 

Specifies the screen for which these colors should be allocated.
* **`colormap`** 

Specifies the colormap from which these colors should be allocated.
* **`background`** 

Specifies the background on which the colors should be based.
* **`foreground`** 

Specifies a pointer to the returned foreground pixel value.
If this argument is NULL, no value is allocated or returned for this color.
* **`top_shadow`** 

Specifies a pointer to the returned top shadow pixel value.
If this argument is NULL, no value is allocated or returned for this color.
* **`bottom_shadow`** 

Specifies a pointer to the returned bottom shadow pixel value.
If this argument is NULL, no value is allocated or returned for this color.
* **`select`** 

Specifies a pointer to the returned select pixel value.
If this argument is NULL, no value is allocated or returned for this color.

## RELATED


&cdeman.XmChangeColor;,
&cdeman.XmGetColorCalculation;, and
&cdeman.XmSetColorCalculation;.