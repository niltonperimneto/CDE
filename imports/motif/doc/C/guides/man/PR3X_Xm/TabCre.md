# XmTabCreate
library call`XmTabCreate`A convenience function that creates a tab stopXmTabCreate#include <Xm/Xm.h>XmTab`XmTabCreate`floatvalueunsigned charunitsXmOffsetModeloffset_modelunsigned charalignmentchar*decimal
## DESCRIPTION


`XmTabCreate`creates a tab stop at a position defined by the`value`and`units`arguments.

* **`value`** 

Specifies the floating point value to be used in conjunction
with`units`to calculate the location of the tab stop.
Note that negative values are not permitted.
* **`units`** 

Specifies the unit type (for example,`XmMILLIMETERS`)
to be used in conjunction with`value`to calculate the location of the tab stop. You can specify
any unit described by the`XmConvertUnits`reference page.
For resources of type, dimension, or position, you can specify units
as described in the`XmNunitType`resource of the`XmGadget`,`XmManager`, or`XmPrimitive`reference page.
* **`offset_model`** 

Specifies whether the tab value represents an absolute position or a
relative offset from the previous tab. Valid values are`XmABSOLUTE`and`XmRELATIVE`.
* **`alignment`** 

Specifies how the text should be aligned relative to this tab stop.
Valid values are`XmALIGNMENT_BEGINNING`.
* **`decimal`** 

Specifies the multibyte character in the current language environment
to be used as the decimal point for a decimal aligned tab stop. This
is currently unused.

## RETURN


Returns a newly allocatedXmTab.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XmTabFree`.
## RELATED


&cdeman.XmTab; and
&cdeman.XmTabFree;.