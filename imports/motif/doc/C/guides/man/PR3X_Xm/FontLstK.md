# XmFontListEntryCreate
library call`XmFontListEntryCreate`A font list function that creates a font list entryXmFontListEntryCreatefont list functionsXmFontListEntryCreate#include <Xm/Xm.h>XmFontListEntry`XmFontListEntryCreate`char *tagXmFontTypetypeXtPointerfont
## DESCRIPTION


`XmFontListEntryCreate`creates a font list entry that
contains either a font or font set and is identified by a tag.

* **tag** 

Specifies a NULL terminated string for the tag of the font
list entry. The tag may be specified as`XmFONTLIST_DEFAULT_TAG`,
which is used to identify the default font list element in a
font list.
* **`type`** 

Specifies whether the`font`argument is a font structure or
a font set. Valid values are`XmFONT_IS_FONT`and`XmFONT_IS_FONTSET`.
* **`font`** 

Specifies either an`XFontSet`returned by`XCreateFontSet`or
a pointer to an`XFontStruct`returned by`XLoadQueryFont`.


The toolkit does not copy the X Font structure specified by the`font`argument. Therefore, an application programmer must not
free`XFontStruct`or`XFontSet`until all font lists and/or font
entries that reference it have been freed.
## RETURN


Returns a font list entry.
The function allocates space to hold the
returned font list entry. The application is responsible for managing the
allocated space. The application can recover the allocated space by calling`XmFontListEntryFree`.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmFontListEntryFree;,
&cdeman.XmFontListEntryGetFont;,
&cdeman.XmFontListEntryGetTag;,
&cdeman.XmFontListEntryLoad;, and
&cdeman.XmFontListRemoveEntry;.