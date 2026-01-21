# XmStringPutRendition
library call`XmStringPutRendition`A convenience function that places renditions around stringsXmStringPutRendition#include <Xm/Xm.h>XmString`XmStringPutRendition`XmStringstringXmStringTagrendition
## DESCRIPTION


`XmStringPutRendition`places matching`Xm_STRING_COMPONENT_RENDITION_BEGIN`and`XmSTRING_COMPONENT_RENDITION_END`components containing`rendition`around`string`. The original string is preserved.

* **`string`** 

Specifies the compound string to which begin and end rendition
components should be added.
* **`rendition`** 

Specifies the rendition tag to be used in an`XmSTRING_COMPONENT_RENDITION_BEGIN`component which will begin
the returned string and in an`XmSTRING_COMPONENT_RENDITION_END`component which will end it.

## RETURN


Returns a new compound string.
The function allocates space to hold this returned compound string.
When the application no longer needs the returned compound string,
the application should call`XmStringFree`.
## RELATED


&cdeman.XmString;.