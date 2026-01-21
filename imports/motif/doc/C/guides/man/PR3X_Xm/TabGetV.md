# XmTabGetValues
library call`XmTabGetValues`A convenience function that returns tab valuesXmTabGetValues#include <Xm/Xm.h>float`XmTabGetValues`XmTabtabunsigned char*unitsXmOffsetModel*offsetunsigned char*alignmentchar**decimal
## DESCRIPTION


`XmTabGetValues`takes anXmTabstructure, returns the
floating point number that is set as the value of the tab, and then
sets values for the`units`,`offset`,`alignment`, and`decimal`arguments where they are not NULL. The returned floating
point number represents the distance that the rendering of theXmStringsegment associated with`tab`will be offset. The
offset is from either the
beginning of the rendering or from the previous tab stop, depending on
the setting for the`offset`model. The
distance will use the unit type pointed at by`unit`.

* **`tab`** 

Specifies the tab to get the value from.
* **`units`** 

Specifies a pointer to the unit type.
* **`offset`** 

Specifies a pointer to the offset model.
* **`alignment`** 

Specifies a pointer to the alignment type.
* **`decimal`** 

Specifies a pointer to the multibyte character used as the decimal point.

## RETURN


Returns a floating point number that is set as the value of the tab.
## RELATED


&cdeman.XmTab;.