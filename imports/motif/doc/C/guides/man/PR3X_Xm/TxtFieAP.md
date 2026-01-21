# XmTextFieldGetSubstring
library call`XmTextFieldGetSubstring`A TextField function that retrieves a copy
of a portion of the internal text bufferXmTextFieldGetSubstringTextField functionsXmTextFieldGetSubstring#include <Xm/TextF.h>int`XmTextFieldGetSubstring`WidgetwidgetXmTextPositionstartintnum_charsintbuffer_sizechar *buffer
## DESCRIPTION


`XmTextFieldGetSubstring`retrieves a copy of a portion
of the internal text buffer of a TextField widget. The function
copies a specified number of characters from a given start position
in the internal text buffer into a buffer provided by the application.
A NULL terminator is placed at the end of the copied data.

The size of the required buffer depends on the maximum number
of bytes per character (`MB_CUR_MAX`) for the current locale.`MB_CUR_MAX`is a macro defined instdlib.h. The buffer
should be large enough to contain the substring to be copied
and a NULL terminator. Use the following equation to calculate
the size of buffer the application should provide:buffer_size= (num_chars* MB_CUR_MAX) + 1

* **`widget`** 

Specifies the TextField widget ID.
* **`start`** 

Specifies the beginning character position from which the data
will be retrieved. This is an integer number of characters from
the beginning of the text buffer. The first character position
is 0 (zero).
* **`num_chars`** 

Specifies the number of characters to be copied into the provided
buffer.
* **`buffer_size`** 

Specifies the size of the supplied buffer in bytes. This size
should account for a NULL terminator.
* **`buffer`** 

Specifies the character buffer into which the internal
text buffer will be copied.


For a complete definition of TextField and its associated resources,
see &cdeman.XmTextField;.
## RETURN


* **`XmCOPY_SUCCEEDED`** 

The function was successful.
* **`XmCOPY_FAILED`** 

The function failed because it was unable to copy the
specified number of characters into the buffer provided.
The buffer size may be insufficient. The contents of`buffer`are undefined.
* **`XmCOPY_TRUNCATED`** 

The requested number of characters extended beyond the internal
buffer. The function copied characters between`start`and
the end of the widget's buffer and terminated the string with
a NULL terminator; fewer than`num_chars`characters were
copied.

## RELATED


&cdeman.XmTextField; and &cdeman.XmTextFieldGetSubstringWcs;.