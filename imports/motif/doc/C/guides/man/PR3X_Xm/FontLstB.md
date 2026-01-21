# XmFontListAdd
library call`XmFontListAdd`A font list function that creates a new font listXmFontListAddfont list functionsXmFontListAdd#include <Xm/Xm.h>XmFontList`XmFontListAdd`XmFontListoldlistXFontStruct*fontXmStringCharSetcharset
## DESCRIPTION


`XmFontListAdd`creates a new font list consisting of the contents of`oldlist`and the new font list element being added. This
function deallocates`oldlist`after extracting the required
information; therefore, do not reference`oldlist`thereafter.

`NOTE:`This function is obsolete and exists for compatibility
with previous releases. It has been replaced by`XmFontListAppendEntry`.

* **`oldlist`** 

Specifies a pointer to the font list to which an entry will be added.
* **`font`** 

Specifies a pointer to a font structure for which the new font list is
generated. This is the structure returned by the XLib`XLoadQueryFont`function.
* **`charset`** 

Specifies the character set identifier for the font.
This can be`XmSTRING_DEFAULT_CHARSET`, but this value does not
comply with the AES, and it may be removed in future versions of Motif.
If the value is`XmSTRING_DEFAULT_CHARSET`, the routine derives the
character set from the current language environment.

## RETURN


Returns NULL if`oldlist`is NULL; returns`oldlist`if`font`or`charset`is NULL; otherwise, returns a new font list.
## RELATED


&cdeman.XmFontList; and
&cdeman.XmFontListAppendEntry;.