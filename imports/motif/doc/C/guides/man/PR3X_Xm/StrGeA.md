# XmStringGenerate
library call`XmStringGenerate`A convenience function that generates a compound stringXmStringGenerate#include <Xm/Xm.h>XmString`XmStringGenerate`XtPointertextXmStringTagtagXmTextTypetypeXmStringTagrendition
## DESCRIPTION


`XmStringGenerate`calls the`XmStringParseText`function with a default parse table of entries
consisting of '\n', which maps to
Separator,
and '\t', which maps to Tab.
Matching`RENDITION_BEGIN`and`RENDITION_END`components containing`rendition`are placed
around the resultingXmString.

* **text** 

Specifies a NULL-terminated string containing characters of a type
determined by`type`.
* **tag** 

Specifies the tag to be used in creating the result. The type of tag
created (charset or locale) depends on the text type and the
value given. If specified value is NULL, and`type`indicates
that a charset tag should be created, then the tag will have the
value of`XmFONTLIST_DEFAULT_TAG`.
Iftagis NULL, and`type`indicates a
locale tag, then the tag will have the value of`_MOTIF_DEFAULT_LOCALE`.
* **`type`** 

Specifies the type of text to be passed in, and the tag type.
If a locale tag should be created, then`type`has a value of
either`XmMULTIBYTE_TEXT`or`XmWIDECHAR_TEXT`. If a
charset should be created,`type`has a value of`XmCHARSET_TEXT`.
* **`rendition`** 

Specifies the rendition tag to be used in an`XmSTRING_COMPONENT_RENDITION_BEGIN`component which will begin
the returned string and in an`XmSTRING_COMPONENT_RENDITION_END`component which will end it.
If`rendition`is NULL, no rendition tag is placed.

## RETURN


Returns a new compound string.
The function will allocate space to hold the returned compound string.
When the application no longer needs the returned compound string, the
application should call`XmStringFree`.
## RELATED


&cdeman.XmString; and
&cdeman.XmStringFree;.