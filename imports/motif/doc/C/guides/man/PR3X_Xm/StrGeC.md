# XmStringGetNextComponent
library call`XmStringGetNextComponent`A compound string function that returns the type and value of the next component in a compound stringXmStringGetNextComponentcompound string functionsXmStringGetNextComponent#include <Xm/Xm.h>XmStringComponentType`XmStringGetNextComponent`XmStringContextcontextchar **textXmStringTag *tagXmStringDirection *directionXmStringComponentType *unknown_tagunsigned short *unknown_lengthunsigned char **unknown_value
## DESCRIPTION


This function is obsolete and exists for compatibility with previous
releases. It is replaced by`XmStringGetNextTriple`.`XmStringGetNextComponent`returns the type and value of the next component in
the compound string identified by`context`.
Components are returned one at a time. On return, only
some output parameters will be valid; which ones can be determined by
examining the returned component type.
The following table describes the valid returns for each component
type.`Valid Fields``Component Type`tag`XmSTRING_COMPONENT_LOCALE, XmSTRING_COMPONENT_TAG`text`XmSTRING_COMPONENT_LOCALE_TEXT, XmSTRING_COMPONENT_TEXT,
XmSTRING_COMPONENT_WIDECHAR_TEXT`direction`XmSTRING_COMPONENT_DIRECTION`unknown_tag, unknown_length, unknown_value`XmSTRING_COMPONENT_LAYOUT_POP, XmSTRING_COMPONENT_LAYOUT_PUSH,
XmSTRING_COMPONENT_TAB, XmSTRING_COMPONENT_RENDITION_BEGIN,
XmSTRING_COMPONENT_RENDITION_END`no valid field`XmSTRING_COMPONENT_SEPARATOR, XmSTRING_COMPONENT_END,
XmSTRING_COMPONENT_UNKNOWN`

Note that several components produce a return value of`XmSTRING_COMPONENT_UNKNOWN`. The data returned by these
components is returned in the`unknown_tag`,`unknown_length`,
and`unknown_value`fields. This apparent inconsistency is
designed to accommodate older applications that may not be equipped to
handle the newer component types of Motif version 2.0 and beyond.
Consequently, the use of this procedure is not recommended. Instead,
use the`XmStringGetNextTriple`procedure, which provides all the
functionality of`XmStringGetNextComponent`, and is fully
compatible with the newer component types.

If the function return value is`XmSTRING_COMPONENT_LOCALE_TEXT`or`XmSTRING_COMPONENT_TEXT`, the the function allocates space to hold
the returnedtext.
If the function return value is`XmSTRING_COMPONENT_FONTLIST_ELEMENT_TAG`,
or`XmSTRING_COMPONENT_TAG`, then the function allocates space to hold
the returnedtag.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XtFree`.

* **`context`** 

Specifies the string context structure that was allocated by the`XmStringInitContext`function.
* **text** 

Specifies a pointer to a NULL terminated string.
* **tag** 

Specifies a pointer to the tag component associated with the
text. The value`XmFONTLIST_DEFAULT_TAG`identifies a locale
text segment.
* **direction** 

Specifies a pointer to the direction of the text.
* **`unknown_tag`** 

Specifies a pointer to the tag of an unknown component.
* **`unknown_length`** 

Specifies a pointer to the length of an unknown component.
* **`unknown_value`** 

Specifies a pointer to the value of an unknown component.

## RETURN


Returns the type of component found.
Refer to the
&cdeman.XmStringComponentType; reference page for a list of component types.
## RELATED


&cdeman.XmStringComponentType;,
&cdeman.XmStringCreate;,
and &cdeman.XmStringInitContext;.