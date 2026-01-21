# XmStringConcatAndFree
library call`XmStringConcatAndFree`A compound string function that appends one string to another and frees the original stringsXmStringConcatAndFreecompound string functionsXmStringConcatAndFree#include <Xm/Xm.h>XmString`XmStringConcatAndFree`XmStrings1XmStrings2
## DESCRIPTION


`XmStringConcatAndFree`copies`s2`to the end of`s1`and returns
a copy of the resulting compound string. The original strings are freed.
The function will allocate space to hold the returned compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **`s1`** 

Specifies the compound string to which a copy of`s2`is appended
* **`s2`** 

Specifies the compound string that is appended to the end of`s1`


The`XmStringConcatAndFree`function works like the`XmStringConcat`function, except that it frees the`s1`and`s2`strings, and is therefore more efficient. You should use`XmStringConcatAndFree`instead of`XmStringConcat`if you
want`s1`and`s2`to be freed afterwards.
## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringConcat;,
&cdeman.XmStringCreate;, and
&cdeman.XmStringFree;.