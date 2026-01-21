# XmRenderTableCopy
library call`XmRenderTableCopy`A render table function that copies renditionsXmRenderTableCopy#include <Xm/Xm.h>XmRenderTable`XmRenderTableCopy`XmRenderTabletableXmStringTag*tagsinttag_count
## DESCRIPTION


`XmRenderTableCopy`creates a new render table
which will contain
the renditions of the`table`whose tags match those in`tags`.

* **`table`** 

Specifies the table containing the renditions to be copied.
* **`tags`** 

Specifies an array of tags, whose corresponding renditions are to be
copied. NULL indicates that the complete table should be copied.
* **`tag_count`** 

Specifies the number of tags in`tags`.

## RETURN


Returns NULL if`table`is NULL. Otherwise, this function returns
the new render table.
This function allocates space to hold the new render table.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XmRenderTableFree`.
## RELATED


&cdeman.XmRendition; and
&cdeman.XmRenderTableFree;.