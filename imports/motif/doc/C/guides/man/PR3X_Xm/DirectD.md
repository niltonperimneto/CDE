# XmDirectionToStringDirection
library call`XmDirectionToStringDirection`A function that converts an XmDirection value to an XmStringDirection valueXmDirectionToStringDirection#include <Xm/Xm.h>
XmStringDirection XmDirectionToStringDirection (dir)
        XmDirectiondir;
## DESCRIPTION


`XmDirectionToStringDirection`converts the specifiedXmDirectiondirection value to its equivalentXmStringDirectionvalue.
Basically, if theXmDirectionvalue has a horizontal direction
specification,
that horizontal element is used; otherwise, theXmStringDirectionvalue is interpreted as`XmSTRING_DIRECTION_L_TO_R`.
This function provides backward compatibility with theXmStringDirectiondata type.

Note that the Motif toolkit also contains an`XmStringDirectionToDirection`routine to convert anXmStringDirectionvalue to itsXmDirectionequivalent.

* **`dir`** 

Specifies theXmDirectionvalue to be converted.

## RETURN


Returns the followingXmStringDirectionvalues:

* **`XmSTRING_DIRECTION_R_TO_L`** 

If the`dir`argument has a right to left horizontal direction
value in it, for
example`XmRIGHT_TO_LEFT_TOP_TO_BOTTOM`.
* **`XmSTRING_DIRECTION_L_TO_R`** 

If the`dir`argument has a left to right horizontal direction in
it, for example`XmLEFT_TO_RIGHT_TOP_TO_BOTTOM`, or if the
horizontal direction value in the`dir`argument is ambiguous,
such as in the`XmTOP_TO_BOTTOM`value.
* **`XmSTRING_DIRECTION_DEFAULT`** 

If there was no horizontal direction specified.

## RELATED INFORMATION


&cdeman.XmDirection;, &cdeman.XmDirectionMatch;,
&cdeman.XmDirectionMatchPartial;,
&cdeman.XmDirectionToStringDirection;, &cdeman.XmString;,
&cdeman.XmStringDirection;, and
&cdeman.XmStringDirectionToDirection;,