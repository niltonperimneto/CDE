# XmRenderTableGetRenditions
library call`XmRenderTableGetRenditions`A convenience function that matches rendition tagsXmRenderTableGetRenditions#include <Xm/Xm.h>XmRendition`*XmRenderTableGetRenditions`XmRenderTabletableXmStringTag*tagsCardinaltag_count
## DESCRIPTION


`XmRenderTableGetRenditions`searches`table`and returns an
array of
copies of the renditions whose`XmNtag`resources match a tag
in`tags`.
If no renditions match, then NULL is returned. The size of the
returned array is`tag_count`. The`XmNtag`resource of each
rendition will match the corresponding tag in`tags`. If no match
is found for a particular tag, the corresponding slot in the return
value will be NULL.

It is the responsibility of the caller to call the`XmRenditionFree`function to free the new renditions, and the`XtFree`function to free the array.

* **`table`** 

Specifies the table containing renditions to be searched.
* **`tags`** 

Specifies the tags to search for.
* **`tag_count`** 

Specifies the number of tags in`tags`.

## RETURN


Returns NULL if there is no match; otherwise, this function returns an
array of
newXmRenditions.
## RELATED


&cdeman.XmRenderTableGetRendition;,
&cdeman.XmRenderTableGetTags;, and
&cdeman.XmRendition;.