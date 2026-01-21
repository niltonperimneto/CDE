# XmCvtByteStreamToXmString
library call`XmCvtByteStreamToXmString`A compound string function that converts from a compound string in Byte Stream format to a compound stringXmCvtByteStreamToXmStringcompound string functionsXmCvtByteStreamToXmString#include <Xm/Xm.h>XmString`XmCvtByteStreamToXmString`unsigned char *property
## DESCRIPTION


`XmCvtByteStreamToXmString`converts a stream of bytes representing a
compound string in Byte Stream format to a compound string.
This routine is typically used by the destination of a data transfer
operation to produce a compound string from a transferred Byte Stream
representation.

* **`property`** 

Specifies a compound string representation in Byte Stream format.

## RETURN


Returns a compound string.
The function allocates space to hold the returned compound string.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XmStringFree`.
## RELATED


&cdeman.XmString;,
&cdeman.XmCvtXmStringToByteStream;, and
&cdeman.XmStringFree;.