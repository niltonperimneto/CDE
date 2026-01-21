# XmFontListEntryGetFont
library call`XmFontListEntryGetFont`A font list function
that retrieves font information from a font list entryXmFontListEntryGetFontfont list functionsXmFontListEntryGetFont#include <Xm/Xm.h>XtPointer`XmFontListEntryGetFont`XmFontListEntryentryXmFontType *type_return
## DESCRIPTION


`XmFontListEntryGetFont`retrieves font information for a
specified font list entry. If the font list entry contains
a font,`type_return`returns`XmFONT_IS_FONT`and the
function returns a pointer to an`XFontStruct`. If the font list
entry contains a font set,`type_return`returns`XmFONT_IS_FONTSET`and the function returns the`XFontSet`.

* **`entry`** 

Specifies the font list entry.
* **`type_return`** 

Specifies a pointer to the type of the font element for the current
entry. Valid values are`XmFONT_IS_FONT`and`XmFONT_IS_FONTSET`.


The returned`XFontSet`or`XFontStruct`is not a copy of the
toolkit data and must not be freed.
## RETURN


Returns an`XFontSet`or a pointer to an`XFontStruct`structure.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryGetTag;
&cdeman.XmFontListEntryLoad;, and
&cdeman.XmFontListNextEntry;.