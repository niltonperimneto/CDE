# XmDragContext
library call`XmDragContext`The DragContext widget classXmDragContextwidget classDragContext&npzwc;#include &lt;Xm/DragDrop.h>
## DESCRIPTION


DragContexts are special widgets used in drag and drop transactions.
A DragContext is implemented as a widget, but a client does not
explicitly create a DragContext widget. Instead, a client initiates
a drag and drop transaction by calling`XmDragStart`, and this
routine initializes and returns a DragContext widget. There is a
unique DragContext for each drag operation. The toolkit frees a
DragContext when a transaction is complete; therefore, an application
programmer should not explicitly destroy a DragContext.

Initiator and receiver clients both use DragContexts to track
the state of a transaction. When the initiator and receiver of
a transaction are in the same client, they share the same
DragContext instance. If they are in different clients, there
are two separate DragContexts. In this case, the initiator calls`XmDragStart`and the toolkit provides a DragContext for the
receiver client. The only resources pertinent to the receiver
are`XmNexportTargets`and`XmNnumExportTargets`. These
can both be passed as arguments to the`XmDropSiteRetrieve`function to obtain information about the current drop site.

In general, in order to receive data, a drop site must share at least
one target type and operation in common with a drag source. The
DragContext resource,`XmNexportTargets`, identifies the selection
targets for the drag source. These export targets are compared with the`XmNimportTargets`resource list specified by a drop site.
The DragContext resource,`XmNdragOperations`, identifies the
valid operations that can be applied to the source data by the
initiator. The drop site counterpart resource is`XmNdropSiteOperations`, which indicates a drop site's supported
operations.

A client uses DragIcon widgets to define the drag-over animation
effects associated with a given drag and drop transaction.
An initiator specifies a set of drag icons, selects a blending
model, and sets foreground and background cursor colors with
DragContext resources.

The type of drag-over visual used to represent a drag operation
depends on the drag protocol style. In preregister mode, the server
is grabbed, and either a cursor or a pixmap may be used as a drag-over
visual. In dynamic mode, drag-over visuals must be
implemented with the X cursor. If the resulting drag protocol style is
Drop Only or None and the`XmNdragInitiatorProtocolStyle`is`XmDRAG_DYNAMIC`or`XmDRAG_PREFER_DYNAMIC`,
then a dynamic visual style (cursor) is used. Otherwise, a preregister
visual style is used.
### Classes


DragContext inherits behavior and resources from`Core`.

The class pointer is`xmDragContextClass`.

The class name is`XmDragContext`.
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
set by using XtSetValues (S), retrieved by using
XtGetValues (G), or is not applicable (N/A).

`XmDragContext Resource Set``Name``Class``Type``Default``Access`XmNblendModelXmCBlendModelunsigned charXmBLEND_ALLCGXmNclientDataXmCClientDataXtPointerNULLCSGXmNconvertProcXmCConvertProcXtConvertSelectionIncrProcNULLCSGXmNcursorBackgroundXmCCursorBackgroundPixeldynamicCSGXmNcursorForegroundXmCCursorForegroundPixeldynamicCSGXmNdragDropFinishCallbackXmCCallbackXtCallbackListNULLCSGXmNdragMotionCallbackXmCCallbackXtCallbackListNULLCXmNdragOperationsXmCDragOperationsunsigned charXmDROP_COPY | XmDROP_MOVECXmNdropFinishCallbackXmCCallbackXtCallbackListNULLCXmNdropSiteEnterCallbackXmCCallbackXtCallbackListNULLCXmNdropSiteLeaveCallbackXmCCallbackXtCallbackListNULLCXmNdropStartCallbackXmCCallbackXtCallbackListNULLCXmNexportTargetsXmCExportTargetsAtom *NULLCSGXmNincrementalXmCIncrementalBooleanFalseCSGXmNinvalidCursorForegroundXmCCursorForegroundPixeldynamicCSGXmNnoneCursorForegroundXmCCursorForegroundPixeldynamicCSGXmNnumExportTargetsXmCNumExportTargetsCardinal0CSGXmNoperationChangedCallbackXmCCallbackXtCallbackListNULLCXmNoperationCursorIconXmCOperationCursorIconWidgetdynamicCSGXmNsourceCursorIconXmCSourceCursorIconWidgetdynamicCSGXmNsourcePixmapIconXmCSourcePixmapIconWidgetdynamicCSGXmNstateCursorIconXmCStateCursorIconWidgetdynamicCSGXmNtopLevelEnterCallbackXmCCallbackXtCallbackListNULLCXmNtopLevelLeaveCallbackXmCCallbackXtCallbackListNULLCXmNvalidCursorForegroundXmCCursorForegroundPixeldynamicCSG

* **`XmNblendModel`** 

Specifies which combination of DragIcons are blended to produce
a drag-over visual.

* **`XmBLEND_ALL`** 

Blends all three DragIcons: the source, state and operation icons.
The icons are layered from top to bottom with the operation icon
on top and the source icon on the bottom.
The hotspot is derived from the state icon.
* **`XmBLEND_STATE_SOURCE`** 

Blends the state and source icons only. The hotspot is derived
from the state icon.
* **`XmBLEND_JUST_SOURCE`** 

Specifies that only the source icon is used, which the initiator
updates as required.
* **`XmBLEND_NONE`** 

Specifies that no drag-over visual is generated. The client
tracks the drop site status through callback routines and updates
the drag-over visuals as necessary.

* **`XmNclientData`** 

Specifies the client data to be passed to`XmNconvertProc`when it is invoked.
* **`XmNconvertProc`** 

If`XmNincremental`is True, specifies a procedure of type`XtConvertSelectionIncrProc`that
converts the source data to the format(s) requested by the receiver
client.
The`widget`argument passed to this procedure is the DragContext
widget.
The selection atom passed is _MOTIF_DROP.
If`XmNincremental`is False, the procedure is an`XtConvertSelectionProc`, and should ignore the`max_length`,`client_data`, and`request_id`arguments and
should handle the conversion atomically.
Data returned by`XmNconvertProc`must be allocated using`XtMalloc`, and will be freed automatically by the toolkit after the
transfer.
For additional information on selection conversion procedures, seeX
Toolkit Intrinsics&mdash;C Language Interface.
* **`XmNcursorBackground`** 

Specifies the background pixel value of the cursor.
* **`XmNcursorForeground`** 

Specifies the foreground pixel value of the cursor when the state icon
is not blended. This resource defaults to the foreground color of the
widget passed to the`XmDragStart`function.
* **`XmNdragDropFinishCallback`** 

Specifies the list of callbacks that are called when the transaction is
completed. The type of the structure whose address is passed to this
callback isXmDragDropFinishCallbackStruct. The reason sent by
the callback is`XmCR_DRAG_DROP_FINISH`.
* **`XmNdragMotionCallback`** 

Specifies the list of callbacks that are invoked when the pointer moves.
The type of structure whose address is passed to this callback isXmDragMotionCallbackStruct. The reason sent by the callback
is`XmCR_DRAG_MOTION`.
* **`XmNdragOperations`** 

Specifies the set of valid operations associated with an initiator
client for a drag transaction.
This resource is a bit mask that is formed by combining one or
more of the following values using a bitwise operation such as
inclusive OR (|):`XmDROP_COPY`,`XmDROP_LINK`,`XmDROP_MOVE`.
The value`XmDROP_NOOP`for this resource indicates that no
operations are valid.
For Text and TextField widgets, this resource is set to`XmDROP_COPY`|`XmDROP_MOVE`; for List widgets, it is set to`XmDROP_COPY`.
* **`XmNdropFinishCallback`** 

Specifies the list of callbacks that are invoked when the drop
is completed. The type of the structure whose address is passed to
this callback isXmDropFinishCallbackStruct. The reason sent
by the callback is`XmCR_DROP_FINISH`.
* **`XmNdropSiteEnterCallback`** 

Specifies the list of callbacks that are invoked when the pointer enters
a drop site. The type of the structure whose address is passed to this
callback isXmDropSiteEnterCallbackStruct. The reason sent by the
callback is`XmCR_DROP_SITE_ENTER`.
* **`XmNdropSiteLeaveCallback`** 

Specifies the list of callbacks that are invoked when the pointer leaves
a drop site. The type of the structure whose address is passed to this
callback isXmDropSiteLeaveCallbackStruct. The reason sent by
the callback is`XmCR_DROP_SITE_LEAVE`.
* **`XmNdropStartCallback`** 

Specifies the list of callbacks that are invoked when a drop is
initiated. The type of the structure whose address is passed to this
callback isXmDropStartCallbackStruct. The reason sent by the
callback is`XmCR_DROP_START`.
* **`XmNexportTargets`** 

Specifies the list of target atoms associated with this source.
This resource identifies the selection targets this source
can be converted to.
* **`XmNincremental`** 

Specifies a Boolean value that indicates whether the transfer on the
initiator side uses the Xt incremental selection transfer mechanism
described inX Toolkit Intrinsics&mdash;C Language Interface.
If the value is True, the initiator uses incremental transfer; if the
value is False, the initiator uses atomic transfer.
* **`XmNinvalidCursorForeground`** 

Specifies the foreground pixel value of the cursor when the state
is invalid. This resource defaults to the value of the`XmNcursorForeground`resource.
* **`XmNnoneCursorForeground`** 

Specifies the foreground pixel value of the cursor when the state
is none. This resource defaults to the value of the`XmNcursorForeground`resource.
* **`XmNnumExportTargets`** 

Specifies the number of entries in the list of export targets.
* **`XmNoperationChangedCallback`** 

Specifies the list of callbacks that are invoked when the drag
is started and when the user requests that a different operation
be applied to the drop.
The type of the structure whose address is passed to this callback
isXmOperationChangedCallbackStruct. The reason sent by the
callback is`XmCR_OPERATION_CHANGED`.
* **`XmNoperationCursorIcon`** 

Specifies the cursor icon used to designate the type of operation
performed by the drag transaction. If NULL,`XmScreen`resources provide default icons for copy, link, and move
operations.
* **`XmNsourceCursorIcon`** 

Specifies the cursor icon used to represent the source when
a dynamic visual style is used. If NULL, the`XmNdefaultSourceCursorIcon`resource of`XmScreen`provides
a default cursor icon.
* **`XmNsourcePixmapIcon`** 

Specifies the pixmap icon used to represent the source when
a preregister visual style is used. The icon is used in conjunction
with the colormap of the widget passed to`XmDragStart`.
If NULL,`XmNsourceCursorIcon`is used.
* **`XmNstateCursorIcon`** 

Specifies the cursor icon used to designate the state of a drop site.
If NULL,`XmScreen`resources provide default icons for a valid,
invalid, and no drop site condition.
* **`XmNtopLevelEnterCallback`** 

Specifies the list of callbacks that are called when the pointer enters
a top-level window or root window (due to changing screens). The type
of the structure whose address is passed to this callback isXmTopLevelEnterCallbackStruct. The reason sent by the
callback is`XmCR_TOP_LEVEL_ENTER`.
* **`XmNtopLevelLeaveCallback`** 

Specifies the list of callbacks that are called when the pointer
leaves a top level window or the root window (due to changing
screens). The type of the structure whose address is
passed to this callback isXmTopLevelLeaveCallbackStruct. The
reason sent by the callback is`XmCR_TOP_LEVEL_LEAVE`.
* **`XmNvalidCursorForeground`** 

Specifies the foreground pixel value of the cursor designated as a
valid cursor icon.

### Inherited Resources


DragContext inherits behavior and resources from the superclass
described in the following table.
For a complete description of each resource, refer
to the`Core`reference page.

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


Each of the DragContext callbacks has an associated callback
structure.

A pointer to the following structure is passed to the`XmNdragDropFinishCallback`callback:typedef struct
{
        int`reason`;
        XEvent  *`event`;
        Time`timeStamp`;
}XmDragDropFinishCallbackStruct, *XmDragDropFinishCallback;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`timeStamp`** 

Specifies the time at which either the drag or the drop was completed


A pointer to the following structure is passed to callbacks for`XmNdragMotionCallback`:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
        unsigned char`operation`;
        unsigned char`operations`;
        unsigned char`dropSiteStatus`;
        Positionx;
        Positiony;
}XmDragMotionCallbackStruct, *XmDragMotionCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the timestamp of the logical event.
* **`operation`** 

Identifies an operation.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`operation`to the value of the`operation`member of theXmDragProcCallbackStructat the time the DropSite's`XmNdragProc`returns.

If the toolkit has not called an`XmNdragProc`and the pointer is
within an active drop site, the toolkit initializes`operation`by
selecting an operation from the bitwise AND of the initial value of the`operations`member and the value of the DropSite's`XmNdropSiteOperations`resource.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If the toolkit finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.

If the toolkit has not called an`XmNdragProc`and the pointer is
not within an active drop site, the toolkit initializes`operation`by selecting an operation from the initial value of the`operations`member.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If the toolkit finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.
* **`operations`** 

Indicates the set of operations supported for the source data.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`operations`to the bitwise AND of the
DropSite's`XmNdropOperations`and the value of the`operations`member of theXmDragProcCallbackStructat the time the DropSite's`XmNdragProc`returns.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.

If the toolkit has not called an`XmNdragProc`and the user does not
select an operation (by pressing a modifier key), the toolkit
initializes`operations`to the value of the DragContext's`XmNdragOperations`resource.

If the toolkit has not called an`XmNdragProc`and the user does
select an operation, the toolkit initializes`operations`to the
bitwise AND of the corresponding operation and the value of the
DragContext's`XmNdragOperations`resource.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.
* **`dropSiteStatus`** 

Indicates whether or not a drop site is valid.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`dropSiteStatus`to the value of the`dropSiteStatus`member of theXmDragProcCallbackStructat the
time the DropSite's`XmNdragProc`returns.

If the toolkit has not called an`XmNdragProc`, it initializes`dropSiteStatus`as follows:
the toolkit initializes`dropSiteStatus`to`XmNO_DROP_SITE`if
the pointer is over an inactive drop site or is not over a drop site.
The toolkit initializes`dropSiteStatus`to`XmDROP_SITE_VALID`if all the following conditions are met:

The pointer is over an active drop site.

The DragContext's`XmNexportTargets`and the DropSite's`XmNimportTargets`are compatible.

The initial value of the`operation`member is not`XmDROP_NOOP`.

Otherwise, the toolkit initializes`dropSiteStatus`to`XmDROP_SITE_INVALID`.


A pointer to the following structure is passed for the`XmNdropFinishCallback`callback:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
        unsigned char`operation`;
        unsigned char`operations`;
        unsigned char`dropSiteStatus`;
        unsigned char`dropAction`;
        unsigned char`completionStatus`;
}XmDropFinishCallbackStruct, *XmDropFinishCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the time at which the drop was completed.
* **`operation`** 

Identifies an operation.

If the pointer is over an active drop site when the drop begins, the
toolkit initializes`operation`to the value of the`operation`member of theXmDropProcCallbackStructat the time the DropSite's`XmNdropProc`returns.

If the pointer is not over an active drop site when the drop begins, the
toolkit initializes`operation`by selecting an operation from the
initial value of the`operations`member.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If it finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.
* **`operations`** 

Indicates the set of operations supported for the source data.

If the pointer is over an active drop site when the drop begins, the
toolkit initializes`operations`to the bitwise AND of the
DropSite's`XmNdropOperations`and the value of the`operations`member of theXmDropProcCallbackStructat the time the DropSite's`XmNdropProc`returns.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.

If the pointer is not over an active drop site when the drop begins and
if the user does not select an operation (by pressing a modifier key),
the toolkit initializes`operations`to the value of the
DragContext's`XmNdragOperations`resource.

If the pointer is not over an active drop site when the drop begins and
if the user does select an operation, the toolkit initializes`operations`to the bitwise AND of the corresponding operation and
the value of the DragContext's`XmNdragOperations`resource.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.
* **`dropSiteStatus`** 

Indicates whether or not a drop site is valid.

If the pointer is over an active drop site when the drop begins, the
toolkit initializes`dropSiteStatus`to the value of the`dropSiteStatus`member of theXmDropProcCallbackStructat the
time the DropSite's`XmNdropProc`returns.

If the pointer is not over an active drop site when the drop begins, the
toolkit initializes`dropSiteStatus`to`XmNO_DROP_SITE`.
* **`dropAction`** 

Identifies the drop action. The values are`XmDROP`,`XmDROP_CANCEL`,`XmDROP_HELP`, and`XmDROP_INTERRUPT`.
The`XmDROP_INTERRUPT`value is currently unsupported; if
specified, it will be interpreted as an`XmDROP_CANCEL`.
* **`completionStatus`** 

An IN/OUT member that indicates the status of the drop action.
After the last callback procedure has returned, the final value of this
member determines what visual transition effects will be applied.
There are two values:

* **`XmDROP_SUCCESS`** 

The drop was successful.
* **`XmDROP_FAILURE`** 

The drop was unsuccessful.



A pointer to the following structure is passed to callbacks for`XmNdropSiteEnterCallback`:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
        unsigned char`operation`;
        unsigned char`operations`;
        unsigned char`dropSiteStatus`;
        Positionx;
        Positiony;
}XmDropSiteEnterCallbackStruct, *XmDropSiteEnterCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the time the crossing event occurred.
* **`operation`** 

Identifies an operation.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`operation`to the value of the`operation`member of theXmDragProcCallbackStructat the time the DropSite's`XmNdragProc`returns.

If the toolkit has not called an`XmNdragProc`, it initializes`operation`by selecting an operation from the bitwise AND of the
initial value of the`operations`member and the value of the
DropSite's`XmNdropSiteOperations`resource.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If the toolkit finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.
* **`operations`** 

Indicates the set of operations supported for the source data.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`operations`to the bitwise AND of the
DropSite's`XmNdropOperations`and the value of the`operations`member of theXmDragProcCallbackStructat the time the DropSite's`XmNdragProc`returns.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.

If the toolkit has not called an`XmNdragProc`and the user does not
select an operation (by pressing a modifier key), the toolkit
initializes`operations`to the value of the DragContext's`XmNdragOperations`resource.

If the toolkit has not called an`XmNdragProc`and the user does
select an operation, the toolkit initializes`operations`to the
bitwise AND of the corresponding operation and the value of the
DragContext's`XmNdragOperations`resource.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.
* **`dropSiteStatus`** 

Indicates whether or not a drop site is valid.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`dropSiteStatus`to the value of the`dropSiteStatus`member of theXmDragProcCallbackStructat the
time the DropSite's`XmNdragProc`returns.

If the toolkit has not called`XmNdragProc`, it initializes`dropSiteStatus`to`XmDROP_SITE_VALID`if the DragContext's`XmNexportTargets`and the DropSite's`XmNimportTargets`are compatible and if the initial value of the`operation`member is not`XmDROP_NOOP`.
Otherwise, the toolkit initializes`dropSiteStatus`to`XmDROP_SITE_INVALID`.
* **x** 

Indicates the x-coordinate of the pointer in root window coordinates.
* **y** 

Indicates the y-coordinate of the pointer in root window coordinates.


A pointer to the following structure is passed to callbacks for`XmNdropSiteLeaveCallback`:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
}XmDropSiteLeaveCallbackStruct, *XmDropSiteLeaveCallback;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`timeStamp`** 

Specifies the timestamp of the logical event


A pointer to the following structure is passed for the`XmNdropStartCallback`callback:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
        unsigned char`operation`;
        unsigned char`operations`;
        unsigned char`dropSiteStatus`;
        unsigned char`dropAction`;
        Positionx;
        Positiony;
}XmDropStartCallbackStruct, *XmDropStartCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the time at which the drag was completed.
* **`operation`** 

Identifies an operation.

If the pointer is over an active drop site when the drop begins, the
toolkit initializes`operation`to the value of the`operation`member of theXmDropProcCallbackStructat the time the DropSite's`XmNdropProc`returns.

If the pointer is not over an active drop site when the drop begins, the
toolkit initializes`operation`by selecting an operation from the
initial value of the`operations`member.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If it finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.
* **`operations`** 

Indicates the set of operations supported for the source data.

If the pointer is over an active drop site when the drop begins, the
toolkit initializes`operations`to the bitwise AND of the
DropSite's`XmNdropOperations`and the value of the`operations`member of theXmDropProcCallbackStructat the time the DropSite's`XmNdropProc`returns.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.

If the pointer is not over an active drop site when the drop begins and
if the user does not select an operation (by pressing a modifier key),
the toolkit initializes`operations`to the value of the
DragContext's`XmNdragOperations`resource.

If the pointer is not over an active drop site when the drop begins and
if the user does select an operation, the toolkit initializes`operations`to the bitwise AND of the corresponding operation and
the value of the DragContext's`XmNdragOperations`resource.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.
* **`dropSiteStatus`** 

Indicates whether or not a drop site is valid.

If the pointer is over an active drop site when the drop begins, the
toolkit initializes`dropSiteStatus`to the value of the`dropSiteStatus`member of theXmDropProcCallbackStructat the
time the DropSite's`XmNdropProc`returns.

If the pointer is not over an active drop site when the drop begins, the
toolkit initializes`dropSiteStatus`to`XmNO_DROP_SITE`.

This field is invalid if the`dropAction`field is set to`XmDROP_CANCEL`.
* **`dropAction`** 

An IN/OUT member that identifies the drop action.
The values are`XmDROP`,`XmDROP_CANCEL`,`XmDROP_HELP`,
and`XmDROP_INTERRUPT`. The value of`dropAction`can be
modified to change the action actually initiated.
The value`XmDROP_INTERRUPT`is currently unsupported; if
specified, it will be interpreted as an`XmDROP_CANCEL`.
* **x** 

Indicates the x-coordinate of the pointer in root window coordinates.
* **y** 

Indicates the y-coordinate of the pointer in root window coordinates.


A pointer to the following structure is passed to the`XmNoperationChangedCallback`callback:typedef struct
{
        int`reason`;
        XEvent  *`event`;
        Time`timeStamp`;
        unsigned char`operation`;
        unsigned char`operations`;
        unsigned char`dropSiteStatus`;
}XmOperationChangedCallbackStruct, *XmOperationChangedCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the time at which the crossing event occurred.
* **`operation`** 

Identifies an operation.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`operation`to the value of the`operation`member of theXmDragProcCallbackStructat the time the DropSite's`XmNdragProc`returns.

If the toolkit has not called an`XmNdragProc`, and the pointer is
within an active drop site, the toolkit initializes`operation`by
selecting an operation from the bitwise AND of the initial value of the`operations`member and the value of the DropSite's`XmNdropSiteOperations`resource.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If the toolkit finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.

If the toolkit has not called an`XmNdragProc`, and the pointer is
not within an active drop site, the toolkit initializes`operation`by selecting an operation from the initial value of the`operations`member.
The toolkit searches this set first for`XmDROP_MOVE`, then for`XmDROP_COPY`, then for`XmDROP_LINK`, and initializes`operation`to the first operation it finds in the set.
If the toolkit finds none of these operations in the set, it initializes`operation`to`XmDROP_NOOP`.
* **`operations`** 

Indicates the set of operations supported for the source data.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`operations`to the bitwise AND of the
DropSite's`XmNdropOperations`and the value of the`operations`member of theXmDragProcCallbackStructat the time the DropSite's`XmNdragProc`returns.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.

If the toolkit has not called an`XmNdragProc`, and the user does not
select an operation (by pressing a modifier key), the toolkit
initializes`operations`to the value of the DragContext's`XmNdragOperations`resource.

If the toolkit has not called an`XmNdragProc`, and the user does
select an operation, the toolkit initializes`operations`to the
bitwise AND of the corresponding operation and the value of the
DragContext's`XmNdragOperations`resource.
If the resulting set of operations is empty, the toolkit initializes`operations`to`XmDROP_NOOP`.
* **`dropSiteStatus`** 

Indicates whether or not a drop site is valid.

If the toolkit has just called a DropSite's`XmNdragProc`, the
toolkit initializes`dropSiteStatus`to the value of the`dropSiteStatus`member of theXmDragProcCallbackStructat the
time the DropSite's`XmNdragProc`returns.

If the toolkit has not called an`XmNdragProc`it initializes`dropSiteStatus`to`XmNO_DROP_SITE`if
the pointer is over an inactive drop site or is not over a drop site.
The toolkit initializes`dropSiteStatus`to`XmDROP_SITE_VALID`if all the following conditions are met:

The pointer is over an active drop site

The DragContext's`XmNexportTargets`and the DropSite's`XmNimportTargets`are compatible

The initial value of the`operation`member is not`XmDROP_NOOP`

Otherwise, the toolkit initializes`dropSiteStatus`to`XmDROP_SITE_INVALID`.


A pointer to the following structure is passed to callbacks for`XmNtopLevelEnterCallback`:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Time`timeStamp`;
        Screen`screen`;
        Window`window`;
        Positionx;
        Positiony;
        unsigned char`dragProtocolStyle`;
}XmTopLevelEnterCallbackStruct, *XmTopLevelEnterCallback;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
* **`timeStamp`** 

Specifies the timestamp of the logical event.
* **`screen`** 

Specifies the screen associated with the top-level window or root
window being entered.
* **`window`** 

Specifies the ID of the top-level window or root window being entered.
* **x** 

Indicates the x-coordinate of the pointer in root window coordinates.
* **y** 

Indicates the y-coordinate of the pointer in root window coordinates.
* **`dragProtocolStyle`** 

Specifies the protocol style adopted by the initiator. The values
are`XmDRAG_DROP_ONLY`,`XmDRAG_DYNAMIC`,`XmDRAG_NONE`,
and`XmDRAG_PREREGISTER`.


A pointer to the following structure is passed to callbacks for`XmNtopLevelLeaveCallback`:typedef struct
{
        int`reason`;
        XEvent  *`event`;
        Time`timeStamp`;
        Screen`screen`;
        Window`window`;
}XmTopLevelLeaveCallbackStruct, *XmTopLevelLeaveCallback;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`timeStamp`** 

Specifies the timestamp of the logical event
* **`screen`** 

Specifies a screen associated with the top-level window or root
window being left
* **`window`** 

Specifies the ID of the top-level window or root window being left



### Translations


The XmDragContext translations are described in the following list.
The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **`Button1<Enter>`:** 

DragMotion()
* **`Button1<Leave>`:** 

DragMotion()
* **`Button1<Motion>`:** 

DragMotion()
* **`Button2<Enter>`:** 

DragMotion()
* **`Button2<Leave>`:** 

DragMotion()
* **`Button2<Motion>`:** 

DragMotion()
* **Btn2Up:** 

FinishDrag()
* **Btn1Up:** 

FinishDrag()
* **Key`Return`:** 

FinishDrag()
* **KeyosfActivate:** 

FinishDrag()
* **BtnDown:** 

IgnoreButtons()
* **BtnUp:** 

IgnoreButtons()
* **`:`KeyosfCancel:** 

CancelDrag()
* **`:`KeyosfHelp:** 

HelpDrag()
* **`:`KeyosfUp:** 

DragKey(`Up`)
* **`:`KeyosfDown:** 

DragKey(`Down`)
* **`:`KeyosfLeft:** 

DragKey(`Left`)
* **`:`KeyosfRight:** 

DragKey(`Right`)
* **`:<KeyUp>`:** 

DragKey(`Update`)
* **`:<KeyDown>`:** 

DragKey(`Update`)

### Action Routines


The XmDragContext action routines are

* **CancelDrag():** 

Cancels the drag operation and frees the associated
DragContext.
* **DragKey(String`)`** 

If the value ofStringis`Left`,`Right`,`Up`, or`Down`, this action
moves the dragged object in the corresponding location. Any other values ofStringare ignored.
* **DragMotion():** 

Drags the selected data as the pointer is moved.
* **FinishDrag():** 

Finishes the drag operation and starts the drop operation.
* **HelpDrag():** 

Initiates a conditional drop that enables the receiver to provide
help information to the user. The user can cancel or continue the
drop operation in response to this information.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys,
see &cdeman.VirtualBindings;.
## RELATED INFORMATION


&cdeman.Core;,
&cdeman.XmDisplay;,
&cdeman.XmDragCancel;,
&cdeman.XmDragIcon;,
&cdeman.XmDragStart;,
&cdeman.XmDropSite;,
&cdeman.XmDropTransfer;, and
&cdeman.XmScreen;.