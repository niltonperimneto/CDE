# XmSetFontUnits
library call`XmSetFontUnits`A function that sets the font unit value for a displayXmSetFontUnits#include <Xm/Xm.h>void`XmSetFontUnits`Display* displayinth_valueintv_value
## DESCRIPTION


`XmSetFontUnits`provides an external function to initialize font unit
values. Applications may want to specify resolution-independent data based
on a global font size. This function must be called before any widgets with
resolution-independent data are created. See the`XmNunitType`resource description in the reference pages for`XmGadget`,`XmManager`, and`XmPrimitive`for more information
on resolution independence.

This function sets the font units for all screens on the display.

`NOTE:``XmSetFontUnits`is obsolete and exists for compatibility with
previous releases.
Instead of using this function, provide initial values or call`XtSetValues`for the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.

* **`display`** 

Defines the display for which this font unit value is to be applied.
* **`h_value`** 

Specifies the value to be used for horizontal
units in the conversion calculations.
* **`h_value`** 

Specifies the value to be used for vertical
units in the conversion calculations.

## RELATED


&cdeman.XmConvertUnits;,
&cdeman.XmSetFontUnit;,
&cdeman.XmGadget;,
&cdeman.XmManager;,
&cdeman.XmPrimitive;, and
&cdeman.XmScreen;.