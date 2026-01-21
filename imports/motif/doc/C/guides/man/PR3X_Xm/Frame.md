# XmFrame
library call`XmFrame`The Frame widget classXmFramewidget classFrame&npzwc;#include &lt;Xm/Frame.h>
## DESCRIPTION


Frame is a very simple manager used to enclose a single work
area child in a border drawn by Frame.
It uses the Manager class resources for border drawing and performs
geometry management so that its size always matches its child's outer size
plus the Frame's margins and shadow thickness.

Frame is most often used to enclose other managers when the
application developer wants the manager to have the same border
appearance as the primitive widgets. Frame can also be
used to enclose primitive widgets that do not support the same
type of border drawing. This gives visual consistency when
you develop applications using diverse widget sets.
Constraint resources are used to designate a child as the Frame title,
align its text, and control its vertical alignment in relation to
Frame's top shadow. The title appears only at the top of the Frame.

If the Frame's parent is a Shell widget,
the`XmNshadowType`resource defaults to`XmSHADOW_OUT`, and
the Manager's`XmNshadowThickness`resource defaults to 1.

If the Frame's parent is not a Shell widget,
the`XmNshadowType`resouce defaults to`XmSHADOW_ETCHED_IN`, and
the Manager's`XmNshadowThickness`resource defaults to 2.
### Classes


Frame inherits behavior and
resources from the`Core`,`Composite`,`Constraint`, and`XmManager`classes.

The class pointer is`xmFrameWidgetClass`.

The class name is`XmFrame`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile,
remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmFrame Resource Set``Name``Class``Type``Default``Access`XmNmarginWidthXmCMarginWidthDimension0CSGXmNmarginHeightXmCMarginHeightDimension0CSGXmNshadowTypeXmCShadowTypeunsigned chardynamicCSG

* **`XmNmarginWidth`** 

Specifies the padding space on the left and right
sides between Frame's child and Frame's shadow drawing.
* **`XmNmarginHeight`** 

Specifies the padding space on the top and bottom
sides between Frame's child and Frame's shadow drawing.
When a title is present, the top margin equals the value
specified by this resource plus the distance (if any) that the
title extends below the top shadow.
* **`XmNshadowType`** 

Describes the drawing style for Frame. This resource can have the
following values:

* **`XmSHADOW_IN`** 

Draws Frame so that it appears inset.
This means that the bottom shadow visuals and top shadow visuals
are reversed.
* **`XmSHADOW_OUT`** 

Draws Frame so that it appears outset.
This is the default if Frame's parent is a Shell widget.
* **`XmSHADOW_ETCHED_IN`** 

Draws Frame using a double line giving the
effect of a line etched into the window. The thickness of the double
line is equal to the value of`XmNshadowThickness`.
This is the default when Frame's parent is not a Shell widget.
* **`XmSHADOW_ETCHED_OUT`** 

Draws Frame using a double line giving the
effect of a line coming out of the window. The thickness of the double
line is equal to the value of`XmNshadowThickness`.



`XmFrame Constraint Resource Set``Name``Class``Type``Default``Access`XmNchildTypeXmCChildTypeunsigned charXmFRAME_WORKAREA_CHILDCSGXmNchildHorizontalAlignmentXmCChildHorizontalAlignmentunsigned charXmALIGNMENT_BEGINNINGCSGXmNchildHorizontalSpacingXmCChildHorizontalSpacingDimensiondynamicCSGXmNchildVerticalAlignmentXmCChildVerticalAlignmentunsigned charXmALIGNMENT_CENTERCSGXmNframeChildTypeXmCFrameChildTypeunsigned charXmFRAME_WORKAREA_CHILDCSG

* **`XmNchildType`** 

Refer to the`XmNframeChildType`resource description. The`XmNchildType`resource is obsoleted by`XmNframeChildType`,
but is kept here for backward compatibility.
* **`XmNchildHorizontalAlignment`** 

Specifies the alignment of the title. This resource has the
following values:

`XmALIGNMENT_BEGINNING`

`XmALIGNMENT_CENTER`

`XmALIGNMENT_END`

See the description of`XmNalignment`in the`XmLabel`reference page for an explanation of these values.
* **`XmNchildHorizontalSpacing`** 

Specifies the minimum distance between either edge of the title text
and the inner edge of the Frame shadow. Clipping of the title
text occurs in order to maintain this spacing. The default value
is the margin width of the Frame.
* **`XmNchildVerticalAlignment`** 

Specifies the vertical alignment of the title text, or the title
area in relation to the top shadow of the Frame.

* **`XmALIGNMENT_BASELINE_BOTTOM`** 

Causes the baseline of the
title to align vertically with the
top shadow of the Frame. In the case of a multi-line title,
the baseline of the last line of text aligns vertically with
the top shadow of the Frame.
* **`XmALIGNMENT_BASELINE_TOP`** 

Causes the baseline of the first
line of the title to align vertically with the top shadow
of the Frame.
* **`XmALIGNMENT_CHILD_TOP`** 

Causes the top edge of the title
area to align vertically with the top shadow of the Frame.
* **`XmALIGNMENT_CENTER`** 

Causes the center of the title
area to align vertically with the top shadow of the Frame.
* **`XmALIGNMENT_CHILD_BOTTOM`** 

Causes the bottom edge of the title
area to align vertically with the top shadow of the Frame.

* **`XmNframeChildType`** 

Specifies whether a child is a title or work area. Frame supports
a single title and/or work area child. The possible
values are

`XmFRAME_TITLE_CHILD`

`XmFRAME_WORKAREA_CHILD`

`XmFRAME_GENERIC_CHILD`

The Frame geometry manager ignores any child of type`XmFRAME_GENERIC_CHILD`.
This resource replaces`XmNchildType`.

### Inherited Resources


Frame inherits behavior and resources from the following
superclasses. For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


XmFrame inherits translations from XmManager.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmCreateFrame;, and
&cdeman.XmManager;.