# XmRegisterSegmentEncoding
library call`XmRegisterSegmentEncoding`A compound string function that registers
a compound text encoding format for a specified font list element tagXmRegisterSegment\\%Encodingcompound string functionsXmRegisterSegment\\%Encoding#include <Xm/Xm.h>char *`XmRegisterSegmentEncoding`char *fontlist_tagchar *ct_encoding
## DESCRIPTION


`XmRegisterSegmentEncoding`registers a compound text encoding format
with the specified font list element tag. The`XmCvtXmStringToCT`function uses this registry to map the font list tags of compound string
segments to compound text encoding formats. Registering
a font list tag that already exists in the registry overwrites the original
entry. You can unregister a font list tag by passing a NULL value for the`ct_encoding`parameter.

* **`fontlist_tag`** 

Specifies the font list element tag to be registered. The
tag must be a NULL-terminated ISO8859-1 string.
* **`ct_encoding`** 

Specifies the compound text character set to be used for segments
with the font list tag. The value must be a NULL-terminated ISO8859-1 string.
A value of`XmFONTLIST_DEFAULT_TAG`maps the specified font list tag to
the code set of the locale.

## RETURN


Returns NULL for a new font list tag or the old`ct_encoding`value for an already registered font list tag. The
application is responsible for freeing the
storage associated with the returned data (if any) by
calling`XtFree`.
## RELATED


&cdeman.XmCvtXmStringToCT;,
&cdeman.XmFontList;,
&cdeman.XmMapSegmentEncoding;, and
&cdeman.XmString;.