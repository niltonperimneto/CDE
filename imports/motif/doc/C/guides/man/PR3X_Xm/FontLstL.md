# XmFontListEntryFree
library call`XmFontListEntryFree`A font list function that recovers
memory used by a font list entryXmFontListEntryFreefont list functionsXmFontListEntryFree#include <Xm/Xm.h>void`XmFontListEntryFree`XmFontListEntry *entry
## DESCRIPTION


`XmFontListEntryFree`recovers memory used by a font list entry.
This routine does not free the`XFontSet`or`XFontStruct`associated
with the font list entry.

* **`entry`** 

Specifies a pointer to the font list entry to be freed.
In addition, it may be necessary to take the address of the font list
entry (via the`&`operator) before passing it to this function.

## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryLoad;,
&cdeman.XmFontListNextEntry;, and
&cdeman.XmFontListRemoveEntry;.