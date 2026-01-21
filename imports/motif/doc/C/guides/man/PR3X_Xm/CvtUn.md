# XmConvertUnits
library call`XmConvertUnits`A function that converts a value in one unit type to another unit typeXmConvertUnits#include <Xm/Xm.h>int`XmConvertUnits`Widgetwidgetintorientationintfrom_unit_typeintfrom_valueintto_unit_type
## DESCRIPTION


`XmConvertUnits`converts the value and returns it as the return value from
the function.
For resources of type, dimension, or position, you can specify units
using the syntax described in the`XmNunitType`resource of the`XmPrimitive`reference page.

* **`widget`** 

Specifies the widget for which the data is to be converted.
* **`orientation`** 

Specifies whether the converter uses the horizontal or vertical screen
resolution when performing the conversions. The`orientation`parameter can have values of`XmHORIZONTAL`or`XmVERTICAL`.
* **`from_unit_type`** 

Specifies the current unit type of the supplied value
* **`from_value`** 

Specifies the value to be converted
* **`to_unit_type`** 

Converts the value to the unit type specified


The parameters`from_unit_type`and`to_unit_type`can have the
following values:

* **`XmPIXELS`** 

All values provided to the widget are treated as
pixel values. This
is the default for the resource.
* **`XmMILLIMETERS`** 

All values provided to the widget are treated as millimeter
values.
* **`Xm100TH_MILLIMETERS`** 

All values provided to the widget are
treated as 1/100 of a millimeter.
* **`XmCENTIMETERS`** 

All values provided to the widget are treated as centimeter
values.
* **`XmINCHES`** 

All values provided to the widget are treated as inch
values.
* **`Xm1000TH_INCHES`** 

All values provided to the widget are treated
as 1/1000 of an inch.
* **`XmPOINTS`** 

All values provided to the widget are treated as point
values. A point is a unit used in text processing
applications and is defined as 1/72 of an inch.
* **`Xm100TH_POINTS`** 

All values provided to the widget are treated as
1/100 of a point.
A point is a unit typically used in text processing applications and
is defined as 1/72 of an inch.
* **`XmFONT_UNITS`** 

All values provided to the widget are treated as normal font
units. A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.
* **`Xm100TH_FONT_UNITS`** 

All values provided to the widget are
treated as 1/100 of a font unit.
A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.

## RETURN


Returns the converted value.
If a NULL widget, incorrect`orientation`, or incorrect`unit_type`is supplied as parameter data, 0 (zero) is returned.
## RELATED


`XmPrimitive`,
&cdeman.XmSetFontUnits;,
and
&cdeman.XmScreen;.