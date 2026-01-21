# XmDropTransfer
library call`XmDropTransfer`The DropTransfer widget classXmDropSitewidget classDropTransfer&npzwc;#include &lt;Xm/DragDrop.h>
## DESCRIPTION


DropTransfer provides a set of resources that identifies
the procedures and associated information required by the
toolkit in order to process and complete a drop transaction.
Clients should not explicitly create a DropTransfer widget.
Instead, a client initiates a transfer by calling`XmDropTransferStart`, which initializes and returns a
DropTransfer widget. If this function is called within an`XmNdropProc`callback, the actual transfers are initiated
after the callback returns. Even if no data needs to be transferred,`XmDropTransferStart`needs to be called (typically with
no arguments, or just setting`XmNtransferStatus`) to finish
the drag and drop transaction.

The`XmNdropTransfers`resource specifies a transfer
list that describes the requested target types for the source
data. A transfer list is an array ofXmDropTransferEntryRecstructures, each of which identifies a target type. The
transfer list is analogous to the MULTIPLE selections capability
defined in theInter-Client Communication Conventions Manual(ICCCM).

The DropTransfer resource,`XmNtransferProc`, specifies a
transfer procedure of type XtSelectionCallbackProc that
delivers the requested selection data. This procedure operates
in conjunction with the underlying Xt selection capabilities and
is called for each target in the transfer list. Additional target
types can be requested after a transfer is initiated by calling
the`XmDropTransferAdd`function.
### Structures


AnXmDropTransferEntryis a pointer to the following structure of
typeXmDropTransferEntryRec, which identifies a selection
target associated with a given drop transaction:typedef struct
{
        XtPointer`client_data``;
        Atom``target``;
} XmDropTransferEntryRec, *XmDropTransferEntry;`

* **`client_data`** 

Specifies any additional information required
by this selection target
* **`target`** 

Specifies a selection target associated with the drop
operation

### Classes


DropTransfer inherits behavior and a resource from`Object`.

The class pointer is`xmDropTransferObjectClass`.

The class name is`XmDropTransfer`.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. The programmer can also set the
resource values for the inherited classes to set attributes for
this widget. To reference a resource by name or by class in
a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use
the remaining letters. To specify one of the defined values for a
resource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include
any underscores between words). The codes in the access column
indicate if the given resource can be set at creation time (C),
set by using`XtSetValues`(S), retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmDropTransfer Resource Set``Name``Class``Type``Default``Access`XmNdropTransfersXmCDropTransfersXmDropTransferEntryRec *NULLCGXmNincrementalXmCIncrementalBooleanFalseCSGXmNnumDropTransfersXmCNumDropTransfersCardinal0CSGXmNtransferProcXmCTransferProcXtSelectionCallbackProcNULLCSGXmNtransferStatusXmCTransferStatusunsigned charXmTRANSFER_SUCCESSCSG

* **`XmNdropTransfers`** 

Specifies the address of an array of drop transfer entry records. The
drop transfer is complete when all the entries in the list have been
processed.
* **`XmNincremental`** 

Specifies a Boolean value that indicates whether the transfer on the
receiver side uses the Xt incremental selection transfer mechanism
described inX Toolkit Intrinsics&mdash;C Language Interface.
If the value is True, the receiver uses incremental transfer; if the
value is False, the receiver uses atomic transfer.
* **`XmNnumDropTransfers`** 

Specifies the number of entries in`XmNdropTransfers`. If
this resource is set to 0 at any time, the transfer is considered
complete. The value of`XmNtransferStatus`determines the
completion handshaking process.
* **`XmNtransferProc`** 

Specifies a procedure of type`XtSelectionCallbackProc`that
delivers the requested selection values.
The`widget`argument passed to this procedure is the DropTransfer
widget.
The selection atom passed is _MOTIF_DROP.
For additional information on selection callback procedures, seeX
Toolkit Intrinsics&mdash;C Language Interface.
* **`XmNtransferStatus`** 

Specifies the current status of the drop transfer. The client
updates this value when the transfer ends and communicates
the value to the initiator. The possible values are

* **`XmTRANSFER_SUCCESS`** 

The transfer succeeded.
* **`XmTRANSFER_FAILURE`** 

The transfer failed.


### Inherited Resources


DropTransfer inherits behavior and a resource from`Object`.
For a complete description of this resource, refer
to the`Object`reference page.

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
## RELATED INFORMATION


&cdeman.Object;,
&cdeman.XmDisplay;,
&cdeman.XmDragContext;,
&cdeman.XmDragIcon;,
&cdeman.XmDropSite;,
&cdeman.XmDropTransferAdd;, and
&cdeman.XmDropTransferStart;.