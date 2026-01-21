# XmFontListFreeFontContext
library call`XmFontListFreeFontContext`A font list function that instructs the toolkit that the font list context is no longer neededXmFontListFreeFontContextfont list functionsXmFontListFreeFontContext#include <Xm/Xm.h>void`XmFontListFreeFontContext`XmFontContextcontext
## DESCRIPTION


`XmFontListFreeFontContext`instructs the toolkit that the context
is no longer needed and will not be used without reinitialization.

* **`context`** 

Specifies the font list context structure that was allocated by the`XmFontListInitFontContext`function

## RELATED


&cdeman.XmFontListInitFontContext; and
&cdeman.XmFontListNextEntry;.