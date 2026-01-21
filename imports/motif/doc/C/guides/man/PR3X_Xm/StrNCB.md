# XmStringNCopy
library call`XmStringNCopy`A compound string function that creates a copy of a compound stringXmStringNCopycompound string functionsXmStringNCopy#include <Xm/Xm.h>XmString`XmStringNCopy`XmStrings1intnum_bytes
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases.`XmStringNCopy`creates a copy of`s1`that contains a specified
number of bytes, including tags, directional indicators,
and separators. It then returns the
resulting compound string. The original strings are preserved.
The function allocates space for the resulting compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **`s1`** 

Specifies the compound string.
* **`num_bytes`** 

Specifies the number of bytes of`s1`to copy.
If this value is less than the
length of`s1`, as many bytes as possible, but possibly fewer than
this value, will be appended to`s1`such that the resulting string
is still a valid compound string.

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate; and &cdeman.XmStringFree;.