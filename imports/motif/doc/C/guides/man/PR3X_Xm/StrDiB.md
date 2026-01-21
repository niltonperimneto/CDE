# XmStringDirectionCreate
library call`XmStringDirectionCreate`A compound string function that creates a compound stringXmStringDirectionCreatecompound string functionsXmStringDirectionCreate#include <Xm/Xm.h>XmString`XmStringDirectionCreate`XmStringDirectiondirection
## DESCRIPTION


`XmStringDirectionCreate`creates a compound string with a single component,
a direction with the given value.
When the application no longer needs the returned compound string, the
application should call`XmStringFree`.

* **direction** 

Specifies the value of the direction component.
The possible values are:

* **`XmSTRING_DIRECTION_L_TO_R`** 

Specifies left to right display.
* **`XmSTRING_DIRECTION_R_TO_L`** 

Specifies right to left display.
* **`XmSTRING_DIRECTION_DEFAULT`** 

Specifies that the display direction will be set by the widget in
which the compound string is to be displayed.


## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate;.