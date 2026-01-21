# XmStringParseText
library call`XmStringParseText`A function that converts a character string to a compound stringXmStringParseTextcompound string functionsXmStringParseText#include <Xm/Xm.h>XmString`XmStringParseText`XtPointertextXtPointer *text_endXmStringTagtagXmTextTypetypeXmParseTableparse_tableCardinalparse_countXtPointercall_data
## DESCRIPTION


`XmStringParseText`converts characters specified intextto
corresponding components in the returned compound string. The
resulting compound string consists of at least one locale or charset
tag component and a series ofXmStringtext components and other
components. The conversion proceeds according to the parse information
contained in`parse_table`. See the &MotifProgGd; for more information
about parsing and parse tables.

If`type`is`XmCHARSET_TEXT`, the associatedtagis
interpreted as a charset name.
Iftaghas a value of NULL, a charset component whose value
is the result of mapping`XmFONTLIST_DEFAULT_TAG`is created.

If`type`is`XmMULTIBYTE_TEXT`or`XmWIDECHAR_TEXT`, the
associatedtagis interpreted as a language environment name.
Iftaghas a value of NULL, a locale component with a value of`_MOTIF_DEFAULT_LOCALE`is created.
If`type`is`XmMULTIBYTE_TEXT`or`XmWIDECHAR_TEXT`,tagmust be NULL or`_MOTIF_DEFAULT_LOCALE`.

`XmStringParseText`also scans the string for characters that have
matches in`parse_table`.
Whenever a match is found, the text up to that point is concatenated
with the mapped component.

* **text** 

Specifies the NULL-terminated string containing characters of a type
determined by`type`.
This is updated to point to after the last character scanned.
* **`text_end`** 

Specifies a pointer intotext. If a NULL is supplied to the`text_end`parameter, then`XmStringParseText`parsestextuntil NULL is encountered, or until it reaches a point intextwhere it is directed to stop
(for example, by a`parse_proc`). Otherwise, the value supplied
to the`text_end`parameter is the pointer intotextwhere
parsing is to stop, and the returned character is the one where
parsing did stop.
* **tag** 

Specifies the tag to be used in creating the result.
The type of string tag created (charset or locale) depends on the text
type and the passed intagvalue.
If thetagvalue is NULL and if`type`indicates that a
charset string tag should be created, the string tag has the value
that is the result of mapping`XmFONTLIST_DEFAULT_TAG`.
If`type`indicates a locale string tag, the string tag has the
value`_MOTIF_DEFAULT_LOCALE`.
* **`type`** 

Specifies the type of text and the tag type.
If a locale tag should be created,`type`has a value of either`XmMULTIBYTE_TEXT`or`XmWIDECHAR_TEXT`.
If`type`has value of`XmCHARSET_TEXT`, a charset tag will be created.
* **`parse_table`** 

Specifies the parse table to be used in scanning for characters to be
converted to other compound string components.
* **`parse_count`** 

Specifies the number of entries in`parse_table`.
* **`call_data`** 

Specifies data to be passed to the parse procedures.

## RETURN


Returns a new compound string.
The function allocates space to hold the returned compound string.
When the application no longer needs the returned compound string,
the application should call`XmStringFree`.
## RELATED


&cdeman.XmString;,
&cdeman.XmStringFree;, &cdeman.XmParseTable;, &cdeman.XmParseMapping;.