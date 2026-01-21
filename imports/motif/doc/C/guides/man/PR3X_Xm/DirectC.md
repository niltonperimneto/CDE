# XmDirectionMatchPartial
library call`XmDirectionMatchPartial`A function that checks for a specified direction componentXmDirectionMatchPartial#include <Xm/Xm.h>
Boolean XmDirectionMatchPartial (d1, d2, dmask)
        XmDirectiond1;
        XmDirectiond2;
        XmDirectiondmask;
## DESCRIPTION


`XmDirectionMatchPartial`compares`d1`and`d2`along the
direction component specified by`dmask`.
For example, if`dmask`equals`XmVERTICAL_MASK`, then
the function will compare only the vertical components of`d1`and`d2`.

* **`d1`** 

Specifies anXmDirectionvalue to check.
* **`d2`** 

Specifies anXmDirectionvalue to check.
* **`dmask`** 

Specifies the direction component along which`d1`and`d2`are to be checked. Appropriate values for`dmask`are`XmHORIZONTAL_MASK`,`XmVERTICAL_MASK`,
and`XmPRECEDENCE_MASK`.

## RETURN VALUES


Returns True if the`d1`and`d2`match in the component
specified by`dmask`; otherwise, returns False.
## RELATED INFORMATION


&cdeman.XmDirection;, &cdeman.XmDirectionMatch;,
&cdeman.XmDirectionToStringDirection;,
&cdeman.XmStringDirection;, and &cdeman.XmStringDirectionToDirection;.