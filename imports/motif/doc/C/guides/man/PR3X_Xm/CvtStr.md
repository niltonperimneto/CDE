# XmCvtStringToUnitType
library call`XmCvtStringToUnitType`A function that converts a string to a unit-type valueXmCvtStringToUnitType#include <Xm/Xm.h>void`XmCvtStringToUnitType`XrmValuePtrargsCardinal* num_argsXrmValue* from_valXrmValue* to_val
## DESCRIPTION


`XmCvtStringToUnitType`converts a string to a unit type. Refer to
the reference pages for`XmGadget`,`XmManager`, or`XmPrimitive`for a
description of the valid unit types.
Use of this function as a resource converter is obsolete. It has been replaced
by a new resource converter that uses the RepType facility.

* **`args`** 

Specifies a list of additional`XrmValue`arguments to the converter
if additional context is needed to perform the conversion. For example,
the string-to-font converter needs the widget's screen and the
string-to-pixel converter needs the widget's screen and color map. This
argument is often NULL.
* **`num_args`** 

Specifies the number of additional`XrmValue`arguments. This argument
is often zero.
* **`from_val`** 

Specifies the value to convert
* **`to_val`** 

Specifies the descriptor to use to return the converted value

## RELATED


&cdeman.XmGadget;, &cdeman.XmManager;, and &cdeman.XmPrimitive;.