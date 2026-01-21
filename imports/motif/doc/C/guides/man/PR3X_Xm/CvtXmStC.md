# XmCvtXmStringToCT
library call`XmCvtXmStringToCT`A compound string function that converts a compound string to compound textXmCvtXmStringToCTcompound string functionsXmCvtXmStringToCT#include <Xm/Xm.h>char *`XmCvtXmStringToCT`XmStringstring
## DESCRIPTION


`XmCvtXmStringToCT`converts a compound string to a (char *) string
in compound text format.
The application must call`XtAppInitialize`before calling this
function.
The converter uses the font list tag associated with a given compound
string segment to select a compound text format for that segment.
A registry defines a mapping between font list tags and compound text
encoding formats.
The converter uses the following algorithm for each compound string
segment:

If the compound string segment tag is mapped to`XmFONTLIST_DEFAULT_TAG`in the registry, the converter passes the
text of the compound string segment to`XmbTextListToTextProperty`with an encoding style of`XCompoundTextStyle`and uses the
resulting compound text for that segment.

If the compound string segment tag is mapped to an MIT registered
charset in the registry, the converter creates the compound text for
that segment using the charset (from the registry) and the text of the
compound string segment as defined in the X Consortium StandardCompound Text Encoding.

If the compound string segment tag is mapped to a charset in the
registry that is neither`XmFONTLIST_DEFAULT_TAG`nor an MIT
registered charset, the converter creates the compound text for that
segment using the charset (from the registry) and the text of the
compound string segment as an "extended segment" with a variable number
of octets per character.

If the compound string segment tag is not mapped in the registry, the
result is implementation dependent.

* **`string`** 

Specifies a compound string to be converted to compound text.

## RETURN


Returns a (char *) string in compound text format.
This format is described in the X Consortium StandardCompound Text
Encoding.
The function allocates space to hold the returned string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XtFree`.
## RELATED


&cdeman.XmCvtCTToXmString;,
&cdeman.XmFontList;,
&cdeman.XmMapSegmentEncoding;,
&cdeman.XmRegisterSegmentEncoding;, andXmString.