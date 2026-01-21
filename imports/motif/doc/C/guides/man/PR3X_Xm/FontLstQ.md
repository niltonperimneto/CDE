# XmFontListFree
library call`XmFontListFree`A font list function that recovers memory used by a font listXmFontListFreefont list functionsXmFontListFree#include <Xm/Xm.h>void`XmFontListFree`XmFontListlist
## DESCRIPTION


`XmFontListFree`recovers memory used by a font list.
This routine does not free the`XFontSet`or`XFontStruct`associated
with the specified font list.

* **`list`** 

Specifies the font list to be freed

## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmFontListCopy;, and
&cdeman.XmFontListRemoveEntry;.