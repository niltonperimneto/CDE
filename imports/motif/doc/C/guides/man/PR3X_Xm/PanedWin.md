# XmPanedWindow
library call`XmPanedWindow`The PanedWindow widget classXmPanedWindowwidget classPanedWindow&npzwc;#include &lt;Xm/PanedW.h>
## DESCRIPTION


PanedWindow is a composite widget that lays out children in a
vertically tiled format. Children appear in top-to-bottom fashion, with
the first child inserted appearing at the top of the PanedWindow and the
last child inserted appearing at the bottom. The
PanedWindow grows to match the width of its widest child and all
other children are forced to this width. The height of the PanedWindow
is equal to the sum of the heights of all its children, the spacing
between them, and the size of the top and bottom margins.

The user can also adjust the size of the panes. To
facilitate this adjustment, a pane control sash is created for most
children. The sash appears as a square box positioned on the bottom of
the pane that it controls. The user can adjust the size of
a pane by using the mouse or keyboard.

The PanedWindow is also a constraint widget, which means that it
creates and manages a set of constraints for each child. You can
specify a minimum and maximum size for each pane. The PanedWindow
does not allow a pane to be resized below its minimum size or beyond its
maximum size. Also, when the minimum size of a pane is equal to its maximum
size, no control sash is presented for that pane or
for the lowest pane.

The default`XmNinsertPosition`procedure for PanedWindow causes
all panes to appear first in the`XmNchildren`list and
all sashes to appear after the panes. For a pane child, it returns the
value of`XmNpositionIndex`if one has been specified for the
child. Otherwise, it returns the number ofpanesin the
PanedWindow's`XmNchildren`list. Other than the fact that all
sashes appear after all panes, the insertion order of sashes is unspecified.
This procedure also causes
nonsash widgets to be inserted after other nonsash children but before
any sashes.
The behavior of PanedWindow is undefined if`XmNinsertPosition`is set
to a procedure other than the default.

All panes and sashes in a PanedWindow must be tab groups. When a pane is
inserted as a child of the PanedWindow, if the pane's`XmNnavigationType`is not`XmEXCLUSIVE_TAB_GROUP`, PanedWindow
sets it to`XmSTICKY_TAB_GROUP`.
### Descendants


PanedWindow automatically creates for each paned window the
descendants shown in the
following table.
An application can use`XtName`and the child list information to
gain access to the named descendant.
In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`Sash``subclass of``XmPrimitive`sash`Separator``XmSeparatorGadget`dividing line between window panes
### Classes


PanedWindow inherits behavior and resources from the`Core`,`Composite`,`Constraint`, and`XmManager`classes.

The class pointer is`xmPanedWindowWidgetClass`.

The class name is`XmPanedWindow`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmPanedWindow Resource Set``Name``Class``Type``Default``Access`XmNmarginHeightXmCMarginHeightDimension3CSGXmNmarginWidthXmCMarginWidthDimension3CSGXmNorientationXmCOrientationunsigned charXmVERTICALCSGXmNrefigureModeXmCBooleanBooleanTrueCSGXmNsashHeightXmCSashHeightDimension10CSGXmNsashIndentXmCSashIndentPosition-10CSGXmNsashShadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNsashWidthXmCSashWidthDimension10CSGXmNseparatorOnXmCSeparatorOnBooleanTrueCSGXmNspacingXmCSpacingDimension8CSG

* **`XmNmarginHeight`** 

Specifies the distance between the top and bottom edges of the PanedWindow
and its children.
* **`XmNmarginWidth`** 

Specifies the distance between the left and right edges of the PanedWindow
and its children.
* **`XmNorientation`** 

Specifies the layout as either vertical (with the`XmVERTICAL`value) or horizontal (with the`XmHORIZONTAL`value). In the
vertical layout, the children are laid out in a vertically tiled
format. In the horizontal layout, the children are laid out in a
horizontal layout, with the sash moveable along the horizontal axis.
* **`XmNrefigureMode`** 

Determines whether the panes' positions are recomputed and repositioned
when programmatic changes are being made to the PanedWindow.
Setting this resource to True resets the children to their appropriate
positions.
* **`XmNsashHeight`** 

Specifies the height of the sash.
* **`XmNsashIndent`** 

Specifies the horizontal placement of the sash along each pane. A positive
value causes the sash to be offset from the near (left) side of the PanedWindow,
and a negative value causes the sash to be offset from the far (right)
side of the PanedWindow. If the offset is greater than the width of the
PanedWindow minus the width of the sash, the sash is placed flush
against the near side of the PanedWindow.

Whether the placement actually corresponds to the left or right side of
the PanedWindow
depends on the`XmNlayoutDirection`resource of the widget.
* **`XmNsashShadowThickness`** 

Specifies the thickness of the shadows of the sashes.
* **`XmNsashWidth`** 

Specifies the width of the sash.
* **`XmNseparatorOn`** 

Determines whether a separator is created between each of the panes.
Setting this resource to True creates a Separator at the
midpoint between each of the panes.
* **`XmNspacing`** 

Specifies the distance between each child pane.

`XmPanedWindow Constraint Resource Set``Name``Class``Type``Default``Access`XmNallowResizeXmCBooleanBooleanFalseCSGXmNpaneMaximumXmCPaneMaximumDimension1000CSGXmNpaneMinimumXmCPaneMinimumDimension1CSGXmNpositionIndexXmCPositionIndexshortXmLAST_POSITIONCSGXmNskipAdjustXmCBooleanBooleanFalseCSG
* **`XmNallowResize`** 

Allows an application to specify whether the PanedWindow
should allow a pane to request to be resized. This flag has an
effect only after the PanedWindow and its children have been realized.
If this flag is set to True, the PanedWindow tries to honor requests
to alter the height of the pane. If False, it always denies pane
requests to resize.
* **`XmNpaneMaximum`** 

Allows an application to specify the maximum size to which a pane
may be resized. This value must be greater than the specified minimum.
* **`XmNpaneMinimum`** 

Allows an application to specify the minimum size to which a pane
may be resized. This value must be greater than 0 (zero).
* **`XmNpositionIndex`** 

Specifies the position of the widget in its parent's list of
children (the list of pane children, not including sashes). The value
is an integer that is no less than 0 (zero) and no greater than
the number of children in the list at the time the value is
specified. A value of 0 means that the child is placed at the
beginning of the list. The value can also be specified as`XmLAST_POSITION`(the default), which means that the child
is placed at the end of the list. Any other value is ignored.`XtGetValues`returns the position of the widget in its parent's
child list at the time of the call to`XtGetValues`.

When a widget is inserted into its parent's child list, the positions
of any existing children that are greater than or equal to the
specified widget's`XmNpositionIndex`are increased by 1.
The effect of a call to`XtSetValues`for`XmNpositionIndex`is to remove the specified widget from its parent's child list, decrease
by one the positions of any existing children that are greater than
the specified widget's former position in the list, and then insert
the specified widget into its parent's child list as described in the
preceding sentence.
* **`XmNskipAdjust`** 

When set to True, this Boolean resource allows an application to specify
that the PanedWindow should not automatically resize this pane.

### Inherited Resources


PanedWindow inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcdefault procedureCSGXmNnumChildrenXmCReadOnlyCardinal0G
### Translations


`XmPanedWindow`inherits translations from`XmManager`.

The translations for sashes within the PanedWindow are
described in the following table.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **`&ap;c &ap;s &ap;m &ap;a`Btn1Down:** 

SashAction(`Start`)
* **`&ap;c &ap;s &ap;m &ap;a`Btn1Motion:** 

SashAction(`Move`)
* **`&ap;c &ap;s &ap;m &ap;a`Btn1Up:** 

SashAction(`Commit`)
* **`&ap;c &ap;s &ap;m &ap;a`Btn2Down:** 

SashAction(`Start`)
* **`&ap;c &ap;s &ap;m &ap;a`Btn2Motion:** 

SashAction(`Move`)
* **`&ap;c &ap;s &ap;m &ap;a`Btn2Up:** 

SashAction(`Commit`)
* **`:`KeyosfActivate:** 

PrimitiveParentActivate()
* **`:`KeyosfCancel:** 

PrimitiveParentCancel()
* **`:`KeyosfHelp:** 

Help()
* **`:c`KeyosfUp:** 

SashAction(`Key,LargeIncr,Up`)
* **`:`KeyosfUp:** 

SashAction(`Key,DefaultIncr,Up`)
* **`:c`KeyosfRight:** 

SashAction(`Key,LargeIncr,Right`)
* **`:`KeyosfRight:** 

SashAction(`Key,DefaultIncr,Right`)
* **`:c`KeyosfDown:** 

SashAction(`Key,LargeIncr,Down`)
* **`:`KeyosfDown:** 

SashAction(`Key,DefaultIncr,Down`)
* **`:c`KeyosfLeft:** 

SashAction(`Key,LargeIncr,Left`)
* **`:`KeyosfLeft:** 

SashAction(`Key,DefaultIncr,Left`)
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

PrimitiveParentActivate()
* **`s &ap;m &ap;a`Key`Tab`:** 

PrevTabGroup()
* **`&ap;m &ap;a`Key`Tab`:** 

NextTabGroup()

### Action Routines


The`XmPanedWindow`action routines are

* **Help():** 

Calls the callbacks for`XmNhelpCallback`if any exist. If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **NextTabGroup():** 

Moves the keyboard focus to the next tab group.
By default, each pane and sash is a tab group.
* **PrevTabGroup():** 

Moves the keyboard focus to the previous tab group.
By default, each pane and sash is a tab group.
* **SashAction(`action``)`&ensp;or&ensp;SashAction(`Key,&npzwc;``increment``,&npzwc;`direction`)`:** 

The`Start`action activates the interactive placement of the pane's borders.
The`Move`action causes the sash to track the position of the pointer.
If one of the panes reaches its minimum or maximum size, adjustment
continues with the next adjustable pane.
The`Commit`action ends sash motion.

When sash action is caused by a keyboard event, the sash with the keyboard
focus is moved according to the`increment`anddirectionspecified.`DefaultIncr`adjusts the sash by one line.`LargeIncr`adjusts the sash by one view region. Thedirectionis specified as either`Up`,`Down`,`Left`, or`Right`.

Note that the SashAction action routine is not a direct action routine
of the`XmPanedWindow,`but rather an action of the Sash control created
by the`XmPanedWindow`.

### Additional Behavior


This widget has the following additional behavior:

* **FocusIn:** 

Moves the keyboard focus to the sash and highlights it
* **FocusOut:** 

Unsets the keyboard focus in the sash and unhighlights it

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;, &cdeman.Constraint;,
&cdeman.Core;, &cdeman.XmCreatePanedWindow;, and &cdeman.XmManager;.