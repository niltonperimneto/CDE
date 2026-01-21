# XmStringConcat
library call`XmStringConcat`A compound string function that appends one string to anotherXmStringConcatcompound string functionsXmStringConcat#include <Xm/Xm.h>XmString`XmStringConcat`XmStrings1XmStrings2
## DESCRIPTION


`XmStringConcat`copies`s2`to the end of`s1`and returns
a copy of the resulting compound string. The original strings are preserved.
The function will allocate space to hold the returned compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **`s1`** 

Specifies the compound string to which a copy of`s2`is appended
* **`s2`** 

Specifies the compound string that is appended to the end of`s1`

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate; and &cdeman.XmStringFree;.