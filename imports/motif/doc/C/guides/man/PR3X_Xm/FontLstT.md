# XmFontListGetNextFont
library call`XmFontListGetNextFont`A font list function that allows applications to access the fonts and character sets in a font listXmFontListGetNextFontfont list functionsXmFontListGetNextFont#include <Xm/Xm.h>Boolean`XmFontListGetNextFont`XmFontContextcontextXmStringCharSet *charsetXFontStruct **font
## DESCRIPTION


`XmFontListGetNextFont`accesses the character set and font for the
next entry of the font list. The application first uses the`XmFontListInitFontContext`routine to create a font list context.
The application then calls`XmFontListGetNextFont`repeatedly with
the same context. Each succeeding call accesses the next element of
the font list. When finished, the application calls`XmFontListFreeFontContext`to free the allocated font list context.

This routine allocates memory for the character set string that must be
freed by the application.
The function allocates memory for`charset`.
The application is responsible for managing the allocated memory.
The application can recover the allocated memory by calling`XtFree`.

This function is obsolete and exists for compatibility with previous
releases. It is replaced by`XmFontListNextEntry`.
If`XmFontListGetNextFont`is passed a context that contains
a font set entry, it will return the first font of the
font set. The next call to the function will move to the next
entry in the font list.

* **`context`** 

Specifies the font list context
* **`charset`** 

Specifies a pointer to a character set string; the routine returns the
character set for the current font list element
* **`font`** 

Specifies a pointer to a pointer to a font structure; the routine
returns the font for the current font list element

## RETURN


Returns True if the returned values are valid; otherwise, returns False.
## RELATED


&cdeman.XmFontList; and
&cdeman.XmFontListNextEntry;.