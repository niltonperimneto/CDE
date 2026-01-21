# XmScale
library call`XmScale`The Scale widget classXmScalewidget classScale&npzwc;#include &lt;Xm/Scale.h>
## DESCRIPTION


Scale is used by an application to indicate a
value from within a range of values, and it allows the user to input or modify
a value from the same range.

A Scale has an elongated rectangular region similar to a
ScrollBar. A slider inside this region
indicates the current value along the Scale. The user can also
modify the Scale's value by moving the slider within the
rectangular region of the Scale. A Scale can also include a label set
located outside the Scale region.
These can indicate the relative value at various positions
along the scale.
The placement of this label depends on the`XmNlayoutDirection`resource of the widget.

A Scale can be either input/output or output only. An input/output
Scale's value can be set by the application and also
modified by the user with the slider. An output-only Scale
is used strictly as an indicator of the current value of something
and cannot be modified interactively by the user.
The`XmScale`resource`XmNeditable`specifies whether the user can
interactively modify the Scale's value.

The user can specify resources in a resource file for the automatically
created gadget that contains the title of the Scale widget. The name of the
gadget is`Title`.
The placement of the title depends on the`XmNlayoutDirection`resource of the widget. The direction of the title is based on
the widget's layout direction.

Scale uses the`XmQTspecifyRenderTable`trait, and
holds the`XmQTtransfer`trait.
### Data Transfer Behavior


Scale supports dragging of the representation of the Scale value from
the Scale when the value is displayed
and when the value of the`XmNenableUnselectableDrag`resource
of`XmDisplay`is set to True.

As a source of data, Scale supports the following targets and associated
conversions of data to these targets:

* **`COMPOUND_TEXT`** 

The widget transfers a string representation of`XmNvalue`as type`COMPOUND_TEXT`.
* **`STRING`** 

The widget transfers a string representation of`XmNvalue`as type`STRING`.
* **_MOTIF_CLIPBOARD_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets it
supports for the`CLIPBOARD`selection.
These include`STRING`and`COMPOUND_TEXT`.
* **_MOTIF_EXPORT_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets to be
used as the value of the DragContext's`XmNexportTargets`in a
drag-and-drop transfer.
These include`STRING`and`COMPOUND_TEXT`.


As a source of data, Scale also supports the following standard Motif
targets:

* **`BACKGROUND`** 

The widget transfers`XmNbackground`as type`PIXEL`.
* **`CLASS`** 

The widget finds the first shell in the widget hierarchy that has aWM_CLASSproperty and transfers the contents as text in the
current locale.
* **`CLIENT_WINDOW`** 

The widget finds the first shell in the widget hierarchy and transfers
its window as type`WINDOW`.
* **`COLORMAP`** 

The widget transfers`XmNcolormap`as type`COLORMAP`.
* **`FOREGROUND`** 

The widget transfers`XmNforeground`as type`PIXEL`.
* **`NAME`** 

The widget finds the first shell in the widget hierarchy that has aWM_NAMEproperty and transfers the contents as text in the current
locale.
* **`TARGETS`** 

The widget transfers, as type`ATOM`, a list of the targets it
supports.
These include the standard targets in this list.
These also include`STRING`and`COMPOUND_TEXT`.
* **`TIMESTAMP`** 

The widget transfers the timestamp used to acquire the selection as type`INTEGER`.
* **_MOTIF_RENDER_TABLE** 

The widget transfers`XmNrenderTable`if it exists, or else the
default text render table, as type`STRING`.
* **_MOTIF_ENCODING_REGISTRY** 

The widget transfers its encoding registry as type`STRING`.
The value is a list of NULL separated items in the
form of tag encoding pairs.
This target symbolizes the transfer target for the
Motif Segment Encoding Registry.
Widgets and applications can use this Registry to register
text encoding formats for specified render table tags.
Applications access this Registry by calling`XmRegisterSegmentEncoding`and`XmMapSegmentEncoding`.

### Descendants


Scale automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`Scrollbar``XmScrollBar`scroll bar`Title``XmLabelGadget`title of scale
### Classes


Scale inherits behavior, resources, and traits from`Core`,`Composite`,`Constraint`, and`XmManager`classes.

The class pointer is`xmScaleWidgetClass`.

The class name is`XmScale`.
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

`XmScale Resource Set``Name``Class``Type``Default``Access`XmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNdecimalPointsXmCDecimalPointsshort0CSGXmNdragCallbackXmCCallbackXtCallbackListNULLCXmNeditableXmCEditableBooleanTrueCSGXmNfontListXmCFontListXmFontListdynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNmaximumXmCMaximumint100CSGXmNminimumXmCMinimumint0CSGXmNorientationXmCOrientationunsigned charXmVERTICALCSGXmNprocessingDirectionXmCProcessingDirectionunsigned chardynamicCSGXmNscaleHeightXmCScaleHeightDimension0CSGXmNscaleMultipleXmCScaleMultipleintdynamicCSGXmNscaleWidthXmCScaleWidthDimension0CSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNshowArrowsXmCShowArrowsXtEnumXmNONECSGXmNshowValueXmCShowValueXtEnumXmNONECSGXmNsliderMarkXmCSliderMarkXtEnumdynamicCSGXmNsliderVisualXmCSliderVisualXtEnumdynamicCSGXmNslidingModeXmCSlidingModeXtEnumXmSLIDERCSGXmNtitleStringXmCTitleStringXmStringNULLCSGXmNvalueXmCValueintdynamicCSGXmNvalueChangedCallbackXmCCallbackXtCallbackListNULLC

* **`XmNconvertCallback`** 

Specifies a list of callbacks called when the Scale is asked to convert
a selection.
The type of the structure whose address is passed to these callbacks isXmConvertCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNdecimalPoints`** 

Specifies the number of decimal points to shift the slider value when
displaying it. For example, a slider value of 2,350
and an`XmdecimalPoints`value of 2 results in a display value of 23.50.
The value must not be negative.
* **`XmNdragCallback`** 

Specifies the list of callbacks that is called
when the slider position changes as the slider is being
dragged. The reason sent by the callback is`XmCR_DRAG`.
* **`XmNeditable`** 

Specifies how the Scale scrollbar will react to user input. This
resource can be True or False values, as follows:

* **`True`** 

Allows the scrollbar to be sensitive to user input. This is the
default value.
* **`False`** 

Makes the Scale scrollbar insensitive to user input. The visual is not
greyed out. This value would mostly be used in`XmTHERMOMETER`mode.
When`XmNeditable`is used on a widget
it sets the dropsite to`XmDROP_SITE_ACTIVE`.
* **`XmNfontList`** 

Specifies the font list to use for the title text string specified by`XmNtitleString`, and the label displayed when`XmNshowValue`is True. The font list is an obsolete structure, and is retained only
for compatibility with earlier releases of Motif. See the`XmNrenderTable`resource.
* **`XmNhighlightOnEnter`** 

Specifies whether the highlighting rectangle is drawn when the cursor moves
into the widget.
If the shell's focus policy is`XmEXPLICIT`, this resource is
ignored, and the widget is highlighted when it has the focus.
If the shell's focus policy
is`XmPOINTER`and if this resource is
True, the highlighting rectangle is drawn when the the cursor moves into
the widget.
If the shell's focus
policy is`XmPOINTER`and if this resource is
False, the highlighting rectangle is not drawn when the the cursor moves
into the widget.
The default is False.
* **`XmNhighlightThickness`** 

Specifies the size of the
slider's border drawing rectangle used for enter window and
traversal highlight drawing.
* **`XmNmaximum`** 

Specifies the slider's maximum value.`XmNmaximum`must be greater than`XmNminimum`.
* **`XmNminimum`** 

Specifies the slider's minimum value.`XmNmaximum`must be greater than`XmNminimum`.
* **`XmNorientation`** 

Displays Scale vertically or horizontally.
This resource can have values of`XmVERTICAL`and`XmHORIZONTAL`.
* **`XmNprocessingDirection`** 

Specifies whether the value for`XmNmaximum`is on the right or
left side of`XmNminimum`for horizontal Scales
or above or below`XmNminimum`for vertical Scales.
This resource can have values of`XmMAX_ON_TOP, XmMAX_ON_BOTTOM,
XmMAX_ON_LEFT`, and`XmMAX_ON_RIGHT`.
If the Scale is oriented vertically, the default value is`XmMAX_ON_TOP`.
If the XmScale is oriented horizontally, the default value
depends on the`XmNlayoutDirection`resource of the widget.
* **`XmNrenderTable`** 

Specifies the render table to use for the title text string specified
by`XmNtitleString`, and the label displayed when`XmNshowValue`is True. If this value is NULL at initialization,
the parent hierarchy is searched for an ancestor that holds the`XmQTspecifyRenderTable`trait. If such an ancestor is found, the
render table is initialized to the`XmLABEL_RENDER_TABLE`value of
the ancestor widget. If no such ancestor is found, the default is
implementation dependent. If a font list (`XmNfontList`) and a
render table are both specified, the render table will take
precedence. Refer to &cdeman.XmRenderTable; for more information on
the creation and structure of a render table.
* **`XmNscaleHeight`** 

Specifies the height of the slider area.
The value should be in the specified unit type (the default is pixels).
If no value is specified a default height is computed.
* **`XmNscaleMultiple`** 

Specifies the amount to move the slider when the user takes an action
that moves the slider by a multiple increment.
The default is (`XmNmaximum`&minus;`XmNminimum`) divided by 10,
with a minimum of 1.
* **`XmNscaleWidth`** 

Specifies the width of the slider area.
The value should be in the specified unit type (the default is pixels).
If no value is specified a default width is computed.
* **`XmNshowArrows`** 

Specifies whether the arrows are displayed
and how they are to be displayed. This resource can take the
following values:

* **`XmEACH_SIDE`** 

Indicates that one arrow is displayed on each end of the ScrollBar slider.
* **`XmMAX_SIDE`** 

Indicates that one arrow is displayed on the`XmNmaximum`side of
the ScrollBar slider.
* **`XmMIN_SIDE`** 

Indicates that one arrow is displayed on the`XmNminimum`side of
the ScrollBar slider.
* **`XmNONE`** 

Indicates that no arrows are displayed.


`XmNONE`is the default value.
* **`XmNshowValue`** 

Specifies whether a label for the current slider
value should be displayed next to the slider. If the value is`XmNEAR_SLIDER`, the current slider value is displayed. If the
value is`XmNONE`, no slider value is displayed.
If the value is`XmNEAR_BORDER`, the current slider value is
displayed near the border.
* **`XmNsliderMark`** 

Specifies the shape the slider is to be displayed in. This resource
can take the following values:

* **`XmETCHED_LINE`** 

Specifies the slider as an etched line. This is the default when`XmNslidingMode`is`XmSLIDER`.
* **`XmNONE`** 

Specifies the slider as a foregrounded rectangle. This is the default when`XmNslidingMode`is`XmTHERMOMETER`and the Scale scrollbar is
insensitive to user input (`XmNeditable`is`False`.
* **`XmROUND_MARK`** 

Specifies the slider as a shadowed circle. This is the default when`XmNslidingMode`is`XmTHERMOMETER`and the Scale scrollbar is
sensitive to user input (`XmNeditable`is`True`.
* **`XmTHUMB_MARK`** 

Specifies the slider as a series of three etched lines centered in the
middle of the slider.

* **`XmNslidingMode`** 

Specifies the mode the slider works in. There are two possible modes:

* **`XmSLIDER`** 

Allows the slider to move freely between the minimum and maximum ends
of the scale. This is the default value.
* **`XmTHERMOMETER`** 

Forces the slider to be anchored to one side of the trough area.

* **`XmNsliderVisual`** 

Specifies the color of the slider visual.
This resource can take
the following values:

* **`XmBACKGROUND_COLOR`** 

Specifies that the slider visual is in the background color.
* **`XmFOREGROUND_COLOR`** 

Specifies that the slider visual is in the foreground color.
* **`XmSHADOWED_BACKGROUND`** 

Specifies that the slider visual is in the background color, with a shadow.
This is the
default when the`XmNslidingModel`resource is`XmSLIDER`.
* **`XmTROUGH_COLOR`** 

Specifies that the slider visual is in the trough color. This is the
default when the`XmNslidingModel`resource is`XmTHERMOMETER`.

* **`XmNtitleString`** 

Specifies the title text string to appear in the Scale widget window.
* **`XmNvalue`** 

Specifies the slider's current position along the scale,
between`XmNminimum`and`XmNmaximum`.
The value is constrained to be within these inclusive bounds.
The initial value of this resource is the larger of 0 (zero)
and`XmNminimum`.
* **`XmNvalueChangedCallback`** 

Specifies the list of callbacks that is called
when the value of the slider has changed. The reason
sent by the callback is`XmCR_VALUE_CHANGED`.

### Inherited Resources


Scale inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to the`XmNdragCallback`and`XmNvalueChangedCallback`procedures:typedef struct
{
        int`reason`;
        XEvent`* event`;
        int`value`;
} XmScaleCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`value`** 

Is the new slider value


A pointer to the following structure is passed to the`XmNconvertCallback`procedures:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Atom`selection`;
        Atom`target`;
        XtPointer`source_data`;
        XtPointer`location_data`;
        int`flags`;
        XtPointer`parm`;
        int`parm_format`;
        unsigned long`parm_length`;
        int`status`;
        XtPointer`value`;
        Atom`type`;
        int`format`;
        unsigned long`length`;
} XmConvertCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
It can be NULL.
* **`selection`** 

Indicates the selection for which conversion is being requested.
Possible values are`CLIPBOARD`,`PRIMARY`,`SECONDARY`,
and_MOTIF_DROP.
* **`target`** 

Indicates the conversion target.
* **`source_data`** 

Contains information about the selection source.
When the selection is_MOTIF_DROP,`source_data`is the
DragContext.
Otherwise, it is NULL.
* **`location_data`** 

Contains information about the location of data to be converted.
If the value is NULL, the data to be transferred consists of the
widget's current selection.
* **`flags`** 

Indicates the status of the conversion. Following are the possible
values:

* **`XmCONVERTING_NONE`** 

This flag is currently unused.
* **`XmCONVERTING_PARTIAL`** 

The target widget was able to be converted, but some data was lost.
* **`XmCONVERTING_SAME`** 

The conversion target is the source of the data to be transferred.
* **`XmCONVERTING_TRANSACT`** 

This flag is currently unused.

* **`parm`** 

Contains parameter data for this target.
If no parameter data exists, the value is NULL.

When`selection`is`CLIPBOARD`and`target`is_MOTIF_CLIPBOARD_TARGETSor_MOTIF_DEFERRED_CLIPBOARD_TARGETS, the value is the requested
operation (`XmCOPY`,`XmMOVE`, or`XmLINK`).
* **`parm_format`** 

Specifies whether the data in`parm`should be viewed
as a list of`char`,`short`, or`long`quantities.
Possible values are 0 (when`parm`is NULL),
8 (when the data in`parm`should be viewed as a list of`char`s),
16 (when the data in`parm`should be viewed as a list of`short`s),
or 32 (when the data in`parm`should be viewed as a list of`long`s).
Note that`parm_format`symbolizes a data type, not the number of bits
in each list element.
For example, on some machines, a`parm_format`of 32 means that
the data in`parm`should be viewed as a list of 64-bit quantities,
not 32-bit quantities.
* **`parm_length`** 

Specifies the number of elements of data in`parm`, where each
element has the size specified by`parm_format`.
When`parm`is NULL, the value is 0.
* **`status`** 

An IN/OUT member that specifies the status of the conversion.
The initial value is`XmCONVERT_DEFAULT`.
The callback procedure can set this member to one of the following
values:

* **`XmCONVERT_DEFAULT`** 

This value means that the widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it
overwrites the data provided by the callback procedures in the`value`member.
* **`XmCONVERT_MERGE`** 

This value means that the widget class conversion procedure, if any, is
called after the callback procedures return.
If the widget class conversion procedure produces any data, it appends
its data to the data provided by the callback procedures in the`value`member.
This value is intended for use with targets that result in lists of
data, such as`TARGETS`.
* **`XmCONVERT_DONE`** 

This value means that the callback procedure has successfully finished
the conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.
* **`XmCONVERT_REFUSE`** 

This value means that the callback procedure has terminated the
conversion process without completing the requested conversion.
The widget class conversion procedure, if any, is not called after the
callback procedures return.

* **`value`** 

An IN/OUT parameter that contains any data that the callback procedure
produces as a result of the conversion.
The initial value is NULL.
If the callback procedure sets this member, it must ensure that the`type`,`format`, and`length`members correspond
to the data in`value`.
The callback procedure is responsible for allocating, but not for
freeing, memory when it sets this member.
* **`type`** 

An IN/OUT parameter that indicates the type of the data in the`value`member.
The initial value is`INTEGER`.
* **`format`** 

An IN/OUT parameter that specifies whether the data in`value`should
be viewed as a list of`char`,`short`, or`long`quantities.
The initial value is 8.
The callback procedure can set this member to 8 (for a list of`char`),
16 (for a list of`short`), or 32 (for a list of`long`).
* **`length`** 

An IN/OUT member that specifies the number of elements of data in`value`, where each element has the size symbolized by`format`.
The initial value is 0.

### Behavior


XmScale has the following behavior:

* **Btn1Down&ensp;or&ensp;Btn2Down:** 

`In the region between an end of the Scale and the slider`:
Moves the slider by one multiple increment in
the direction of the end of the
Scale and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`, movement toward
the right or
bottom increments the Scale value,
and movement toward the left or top decrements the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`, movement toward the right or
bottom decrements the Scale value,
and movement toward the left or top increments the Scale value.
If the button is held down longer than a delay period, the slider is
moved again by the same increment and the same callbacks are called.

`In slider:`Activates the interactive dragging of the slider.
* **Btn2Down&ensp;in&ensp;value&ensp;label:** 

Drags the contents of the label showing the current slider value.
This action sets the`XmNconvertProc`of the DragContext to a
function that calls the`XmNconvertCallback`procedures, possibly
multiple times, for the_MOTIF_DROPselection.
* **Btn1Motion&ensp;or&ensp;Btn2Motion:** 

If the button press occurs within the slider, the subsequent motion
events move the slider to the position of the pointer and call the
callbacks for`XmNdragCallback`.
* **Btn1Up&ensp;or&ensp;Btn2Up:** 

If the button press occurs within the slider and the slider position
is changed, the callbacks for`XmNvalueChangedCallback`are called.
* **CtrlBtn1Down:** 

`In the region between an end of the Scale and the slider`:
Moves the slider to that end of the Scale and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`or`XmMAX_ON_BOTTOM`,
movement toward the right or bottom increments the Scale value,
and movement toward
the left or top
decrements
the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`or`XmMAX_ON_TOP`, movement toward the right or
bottom decrements the Scale value,
and movement toward the left or top increments the Scale value.
* **KeyosfUp:** 

For vertical Scales,
moves the slider up one increment and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_TOP`, movement toward
the top increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_BOTTOM`, movement toward
the top decrements the Scale value.
* **KeyosfDown:** 

For vertical Scales,
moves the slider down one increment and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_BOTTOM`, movement toward
the bottom increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_TOP`, movement toward
the bottom decrements the Scale value.
* **KeyosfLeft:** 

For horizontal Scales,
moves the slider one increment to the left and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`, movement toward
the left increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`, movement toward
the left decrements the Scale value.
* **KeyosfRight:** 

For horizontal Scales,
moves the slider one increment to the right and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`, movement toward
the right increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`, movement toward
the right decrements the Scale value.
* **CtrlKeyosfUp&ensp;or&ensp;KeyosfPageUp:** 

For vertical Scales,
moves the slider up one multiple increment and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_TOP`, movement toward
the top increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_BOTTOM`, movement toward
the top decrements the Scale value.
* **CtrlKeyosfDown&ensp;or&ensp;KeyosfPageDown:** 

For vertical Scales,
moves the slider down one multiple increment and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_BOTTOM`, movement toward
the bottom increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_TOP`, movement toward
the bottom decrements the Scale value.
* **CtrlKeyosfLeft&ensp;or&ensp;KeyosfPageLeft:** 

For horizontal Scales,
moves the slider one multiple increment to the left and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`, movement toward
the left increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`, movement toward
the left decrements the Scale value.
* **CtrlKeyosfRight&ensp;or&ensp;KeyosfPageRight:** 

For horizontal Scales,
moves the slider one multiple increment to the right and calls the`XmNvalueChangedCallback`callbacks.
If`XmNprocessingDirection`is`XmMAX_ON_RIGHT`, movement toward
the right increments the Scale value.
If`XmNprocessingDirection`is`XmMAX_ON_LEFT`, movement toward
the right decrements the Scale value.
* **KeyosfBeginLine&ensp;or&ensp;KeyosfBeginData:** 

Moves the slider to the minimum value and calls the`XmNvalueChangedCallback`callbacks.
* **KeyosfEndLine&ensp;or&ensp;KeyosfEndData:** 

Moves the slider to the maximum value and calls the`XmNvalueChangedCallback`callbacks.
* **KeyosfNextField:** 

Traverses to the first item in the next tab group. If
the current tab group is the last entry in the tab group list, it
wraps to the beginning of the tab group list.
* **KeyosfPrevField:** 

Traverses to the first item in the previous tab group.
If the beginning of the tab group list is reached, it wraps to the end
of the tab group list.
* **KeyosfHelp:** 

Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;, &cdeman.Constraint;, &cdeman.Core;,
&cdeman.XmCreateScale;,
&cdeman.XmManager;,
&cdeman.XmScaleGetValue;, and
&cdeman.XmScaleSetValue;.