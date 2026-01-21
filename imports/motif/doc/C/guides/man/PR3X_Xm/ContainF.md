# XmContainerPaste
library call`XmContainerPaste`Container widget function to insert items from the
clipboardXmContainerPasteXmContainer#include <Xm/Container.h>Boolean`XmContainerPaste`Widgetcontainer
## DESCRIPTION


`XmContainerPaste`requests data transfer from the clipboard
selection to the Container.
This routine calls the widget's`XmNdestinationCallback`procedures
with the`selection`member of theXmDestinationCallbackStructset to`CLIPBOARD`and with the`operation`member set to`XmCOPY`.
The Container widget itself performs no transfers; the`XmNdestinationCallback`procedures are responsible for inserting
the clipboard selection and for taking any related actions.

* **`container`** 

Specifies the Container widget ID.


For a complete definition of Container and its associated resources, see
&cdeman.XmContainer;.
## RETURN


The function returns False if no data transfer takes place.
Otherwise, it returns True.
## RELATED


&cdeman.XmContainer;.