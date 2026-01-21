# XmStringGetLtoR
library call`XmStringGetLtoR`A compound string function that searches for a text segment in the input compound stringXmStringGetLtoRcompound string functionsXmStringGetLtoR#include <Xm/Xm.h>Boolean`XmStringGetLtoR`XmStringstringXmStringCharSettagchar **text
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases. It is replaced by`XmStringUnparse`.`XmStringGetLtoR`returns the first text component in the input
compound string that is tagged with the given tag component. The
returned text is to be a NULL-terminated sequence of single byte characters.
If the function returns True, the function will allocate space
to hold the returnedtext. The application is responsible
for managing the allocated space. The application can recover the
allocated space by calling`XtFree`.

* **`string`** 

Specifies the compound string.
* **tag** 

Specifies the font list element tag associated with the text.
A value of`XmFONTLIST_DEFAULT_TAG`identifies a locale text
segment.
* **text** 

Specifies a pointer to a NULL terminated string.

## RETURN


Returns True if the matching text segment can be found.
On return,textwill have a NULL terminated
byte
sequence
containing the matched segment.
## RELATED


&cdeman.XmStringCreate;.