# XmStringCompare
library call`XmStringCompare`A compound string function that compares two stringsXmStringComparecompound string functionsXmStringCompare#include <Xm/Xm.h>Boolean`XmStringCompare`XmStrings1XmStrings2
## DESCRIPTION


`XmStringCompare`returns a Boolean value indicating the results of a
semantically equivalent comparison of two compound strings.

Semantically equivalent means that the strings have the
same text components, font list element tags, directions,
and separators. In general, if two compound strings are created
with the same (char *) string using`XmStringCreateLocalized`in
the same language environment, the compound strings compare as equal.
If two compound strings are created with the same text and tag argument
using`XmStringCreate`, the strings compare as equal.

* **`s1`** 

Specifies a compound string to be compared with`s2`
* **`s2`** 

Specifies a compound string to be compared with`s1`

## RETURN


Returns True if two compound strings are equivalent.
## RELATED


&cdeman.XmStringCreate; and
&cdeman.XmStringCreateLocalized;.