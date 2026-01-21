# XmCvtXmStringToByteStream
library call`XmCvtXmStringToByteStream`A compound string function that converts a compound string to a Byte Stream formatXmCvtXmStringToByteStreamcompound string functionsXmCvtXmStringToByteStream#include <Xm/Xm.h>unsigned int`XmCvtXmStringToByteStream`XmStringstringunsigned char **prop_return
## DESCRIPTION


`XmCvtXmStringToByteStream`converts a compound string to a
string of
bytes
representing the compound string in Byte Stream format.
This routine is typically used by the source of a data transfer
operation to produce a Byte Stream representation for transferring a compound
string to a destination.

If`prop_return`is not NULL, this function creates a string of
characters in Byte Stream format and returns it in`prop_return`.
The function also returns the number of bytes in`prop_return`.
If`prop_return`is NULL, the function does not return the Byte
Stream format
string, but it does calculate and return the number of bytes that would
appear in the Byte Stream format string.

* **`string`** 

Specifies a compound string to be converted to Byte Stream format
* **`prop_return`** 

Specifies a pointer to a string in Byte Stream format that is created and
returned by this function.
If`prop_return`is NULL, no Byte Stream format string is returned.
When a Byte Stream format string is returned, the function allocates
space to hold it.
The application is responsible for managing this allocated space.
The application can recover the allocated space by calling`XtFree`.

## RETURN


Returns the number of bytes in the Byte Stream representation (whether or not
the Byte Stream representation is returned).
## RELATED


&cdeman.XmString; and
&cdeman.XmCvtByteStreamToXmString;.