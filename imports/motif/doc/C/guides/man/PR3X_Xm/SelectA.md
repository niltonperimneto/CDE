# XmSelectionBox
library call`XmSelectionBox`The SelectionBox widget classXmSelectionBoxwidget classSelectionBox&npzwc;#include &lt;Xm/SelectioB.h>
## DESCRIPTION


SelectionBox is a general dialog widget that allows the user to
select one item from a list.
By default, a SelectionBox includes the following:

A scrolling list of alternatives

An editable text field for the selected alternative

Labels for the list and text field

Three or four buttons

The default button labels are`OK`,`Cancel`, and`Help`.
By default an`Apply`button is also created; if the parent of the
SelectionBox is a DialogShell, it is managed; otherwise it is unmanaged.
Additional children may be added to the SelectionBox after
creation.
The first child is used as a work area. The value of`XmNchildPlacement`determines if the work area is placed above
or below the Text area, or above or below the List area. Additional
children are laid out in the following manner:

* **Menubar** 

The first menu bar child is placed at the top of the window.
The`XmQTmenuSystem`trait is used to check that it is the first
MenuBar child.
* **Buttons** 

All`XmPushButton`widgets or gadgets, and their subclasses are
placed after the`OK`button in the order of their
creation (this order is checked using the`XmQTactivatable`trait).
The layout direction of the buttons depends on the`XmNlayoutDirection`resource.


The layout of additional children that are not in the above categories
is undefined.

The user can select an item in two ways:
by scrolling through the list and selecting the desired
item or by entering the item name directly into the text edit area.
Selecting an item from
the list causes that item name to appear in the selection text edit
area.

The user may select a new item
as many times as desired. The item is not actually selected until
the user presses the`OK`PushButton.

The default value for the`XmBulletinBoard`resource`XmNcancelButton`is the Cancel button, unless`XmNdialogType`is`XmDIALOG_COMMAND`, when the default is NULL.
The default value for the`XmBulletinBoard``XmNdefaultButton`resource is the OK button, unless`XmNdialogType`is`XmDIALOG_COMMAND`, when the default is NULL.

For SelectionBox and its subclasses, the default value for`XmNinitialFocus`is the text edit area.

The user can specify resources in a resource file for the automatically
created widgets and gadgets of SelectionBox. The following list
identifies the names of these widgets (or gadgets) and the associated
SelectionBox areas:

* **List Items Label** 

`Items`
* **List Items** 

`ItemsList`
* **Selection Label** 

`Selection`
* **Selection Text** 

`Text`or`TextField`
* **Selection Separator** 

`Separator`


SelectionBox uses the`XmQTaccessTextual`,`XmQTactivatable`,
and`XmQTmenuSystem`traits.
### Descendants


SelectionBox automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`Apply``XmPushButtonGadget`Apply button`Cancel``XmPushButtonGadget`Cancel button`Help``XmPushButtonGadget`Help button`Items``XmLabelGadget`title above the list of items`ItemsList``XmList`list of items from which the user will select`ItemsListSW``XmScrolledWindow`ScrolledWindow parent of`ItemsList``OK``XmPushButtonGadget`OK button`Selection``XmLabelGadget`title above the selection box`Separator``XmSeparatorGadget`dividing line between selection box and buttons`Text``XmTextField`selection box containing text of selected item
### Classes


SelectionBox inherits behavior, resources, and traits from`Core`,`Composite`,`Constraint`,`XmManager`,
and`XmBulletinBoard`.

The class pointer is`xmSelectionBoxWidgetClass`.

The class name is`XmSelectionBox`.
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

`XmSelectionBox Resource Set``Name``Class``Type``Default``Access`XmNapplyCallbackXmCCallbackXtCallbackListNULLCXmNapplyLabelStringXmCApplyLabelStringXmStringdynamicCSGXmNcancelCallbackXmCCallbackXtCallbackListNULLCXmNcancelLabelStringXmCCancelLabelStringXmStringdynamicCSGXmNchildPlacementXmCChildPlacementunsigned charXmPLACE_ABOVE_SELECTIONCSGXmNdialogTypeXmCDialogTypeunsigned chardynamicCGXmNhelpLabelStringXmCHelpLabelStringXmStringdynamicCSGXmNlistItemCountXmCItemCountint0CSGXmNlistItemsXmCItemsXmStringTableNULLCSGXmNlistLabelStringXmCListLabelStringXmStringdynamicCSGXmNlistVisibleItemCountXmCVisibleItemCountintdynamicCSGXmNminimizeButtonsXmCMinimizeButtonsBooleanFalseCSGXmNmustMatchXmCMustMatchBooleanFalseCSGXmNnoMatchCallbackXmCCallbackXtCallbackListNULLCXmNokCallbackXmCCallbackXtCallbackListNULLCXmNokLabelStringXmCOkLabelStringXmStringdynamicCSGXmNselectionLabelStringXmCSelectionLabelStringXmStringdynamicCSGXmNtextAcceleratorsXmCTextAcceleratorsXtAcceleratorsdefaultCXmNtextColumnsXmCColumnsshortdynamicCSGXmNtextStringXmCTextStringXmString""CSG

* **`XmNapplyCallback`** 

Specifies the list of callbacks called
when the user activates the`Apply`button. The callback reason is`XmCR_APPLY`.
* **`XmNapplyLabelString`** 

Specifies the string label for the`Apply`button.
The default for this resource depends on the locale.
In the C locale the default is`Apply`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNcancelCallback`** 

Specifies the list of callbacks called
when the user activates
the`Cancel`button. The callback reason is`XmCR_CANCEL`.
* **`XmNcancelLabelString`** 

Specifies the string label for the`Cancel`button.
The default for this resource depends on the locale.
In the C locale the default is`Cancel`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNchildPlacement`** 

Specifies the placement of the work area child. The
possible values are

* **`XmPLACE_ABOVE_SELECTION`** 

Places the work area child above the Text area
* **`XmPLACE_BELOW_SELECTION`** 

Places the work area child below the Text area
* **`XmPLACE_TOP`** 

Places the work area child above the List area,
and below a MenuBar, if one is present

* **`XmNdialogType`** 

Determines the set of SelectionBox children widgets that are created
and managed at initialization. The possible values are

* **`XmDIALOG_PROMPT`** 

All standard children except the list and
list label are created, and all except the`Apply`button are
managed
* **`XmDIALOG_COMMAND`** 

Only the list, the selection label, and the
text field are created and managed
* **`XmDIALOG_SELECTION`** 

All standard children are created and
managed
* **`XmDIALOG_FILE_SELECTION`** 

All standard children are created and
managed
* **`XmDIALOG_WORK_AREA`** 

All standard children are created, and all
except the`Apply`button are managed


If the parent of the SelectionBox is a DialogShell, the default is`XmDIALOG_SELECTION`; otherwise, the default is`XmDIALOG_WORK_AREA`.`XmCreatePromptDialog`and`XmCreateSelectionDialog`set and append
this resource to the creation`arglist`supplied by the application.
This resource cannot be modified after creation.
* **`XmNhelpLabelString`** 

Specifies the string label for the`Help`button.
The default for this resource depends on the locale.
In the C locale the default is`Help`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNlistItems`** 

Specifies the items in the SelectionBox list.`XtGetValues`for this resource returns the list items themselves,
not a copy of the list items.
The application must not free the returned items.
* **`XmNlistItemCount`** 

Specifies the number of items in the SelectionBox
list.
The value must not be negative.
* **`XmNlistLabelString`** 

Specifies the string label to appear above the SelectionBox list
containing the selection items.
The default for this resource depends on the locale.
In the C locale the default is`Items`unless`XmNdialogType`is`XmDIALOG_PROMPT`; in this case the default is NULL.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNlistVisibleItemCount`** 

Specifies the number of items displayed in the SelectionBox
list.
The value must be greater than 0 (zero) unless`XmNdialogType`is`XmDIALOG_PROMPT`; in this case, the value is always 0.
The default is dynamic based on the height of the list.
* **`XmNminimizeButtons`** 

Sets the buttons to the width of the widest button and height of the
tallest button if False. If True, button width and height are not
modified.
* **`XmNmustMatch`** 

Specifies whether the selection widget should check if the
user's selection in the text edit field has an exact match in
the SelectionBox list when the`OK`button is activated.
If the selection does not have an exact match, and`XmNmustMatch`is True, the`XmNnoMatchCallback`callbacks are
called.
If the selection does have an exact match or if`XmNmustMatch`is
False,`XmNokCallback`callbacks are called.
* **`XmNnoMatchCallback`** 

Specifies the list of callbacks called
when the user makes a selection
from the text edit field that does not have an exact match with
any of the items in the list box.
The callback reason is`XmCR_NO_MATCH`.
Callbacks in this list are called only if`XmNmustMatch`is true.
* **`XmNokCallback`** 

Specifies the list of callbacks called
when the user activates the`OK`button.
The callback reason is`XmCR_OK`.
If the selection text does not match a list item, and`XmNmustMatch`is True, the`XmNnoMatchCallback`callbacks are
called instead.
* **`XmNokLabelString`** 

Specifies the string label for the`OK`button.
The default for this resource depends on the locale.
In the C locale the default is`OK`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNselectionLabelString`** 

Specifies the string label for the selection text edit field.
The default for this resource depends on the locale.
In the C locale the default is`Selection`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNtextAccelerators`** 

Specifies translations added to the Text widget child of the
SelectionBox.
The default includes bindings for the up and down keys
for auto selection of list items.
This resource is ignored if`XmNaccelerators`is initialized to a
nondefault value.
* **`XmNtextColumns`** 

Specifies the number of columns in the Text widget.
The value must be greater than 0 (zero).
* **`XmNtextString`** 

Specifies the text in the text edit selection field.

### Inherited Resources


SelectionBox inherits behavior and resources from the
superclasses in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmBulletinBoard Resource Set``Name``Class``Type``Default``Access`XmNallowOverlapXmCAllowOverlapBooleanTrueCSGXmNautoUnmanageXmCAutoUnmanageBooleanTrueCGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNcancelButtonXmCWidgetWidgetdynamicSGXmNdefaultButtonXmCWidgetWidgetdynamicSGXmNdefaultPositionXmCDefaultPositionBooleanTrueCSGXmNdialogStyleXmCDialogStyleunsigned chardynamicCSGXmNdialogTitleXmCDialogTitleXmStringNULLCSGXmNfocusCallbackXmCCallbackXtCallbackListNULLCXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTableXmRenderTabledynamicCSGXmNmapCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension10CSGXmNmarginWidthXmCMarginWidthDimension10CSGXmNnoResizeXmCNoResizeBooleanFalseCSGXmNresizePolicyXmCResizePolicyunsigned charXmRESIZE_ANYCSGXmNshadowTypeXmCShadowTypeunsigned charXmSHADOW_OUTCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNtextTranslationsXmCTranslationsXtTranslationsNULLCXmNunmapCallbackXmCCallbackXtCallbackListNULLC

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetdynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicN/AXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
        XmString`value`;
        int`length`;
} XmSelectionBoxCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`value`** 

Indicates theXmStringvalue selected by the user from the
SelectionBox list or entered into the SelectionBox text field
* **`length`** 

Indicates the size in bytes of theXmStringvalue
This member is obsolete and exists for compatibility with
earlier releases.

### Translations


`XmSelectionBox`inherits translations from`XmBulletinBoard`.
### Accelerators


The`XmNtextAccelerators`are added to the Text descendant of`XmSelectionBox`.
The default accelerators are described in the following list.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **`:`KeyosfUp:** 

SelectionBoxUpOrDown(`Previous`)
* **`:`KeyosfDown:** 

SelectionBoxUpOrDown(`Next`)
* **`:`KeyosfBeginLine:** 

SelectionBoxUpOrDown(`First`)
* **`:`KeyosfEndLine:** 

SelectionBoxUpOrDown(`Last`)
* **`:`KeyosfRestore:** 

SelectionBoxRestore()
* **`s c &ap;m &ap;a`Key`space`:** 

SelectionBoxRestore()

### Action Routines


The XmSelectionBox action routines are

* **SelectionBoxUpOrDown(`Previous|Next|First|Last`):** 

When called with an argument of`Previous`, or 0 (zero) for
compatibility, selects the previous
item in the
list and replaces the text with that item.

When called with an argument of`Next`, or 1 for
compatibility, selects the next item in the
list and replaces the text with that item.

When called with an argument of`First`, or 2 for
compatibility, selects the first item in the
list and replaces the text with that item.

When called with an argument of`Last`, or 3 for
compatibility, selects the last item in the
list and replaces the text with that item.
* **SelectionBoxRestore():** 

Replaces the text value with the list selection.
If no item in the list is selected, clears the text.

### Additional Behavior


The SelectionBox widget has the following additional behavior:

* **KeyosfCancel:** 

Calls the activate callbacks for the cancel button if it is sensitive.
If no cancel button exists and the parent of the SelectionBox is a manager,
passes the event to the parent.
* **KeyosfActivate:** 

Calls the activate callbacks for the button with the keyboard focus.
If no button has the keyboard focus, calls the activate callbacks
for the default button if it is sensitive.
In a List widget or single-line Text widget,
the List or Text action associated withKeyosfActivateis called before the SelectionBox actions associated withKeyosfActivate.
In a multiline Text widget, anyKeyosfActivateevent exceptKeyosfEntercalls
the Text action associated withKeyosfActivate,
then the SelectionBox actions associated withKeyosfActivate.
If no button has the focus, no default button exists, and the parent
of the SelectionBox is a manager, passes the event to the parent.
* **OK&ensp;Button&ensp;Activated:** 

If`XmNmustMatch`is True and the text does not match an item in the
file list, calls the`XmNnoMatchCallback`callbacks with reason`XmCR_NO_MATCH`.
Otherwise, calls the`XmNokCallback`callbacks with reason`XmCR_OK`.
* **Apply&ensp;Button&ensp;Activated:** 

Calls the`XmNapplyCallback`callbacks with reason`XmCR_APPLY`.
* **Cancel&ensp;Button&ensp;Activated:** 

Calls the`XmNcancelCallback`callbacks with reason`XmCR_CANCEL`.
* **Help&ensp;Button&ensp;Activated:** 

Calls the`XmNhelpCallback`callbacks with reason`XmCR_HELP`.
* **MapWindow:** 

Calls the callbacks for`XmNmapCallback`if the SelectionBox
is a child of a Dialog shell.
* **UnmapWindow:** 

Calls the callbacks for`XmNunmapCallback`if the SelectionBox is
the child of a DialogShell.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmBulletinBoard;,
&cdeman.XmCreateSelectionBox;,
&cdeman.XmCreateSelectionDialog;,
&cdeman.XmCreatePromptDialog;,
&cdeman.XmManager;, and
&cdeman.XmSelectionBoxGetChild;.