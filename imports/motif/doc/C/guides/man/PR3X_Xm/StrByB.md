# XmStringByteStreamLength
library call`XmStringByteStreamLength`A function that returns the size of a stringXmStringByteStreamLength#include <Xm/Xm.h>
unsigned int XmStringByteStreamLength (string)
        unsigned char   *string;
## DESCRIPTION


`XmStringByteStreamLength`receives a byte stream format string
and returns the size, in bytes, of that stream, including the header.
Because of this header information, even a NULL`string`will cause`XmStringByteStreamLength`to return a non-zero value.

* **`string`** 

Specifies the byte stream format string.

## RETURN VALUES


Returns the size of`string`, including the header.