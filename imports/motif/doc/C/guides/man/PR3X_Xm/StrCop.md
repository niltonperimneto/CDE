# XmStringCopy
library call`XmStringCopy`A compound string function that makes a copy of a stringXmStringCopycompound string functionsXmStringCopy#include <Xm/Xm.h>XmString`XmStringCopy`XmStrings1
## DESCRIPTION


`XmStringCopy`makes a copy of an existing compound string.
When the application no longer needs the returned compound string,
the application should call`XmStringFree`.

* **`s1`** 

Specifies the compound string to be copied

## RETURN


Returns a compound string.
## RELATED


&cdeman.XmStringCreate; and &cdeman.XmStringFree;.