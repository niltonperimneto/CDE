# XmFontListCreate
library call`XmFontListCreate`A font list function that creates a font listXmFontListCreatefont list functionsXmFontListCreate#include <Xm/Xm.h>XmFontList`XmFontListCreate`XFontStruct* fontXmStringCharSetcharset
## DESCRIPTION


`XmFontListCreate`creates a new font list with a single element specified
by the provided font and character set. It also allocates the space for
the font list.

`NOTE:`This function is obsolete and exists for compatibility
with previous releases. It is replaced by`XmFontListAppendEntry`.

* **`font`** 

Specifies a pointer to a font structure for which the new font list is
generated. This is the structure returned by the XLib`XLoadQueryFont`function.
* **`charset`** 

Specifies the character set identifier for the font.
This can be`XmSTRING_DEFAULT_CHARSET`, but this value does not
comply with the AES, and it may be removed in future versions of Motif.
If the value
is`XmSTRING_DEFAULT_CHARSET`, the routine derives the
character set from the current language environment.

## RETURN


Returns NULL if`font`or`charset`is NULL; otherwise, returns a new font list.
## RELATED


&cdeman.XmFontList; and
&cdeman.XmFontListAppendEntry;.