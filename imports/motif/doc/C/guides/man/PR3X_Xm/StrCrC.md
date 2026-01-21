# XmStringCreateLtoR
library call`XmStringCreateLtoR`A compound string function that creates a compound stringXmStringCreateLtoRcompound string functionsXmStringCreateLtoR#include <Xm/Xm.h>XmString`XmStringCreateLtoR`char *textchar *tag
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases. It is replaced by`XmStringGenerate`.`XmStringCreateLtoR`creates a compound
string with two components: text and a
tag component.
This function
scans
for`&bsol;n`characters in the text. When one is found, the text up to that point
is put into a segment followed by a separator component. No final
separator component is appended to the end of the compound string.
The direction
component
defaults to left-to-right.
This function assumes that the encoding is single
byte
rather than
multibyte.

The function will allocate space to hold the returned compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **text** 

Specifies a NULL-terminated string to be used as the text component of
the compound string.
* **tag** 

Specifies the tag component to be associated with the given
text. The value`XmFONTLIST_DEFAULT_TAG`is retained for
compatibility with previous releases.

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate;
and &cdeman.XmStringGenerate;.