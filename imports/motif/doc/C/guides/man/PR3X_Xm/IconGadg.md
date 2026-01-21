# XmIconGadget
library callXmIconGadgetThe IconGadget widget classXmIconGadgeticonGadgetIconGadget#include <Xm/IconG.h>
## DESCRIPTION


IconGadget is an instantiable widget used to display both text and a pixmap
in various combinations.
Other widgets that hold the`XmQTcontainer`trait, such as
Container, can use IconGadget to represent objects.

IconGadget text is a compound string. If no text is supplied, then
the compound string is generated from the gadget name. IconGadget
text is placed relative to the type of associated pixmap.

Depending upon the`XmNviewType`resource, IconGadget can display
two views:

* **`XmLARGE_ICON`** 

The IconGadget text string is displayed below the
pixmap, and centered.
* **`XmSMALL_ICON`** 

The IconGadget text string is placed on the side of the small icon, in the
widget's`XmNlayoutDirection`.


A bitmap mask can be supplied for each pixmap to
clip the pixmap into some shape other than a rectangle. The`XmNlargeIconMask`and`XmNsmallIconMask`resources specify
the large and small bitmap masks respectively. Visual emphasis for
the IconGadget is
provided with the`XmNvisualEmphasis`resource.
IconGadget's`XmNdetail`and`XmNdetailCount`resources provide
a detail view for IconGadgets, enabling the display of Strings alongside the
IconGadget. The exact layout ordering of the strings depends on the
associated containing widget.

IconGadget uses the`XmQTcontainer`and`XmQTspecifyRenderTable`traits, and
holds the`XmQTcareParentVisual`and`XmQTcontainerItem`traits.
### Classes


IconGadget inherits behaviour, resources, and traits from`Object,
RectObject`,
and`XmGadget`classes.

The class pointer is`xmIconGadgetClass`.

The class name isXmIconGadget.
### New Resources


The following table defines a set of widget resources used by the
programmer to specify data. The programmer can also set the resource
values for the inherited classes to set attributes for this widget.
To reference a resource by name or by class in a.Xdefaultsfile,
remove the`XmN`or`XmC`prefix and use the remaining letters.
To specify one of the defined values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use the remaining letters (in
either lowercase or uppercase, but include any underscores between
words). The codes in the access column indicate whether the given resource
can be set at creation time (C), set by using`XtSetValues`(S), retrieved by using`XtGetValues`(G), or is not
applicable (N/A).`XmIconGadget Resource Set``Name``Class``Type``Default``Access`XmalignmentXmCAlignmentunsigned charXmALIGNMENT_CENTERCSGXmNdetailXmCDetailXmStringTableNULLCSGXmNdetailCountXmCDetailCountCardinal0CSGXmNfontListXmCFontListXmFontListNULLCSGXmNlabelStringXmCXmStringXmStringdynamicCSGXmNlargeIconMaskXmCIconMaskPixmapdynamicCSGXmNlargeIconPixmapXmCIconPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginWidthXmCMarginWidthDimension2CSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNsmallIconMaskXmCIconMaskPixmapdynamicCSGXmNsmallIconPixmapXmCIconPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNviewTypeXmCViewTypeunsigned charXmLARGE_ICONCSGXmNvisualEmphasisXmCVisualEmphasisunsigned charXmNOT_SELECTEDCSGXmNspacingXmCSpacingDimension4CSG

* **`XmNalignment`** 

Specifies the horizontal alignment of the pixmap with
respect to the label when the icon is in`LARGE_ICON`view. Valid values areXmALIGNMENT_BEGINNING,XmALIGNMENT_CENTER,
andXmALIGNMENT_END.
* **`XmNdetail`** 

Specifies an array ofXmStrings that are the detail information
associated with the gadget.
* **`XmNdetailCount`** 

Specifies the size of the`XmNdetail`array.
* **`XmNfontList`** 

Specifies the font list associated withXmIconGadget. The font list
is an obsolete construct, and has been superseded by the render table.
It is included for compatibility with earlier versions of Motif, and
for applications that do not easily support render tables. The
default font list is derived from the default render table, and if
both a font list and a render table are specified, the render table
takes precedence.
* **`XmNlabelString`** 

Specifies the compound string.
If this value is NULL, it is initialized by converting the name of the
gadget to a compound string.
Refer to &cdeman.XmString;
for more information on the
creation and structure of compound strings.
* **`XmNlargeIconMask`** 

Specifies the icon mask used when`XmNviewType`is`XmLARGE_ICON`.
* **`XmNlargeIconPixmap`** 

Specifies the pixmap when`XmNviewType`is`XmLARGE_ICON`. If
this resource's value is`XmUNSPECIFIED_PIXMAP`, there is no
pixmap. If a large
icon pixmap is specified, and if during conversion an associated mask
can be fetched, then the`XmNlargeIconMask`resource is set to that mask.
* **`XmNmarginHeight`** 

Specifies the amount of vertical space between the highlight
and the inside (pixmap and label).
* **`XmNmarginWidth`** 

Specifies the amount of horizontal space between the highlight
and the inside (pixmap and label).
* **`XmNrenderTable`** 

Specifies theXmRenderTableof the text used in the gadget.
If`XmNrenderTable`is NULL when the IconGadget is created, the parent's
render table resource value is used if there is a render table. If
the parent does not have a render table,
the parent hierarchy of the widget is
searched for a widget that
holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found, the render table is initialized to the`XmLABEL_RENDER_TABLE`value of the ancestor widget. If no such widget
is found, the default is implementation dependent.
Refer to
&cdeman.XmRenderTable; for more information on the creation and
structure of aXmRenderTable.
If both a render table and a font list are specified, the render table
will take precedence.
* **`XmNsmallIconMask`** 

Specifies the icon mask used when`XmNviewType`is`XmSMALL_ICON`.
* **`XmNsmallIconPixmap`** 

Specifies the pixmap when`XmNviewType`is`XmSMALL_ICON`.
If
this resource's value is`XmUNSPECIFIED_PIXMAP`, there is no
pixmap. If a small
icon pixmap is specified, and if during conversion an associated mask
can be fetched, then the`XmNsmallIconMask`resource is set to that mask.
* **`XmNspacing`** 

Specifies the amount of space between the pixmap and
the label parts of the icon.
* **`XmNviewType`** 

Specifies the view (combination of pixmaps/text) that will be displayed.
If the IconGadget is a child of a Container widget, however, then the
specification of this resource will be taken from the Container&mdash;
if Container's`XmNentryViewType`is either`XmLARGE_ICON`or`XmSMALL_ICON`, then IconGadget's`XmNviewType`takes that value;
otherwise, the default is`XmLARGE_ICON`.
This resource is set to one of the following:

* **`XmLARGE_ICON`** 

The pixmap specified by`XmNlargeIconPixmap`is
displayed with the`XmNlabelString`beneath it.
* **`XmSMALL_ICON`** 

The pixmap specified by`XmNsmallIconPixmap`is
displayed with the`XmNlabelString`displayed in the direction of the`XmNlayoutDirection`resource.

* **`XmNvisualEmphasis`** 

Specifies the visual state of the IconGadget. If the IconGadget is in a
selected state
all visuals are displayed using the Container`XmNselectColor`resource.
It is set to one of the following:

* **`XmSELECTED`** 

The IconGadget is in the selected state and displays
the appropriate visuals.
* **`XmNOT_SELECTED`** 

The IconGadget is not in the selected state.


### Inherited Resources


IconGadget inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.`XmGadget Resource Set``Name``Class``Type``Default``Access`XmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapdynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension0CSGXmNlayoutDirectionXmNCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmNONECSGXmNshadowThicknessXmCShadowThicknessDimension0CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanFalseCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG`RectObj Resource Set``Name``Class``Type``Default``Access`XmNancestorSensitiveXmCSensitiveBooleandynamicGXmNborderWidthXmCBorderWidthDimension0N/AXmNheightXmCHeightDimensiondynamicCSGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG`Object Resource Set``Name``Class``Type``Default``Access`XmNdestroyCallbackXmCCallbackXtCallbackListNULLC
### Additional Behavior


IconGadget has no behavior.
### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see
&cdeman.VirtualBindings;.
### Errors/Warnings


The toolkit will display a warning if an incorrect value is given
for an enumeration resource.
## RELATED


&cdeman.Core;,
&cdeman.XmContainer;,
&cdeman.XmCreateIconGadget;, and
&cdeman.XmGadget;.