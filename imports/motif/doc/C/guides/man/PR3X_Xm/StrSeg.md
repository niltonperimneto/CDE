# XmStringSegmentCreate
library call`XmStringSegmentCreate`A compound string function that creates a compound stringXmStringSegmentCreatecompound string functionsXmStringSegmentCreate#include <Xm/Xm.h>XmString`XmStringSegmentCreate`char* textXmStringTagtagXmStringDirectiondirectionBooleanseparator
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases. It can be replaced by using a combination of`XmStringComponentCreate`and`XmStringConcat`.`XmStringSegmentCreate`is a high-level function that assembles a compound
string consisting of a font list element tag, a direction component,
a text component, and an optional separator component.

The function allocates space for the returned compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **text** 

Specifies a NULL-terminated string to be used as the text component of
the compound string.
* **tag** 

Specifies the tag component to be associated with the text.
The value`XmFONTLIST_DEFAULT_TAG`is for compatibility with
previous releases.
* **direction** 

Specifies the direction of the text.
* **`separator`** 

A value of False means
the compound string does not
have a separator at the end. A value of True, means a separator immediately
follows the text component.

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate;.