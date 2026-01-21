# XmDirectionMatch
library call`XmDirectionMatch`A function that checks for a specified direction componentXmDirectionMatch#include <Xm/Xm.h>
Boolean XmDirectionMatch (d1, d2)
        XmDirectiond1;
        XmDirectiond2;
## DESCRIPTION


`XmDirectionMatch`compares twoXmDirectionvalues.
The function returns a Boolean value depending on whether or
not the two input values "match."
The simplest match is when`d1`and`d2`are identical.
However, other matches are possible.`XmDirectionMatch`attempts to compare specified bits only;
nonspecified bits automatically match.

For example, suppose that`d1`equals`XmTOP_TO_BOTTOM_RIGHT_TO_LEFT`. In this case, the function will
return True if`d2`equals either`XmRIGHT_TO_LEFT`or`XmTOP_TO_BOTTOM`. However, the function will return False if`d2`equals`XmTOP_TO_BOTTOM_LEFT_TO_RIGHT`,`XmBOTTOM_TO_TOP_RIGHT_TO_LEFT`, or`XmBOTTOM_TO_TOP_LEFT_TO_RIGHT`.

Note that direction can be thought of as having three components, a
horizontal component, a vertical component, and the precedence among
them. This means that in addition to the previously mentioned
directions, the function will still return False if`d1`equals`XmTOP_TO_BOTTOM_RIGHT_TO_LEFT`and`d2`equals`XmRIGHT_TO_LEFT_TOP_TO_BOTTOM`.

* **`d1`** 

Specifies anXmDirectionvalue.
* **`d2`** 

Specifies anXmDirectionvalue.

## RETURN VALUES


Returns True if`d1`"matches"`d2`; otherwise, returns False.
## RELATED INFORMATION


&cdeman.XmDirection;,
&cdeman.XmDirectionMatchPartial;,
&cdeman.XmDirectionToStringDirection;, &cdeman.XmString;,
&cdeman.XmStringDirection;, and
&cdeman.XmStringDirectionToDirection;.