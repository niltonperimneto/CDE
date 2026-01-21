# XmFontListNextEntry
library call`XmFontListNextEntry`A font list function that returns
the next entry in a font listXmFontListNextEntryfont list functionsXmFontListNextEntry#include <Xm/Xm.h>XmFontListEntry`XmFontListNextEntry`XmFontContextcontext
## DESCRIPTION


`XmFontListNextEntry`returns the next entry in the
font list. The application uses the`XmFontListInitFontContext`routine to create a font list context. The first call to`XmFontListNextEntry`sets the context to the first entry
in the font list. The application then calls`XmFontListNextEntry`repeatedly with the same context. Each succeeding call accesses
the next entry of the font list. When finished, the application
calls`XmFontListFreeFontContext`to free the allocated font
list context.

* **`context`** 

Specifies the font list context

## RETURN


Returns NULL if the context does not refer to a valid entry or if
it is at the end of the font list; otherwise, it returns a font list entry.
If the function does return a font list entry, the font list entry is not
a copy. Therefore, the application should not free the returned font list entry.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListEntryFree;,
&cdeman.XmFontListEntryGetFont;,
&cdeman.XmFontListEntryGetTag;,
&cdeman.XmFontListFreeFontContext;, and
&cdeman.XmFontListInitFontContext;.