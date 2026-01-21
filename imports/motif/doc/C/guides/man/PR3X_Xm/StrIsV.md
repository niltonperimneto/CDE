# XmStringIsVoid
library call`XmStringIsVoid`A compound string function that provides information on the existence of non-zero-length text components, tab components, or separator componentsXmStringIsVoidcompound string functionsXmStringIsVoid#include <Xm/Xm.h>Boolean`XmStringIsVoid`XmStrings1
## DESCRIPTION


`XmStringIsVoid`returns a Boolean value indicating whether
or not string`s1`is void.

* **`s1`** 

Specifies the compound string

## RETURN


Returns True if any non-zero-length
text components,
tab components,
or separator components
exist in`s1`.
That is, the function returns True if the string has
no text, tabs, or separators.
If`s1`contains the NULL string, the function returns True.
## RELATED


&cdeman.XmStringCreate;.