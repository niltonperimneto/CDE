# XmFontListEntryLoad
library call`XmFontListEntryLoad`A font list function that loads
a font or creates a font set and creates an accompanying font list entryXmFontListEntryLoadfont list functionsXmFontListEntryLoad#include <Xm/Xm.h>XmFontListEntry`XmFontListEntryLoad`Display *displaychar *font_nameXmFontTypetypechar *tag
## DESCRIPTION


`XmFontListEntryLoad`loads a font or creates a font set
based on the value of the`type`argument. It creates and returns
a font list entry that contains the font or font set and the
specified tag.

If the value of`type`is`XmFONT_IS_FONT`, the function uses
the`XtCvtStringToFontStruct`routine to convert the value of`font_name`to a font struct.
If the value of`type`is`XmFONT_IS_FONTSET`, the function uses
the`XtCvtStringToFontSet`converter to create a font set in the
current locale.`XmFontListEntryLoad`creates a font list entry that contains the
font or font set derived from the converter.
For more information about`XtCvtStringToFontStruct`and`XtCvtStringToFontSet`, seeX Toolkit Intrinsics&mdash;C Language
Interface.



* **`display`** 

Specifies the display where the font list will be used.
* **`font_name`** 

Specifies an X Logical Font Description (XLFD) string,
which is interpreted either as a font name or as a base font name
list.
A base font name list is a comma-separated and NULL-terminated string.
* **`type`** 

Specifies whether the`font_name`argument refers to a font name or
to a base font name list.
Valid values are`XmFONT_IS_FONT`and`XmFONT_IS_FONTSET`.
* **tag** 

Specifies the tag of the font list entry to be created.
The tag may be specified as`XmFONTLIST_DEFAULT_TAG`,
which is used to identify the default font list
element in a font list when specified as part of a resource.

## RETURN


If the specified font is not found, or if the specified font
set cannot be created, then either an implementation-defined font will
be opened or a font set will be created, and a warning messge will be
generated. If no suitable font can be found or a font set cannot be created,
then another message will be generated and the function will return
NULL; otherwise the function returns a font list entry.
If the function returns a font list entry, the function allocates space
to hold the font list entry. The application is responsible for managing
the allocated space. The application can recover the allocated space by
calling`XmFontListEntryFree`.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryFree;,
&cdeman.XmFontListEntryGetFont;,
&cdeman.XmFontListEntryGetTag;, and
&cdeman.XmFontListRemoveEntry;.