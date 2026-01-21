# XmFontListRemoveEntry
library call`XmFontListRemoveEntry`A font list function that removes a font list entry from a font listXmFontListRemoveEntryfont list functionsXmFontListRemoveEntry#include <Xm/Xm.h>XmFontList`XmFontListRemoveEntry`XmFontListoldlistXmFontListEntryentry
## DESCRIPTION


`XmFontListRemoveEntry`creates a new font list
that contains the contents of`oldlist`minus those entries specified in`entry`.
The routine removes any entries from`oldlist`that match the components (tag, type font/font set)
of the specified entry. The function deallocates the
original font list after extracting the required
information. The caller uses`XmFontListEntryFree`to recover memory allocated for the specified entry. This
routine does not free the`XFontSet`or`XFontStruct`associated
with the font list entry that is removed.

* **`oldlist`** 

Specifies the font list
* **`entry`** 

Specifies the font list entry to be removed

## RETURN


If`oldlist`is NULL, the function returns NULL. If`entry`is NULL or no entries are removed, the function
returns`oldlist`. Otherwise, it returns a new font list.
If the function returns a new font list, the function allocates
space to hold the new font list. The application is responsible
for managing the allocated space. The application can recover the
allocated space by calling`XmFontListFree`.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryFree;,
&cdeman.XmFontListEntryLoad;, and
&cdeman.XmFontListFree;.