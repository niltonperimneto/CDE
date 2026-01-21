# XmCvtCTToXmString
library call`XmCvtCTToXmString`A compound string function that converts compound text to a compound stringXmCvtCTToXmStringcompound string functionsXmCvtCTToXmString#include <Xm/Xm.h>XmString`XmCvtCTToXmString`char* text
## DESCRIPTION


`XmCvtCTToXmString`converts a (char *) string in compound text
format to a compound string.
The application must call`XtAppInitialize`before calling this
function.
Conversion of compound text to compound strings is implementation
dependent.

* **text** 

Specifies a string in compound text format to be converted to a compound
string.

## RETURN


Returns a compound string derived from the compound text.
The function allocates space to hold the returned compound string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XmStringFree`.
The compound text is assumed to be NULL-terminated; NULLs within the
compound text are handled correctly.
The handling of HORIZONTAL TABULATION (HT) control characters within the
compound text is undefined.
The compound text format is described in the X Consortium StandardCompound Text Encoding.
## RELATED


&cdeman.XmCvtXmStringToCT;.