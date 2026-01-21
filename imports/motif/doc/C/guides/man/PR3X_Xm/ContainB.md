# XmContainerCopy
library call`XmContainerCopy`Container widget function to copy primary selection
to the clipboardXmContainerCopyXmContainer#include <Xm/Container.h>Boolean`XmContainerCopy`WidgetcontainerTimetimestamp
## DESCRIPTION


`XmContainerCopy`copies the primary selected container items to the
clipboard.
This routine calls the`XmNconvertCallback`procedures, possibly
multiple times, with the`selection`member of theXmConvertCallbackStructset to`CLIPBOARD`and with the`parm`member set to`XmCOPY`.

* **`container`** 

Specifies the Container widget ID.
* **`timestamp`** 

Specifies the server time at which to modify the selection value.


For a complete definition of Container and its associated resources, see
&cdeman.XmContainer;.
## RETURN


The function returns False in the following cases: if the primary selection
is NULL, if the widget does not own the primary selection, or if the function
is unable to gain ownership of the clipboard selection.
Otherwise, it returns True.
## RELATED


&cdeman.XmContainer;.