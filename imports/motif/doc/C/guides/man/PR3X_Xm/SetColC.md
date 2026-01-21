# XmSetColorCalculation
library call`XmSetColorCalculation`A function to set the procedure used for default color calculationXmSetColorCalculationColor functionsXmSetColorCalculation#include <Xm/Xm.h>XmColorProc`XmSetColorCalculation`XmColorProccolor_proc
## DESCRIPTION


`XmSetColorCalculation`sets the procedure to calculate
default colors.
This procedure is used to calculate the foreground, top shadow, bottom
shadow, and select colors on the basis of a given background color.
If called with an argument of NULL, it restores the default procedure
used to calculate colors.

* **`color_proc`** 

Specifies the procedure to use for color calculation.


Following is a description of theXmColorProctype used by`XmSetColorCalculation`:void (*color_proc) (background_color, foreground_color, select_color, top_shadow_color,
bottom_shadow_color)
        XColor*background_color;
        XColor*foreground_color;
        XColor*select_color;
        XColor*top_shadow_color;
        XColor*bottom_shadow_color;

Specifies the procedure used to calculate default colors.
The procedure is passed a pointer to an`XColor`structure
representing the background color.
The`pixel`,`red`,`green`, and`blue`members of this
structure are filled in with values that are valid for the current
colormap.

The procedure is passed pointers to`XColor`structures representing
the foreground, select, top shadow, and bottom shadow colors to be
calculated.
The procedure calculates and fills in the`red`,`green`, and`blue`members of these structures.
The procedure should not allocate color cells for any of these colors.

* **`background_color`** 

Specifies the background color.
* **`foreground_color`** 

Specifies the foreground color to be calculated.
* **`select_color`** 

Specifies the select color to be calculated.
* **`top_shadow_color`** 

Specifies the top shadow color to be calculated.
* **`bottom_shadow_color`** 

Specifies the bottom shadow color to be calculated.

## RETURN


Returns the color calculation procedure that was used at the time this
routine was called.
## RELATED


&cdeman.XmChangeColor;,
&cdeman.XmGetColors;, and
&cdeman.XmGetColorCalculation;.