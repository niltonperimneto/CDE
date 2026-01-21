# XmComboBox
library call`XmComboBox`The ComboBox widget classXmComboBoxwidget classComboBox#include <Xm/ComboBox.h>
## DESCRIPTION


`XmComboBox`combines the capabilities of a single-line
TextField widget
and a List widget. It allows users to perform opoerations like
typing and pasting information, and it also
provides a list of possible choices that the user
can select from to complete
the TextField entry field. The list can either be displayed at all
times or can be dropped down by the user. When the list portion of the
ComboBox is hidden, users are given a visual cue (a
downward-pointing arrow)
next to the TextField field. The position of the arrow relative to the
TextField field depends on the`XmNlayoutDirection`resource of the
widget. This
version of the
ComboBox is called
the "drop-down" ComboBox. Drop-down ComboBoxes are useful
when screen space is limited, or when users will
complete the text entry field more often by typing text than by
selecting the entry field text from the list.
The user can access the drop-down ComboBox in either one of two ways:

By clicking and releasingBtn1on the downward-pointing arrow,
which
pops the list up, and the list stays up. A later selection of an item
in the list will cause the item to appear in the text entry field, and
the list will unpost itself.

By pressingBtn1on the downward-pointing arrow, dragging
it to a list item, and then releasing it there, which selects that
item. The list
disappears, and the selected item appears in the text entry field.

The application provides an array of strings that fill the list. At
creation time, string items can be passed to the ComboBox via an
arglist.
Each string becomes an item in the list, with the first string becoming
the item in position 1, the second string becoming the item in position 2,
and so on. The size of the list is
set by specifying the number of items that are visible in the list
(`XmNvisibleItemCount`). If the number of items in the list exceeds the
number of items that are visible, a vertical scroll bar will
automatically appear that allows the user to scroll through a
large number of items.

ComboBox creates two child widgets: a TextField widget for entering
text and a ScrolledWindow containing a List for the list of items.
The name of the items list itself is`List`, and the name of the
TextField is`Text`.
The application or user can specify resource values for these widgets in
a resource file, and the application can use`XtNameToWidget`(specifying`"*List"`for the items list or`"*Text"`for the
TextField widget) to obtain the widget IDs of the
descendant widgets.
At creation time, ComboBox passes appropriate resource values in the
creation arglist, including`XmNitems`, to the items list.
Note that the result of providing the`XmNdestroyCallback`resource in the creation`arglist`is unspecified. The
application should use the`XtAddCallback`function to add
callbacks to the appropriate widget (TextField or List) after
creating it.

ComboBox forces the following resource values
on its List child:

If`XmNcomboBoxType`is`XmCOMBO_BOX`,`XmNtraversalOn`is forced to False.

`XmNhighlightThickness`is forced to 2 in a drop-down ComboBox and
to 0 in other types of ComboBoxes.

`XmNborderWidth`is forced to 0.

`XmNnavigationType`is forced to`XmNONE`.

`XmNselectionPolicy`is forced to`XmBROWSE_SELECT`.

`XmNlistSizePolicy`is forced to`XmVARIABLE`.

`XmNspacing`is forced to 0.

`XmNvisualPolicy`is forced to`XmVARIABLE`.

`XmNselectedPositions`is forced to NULL.

`XmNselectedPositionsCount`is forced to 0.

When`XmNcomboBoxType`is`XmDROP_DOWN_LIST`,
ComboBox forces the following resource values on
its TextField child:

`XmNeditable`is forced to False.

`XmNcursorPositionVisible`is forced to False.

`XmNshadowThickness`is forced to 0.

By contrast, when`XmNcomboBoxType`is`XmCOMBO_BOX`or`XmDROP_DOWN_COMBO_BOX`, ComboBox forces the following
resource values on its TextField child:

`XmNeditable`is forced to True.

`XmNcursorPositionVisible`is forced to True.

`XmNeditMode`is forced to`XmSINGLE_LINE_EDIT`.

ComboBox always forces the values of the following resources on
the TextField:

`XmNnavigationType`is forced to`XmNONE`.

`XmNhighlightThickness`is forced to 0.

`XmNborderWidth`is forced to 0.

ComboBox allows a single item to be selected in two ways: by
selecting the desired item from the List or by entering text
into the TextField.
ComboBox does not automatically select a list item if the user types
that string into the TextField. It selects the item when the user
presses`KActivate`or moves the focus.
ComboBox supports the Browse
Select selection model of List (see the`XmList`reference page
for a description of this model), so selections are mutually
exclusive. Selecting an item from the list causes
that item to be displayed in the TextField portion of the
ComboBox.
If an application sets the`XmNvalue`resource of TextField,
that string is shown in the TextField.
If the application has not provided any list items,
or if there is no current selection, the TextField
is empty.

The TextField in the ComboBox widget
can be either editable or noneditable, depending on the value of
the`XmNcomboBoxType`resource.

If the TextField is editable, the user can type into it.
When the user presses the Return key, the ComboBox will
compare the typed entry to the items in the List.
If there is an exact match, then the matched List item is
selected.
If the application wishes to validate the entered text (for example, to
ensure that the typed selection is a valid one), it can do so by
setting`XmNmodifyVerifyCallback`on the TextField widget.

If the TextField is noneditable, typing text may invoke a matching
algorithm that will attempt to match the entered text with items in
the list. The specific matching algorithm applied, which may be none,
is determined by the value of the`XmNmatchBehavior`resource in
ComboBox, which can be either`XmNONE`or`XmQUICK_NAVIGATE`. A value
of`XmNONE`indicates that no matching algorithm will occur. A value of`XmQUICK_NAVIGATE`indicates that when the List widget has focus,
one-character navigation is supported. In this algorithm, if the typed
character is the initial character of some item in the List, this
algorithm causes that item to be navigated to and selected, and the
item is displayed in the TextField. Subsequently typing the same
character will cycle among the items with the same first character.

Regardless of the selection mechanism used (either selected directly
from the List or typed into the TextField), when an item in the
List is selected, that item is highlighted in the List.
In addition, the selected item is displayed in the
TextField of the ComboBox. If the user performs an action
that would move focus away from ComboBox, or selects a List item,
the`XmNselectionCallback`callbacks are invoked to notify
the application of the current contents of the TextField (or choice). The
application then takes whatever action is required based on those
contents (or choice).

ComboBox uses the`XmQTspecifyRenderTable`trait and holds the`XmQTaccessTextual`trait.
### Classes


`XmComboBox`inherits behavior, resources, and traits from`Core`,`Composite`,
and`XmManager`classes.

The class pointer is`xmComboBoxWidgetClass`.

The class name is`XmComboBox`.
### New Resources


The following table defines a set of widget resources used by
the programmer to specify data. The programmer can also set the
resource values for the inherited classes to set attributes for
this widget. To reference a resource by name or by class in
a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and
use the
remaining letters. To specify one of the defined values for a
resource in a.Xdefaultsfile, remove the`Xm`prefix and use the
remaining letters (in either lowercase or uppercase, but
include any underscores between words). The codes in the
access column indicate if the given resource can be at
creation time (C), set by using`XtSetValues`(S), retrieved by
using`XtGetValues`(G), or is not applicable (N/A).`XmComboBox Resource Set``Name``Class``Type``Default``Access`XmNarrowSizeXmCArrowSizeDimensiondynamicCSGXmNarrowSpacingXmCArrowSpacingDimensiondynamicCSGXmNcolumnsXmCColumnshortdynamicCSGXmNcomboBoxTypeXmCComboBoxTypeunsigned charXmCOMBO_BOXCGXmNfontListXmCFontListXmFontListNULLCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNitemCountXmCItemCountintdynamicCSGXmNitemsXmCItemsXmStringTabledynamicCSGXmNlistXmCListWidgetdynamicGXmNmarginHeightXmCMarginHeightDimension2CSGXmNmarginWidthXmCMarginWidthDimension2CSGXmNmatchBehaviorXmCMatchBehaviorunsigned chardynamicCSGXmNpositionModeXmCPositionModeXtEnumXmZERO_BASEDCGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNselectedItemXmCSelectedItemXmStringNULLCSGXmNselectedPositionXmCSelectedPositionintdynamicCSGXmNselectionCallbackXmCCallbackXmCallbackListNULLCXmtextFieldXmCTextFieldWidgetdynamicGXmNvisibleItemCountXmCVisibleItemCountint10CSG

* **`XmNarrowSize`** 

Specifies the width of the arrow. The default size
depends on the size of the text, as well as
the size of the ComboBox.
* **`XmNarrowSpacing`** 

Specifies the space between the text and arrow visual in pixels. The
default value is obtained from the`XmNmarginWidth`resource.
* **`XmNcolumns`** 

Specifies the number of columns in the text field. If unset,
the text field's value is used. Refer to the`XmTextField`man page
for more detailed information.
* **`XmNcomboBoxType`** 

Specifies the type of ComboBox to be created. This can be one of
the following:

* **`XmCOMBO_BOX`** 

Generates a ComboBox where the list is always displayed, and the text
entry field is editable.
* **`XmDROP_DOWN_COMBO_BOX`** 

Generates a ComboBox where the list is hidden unless specifically
requested, and the text entry field is editable.
* **`XmDROP_DOWN_LIST`** 

Generates a ComboBox where the list is hidden unless specifically
requested, and the text entry field is noneditable.

* **`XmNfontList`** 

Specifies the fontlist associated with`XmComboBox`. The fontlist
is an obsolete construct, and has been superseded by the render table.
It is included for compatibility with earlier versions of Motif, and
for applications that do not easily support render tables. The
default fontlist is derived from the default render table, and if
both a fontlist and a render table are specified, the render table
takes precedence.
* **`XmNhighlightThickness`** 

Specifies the thickness of the highlighting rectangle.
* **`XmNitemCount`** 

Specifies the number of items in the list. If unset,
the lists's value is used.
Refer to the`XmList`man page for more detailed information.
* **`XmNitems`** 

Specifies the items in the list. If unset,
the lists's value is used. Refer to the`XmList`man page for more detailed information.
* **`XmNlist`** 

The list widget.
* **`XmNmarginWidth`** 

Specifies the horizontal spacing between the child widgets and the boundary
of the ComboBox.
* **`XmNmarginHeight`** 

Specifies the vertical spacing between the child widgets and the boundary
of the ComboBox.
* **`XmNmatchBehavior`** 

Defines the matching algorithm applied to match the text
typed by the user in the TextField field with items in the list.
The current values are`XmNONE`and`XmQUICK_NAVIGATE`, as follows:

* **`XmNONE`** 

Indicates that there is no assigned matching algorithm.
* **`XmQUICK_NAVIGATE`** 

Is only valid for noneditable ComboBoxes (`XmNcomboBoxType`resource value`XmDROP_DOWN_LIST`). This algorithm supports 1-character navigation
when the List widget has focus. If the typed character is the
initial character of some item in the List, this algorithm causes
that item to be navigated to and selected. Subsequently typing the
same character will cycle among the items with the same first character.

* **`XmNpositionMode`** 

Specifies how the value of the`XmNselectedPosition`resource and the`item_position`field
of the callback structure are to be interpreted. Note that
the convenience functions`XmComboBoxDeletePos`and`XmComboBoxAddItem`are not affected by this resource, and (like`XmList`) always use
1-based positions. Valid values for this resource are:

* **`XmZERO_BASED`** 

(DtComboBox compatibility mode: default)`XmNselectedPosition`is in`[0,itemcount-1]`.
The`item_position`in the`XmComboBoxCallbackStruct`is 0 if the first element in the list
was selected. Note that 0 is also returned if no element in the list
was selected (that is, a new item was entered in the text field).
* **`XmONE_BASED`** 

(Motif mode) Both the resource value and the callback fields
are 1-based. This is consistent with other Motif widgets.

* **`XmNrenderTable`** 

Specifies the render table associated with ComboBox.
This render table is used in both the TextField field and the
List in the ComboBox. This is used in conjunction with the`XmNvisibleItemCount`resource of the List to determine the height
of the ComboBox widget.

If this value is NULL at initialization, and if the widget parent is`XmBulletinBoard`or its subclasses,`VendorShell`or its subclasses, or`XmMenuShell`, then the
widget parent provides the default render table associated with
the widget.
If both a render table and a fontlist are specified, the render table
will take precedence.
* **`XmNselectedItem`** 

Specifies a compound string that represents the current selection of
the ComboBox. The selected item is the content of the
ComboBox text entry field.
* **`XmNselectedPosition`** 

If the selection in the ComboBox is an item in the list, this is
the index of the selected item in the list. If no item in
the list is selected, this is 0.
* **`XmNselectionCallback`** 

Specifies the list of callbacks called when an item is selected.
The reason field in theXmComboBoxCallbackStructis`XmCR_SELECT`.
* **`XmNtextField`** 

The text field widget.
* **`XmNvisibleItemCount`** 

Specifies the number of visible items in the list. This will override any
value specified for the list.
Refer to the`XmList`man page for more detailed information.

### Inherited Resources


ComboBox inherits behavior and resources from superclasses described
in the following tables. For a complete description of each
resource, refer to the reference page for that superclass.`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmSTICKY_TAG_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback.
The callback structure is defined as follows:typedef struct
{
        int`reason`;
        XEvent *`event`;
        XmString`item_or_text`;
        int`item_position`;
} XmComboBoxCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback. It can be NULL.
* **`item_or_text`** 

The contents of the text field at the time the event caused the
callback. The`item_or_text`parameter points to a temporary
storage space that is
reused after the callback is finished. If an application needs to save
the item, it should copy`item_or_text`into its own data space.
* **`item_position`** 

The position of item in the list's`XmNitems`1-based array. If this
is 0, it means that the`item_or_text`was not selected from the
List.

### Translations


The ComboBox translations are listed below.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.The KPageUp and KPageDown translations do not take effect unless theListchild widget is posted.

* **Btn1Down:** 

CBArmAndDropDownList()
* **Btn1Up:** 

CBDisarm()


The following lists the List translations in the drop-down list. When
ComboBox`XmNcomboBoxType`is`XmDROP_DOWN_LIST`,osfActivate,osfCancel, and`Return`are overriden by ComboBox actions.

* **`:c`<Key>osfDown:** 

CBDropDownList()
* **`:c`<Key>osfUp:** 

CBDropDownList()
* **`:`<Key>osfCancel:** 

CBCancel()
* **`:`<Key>osfActivate:** 

CBActivate()
* **`&ap;s &ap;m &ap;a`<Key>`Return`:** 

CBActivate()

### Accelerators


The following accelerators are added to ComboBox and its children.
The accelerators may not directly correspond to a translation table.
If the translation is not listed below, it may depend on the context
of the event.

* **`:c`<Key>osfUp:** 

CBDropDownList()
* **`:`<Key>osfUp:** 

CBListAction(`Up`)
* **`:c`<Key>osfDown:** 

CBDropDownList()
* **`:`<Key>osfDown:** 

CBListAction(`Down`)
* **`:c`<Key>osfBeginLine:** 

CBListAction(`ListBeginData`)
* **`:c`<Key>osfEndLine:** 

CBListAction(`ListEndData`)
* **`:`<Key>osfPageUp:** 

CBListAction(`ListPrevPage`)
* **`:`<Key>osfPageDown:** 

CBListAction(`ListNextPage`)


A drop-down ComboBox also adds accelerators to its List child.
Aside from the accelerators that are already listed in this section,
those accelerators are the default TextField key translations.
### Action Routines


The`XmComboBox`action routines are as follows:

* **CBActivate():** 

Calls the`XmNselectionCallback`callbacks. If the`XmNcomboBoxType`is`XmDROP_DOWN_COMBO_BOX`or`XmDROP_DOWN_LIST`, it unposts the
list. If the
parent is a manager, passes the event to the parent.
* **CBArmAndDropDownList():** 

If the pointer is within the down arrow, draws the shadow of the arrow
in the selected state, and then posts the list.
* **CBCancel():** 

If the`XmNcomboBoxType`is`XmDROP_DOWN_COMBO_BOX`or`XmDROP_DOWN_LIST`, pops down
the list. If the parent is a manager, passes the event to the parent.
* **CBDisarm():** 

Redraws the arrow in an unselected state.
* **CBDropDownList():** 

If`XmNcomboBoxType`is`XmDROP_DOWN`or`XmDROP_DOWN_LIST`, and
list is not
displayed, posts the list. If list is displayed, it unposts the list.
* **CBListAction(`ListBeginData`):** 

Moves the location cursor to the first item in the list.
In Normal Mode, this also deselects any current selection,
selects the first item in the list, and calls the`XmNbrowseSelectionCallback`selection callback.
* **CBListAction(`ListEndData`):** 

Moves the location cursor to the last item in the list.
In Normal Mode, this also deselects any current selection,
selects the last item in the list, and calls the`XmNbrowseSelectionCallback`selection callback.
* **CBListAction(`ListPrevPage`):** 

Scrolls the list to the previous page, moving the location cursor to a
new item.
This action also selects the new item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.
If the ComboBox is not a drop-down type, then this action does
nothing.
* **CBListAction(`ListNextPage`):** 

Scrolls the list to the next page, moving the location cursor to a new
item.
This action also selects the new item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.
If the ComboBox is not a drop-down type, then this action does
nothing.
* **CBListAction(`Up`):** 

Moves the location cursor to the previous item in the list.
This action also selects the previous item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.
Note that, unlike the List`ListPrevItem`action, this action wraps
around.
* **CBListAction(`Down`):** 

Moves the location cursor to the next item in the list.
This action also selects the next item, deselects any current
selection, and calls the`XmNbrowseSelectionCallback`callbacks.
Note that, unlike the List`ListNextItem`action, this action wraps
around.

### Virtual Bindings


The bindings for virtual keys are vendor specific. For information
about bindings for virtual buttons and keys, see
&cdeman.VirtualBindings;.
## ERRORS/WARNINGS


The toolkit will display a warning if the application tries to
set the value of`XmNlist`or the`XmNtextField`resource, which are read-only (marked G in the resource
table).
## RELATED


&cdeman.Composite;,
&cdeman.Core;,
&cdeman.XmCreateComboBox;,
&cdeman.XmList;,
&cdeman.XmManager;, and
&cdeman.XmTextField;.