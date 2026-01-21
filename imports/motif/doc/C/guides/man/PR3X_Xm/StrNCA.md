# XmStringNConcat
library call`XmStringNConcat`A compound string function that appends a specified number of bytes to a compound stringXmStringNConcatcompound string functionsXmStringNConcat#include <Xm/Xm.h>XmString`XmStringNConcat`XmStrings1XmStrings2intnum_bytes
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases. It is replaced by`XmStringConcat`.`XmStringNConcat`appends a specified number of bytes from`s2`to
the end of`s1`, including tags, directional indicators, and
separators. It then returns the
resulting compound string. The original strings are preserved.
The function allocates space for the resulting compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **`s1`** 

Specifies the compound string to which a copy of`s2`is appended.
* **`s2`** 

Specifies the compound string that is appended to the end of`s1`.
* **`num_bytes`** 

Specifies the number of bytes of`s2`to append to`s1`.
If this value is less than the
length of`s2`, as many bytes as possible, but possibly fewer than
this value, will be appended to`s1`such that the resulting string
is still a valid compound string.

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate; and &cdeman.XmStringFree;.