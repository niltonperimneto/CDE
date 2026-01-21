# XmStringHasSubstring
library call`XmStringHasSubstring`A compound string function that indicates whether one compound string is contained within anotherXmStringHasSubstringcompound string functionsXmStringHasSubstring#include <Xm/Xm.h>Boolean`XmStringHasSubstring`XmStringstringXmStringsubstring
## DESCRIPTION


`XmStringHasSubstring`indicates whether or not one compound string is contained within
another.

* **`string`** 

Specifies the compound string to be searched
* **`substring`** 

Specifies the compound string to be searched for

## RETURN


Returns True if`substring`has a single text component and if its text is completely contained within any
single text component of`string`; otherwise, it returns False.
## RELATED


&cdeman.XmStringCreate; and
&cdeman.XmStringCreateLocalized;.