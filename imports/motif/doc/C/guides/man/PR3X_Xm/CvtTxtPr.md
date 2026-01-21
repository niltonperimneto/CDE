# XmCvtTextPropertyToXmStringTable
library call`XmCvtTextPropertyToXmStringTable`A function that converts from a TextProperty Structure to a StringTableXmCvtTextPropertyToXmStringTable#include <Xm/Xm.h>
int XmCvtTextPropertyToXmStringTable (display, text_prop, string_table_return, count_return)
        Display *display;
        XTextProperty   *text_prop;
        XmStringTable   *string_table_return;
        int     *count_return;
## DESCRIPTION


`XmCvtTextPropertyToXmStringTable`converts the specified`XTextProperty`structure into anXmStringTable, as follows:

If the encoding member of`text_prop`is the Atom`STRING`, each
returnedXmStringhas a tag of "ISO8859-1" and a text type of`XmCHARSET_TEXT`.

If
the encoding member of`text_prop`is the encoding of the current locale,
and if that encoding is not`STRING`, each returnedXmStringhas a
tag of`_MOTIF_DEFAULT_LOCALE`and a text type of`XmMULTIBYTE_TEXT`.

If
the encoding member of`text_prop`is other than`STRING`or
the encoding of the current locale,
the contents of the returned compound strings are
implementation dependent.

If conversion depends on the
locale and the current locale is not supported, the function returns`XLocaleNotSupported`. If conversion to the encoding of the current
locale is required and if the locale is supported but no converter is
available for the encoding specified in`text_prop`, the function
returns`XConverterNotFound`. For supported locales, existence of
a converter
from`COMPOUND_TEXT`,`STRING`, or the encoding of the current
locale is
guaranteed if`XSupportsLocale`returns True for the current locale (but
the actual text may contain unconvertible characters). Conversion of
other encodings to the encoding of the current locale is implementation
dependent. In all of these error cases, the function does not set any
return values.

If an element of the value member of`text_prop`is not
convertible toXmString, the corresponding entry in the returnedXmStringTablewill be NULL, and`XmCvtTextPropertyToXmStringTable`returns Success.

To free the storage for theXmStringTableand its`count_return`compound strings returned by this function, first
free eachXmStringin the table using`XmStringFree`, and
then free theXmStringTableitself using`XtFree`.

* **`display`** 

Specifies the connection to the X server.
* **`text_prop`** 

Specifies a pointer to the`XTextProperty`.
The format member of`text_prop`must be 8.
* **`string_table_return`** 

Specifies theXmStringTablearray into which the converted compound
strings are placed.
* **`count_return`** 

Specifies the number ofXmStrings returned by this function.

## RETURN VALUES


Upon success, this function returns the set ofXmStrings in`string_table_return`, and it returns the number ofXmStrings in`count_return`, and returns Success. Otherwise, it returns the
following:

* **`XLocaleNotSupported`** 

Returned if conversion depends on the
locale and the current locale is not supported.
* **`XConverterNotFound`** 

Returned if conversion to the encoding of the current
locale is required and if the locale is supported but no converter is
available for the encoding specified in`text_prop`.

## RELATED INFORMATION


&cdeman.XmCvtXmStringTableToTextProperty;,
&cdeman.XmText;, and
&cdeman.XmTextGetString;.