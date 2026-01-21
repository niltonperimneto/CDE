# XmStringEmpty
library call`XmStringEmpty`A compound string function that provides information on the existence of non-zero-length text componentsXmStringEmptycompound string functionsXmStringEmpty#include <Xm/Xm.h>Boolean`XmStringEmpty`XmStrings1
## DESCRIPTION


`XmStringEmpty`returns a Boolean value indicating whether any
non-zero-length text components exist in the provided compound string.
It returns True
if there are no text segments in the string. If this routine is passed
NULL as the string, it returns True.

* **`s1`** 

Specifies the compound string

## RETURN


Returns True if there are no text segments in the string.
If this routine is passed
NULL as the string, it returns True.
## RELATED


&cdeman.XmStringCreate;.