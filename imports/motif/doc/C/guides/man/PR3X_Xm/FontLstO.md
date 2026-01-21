# XmFontListEntryGetTag
library call`XmFontListEntryGetTag`A font list function that
retrieves the tag of a font list entryXmFontListEntryGetTagfont list functionsXmFontListEntryGetTag#include <Xm/Xm.h>char*`XmFontListEntryGetTag`XmFontListEntryentry
## DESCRIPTION


`XmFontListEntryGetTag`retrieves a copy of the tag of the specified
font list entry. This routine allocates memory for the tag string that
must be freed by the application.

* **`entry`** 

Specifies the font list entry

## RETURN


Returns the tag for the font list entry.
The function allocates space to hold the returned tag.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XtFree`.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryGetFont;,
&cdeman.XmFontListEntryLoad;, and
&cdeman.XmFontListNextEntry;.