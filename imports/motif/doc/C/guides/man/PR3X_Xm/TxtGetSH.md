# XmTextGetSubstringWcs
library call`XmTextGetSubstringWcs`A Text function that retrieves
a portion of a wide character internal text bufferXmTextGetSubstringWcsText functionsXmTextGetSubstringWcs#include <Xm/Text.h>int`XmTextGetSubstringWcs`WidgetwidgetXmTextPositionstartintnum_charsintbuffer_sizewchar_t *buffer
## DESCRIPTION


`XmTextGetSubstringWcs`retrieves a copy of a portion of the internal
text buffer of a Text widget that is stored in a wide character
format. The function copies a specified number of characters from
a given start position in the internal text buffer into a buffer
provided by the application. A NULL terminator is placed at the
end of the copied data.

* **`widget`** 

Specifies the Text widget ID.
* **`start`** 

Specifies the beginning character position from which the data will be
retrieved. This is an integer number of characters from the beginning of
the text buffer. The first character position is 0 (zero).
* **`num_chars`** 

Specifies the number ofwchar_tcharacters to be copied into
the provided buffer.
* **`buffer_size`** 

Specifies the size of the supplied buffer as a number ofwchar_tstorage locations. The minimum size is`num_chars`+ 1.
* **`buffer`** 

Specifies the wide character buffer into which the internal
text buffer will be copied.


For a complete definition of Text and its associated resources,
see &cdeman.XmText;.
## RETURN


* **`XmCOPY_SUCCEEDED`** 

The function was successful.
* **`XmCOPY_FAILED`** 

The function failed because it was unable to copy the
specified number of characters into the buffer provided.
The buffer size may be insufficient. The contents of`buffer`are undefined.
* **`XmCOPY_TRUNCATED`** 

The requested number of characters extended beyond the internal
buffer. The function copied characters between`start`and the
end of the widget's buffer and terminated the string with a NULL
terminator; fewer than`num_chars`characters were copied.

## RELATED


&cdeman.XmText; and &cdeman.XmTextGetSubstring;.