# XmConvertStringToUnits
library call`XmConvertStringToUnits`A function that converts a string specification to a unit valueXmConvertStringToUnits#include <Xm/Xm.h>int`XmConvertStringToUnits`Screen *screenStringspecintorientationintto_typeXtEnum *parse_error
## DESCRIPTION


`XmConvertStringToUnits`converts a string specification value and
returns the converted value as the return value from the function.
This function uses the specified screen's resolution to compute the
number of units for the string specification.

* **`screen`** 

Specifies the screen whose resolution is to be used for the computation.
* **`spec`** 

Specifies the string, in`<floating value><unit>`format, to be
converted.
* **`orientation`** 

Specifies whether the converter uses the horizontal or vertical screen
resolution when performing the conversion. The`orientation`parameter can have values of`XmHORIZONTAL`or`XmVERTICAL`.
* **`to_type`** 

Converts the value to the unit type specified.
Refer to the`XmNunitType`resource of the`XmGadget`,`XmManager`, or`XmPrimitive`reference page.
This parameter can have one of
the following values:

* **`XmPIXELS`** 

The returned value will be the number of pixels.
* **`XmMILLIMETERS`** 

The returned value will be the number of millimeters.
* **`Xm100TH_MILLIMETERS`** 

The returned values will be the number of 1/100 millimeters.
* **`XmCENTIMETERS`** 

The returned values will be the number of centimeters.
* **`XmINCHES`** 

The returned values will be the number of inches.
* **`Xm1000TH_INCHES`** 

The returned values will be the number of 1/100 inches.
* **`XmPOINTS`** 

The returned values will be the number of points.
A point is a text processing unit
defined as 1/72 of an inch.
* **`Xm100TH_POINTS`** 

The returned values will be the number of 1/100 points.
* **`XmFONT_UNITS`** 

All values provided to the widget are treated as font
units. A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.
* **`Xm100TH_FONT_UNITS`** 

All values provided to the widget are
treated as 1/100 of a font unit.
A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.

* **`parse_error`** 

Specifies if a parsing error occurred. This is set to a value of True
indicates that an error occurred, a value of False to indicate no error.

## RETURN


Returns the converted value.
If a NULL screen, incorrect`orientation`,
or incorrect`unit_type`is supplied as parameter data, or if a parsing error occurred, 0
(zero) is returned.
## RELATED


&cdeman.XmConvertUnits;, &cdeman.XmSetFontUnits;, and &cdeman.XmScreen;.