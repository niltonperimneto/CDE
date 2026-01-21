# XmRenderTableRemoveRenditions
library call`XmRenderTableRemoveRenditions`A convenience function that removes renditionsXmRenderTableRemoveRenditions#include <Xm/Xm.h>XmRenderTable`XmRenderTableRemoveRenditions`XmRenderTableoldtableXmStringTag*tagsinttag_count
## DESCRIPTION


`XmRenderTableRemoveRenditions`removes from`oldtable`the
renditions whose tags match the tags specified in`tags`, then
places the remaining renditions in a newly created render table.

* **`oldtable`** 

Specifies the render table from which renditions are to be removed.
This function deallocates the original render table and the matching
renditions after extracting the required information.
* **`tags`** 

Specifies an array of tags, whose corresponding renditions are to be
removed from`oldtable`.
* **`tag_count`** 

Specifies the number of tags in`tags`.

## RETURN


If`oldtable`or`tags`is NULL, or`tag_count`is 0
(zero), or no renditions are removed from`oldtable`, this
function returns`oldtable`. Otherwise, it returns a newly
allocatedXmRenderTable.
The application is responsible for managing this allocated render table.
The application can recover this allocated space by calling`XmRenderTableFree`.
## RELATED


&cdeman.XmRendition; and
&cdeman.XmRenderTableFree;.