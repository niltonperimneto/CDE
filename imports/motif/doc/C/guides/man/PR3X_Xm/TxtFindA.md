# XmTextFindString
library call`XmTextFindString`A Text function that finds the beginning position of a text stringXmTextFindStringText functionsXmTextFindString#include <Xm/Xm.h>Boolean`XmTextFindString`WidgetwidgetXmTextPositionstartchar *stringXmTextDirectiondirectionXmTextPosition *position
## DESCRIPTION


`XmTextFindString`locates the beginning position of a specified
text string. This routine searches forward or backward for the first
occurrence of the string starting from the given start position.If it finds a match, the function
returns the position of the first character of the string in`position`.
If the match string begins at the current position, this routine returns the current position.

* **`widget`** 

Specifies the Text widget ID.
* **`start`** 

Specifies the character position from which the search proceeds. This
is an integer number of characters from the beginning of the text
buffer. The first character position is 0 (zero).
* **`string`** 

Specifies the search string.
* **direction** 

Indicates the search direction. It is relative to the primary
direction of the text. The possible values are

* **`XmTEXT_FORWARD`** 

The search proceeds toward the end of the text buffer.
* **`XmTEXT_BACKWARD`** 

The search proceeds toward the beginning of the text buffer.

* **`position`** 

Specifies the pointer in which the first character position
of the string match is returned. This is an integer number
of characters from the beginning of the buffer. The first
character position is 0 (zero). If the function returns False,
this value is undefined.


For a complete definition of Text and its associated resources,
see &cdeman.XmText;.
## RETURN


Returns True if a string match is found; otherwise, returns False.
## RELATED


&cdeman.XmText; and &cdeman.XmTextFindStringWcs;.