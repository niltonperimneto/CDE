# XmCvtXmStringTableToTextProperty
library call`XmCvtXmStringTableToTextProperty`A function that converts from XmStringTable to an XTextProperty StructureXmCvtXmStringTableToTextProperty#include <Xm/Xm.h>
int XmCvtXmStringTableToTextProperty (display, string_table, count, style, text_prop_return)
        Display *display;
        XmStringTablestring_table;
        intcount;
        XmICCEncodingStylestyle;
        XTextProperty   *text_prop_return;
## DESCRIPTION


`XmCvtXmStringTableToTextProperty`converts theXmStrings
in the specifiedXmStringTableinto an`XTextProperty`structure.

The function sets the encoding member of`text_prop_return`to an`Atom`for the specified display naming the encoding determined by
the specified style, and it converts the first`count`compound
strings in the specifiedXmStringTableto this encoding for
storage in the`text_prop_return`value member. Following are the
possible encoding styles:

* **`XmSTYLE_COMPOUND_STRING`** 

The encoding is_MOTIF_COMPOUND_STRING. The function converts
each specifiedXmStringto a compound string in Byte Stream
format.
* **`XmSTYLE_COMPOUND_TEXT`** 

The encoding is`COMPOUND_TEXT`. The function converts each
specifiedXmStringto compound text.
* **`XmSTYLE_LOCALE`** 

The encoding is the encoding of the current locale. The function
converts each specifiedXmStringto the encoding of the current
locale.
* **`XmSTYLE_STRING`** 

The encoding is`STRING`(plain C strings encoded in ISO8859-1), and
the function converts each specifiedXmStringto`STRING`.
* **`XmSTYLE_TEXT`** 

If all specifiedXmStrings are fully convertible to the
encoding of the current locale, the encoding is the encoding of the
current locale, and the function converts each specifiedXmStringto the encoding of the current locale. Otherwise, the
encoding is`COMPOUND_TEXT`, and the function converts each
specified compound
string to compound text.
* **`XmSTYLE_STANDARD_ICC_TEXT`** 

If all specifiedXmStrings are fully convertible to`STRING`, the
encoding is`STRING`, and the function converts each specifiedXmStringto`STRING`. Otherwise, the encoding is`COMPOUND_TEXT`, and the
function converts each specifiedXmStringto compound text.


* **`display`** 

Specifies the connection to the X server.
* **`string_table`** 

Specifies a set ofXmStrings.
* **`count`** 

Specifies the number ofXmStrings to be
converted in`string_table`.
* **`style`** 

Specifies the manner in which the property is encoded.
* **`text_prop_return`** 

Returns the`XTextProperty`structure.


To free the storage for the value member of the`XTextProperty`, use`XtFree`.
## RETURN VALUES


If conversion depends on the locale and the
current locale is not supported, the function returns`XLocaleNotSupported`. In both of these cases, the function does not set`text_prop_return`.

To determine whether the function is guaranteed not to return`XLocaleNotSupported`, use`XSupportsLocale`.
## RELATED INFORMATION


&cdeman.XmCvtXmStringToByteStream;,
&cdeman.XmCvtTextPropertyToXmStringTable;, and
&cdeman.XmStringTable;.