# XmStringCreateLocalized
library call`XmStringCreateLocalized`A compound string function that creates
a compound string in the current localeXmStringCreateLocalizedcompound string functionsXmStringCreateLocalized#include <Xm/Xm.h>XmString`XmStringCreateLocalized`char *text
## DESCRIPTION


`XmStringCreateLocalized`creates a compound string containing
the specified text in the current language environment.
An identical compound string would result
from the function`XmStringCreate`called with`XmFONTLIST_DEFAULT_TAG`explicitly as the tag component.

The function will allocate space to hold the returned compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.

* **text** 

Specifies a NULL-terminated string of text encoded in the current
language environment
to be used as the text component of the compound string

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmStringCreate;.