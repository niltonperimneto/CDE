# XmForm
library call`XmForm`The Form widget classXmFormwidget classForm&npzwc;#include &lt;Xm/Form.h>
## DESCRIPTION


Form is a container widget with no input semantics of its own.
Constraints are placed on children of the Form to define attachments
for each of the child's four sides.
These attachments can be to the Form, to another child widget or gadget,
to a relative position within the Form, or to the initial position of
the child.
The attachments determine the layout behavior of the Form when resizing
occurs.

The default value for`XmNinitialFocus`is the value of`XmNdefaultButton`.

Following are some important considerations in using a Form:

Every child must have an attachment on either the left or the right.
If initialization or`XtSetValues`leaves a widget without
such an attachment, the result depends upon the value of`XmNrubberPositioning`.

If`XmNrubberPositioning`is False, the child is given an`XmNleftAttachment`of`XmATTACH_FORM`and an`XmNleftOffset`equal to its currentxvalue.

If`XmNrubberPositioning`is True, the child is given an`XmNleftAttachment`of`XmATTACH_POSITION`and an`XmNleftPosition`proportional to the currentxvalue divided
by the width of the Form.

In either case, if the child has not been previously given anxvalue, itsxvalue is taken to be 0 (zero), which places the child at the
left side of the Form.

If you want to create a child without any attachments, and then
later (for example, after creating and managing it, but before realizing it)
give it a right attachment through`XtSetValues`, you must set its`XmNleftAttachment`to`XmATTACH_NONE`at the same time.

The`XmNresizable`resource controls only whether a geometry request
by the child will be granted.
It has no effect on whether the child's size can be changed because of
changes in geometry of the Form or of other children.

Every child has a preferred width, based on geometry requests it
makes (whether they are granted or not).

If a child has attachments on both the left and the right sides,
its size is completely controlled by the Form.
It can be shrunk below its preferred width or enlarged above it, if
necessary, due to other constraints.
In addition, the child's geometry requests to change its own width may
be refused.

If a child has attachments on only its left or right side, it will
always be at its preferred width (if resizable, otherwise at is current
width).
This may cause it to be clipped by the Form or by other children.

If a child's left (or right) attachment is set to`XmATTACH_SELF`, its
corresponding left (or right) offset is forced to 0 (zero).
The attachment is then changed to`XmATTACH_POSITION`, with a
position that corresponds to thexvalue of the child's left (or
right) edge.
To fix the position of a side at a specificxvalue, use`XmATTACH_FORM`or`XmATTACH_OPPOSITE_FORM`with thexvalue as the left (or right) offset.

Unmapping a child has no effect on the Form except that the child
is not mapped.

Unmanaging a child unmaps it.
If no other child is attached to it, or if all children attached to it
and all children recursively attached to them are also all unmanaged,
all of those children are treated as if they did not exist in
determining the size of the Form.

When using`XtSetValues`to change the`XmNx`resource of a
child, you must simultaneously set its left attachment to either`XmATTACH_SELF`or`XmATTACH_NONE`.
Otherwise, the request is not granted.
If`XmNresizable`is False, the request is granted only if the
child's size can remain the same.

A left (or right) attachment of`XmATTACH_WIDGET`, where`XmNleftWidget`(or`XmNrightWidget`) is NULL, acts like an
attachment of`XmATTACH_FORM`.

If an attachment is made to a widget that is not a child of the
Form, but an ancestor of the widget is a child of the Form, the
attachment is made to the ancestor.

All these considerations are true of top and bottom attachments as well,
with top acting like left, bottom acting like right,yacting likex, and height acting like width.
### Classes


Form inherits behavior, resources, and traits from`Core`,`Composite`,`Constraint`,`XmManager`, and`XmBulletinBoard`.

The class pointer is`xmFormWidgetClass`.

The class name is`XmForm`.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. The programmer can also set the resource
values for the inherited classes to set attributes for this widget.
To reference a resource by name or by class in a.Xdefaultsfile,
remove the`XmN`or`XmC`prefix and use the remaining letters.
To specify one of the defined values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use the remaining letters (in
either lowercase or uppercase, but include any underscores between
words). The codes in the access column indicate if the given resource
can be set at creation time (C), set by using`XtSetValues`(S), retrieved by using`XtGetValues`(G), or is not
applicable (N/A).

`XmForm Resource Set``Name``Class``Type``Default``Access`XmNfractionBaseXmCMaxValueint100CSGXmNhorizontalSpacingXmCSpacingDimension0CSGXmNrubberPositioningXmCRubberPositioningBooleanFalseCSGXmNverticalSpacingXmCSpacingDimension0CSG

* **`XmNfractionBase`** 

Specifies the denominator used in calculating the relative position of
a child widget using`XmATTACH_POSITION`constraints.
The value must not be 0 (zero).

If the value of a child's`XmNleftAttachment`(or`XmNrightAttachment`) is`XmATTACH_POSITION`, the position of
the left (or right) side of the child is relative to the left
side of the Form and is a fraction of the width of the Form.
This fraction is the value of the child's`XmNleftPosition`(or`XmNrightPosition`) resource divided by the value of the Form's`XmNfractionBase`.

If the value of a child's`XmNtopAttachment`(or`XmNbottomAttachment`) is`XmATTACH_POSITION`, the position of
the top (or bottom) side of the child is relative to the top
side
of the Form and is a fraction of the height of the Form.
This fraction is the value of the child's`XmNtopPosition`(or`XmNbottomPosition`) resource divided by the value of the Form's`XmNfractionBase`.
* **`XmNhorizontalSpacing`** 

Specifies the offset for right and left attachments. This resource is
only used if no offset resource is specified (when attaching to a
widget), or if no margin resource is specified (when attaching
to the Form).
* **`XmNrubberPositioning`** 

Indicates the default near (left) and top attachments for a child of the
Form.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`.)

The default left attachment is applied whenever initialization or`XtSetValues`leaves the child without either a left or right
attachment.
The default top attachment is applied whenever initialization or`XtSetValues`leaves the child without either a top or bottom
attachment.

If this Boolean resource is set to False,`XmNleftAttachment`and`XmNtopAttachment`default to`XmATTACH_FORM`,`XmNleftOffset`defaults to the currentxvalue of the left
side of the child,
and`XmNtopOffset`defaults to the currentyvalue of the child.
The effect is to position the child according to its absolute distance
from the left or top side of the Form.

If this resource is set to True,`XmNleftAttachment`and`XmNtopAttachment`default to`XmATTACH_POSITION`,`XmNleftPosition`defaults to a value proportional to the currentxvalue of the left side of the child divided by the width of the
Form, and`XmNtopPosition`defaults to a value proportional to the
currentyvalue of the child divided by the height of the Form.
The effect is to position the child relative to the left or top
side of the Form and in proportion to the width or height of the Form.
* **`XmNverticalSpacing`** 

Specifies the offset for top and bottom attachments. This resource is
only used if no offset resource is specified (when attaching to a
widget), or if no margin resource is specified (when attaching
to the Form).

`XmForm Constraint Resource Set``Name``Class``Type``Default``Access`XmNbottomAttachmentXmCAttachmentunsigned charXmATTACH_NONECSGXmNbottomOffsetXmCOffsetint0CSGXmNbottomPositionXmCPositionint0CSGXmNbottomWidgetXmCWidgetWidgetNULLCSGXmNleftAttachmentXmCAttachmentunsigned charXmATTACH_NONECSGXmNleftOffsetXmCOffsetint0CSGXmNleftPositionXmCPositionint0CSGXmNleftWidgetXmCWidgetWidgetNULLCSGXmNresizableXmCBooleanBooleanTrueCSGXmNrightAttachmentXmCAttachmentunsigned charXmATTACH_NONECSGXmNrightOffsetXmCOffsetint0CSGXmNrightPositionXmCPositionint0CSGXmNrightWidgetXmCWidgetWidgetNULLCSGXmNtopAttachmentXmCAttachmentunsigned charXmATTACH_NONECSGXmNtopOffsetXmCOffsetint0CSGXmNtopPositionXmCPositionint0CSGXmNtopWidgetXmCWidgetWidgetNULLCSG
* **`XmNbottomAttachment`** 

Specifies attachment of the bottom side of the child. It can have the
following values:

* **`XmATTACH_NONE`** 

Do not attach the bottom side of the child.
* **`XmATTACH_FORM`** 

Attach the bottom side of the child to the bottom
side of the Form.
* **`XmATTACH_OPPOSITE_FORM`** 

Attach the bottom side of the child to
the top side of the Form.`XmNbottomOffset`can be used to determine the visibility of the
child.
* **`XmATTACH_WIDGET`** 

Attach the bottom side of the child to the top
side of the widget or
gadget specified in the`XmNbottomWidget`resource.
If`XmNbottomWidget`is NULL,`XmATTACH_WIDGET`is replaced by`XmATTACH_FORM`, and the child is attached to the bottom side of the
Form.
* **`XmATTACH_OPPOSITE_WIDGET`** 

Attach the bottom side of the child to
the bottom side of the widget or
gadget specified in the`XmNbottomWidget`resource.
* **`XmATTACH_POSITION`** 

Attach the bottom side of the child to a
position that is relative to
the top side of the Form and in proportion to the height of the Form.
This position is determined by the`XmNbottomPosition`and`XmNfractionBase`resources.
* **`XmATTACH_SELF`** 

Attach the bottom side of the child to a position
that is proportional
to the currentyvalue of the bottom of the child divided by the
height of the Form.
This position is determined by the`XmNbottomPosition`and`XmNfractionBase`resources.`XmNbottomPosition`is set to a value proportional to the currentyvalue of the bottom of the child divided by the height of the
Form.

* **`XmNbottomOffset`** 

Specifies the constant offset between the bottom side of the
child and the object to which it is
attached.
The relationship established remains, regardless of any resizing operations
that occur.
When this resource is explicitly set, the value of`XmNverticalSpacing`is ignored.
* **`XmNbottomPosition`** 

This resource is used to determine the position of the bottom side of
the child when the child's`XmNbottomAttachment`is set to`XmATTACH_POSITION`.
In this case the position of the bottom side of the child is relative to
the top side of the Form and is a fraction of the height of the Form.
This fraction is the value of the child's`XmNbottomPosition`resource divided by the value of the Form's`XmNfractionBase`.
For example, if the child's`XmNbottomPosition`is 50, the Form's`XmNfractionBase`is 100, and the Form's height is 200, the position
of the bottom side of the child is 100.
* **`XmNbottomWidget`** 

Specifies the widget or gadget to which
the bottom side of the child is attached.
This resource is used if the`XmNbottomAttachment`resource
is set to either`XmATTACH_WIDGET`or`XmATTACH_OPPOSITE_WIDGET`.

A string-to-widget resource converter is automatically installed for use
with this resource.
With this converter, the widget that is to be the value of the resource
must exist at the time the widget that has the resource is created.
* **`XmNleftAttachment`** 

Specifies attachment of the near (left) side of the child.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)
It can have the following values:

* **`XmATTACH_NONE`** 

Do not attach the left side of the child.
If`XmNrightAttachment`is also`XmATTACH_NONE`, this value is
ignored and the child is given a default left attachment.
* **`XmATTACH_FORM`** 

Attach the left side of the child to the left
side of the Form.
* **`XmATTACH_OPPOSITE_FORM`** 

Attach the left side of the child to the
right side of the Form.`XmNleftOffset`can be used to determine the visibility of the
child.
* **`XmATTACH_WIDGET`** 

Attach the left side of the child to the right
side of the widget or
gadget specified in the`XmNleftWidget`resource.
If`XmNleftWidget`is NULL,`XmATTACH_WIDGET`is replaced by`XmATTACH_FORM`, and the child is attached to the left side of the
Form.
* **`XmATTACH_OPPOSITE_WIDGET`** 

Attach the left side of the child to
the left side of the widget or
gadget specified in the`XmNleftWidget`resource.
* **`XmATTACH_POSITION`** 

Attach the left side of the child to a
position that is relative to
the left side of the Form and in proportion to the width of the Form.
This position is determined by the`XmNleftPosition`and`XmNfractionBase`resources.
* **`XmATTACH_SELF`** 

Attach the left side of the child to a position
that is proportional to
the currentxvalue of the left side of the child divided by the
width of the Form.
This position is determined by the`XmNleftPosition`and`XmNfractionBase`resources.`XmNleftPosition`is set to a value proportional to the currentxvalue of the left side of the child divided by the width of the
Form.

* **`XmNleftOffset`** 

Specifies the constant offset between the near (left) side of the
child and the object to which it is attached.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)
The relationship established remains, regardless of any resizing operations
that occur.
When this resource is explicitly set, the value of`XmNhorizontalSpacing`is ignored.
* **`XmNleftPosition`** 

This resource is used to determine the position of the near (left) side
of the child when the child's`XmNleftAttachment`is set to`XmATTACH_POSITION`.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)

In this case, the position of the left side of the child is relative to
the left side of the Form and is a fraction of the width of the Form.
This fraction is the value of the child's`XmNleftPosition`resource
divided by the value of the Form's`XmNfractionBase`.
For example, if the child's`XmNleftPosition`is 50, the Form's`XmNfractionBase`is 100, and the Form's width is 200, the position
of the left side of the child is 100.
* **`XmNleftWidget`** 

Specifies the widget or gadget to which the near (left) side of the
child is attached.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)
The`XmNleftWidget`resource is used if the`XmNleftAttachment`resource is set to either`XmATTACH_WIDGET`or`XmATTACH_OPPOSITE_WIDGET`.

A string-to-widget resource converter is automatically installed for use
with this resource.
With this converter, the widget that is to be the value of the resource
must exist at the time the widget that has the resource is created.
* **`XmNresizable`** 

This Boolean resource specifies whether or not a child's request for a
new size is (conditionally) granted by the Form.
If this resource is set to True the request is granted if possible.
If this resource is set to False the request is always refused.

If a child has both left and right attachments, its width is completely
controlled by the Form, regardless of the value of the child's`XmNresizable`resource.
If a child has a left or right attachment but not both, the child's`XmNwidth`is used in setting its width if the value of the child's`XmNresizable`resource is True.
These conditions are also true for top and bottom attachments, with
height acting like width.
* **`XmNrightAttachment`** 

Specifies attachment of the far (right) side of the child.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)
It can have the following values:

* **`XmATTACH_NONE`** 

Do not attach the right side of the child.
* **`XmATTACH_FORM`** 

Attach the right side of the child to the right
side of the Form.
* **`XmATTACH_OPPOSITE_FORM`** 

Attach the right side of the child to
the left side of the Form.`XmNrightOffset`can be used to determine the visibility of the
child.
* **`XmATTACH_WIDGET`** 

Attach the right side of the child to the left
side of the widget or
gadget specified in the`XmNrightWidget`resource.
If`XmNrightWidget`is NULL,`XmATTACH_WIDGET`is replaced by`XmATTACH_FORM`, and the child is attached to the right side of the
Form.
* **`XmATTACH_OPPOSITE_WIDGET`** 

Attach the right side of the child to
the right side of the widget or
gadget specified in the`XmNrightWidget`resource.
* **`XmATTACH_POSITION`** 

Attach the right side of the child to a
position that is relative to
the left side of the Form and in proportion to the width of the Form.
This position is determined by the`XmNrightPosition`and`XmNfractionBase`resources.
* **`XmATTACH_SELF`** 

Attach the right side of the child to a position
that is proportional to
the currentxvalue of the right side of the child divided by the
width of the Form.
This position is determined by the`XmNrightPosition`and`XmNfractionBase`resources.`XmNrightPosition`is set to a value proportional to the currentxvalue of the right side of the child divided by the width of the
Form.

* **`XmNrightOffset`** 

Specifies the constant offset between the far (right) side of the
child and the object to which it is attached.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)
The relationship established remains, regardless of any resizing operations
that occur.
When this resource is explicitly set, the value of`XmNhorizontalSpacing`is ignored.
* **`XmNrightPosition`** 

This resource is used to determine the position of the far (right) side
of the child when the child's`XmNrightAttachment`is set to`XmATTACH_POSITION`.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)

In this case the position of the right side of the child is relative to
the left side of the Form and is a fraction of the width of the Form.
This fraction is the value of the child's`XmNrightPosition`resource
divided by
the value of the Form's`XmNfractionBase`.
For example, if the child's`XmNrightPosition`is 50, the Form's`XmNfractionBase`is 100, and the Form's width is 200, the position
of the right side of the child is 100.
* **`XmNrightWidget`** 

Specifies the widget or gadget
to which the far (right) side of the child is attached.
(Note that whether this resource actually applies to the left or
right side of the child and its attachment
depends on the value of
the`XmNlayoutDirection`resource.)
The`XmNrightWidget`resource is used if the`XmNrightAttachment`resource
is set to either`XmATTACH_WIDGET`or`XmATTACH_OPPOSITE_WIDGET`.

A string-to-widget resource converter is automatically installed for use
with this resource.
With this converter, the widget that is to be the value of the resource
must exist at the time the widget that has the resource is created.
* **`XmNtopAttachment`** 

Specifies attachment of the top side of the child. It can have
following values:

* **`XmATTACH_NONE`** 

Do not attach the top side of the child.
If the`XmNbottomAttachment`resource
is also`XmATTACH_NONE`, this value is
ignored and the child is given a default top attachment.
* **`XmATTACH_FORM`** 

Attach the top side of the child to the top side
of the Form.
* **`XmATTACH_OPPOSITE_FORM`** 

Attach the top side of the child to the
bottom side of the Form.`XmNtopOffset`can be used to determine the visibility of the
child.
* **`XmATTACH_WIDGET`** 

Attach the top side of the child to the bottom
side of the widget or
gadget specified in the`XmNtopWidget`resource.
If`XmNtopWidget`is NULL,`XmATTACH_WIDGET`is replaced by`XmATTACH_FORM`and the child is attached to the top side of the
Form.
* **`XmATTACH_OPPOSITE_WIDGET`** 

Attach the top side of the child to
the top side of the widget or
gadget specified in the`XmNtopWidget`resource.
* **`XmATTACH_POSITION`** 

Attach the top side of the child to a
position that is relative to
the top side of the Form and in proportion to the height of the Form.
This position is determined by the`XmNtopPosition`and`XmNfractionBase`resources.
* **`XmATTACH_SELF`** 

Attach the top side of the child to a position
that is proportional to
the currentyvalue of the child divided by the height of the
Form.
This position is determined by the`XmNtopPosition`and`XmNfractionBase`resources.`XmNtopPosition`is set to a value proportional to the currentyvalue of the child divided by the height of the Form.

* **`XmNtopOffset`** 

Specifies the constant offset between the top side of the
child and the object to which it is
attached.
The relationship established remains, regardless of any resizing operations
that occur.
When this resource is explicitly set, the value of`XmNverticalSpacing`is ignored.
* **`XmNtopPosition`** 

This resource is used to determine the position of the top side of
the child when the child's`XmNtopAttachment`is set to`XmATTACH_POSITION`.
In this case, the position of the top side of the child is relative to
the top side of the Form and is a fraction of the height of the Form.
This fraction is the value of the child's`XmNtopPosition`resource divided by the value of the Form's`XmNfractionBase`.
For example, if the child's`XmNtopPosition`is 50, the Form's`XmNfractionBase`is 100, and the Form's height is 200, the position
of the top side of the child is 100.
* **`XmNtopWidget`** 

Specifies the widget or gadget to which the top
side of the child is attached.
This resource is used if`XmNtopAttachment`is
set to a value of either`XmATTACH_WIDGET`or`XmATTACH_OPPOSITE_WIDGET`.

A string-to-widget resource converter is automatically installed for use
with this resource.
With this converter, the widget that is to be the value of the resource
must exist at the time the widget that has the resource is created.

### Inherited Resources


Form inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmBulletinBoard Resource Set``Name``Class``Type``Default``Access`XmNallowOverlapXmCAllowOverlapBooleanTrueCSGXmNautoUnmanageXmCAutoUnmanageBooleanTrueCGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNcancelButtonXmCWidgetWidgetNULLSGXmNdefaultButtonXmCWidgetWidgetNULLSGXmNdefaultPositionXmCDefaultPositionBooleanTrueCSGXmNdialogStyleXmCDialogStyleunsigned chardynamicCSGXmNdialogTitleXmCDialogTitleXmStringNULLCSGXmNfocusCallbackXmCCallbackXtCallbackListNULLCXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTableXmRenderTabledynamicCSGXmNmapCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension0CSGXmNmarginWidthXmCMarginWidthDimension0CSGXmNnoResizeXmCNoResizeBooleanFalseCSGXmNresizePolicyXmCResizePolicyunsigned charXmRESIZE_ANYCSGXmNshadowTypeXmCShadowTypeunsigned charXmSHADOW_OUTCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNtextTranslationsXmCTranslationsXtTranslationsNULLCXmNunmapCallbackXmCCallbackXtCallbackListNULLC

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetdynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicN/AXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


XmForm inherits translations from XmBulletinBoard.
## RELATED


&cdeman.Composite;, &cdeman.Constraint;, &cdeman.Core;,
&cdeman.XmBulletinBoard;,`XmCreateForm`, &cdeman.XmCreateFormDialog;, and
&cdeman.XmManager;.