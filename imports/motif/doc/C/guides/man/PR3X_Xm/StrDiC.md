# XmStringDirectionToDirection
library call`XmStringDirectionToDirection`A function that converts from XmStringDirection to XmDirectionXmStringDirectionToDirection#include <Xm/Xm.h>XmDirection`XmStringDirectionToDirection`XmStringDirectiondirection
## DESCRIPTION


`XmStringDirectionToDirection`converts the specifiedXmStringDirectiondirection value to its equivalentXmDirectionvalue. This
function provides backward compatibility with theXmStringDirectiondata type.

* **direction** 

Specifies theXmStringDirectionvalue to be converted.

## RETURN


Returns the followingXmDirectionvalues:

* **`XmLEFT_TO_RIGHT`** 

If thedirectionargument is`XmSTRING_DIRECTION_L_TO_R`.
* **`XmRIGHT_TO_LEFT`** 

If thedirectionargument is`XmSTRING_DIRECTION_R_TO_L`.
* **`XmDEFAULT_DIRECTION`** 

If thedirectionargument was not either of the above.

## RELATED INFORMATION


&cdeman.XmStringDirection; and &cdeman.XmDirection;.