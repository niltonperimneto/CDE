# XmDragIcon
library call`XmDragIcon`The DragIcon widget classXmDragIconwidget classDragIcon&npzwc;#include &lt;Xm/DragDrop.h>
## DESCRIPTION


A DragIcon is a component of the visual used to represent the source
data in a drag and drop transaction. During a drag operation, a real
or simulated X cursor provides drag-over visuals consisting of a
static portion that represents the object being dragged, and dynamic
cues that provide visual feedback during the drag operation. The
visual is attained by blending together various`XmDragIcons`specified in the`XmDragContext`associated with the drag
operation.

The static portion of the drag-over visual is the graphic
representation that identifies the drag source. For example,
when a user drags several items within a list, a DragIcon depicting a
list might be supplied as the visual. The`XmDragContext`resources,`XmNsourceCursorIcon`or`XmNsourcePixmapIcon`,
specify a DragIcon to use for the static portion of the visual.

A drag-over visual incorporates dynamic cues in order to provide
visual feedback in response to the user's actions. For instance,
the drag-over visual might use different indicators to identify
the type of operation (copy, link, or move) being performed. Dynamic
cues could also alert the user that a drop site is valid or invalid
as the pointer traverses the drop site. The`XmNoperationCursorIcon`and`XmNstateCursorIcon`resources of`XmDragContext`specify
DragIcons for dynamic cues.

A drag-over visual typically consists of a source, operation and
state DragIcon. The`XmNblendModel`resource of`XmDragContext`offers several options that determine which icons are blended
to produce the drag-over visual. DragIcon resources control
the relative position of the operation and state icons (if used).
If a particular DragIcon is not specified, the toolkit uses the`XmScreen`default DragIcons.

An application initializes a DragIcon with the function`XmCreateDragIcon`or through entries in the resource
database. If a pixmap and its mask (optional) are specified
in the resource database, the toolkit converts the
values in the X11 Bitmap file format and assigns values to
the corresponding resources.
### Classes


DragIcon inherits behavior and a resource from`Object`.

The class pointer is`xmDragIconObjectClass`.

The class name is`XmDragIcon`.
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

`XmDragIcon Resource Set``Name``Class``Type``Default``Access`XmNattachmentXmCAttachmentunsigned charXmATTACH_NORTH_WESTCSGXmNdepthXmCDepthint1CSGXmNheightXmCHeightDimension0CSGXmNhotXXmCHotPosition0CSGXmNhotYXmCHotPosition0CSGXmNmaskXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNoffsetXXmCOffsetPosition0CSGXmNoffsetYXmCOffsetPosition0CSGXmNpixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNwidthXmCWidthDimension0CSG

* **`XmNattachment`** 

Specifies a relative location on the source icon for the attachment of
the state or operation icon. The origin of the state and operation
icons is aligned with the specified compass point on the source icon.
The`XmNoffsetX`and`XmNoffsetY`resources can be used to further
refine the icon positions. The possible values are

* **`XmATTACH_NORTH_WEST`** 

Attaches the origin of the state or operation icon to the northwest
point on the source icon.
* **`XmATTACH_NORTH`** 

Attaches the origin of the state or operation icon to the north
point on the source icon.
* **`XmATTACH_NORTH_EAST`** 

Attaches the origin of the state or operation icon to the northeast
point on the source icon.
* **`XmATTACH_EAST`** 

Attaches the origin of the state or operation icon to the east
point on the source icon.
* **`XmATTACH_SOUTH_EAST`** 

Attaches the origin of the state or operation icon to the southeast
point on the source icon.
* **`XmATTACH_SOUTH`** 

Attaches the origin of the state or operation icon to the south
point on the source icon.
* **`XmATTACH_SOUTH_WEST`** 

Attaches the origin of the state or operation icon to the southwest
point on the source icon.
* **`XmATTACH_WEST`** 

Attaches the origin of the state or operation icon to the west
point on the source icon.
* **`XmATTACH_CENTER`** 

Attaches the origin of the state or operation icon to the
center of the source icon. The`XmNoffsetX`and`XmNoffsetY`resources may be used to center the attached icon.
* **`XmATTACH_HOT`** 

Attaches the hotspot coordinates of a state or operation DragIcon to
an x,y position on the source icon. The x,y coordinate
is taken from the event passed to the`XmDragStart`function,
and made relative to the widget passed as an argument to the same
function.

* **`XmNdepth`** 

Specifies the depth of the pixmap.
* **`XmNheight`** 

Specifies the height of the pixmap.
* **`XmNhotX`** 

Specifies the x-coordinate of the hotspot of a cursor DragIcon
in relation to the origin of the pixmap bounding box.
* **`XmNhotY`** 

Specifies the y-coordinate of the hotspot of a cursor DragIcon
in relation to the origin of the pixmap bounding box.
* **`XmNmask`** 

Specifies a pixmap of depth 1 to use as the DragIcon mask
pixmap.
* **`XmNoffsetX`** 

Specifies a horizontal offset (in pixels) of the origin of the state
or operation icon relative to the attachment point on the source icon.
A positive offset value moves the origin to the right; a negative value
moves the origin to the left.
* **`XmNoffsetY`** 

Specifies a vertical offset (in pixels) of the origin of the state or
operation icon relative to the attachment point on the source icon. A
positive offset value moves the origin down; a negative value moves the
origin up.
* **`XmNpixmap`** 

Specifies a pixmap to use as the DragIcon pixmap.
* **`XmNwidth`** 

Specifies the width of the pixmap.

### Inherited Resources


DragIcon inherits behavior and a resource from`Object`.
For a complete description of this resource, refer to the`Object`reference page.

`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
## RELATED INFORMATION


&cdeman.Object;,
&cdeman.XmCreateDragIcon;,
&cdeman.XmDisplay;,
&cdeman.XmDragContext;,
&cdeman.XmDropSite;,
&cdeman.XmDropTransfer;, and
&cdeman.XmScreen;.