# XmStringInitContext
library call`XmStringInitContext`A compound string function that creates a data structure for scanning an XmString component by componentXmStringInitContextcompound string functionsXmStringInitContext#include <Xm/Xm.h>Boolean`XmStringInitContext`XmStringContext* contextXmStringstring
## DESCRIPTION


`XmStringInitContext`creates a context to allow applications to read out the
contents of a compound string component by component.
A Boolean status is returned to indicate that the context could not be
initalized.

If the function returns True, the function will allocate space to hold the
returned`context`. The application is responsible for managing the allocated
space. The memory can be recovered by calling`XmStringFreeContext`.

* **`context`** 

Specifies a pointer to the allocated context
* **`string`** 

Specifies the string

## RETURN


Returns True if the context was allocated
## RELATED


&cdeman.XmStringCreate;.