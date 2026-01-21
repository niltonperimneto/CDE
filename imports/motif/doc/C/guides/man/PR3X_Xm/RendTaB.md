# XmRenderTableAddRenditions
library call`XmRenderTableAddRenditions`Creates a new render tableXmRenderTableAddRenditions#include <Xm/Xm.h>XmRenderTable`XmRenderTableAddRenditions`XmRenderTableoldtableXmRendition*renditionsCardinalrendition_countXmMergeModemerge_mode
## DESCRIPTION


`XmRenderTableAddRenditions`is a function to create a new render
table that
includes the renditions listed in`oldtable`, if there is one. This
function also copies
specified renditions (`renditions`) to the new render table. The first`rendition_count`renditions of the`renditions`array are
added to the new table. If a rendition is tagged with a tag
that matches a tag already in`oldtable`, then the existing
rendition using that tag is either modified or freed and replaced with
the new rendition, depending on the value of`merge_mode`. If`oldtable`is NULL,`XmRenderTableAddRenditions`creates a new render table containing
only the specified renditions.

This function deallocates the original render table after extracting
the required information. It is the responsibility of the caller to
free the renditions of the`renditions`array by calling the`XmRenditionFree`function.

* **`oldtable`** 

Specifies the render table to be added to.
* **`renditions`** 

Specifies an array of renditions to be added.
* **`rendition_count`** 

Specifies the number of renditions from`renditions`to be added.
* **`merge_mode`** 

Specifies what to do if the`XmNtag`of a rendition matches that
of one that already exists in`oldtable`. The possible values
are as follows:

* **`XmMERGE_REPLACE`** 

Completely replaces the old rendition with the new one.
* **`XmMERGE_OLD`** 

Replaces any unspecified values of the old rendition with the
corresponding values from the new rendition.
* **`XmMERGE_NEW`** 

Replaces the old rendition with the new rendition, replacing any
unspecified values of the new rendition with the corresponding values
from the old rendition.
* **`XmSKIP`** 

Skips over the new rendition, leaving the old rendition intact.


## RETURN


If`renditions`is NULL or`rendition_count`is 0 (zero), this
function returns`oldtable`.
Otherwise, the function returns a newXmRenderTable.
The function allocates space to hold this new render table.
The application is responsible for managing this allocated space.
The application can recover the allocated space by calling`XmRenderTableFree`.
## RELATED


&cdeman.XmRendition; and
&cdeman.XmRenderTableFree;.