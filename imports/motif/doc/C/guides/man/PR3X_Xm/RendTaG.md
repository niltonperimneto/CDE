# XmRenderTableGetRendition
library call`XmRenderTableGetRendition`A convenience function that matches a rendition tagXmRenderTableGetRendition#include <Xm/Xm.h>XmRendition`XmRenderTableGetRendition`XmRenderTabletableXmStringTagtag
## DESCRIPTION


`XmRenderTableGetRendition`searches`table`and returns a
copy of the rendition whose`XmNtag`resource matchestag.
If no rendition matches, then NULL is returned. This function is to
be used for just one rendition match.

It is the responsibility of the caller to free the returned rendition
with the`XmRenditionFree`function.

* **`table`** 

Specifies the table containing renditions to be searched.
* **tag** 

Specifies the tag to search for.

## RETURN


Returns NULL if there is no match; otherwise, this function returns a
newXmRendition.
## RELATED


&cdeman.XmRenderTableGetRenditions;,
&cdeman.XmRenderTableGetTags;, and
&cdeman.XmRendition;.