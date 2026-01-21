# XmFontListInitFontContext
library call`XmFontListInitFontContext`A font list function that allows
applications to access the entries in a font listXmFontListInitFontContextfont list functionsXmFontListInitFontContext#include <Xm/Xm.h>Boolean`XmFontListInitFontContext`XmFontContext*contextXmFontListfontlist
## DESCRIPTION


`XmFontListInitFontContext`establishes a context to allow applications to access the contents
of a font list. This context is used when reading the font
list entry tag, font, or font set associated with each entry in
the font list. A Boolean status is returned to indicate whether
or not the font list is valid.

If an application deallocates the font list passed to`XmFontListInitFontContext`as the fontlist argument, the context
established by this function is rendered no longer valid.

* **`context`** 

Specifies a pointer to the allocated context
* **`fontlist`** 

Specifies the font list

## RETURN


Returns True if the context was allocated; otherwise, returns False.
If the function allocated a context, the application is responsible
for managing the allocated space. The application can recover the
allocated space by calling`XmFontListFreeFontContext`.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListFreeFontContext;, and
&cdeman.XmFontListNextEntry;.