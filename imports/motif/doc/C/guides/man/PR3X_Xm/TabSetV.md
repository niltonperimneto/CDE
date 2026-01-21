# XmTabSetValue
library call`XmTabSetValue`A convenience function that sets a tab stopXmTabSetValue#include <Xm/Xm.h>void`XmTabSetValue`XmTabtabfloatvalue
## DESCRIPTION


`XmTabSetValue`sets the`value`field of theXmTabstructure associated with`tab`.

* **`tab`** 

Specifies the tab to set the value of.
* **`value`** 

Specifies the floating point number which represents the distance that
the rendering of theXmStringsegment associated with`tab`will be offset. The offset is from either the beginning of the
rendering or from the previous tab stop, depending on the setting for
the`offset`model. The distance depends on the tab's unit type.
Note that negative values are not permitted, and that if a tab stop
would cause text to overlap, the x position for the segment is set
immediately after the end of the previous segment.

## RELATED


See also the &MotifProgGd; for more information about tabs
and tab lists.
&cdeman.XmTab;.