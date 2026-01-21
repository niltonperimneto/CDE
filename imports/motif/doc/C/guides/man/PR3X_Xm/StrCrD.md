# XmStringCreateSimple
library call`XmStringCreateSimple`A compound string function that creates a compound string in the language environment of a widgetXmStringCreateSimplecompound string functionsXmStringCreateSimple#include <Xm/Xm.h>XmString`XmStringCreateSimple`char* text
## DESCRIPTION


`XmStringCreateSimple`creates a compound
string with a text component and a charset tag.
It derives the character set from the current language environment.

The routine attempts to derive a character set from the value of the
LANG environment variable.
If this does not result in a valid character set, the routine uses a
vendor-specific default.
If the vendor has not specified a different value, this default is
ISO8859-1.

The function will allocate space to hold the returned compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

`NOTE:`This routine is obsolete and exists for compatibility with previous
releases. It has been replaced by`XmStringCreateLocalized`.

* **text** 

Specifies a NULL-terminated string to be used as the text component of
the compound string.

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate; and
&cdeman.XmStringCreateLocalized;.