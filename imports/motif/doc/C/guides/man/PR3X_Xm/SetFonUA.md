# XmSetFontUnit
library call`XmSetFontUnit`A function that sets the font unit value for a displayXmSetFontUnit#include <Xm/Xm.h>void`XmSetFontUnit`Display* displayintfont_unit_value
## DESCRIPTION


`XmSetFontUnit`provides an external function to initialize font unit
values. Applications may want to specify resolution-independent data based
on a global font size.
See the`XmNunitType`resource description in the reference pages for`XmGadget`,`XmManager`, and`XmPrimitive`for more information
on resolution independence.

This function sets the font units for all screens on the display.

`NOTE:``XmSetFontUnit`is obsolete and exists for compatibility with
previous releases.
Instead of using this function, provide initial values or call`XtSetValues`for the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.

* **`display`** 

Defines the display for which this font unit value is to be applied.
* **`font_unit_value`** 

Specifies the value to be used for both horizontal and vertical font
units in the conversion calculations.

## RELATED


&cdeman.XmConvertUnits;,
&cdeman.XmSetFontUnits;,
&cdeman.XmGadget;,
&cdeman.XmManager;,
&cdeman.XmPrimitive;, and
&cdeman.XmScreen;.