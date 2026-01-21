# XmStringCreate
library call`XmStringCreate`A compound string function that creates a compound stringXmStringCreatecompound string functionsXmStringCreate#include <Xm/Xm.h>XmString`XmStringCreate`char *textchar *tag
## DESCRIPTION


`XmStringCreate`creates a compound
string with two components: text and a font list element tag.
The function will allocate space to hold the returned compound string.
When the application no longer needs the returned compound string,
the application should call`XmStringFree`.

* **text** 

Specifies a NULL-terminated string to be used as the text component of
the compound string.
* **tag** 

Specifies the tag component to be associated with the given
text. The value`XmFONTLIST_DEFAULT_TAG`identifies a locale
text segment.

## RETURN


Returns a new compound string.
## RELATED


&cdeman.XmFontList;,
&cdeman.XmFontListAdd;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmFontListCopy;,
&cdeman.XmFontListCreate;,
&cdeman.XmFontListEntryCreate;,
&cdeman.XmFontListEntryFree;,
&cdeman.XmFontListEntryGetFont;,
&cdeman.XmFontListEntryGetTag;,
&cdeman.XmFontListEntryLoad;,
&cdeman.XmFontListFree;,
&cdeman.XmFontListFreeFontContext;,
&cdeman.XmFontListGetNextFont;,
&cdeman.XmFontListInitFontContext;,
&cdeman.XmFontListNextEntry;,
&cdeman.XmFontListRemoveEntry;,
&cdeman.XmString;,
&cdeman.XmStringBaseline;,
&cdeman.XmStringByteCompare;,
&cdeman.XmStringCompare;,
&cdeman.XmStringConcat;,
&cdeman.XmStringCopy;,
&cdeman.XmStringCreateLocalized;,
&cdeman.XmStringCreateLtoR;,
&cdeman.XmStringCreateSimple;,
&cdeman.XmStringDirection;,
&cdeman.XmStringDirectionCreate;,
&cdeman.XmStringDraw;,
&cdeman.XmStringDrawImage;,
&cdeman.XmStringDrawUnderline;,
&cdeman.XmStringEmpty;,
&cdeman.XmStringExtent;,
&cdeman.XmStringFree;,
&cdeman.XmStringFreeContext;,
&cdeman.XmStringGetLtoR;,
&cdeman.XmStringGetNextComponent;,
&cdeman.XmStringGetNextSegment;,
&cdeman.XmStringHasSubstring;,
&cdeman.XmStringHeight;,
&cdeman.XmStringInitContext;,
&cdeman.XmStringLength;,
&cdeman.XmStringLineCount;,
&cdeman.XmStringNConcat;,
&cdeman.XmStringNCopy;,
&cdeman.XmStringPeekNextComponent;,
&cdeman.XmStringSegmentCreate;,
&cdeman.XmStringSeparatorCreate;,
&cdeman.XmStringTable;, and
&cdeman.XmStringWidth;.