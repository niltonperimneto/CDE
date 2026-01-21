# XmFontListAppendEntry
library call`XmFontListAppendEntry`A font list function that appends an entry to a font listXmFontListAppendEntryfont list  functionsXmFontListAppendEntry#include <Xm/Xm.h>XmFontList`XmFontListAppendEntry`XmFontListoldlistXmFontListEntryentry
## DESCRIPTION


`XmFontListAppendEntry`creates a new font list that
contains the contents of`oldlist`. This function
copies the contents of the font list entry being added
into this new font list. If`oldlist`is NULL,`XmFontListAppendEntry`creates a new font list containing
only the single entry specified.

This function deallocates the original font list after
extracting the required information. The caller must
free the font list entry by using`XmFontListEntryFree`.

* **`oldlist`** 

Specifies the font list to be added to
* **`entry`** 

Specifies the font list entry to be added

## RETURN


If`entry`is NULL, returns`oldlist`; otherwise, returns
a new font list.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryFree;,
&cdeman.XmFontListEntryLoad;,
&cdeman.XmFontListFree;, and
&cdeman.XmFontListRemoveEntry;.