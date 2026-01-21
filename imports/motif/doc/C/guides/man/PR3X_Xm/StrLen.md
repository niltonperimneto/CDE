# XmStringLength
library call`XmStringLength`A compound string function that obtains the length of a compound stringXmStringLengthcompound string functionsXmStringLength#include <Xm/Xm.h>int`XmStringLength`XmStrings1
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases. It is replaced by`XmStringByteStreamLength`.`XmStringLength`obtains the length of a compound string. It returns the number
of bytes in`s1`including all tags, direction indicators, and
separators. If the compound string has an invalid structure, 0 (zero) is
returned.

* **`s1`** 

Specifies the compound string

## RETURN


Returns the length of the compound string.
## RELATED


&cdeman.XmStringByteStreamLength; and
&cdeman.XmStringCreate;.