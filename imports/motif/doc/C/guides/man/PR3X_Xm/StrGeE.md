# XmStringGetNextSegment
library call`XmStringGetNextSegment`A compound string function that fetches the bytes in the next segment of a compound stringXmStringGetNextSegmentcompound string functionsXmStringGetNextSegment#include <Xm/Xm.h>Boolean`XmStringGetNextSegment`XmStringContextcontextchar **textXmStringTag *tagXmStringDirection *directionBoolean *separator
## DESCRIPTION


This routine is obsolete and exists for compatibility with previous
releases. To read the contents of a compound string, read each
component of the string with`XmStringGetNextTriple`. ThisXmStringfunction returns the type, length, and value of the next
component in the compound string.`XmStringGetNextSegment`fetches the
bytes
in the next segment; repeated calls
fetch sequential segments. Thetext,tag,
anddirectionof the fetched segment are returned each time. A
Boolean status is returned to indicate whether a valid segment was
successfully parsed.

If the function returns True, then the function allocates space to hold the
returnedtextandtag. The application is responsible for
managing the allocated space. The application can recover the allocated space
by calling`XtFree`.

* **`context`** 

Specifies the string context structure which was allocated by the`XmStringInitContext`function
* **text** 

Specifies a pointer to a NULL-terminated string
* **tag** 

Specifies a pointer to the font list element tag associated with the
text
* **direction** 

Specifies a pointer to the direction of the text
* **`separator`** 

Specifies whether the next component of the compound string is a
separator

## RETURN


Returns True if a valid segment is found.
## RELATED


&cdeman.XmStringCreate; and &cdeman.XmStringInitContext;.