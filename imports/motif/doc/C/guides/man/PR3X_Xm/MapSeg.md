# XmMapSegmentEncoding
library call`XmMapSegmentEncoding`A compound string function that returns
the compound text encoding format associated with the specified font list tagXmMapSegmentEncodingcompound string functionsXmMapSegmentEncoding#include <Xm/Xm.h>char *`XmMapSegmentEncoding`char *fontlist_tag
## DESCRIPTION


`XmMapSegmentEncoding`searches the segment encoding registry for
an entry that matches the specified font list tag and returns a copy
of the associated compound text encoding format. The application is
responsible for freeing the storage associated with the returned data
by calling`XtFree`.

* **`fontlist_tag`** 

Specifies the compound string font list tag

## RETURN


Returns a copy of the associated compound text encoding format
if the font list tag is found in the registry; otherwise, returns NULL.
## RELATED


&cdeman.XmCvtXmStringToCT;,
&cdeman.XmFontList;,
&cdeman.XmRegisterSegmentEncoding;, and
&cdeman.XmString;.