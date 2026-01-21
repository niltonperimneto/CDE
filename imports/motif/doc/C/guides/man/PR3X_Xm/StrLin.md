# XmStringLineCount
library call`XmStringLineCount`A compound string function that returns the number of separators plus one in the provided compound stringXmStringLineCountcompound string functionsXmStringLineCount#include <Xm/Xm.h>int`XmStringLineCount`XmStringstring
## DESCRIPTION


`XmStringLineCount`returns the number of separators plus one in the provided
compound string. In effect, it counts the lines of text.

* **`string`** 

Specifies the string

## RETURN


Returns the number of lines in the compound string.
If`string`is empty, the function returns 1. If NULL is passed
into`string`, the function returns 0 (zero).
## RELATED


&cdeman.XmStringCreate;.