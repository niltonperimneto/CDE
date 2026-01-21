# XmDropTransferAdd
library call`XmDropTransferAdd`A Drag and Drop function that enables
additional drop transfer entries to be processed after initiating
a drop transferXmDropTransferAddDrag and Drop functionsXmDropTransferAdd#include <Xm/DragDrop.h>void`XmDropTransferAdd`Widgetdrop_transferXmDropTransferEntryRec *transfersCardinalnum_transfers
## DESCRIPTION


`XmDropTransferAdd`identifies a list of additional drop transfer
entries to be processed after a drop transfer is started.

* **`drop_transfer`** 

Specifies the ID of the DropTransfer widget returned by`XmDropTransferStart`
* **`transfers`** 

Specifies the additional drop transfer entries that the receiver
wants processed
* **`num_transfers`** 

Specifies the number of items in the`transfers`array


For a complete definition of DropTransfer and its associated resources,
see &cdeman.XmDropTransfer;.
## RELATED


&cdeman.XmDragContext;,
&cdeman.XmDropTransfer;, and
&cdeman.XmDropTransferStart;.