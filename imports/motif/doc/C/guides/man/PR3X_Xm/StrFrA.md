# XmStringFree
library call`XmStringFree`A compound string function that
conditionally deallocates memoryXmStringFreecompound string functionsXmStringFree#include <Xm/Xm.h>void`XmStringFree`XmStringstring
## DESCRIPTION


`XmStringFree`conditionally recovers memory used by a compound string.
Applications should call`XmStringFree`when the application
no longer needs`string`.

* **`string`** 

Specifies the compound string to be freed

## RELATED


&cdeman.XmStringCreate;.