# XmParseMapping
library callXmParseMappingData type for a compound string parse mappingXmParseMappingdata typesXmParseMapping&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


XmParseMappingis an opaque data type for a parse mapping used by`XmStringParseText`to create a compound string.
A parse mapping contains a pattern to be matched in text being parsed to
create a compound string.
It also contains a compound string, or a function to be invoked to
provide a compound string, to be included in the compound string being
created whenever the pattern is matched.

An application uses a resource-style interface to specify components for
anXmParseMapping.`XmParseMappingCreate`creates a parse mapping, using a
resource-style argument list.`XmParseMappingGetValues`and`XmParseMappingSetValues`retrieve
and set the components of a parse mapping.`XmParseMappingFree`recovers memory used by a parse mapping.XmParseTableis an array ofXmParseMappingobjects.

The`XmNinvokeParseProc`resource is a function of typeXmParseProc, which is defined as follows:XmIncludeStatus (*XmParseProc) (`text_in_out, text_end, type, tag, entry, pattern_length,
str_include, call_data`)
        XtPointer *`text_in_out`;
        XtPointer`text_end`;
        XmTextType`type`;
        XmStringTagtag;
        XmParseMapping`entry`;
        int`pattern_length`;
        XmString *`str_include`;
        XtPointer`call_data`;

A parse procedure provides an escape mechanism for arbitrarily complex
parsing.
This procedure is invoked when a pattern in the input text is matched
with a pattern in a parse mapping whose`XmNincludeStatus`is`XmINVOKE`.

The input text is a pointer to the first byte of the pattern that was
matched to trigger the call to the parse procedure.
The parse procedure consumes as many bytes of the input string as it
needs and sets the input text pointer to the following byte.
It returns a compound string to be included in the compound string being
constructed, and it also returns anXmIncludeStatusindicating how
the returned compound string should be handled.
If the parse procedure does not set the input text pointer ahead by at
least one byte, the parsing routine continues trying to match the input
text with the patterns in the remaining parse mappings in the parse
table.
Otherwise, the parsing routine begins with the new input text pointer
and tries to match the input text with patterns in the parse mappings
starting at the beginning of the parse table.

* **`text_in_out`** 

Specifies the text being parsed.
The value is a pointer to the first byte of text matching the pattern
that triggered the call to the parse procedure.
When the parse procedure returns, this argument is set to the position
in the text where parsing should resume&mdash;that is, to the byte
following the last character parsed by the parse procedure.
* **`text_end`** 

Specifies a pointer to the end of the`text_in_out`string.
If`text_end`is NULL, the string is scanned until a NULL character
is found.
Otherwise, the string is scanned up to but not including the character
whose address is`text_end`.
* **`type`** 

Specifies the type of text and the tag type.
If a locale tag should be created,`type`has a value of either`XmMULTIBYTE_TEXT`or`XmWIDECHAR_TEXT`.
If a charset should be created,`type`has a value of`XmCHARSET_TEXT`.
* **tag** 

Specifies the tag to be used in creating the result.
The type of string tag created (charset or locale) depends on the text
type and the passed intagvalue.
If thetagvalue is NULL and if`type`indicates that a
charset string tag should be created, the string tag has the value
that is the result of mapping`XmSTRING_DEFAULT_CHARSET`.
If`type`indicates a locale string tag, the string tag has the
value`_MOTIF_DEFAULT_LOCALE`.
* **`entry`** 

Specifies the parse mapping that triggered the call to the parse
procedure.
* **`pattern_length`** 

Specifies the number of bytes in the input text, following`text_in_out`, that constitute the matched pattern.
* **`str_include`** 

Specifies a pointer to a compound string.
The parse procedure creates a compound string to be included in the
compound string being constructed.
The parse procedure then returns the compound string in this argument.
* **`call_data`** 

Specifies data passed by the application to the parsing routine.


The parse procedure returns anXmIncludeStatusindicating how`str_include`is to be included in the compound string being
constructed.
Following are the possible values:

* **`XmINSERT`** 

Concatenate the result to the compound string being constructed and
continue parsing.
* **`XmTERMINATE`** 

Concatenate the result to the compound string being constructed and
terminate parsing.

### New Resources


The following table defines a set of resources used by the programmer
to specify data.
The codes in the access column indicate if the given resource can be set
at creation time (C), set by using`XmParseMappingSetValues`(S),
retrieved by using`XmParseMappingGetValues`(G), or is not
applicable (N/A).

`XmParseMapping Resource Set``Name``Class``Type``Default``Access`XmNclientDataXtPointerNULLCSGXmNincludeStatusXmIncludeStatusXmINSERTCSGXmNinvokeParseProcXmParseProcNULLCSGXmNpatternXtPointerNULLCSGXmNpatternTypeXmTextTypeXmCHARSET_TEXTCSGXmNsubstituteXmStringNULLCSG

* **`XmNclientData`** 

Specifies data to be used by the parse procedure.
* **`XmNincludeStatus`** 

Specifies how the result of the mapping is to be included in the
compound string being constructed.
Unless the value is`XmINVOKE`, the result of the mapping is the
value of`XmNsubstitute`.
Following are the possible values for`XmNincludeStatus`:

* **`XmINSERT`** 

Concatenate the result to the compound string being constructed and
continue parsing.
* **`XmINVOKE`** 

Invoke the`XmNinvokeParseProc`on the text being parsed and use the
returned compound string instead of`XmNsubstitute`as the result to
be inserted into the compound string being constructed.
The include status returned by the parse procedure (`XmINSERT`or`XmTERMINATE`) determines how the returned compound string is
included.
* **`XmTERMINATE`** 

Concatenate the result to the compound string being constructed and
terminate parsing.

* **`XmNinvokeParseProc`** 

Specifies the parse procedure to be invoked when`XmNincludeStatus`is`XmINVOKE`.
* **`XmNpattern`** 

Specifies a pattern to be matched in the text being parsed.
This is a maximum of one character.
* **`XmNpatternType`** 

Specifies the type of the pattern that is the value of`XmNpattern`.
Following are the possible values:

`XmCHARSET_TEXT`

`XmMULTIBYTE_TEXT`

`XmWIDECHAR_TEXT`
* **`XmNsubstitute`** 

Specifies the compound string to be included in the compound string
being constructed when`XmNincludeStatus`is`XmINSERT`or`XmTERMINATE`.

## RELATED


&cdeman.XmParseMappingCreate;,
&cdeman.XmParseMappingFree;,
&cdeman.XmParseMappingGetValues;,
&cdeman.XmParseMappingSetValues;,
&cdeman.XmParseTable;, and
&cdeman.XmString;.