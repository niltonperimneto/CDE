# XmFontListCopy
library call`XmFontListCopy`A font list function that copies a font listXmFontListCopyfont list functionsXmFontListCopy#include <Xm/Xm.h>XmFontList`XmFontListCopy`XmFontListfontlist
## DESCRIPTION


`XmFontListCopy`creates a new font list consisting of the contents
of the`fontlist`argument.

* **`fontlist`** 

Specifies a font list to be copied

## RETURN


Returns NULL if`fontlist`is NULL;
otherwise, returns a new font list
and allocates space to hold the font list.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmFontListFree`.
## RELATED


&cdeman.XmFontList; and
&cdeman.XmFontListFree;.