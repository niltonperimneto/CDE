# XmCommand
library call`XmCommand`The Command widget classXmCommandwidget classCommand&npzwc;#include &lt;Xm/Command.h>
## DESCRIPTION


Command is a special-purpose composite widget for command
entry that provides a built-in command-history mechanism.
Command includes a command-line text-input field, a command-line prompt,
and a command-history list region.

One additional`WorkArea`child may be added to the Command after
creation.

Whenever a command is entered, it is
automatically added to the end of the command-history list and made visible.
This does not change the selected item in the list, if there is one.

Many of the new resources specified for Command are actually SelectionBox
resources that have been renamed for clarity and ease of use.
### Descendants


Command automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`ItemsList``XmList`command-history list region`ItemsListSW``XmScrolledWindow`the ScrolledWindow parent of`ItemsList``Selection``XmLabelGadget`command-line prompt`Text``XmTextField`command-line text-input field
### Classes


Command inherits behavior, resources, and traits from`Core`,`Composite`,`Constraint`,`XmManager`,`XmBulletinBoard`, and`XmSelectionBox`.

The class pointer is`xmCommandWidgetClass`.

The class name is`XmCommand`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters.
To specify one of the defined
values for a resource in a.Xdefaultsfile,
remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`XmCommand Resource Set``Name``Class``Type``Default``Access`XmNcommandXmCTextStringXmString""CSGXmNcommandChangedCallbackXmCCallbackXtCallbackListNULLCXmNcommandEnteredCallbackXmCCallbackXtCallbackListNULLCXmNhistoryItemsXmCItemsXmStringTableNULLCSGXmNhistoryItemCountXmCItemCountint0CSGXmNhistoryMaxItemsXmCMaxItemsint100CSGXmNhistoryVisibleItemCountXmCVisibleItemCountintdynamicCSGXmNpromptStringXmCPromptStringXmStringdynamicCSG

* **`XmNcommand`** 

Contains the current command-line text. This is the`XmNtextString`resource in SelectionBox, renamed for Command.
This resource can also be modified with`XmCommandSetValue`and`XmCommandAppendValue`functions.
The command area is a Text widget.
* **`XmNcommandChangedCallback`** 

Specifies the list of callbacks that is called after
each time the value of the command changes.
The callback reason is`XmCR_COMMAND_CHANGED`.
This is equivalent to the`XmNvalueChangedCallback`of the Text
widget, except that a pointer to an`XmCommandCallbackStructure`is
passed, and the structure's`value`member contains theXmString.
* **`XmNcommandEnteredCallback`** 

Specifies the list of callbacks that is called
when a command is entered in the Command.
The callback reason is`XmCR_COMMAND_ENTERED`. A pointer to an`XmCommandCallback`structure is passed.
* **`XmNhistoryItems`** 

ListsXmStringitems that make up the contents
of the history list. This is the`XmNlistItems`resource in
SelectionBox, renamed for Command.`XtGetValues`for this resource returns the list items themselves,
not a copy of the list items.
The application must not free the returned items.
* **`XmNhistoryItemCount`** 

Specifies the number of`XmStrings`in`XmNhistoryItems`. This is
the`XmNlistItemCount`resource in SelectionBox, renamed for Command.
The value must not be negative.
* **`XmNhistoryMaxItems`** 

Specifies the maximum number of items allowed in the history list. Once
this number is reached, an existing list item must be removed before
a new item can be added to the list. For each command entered, the first list
item is removed from the list, so the new command can be added to the list.
The value must be greater than 0 (zero).
* **`XmNhistoryVisibleItemCount`** 

Specifies the number of items in the history list that should be visible at
one time. In effect, it sets the height (in lines) of the history list
window. This is the`XmNlistVisibleItemCount`resource in SelectionBox,
renamed for Command.
The value must be greater than 0 (zero).
The default is dynamic based on the height of the list.
* **`XmNpromptString`** 

Specifies a prompt for the command line.
This is the`XmNselectionLabelString`resource in SelectionBox, renamed for Command.
The default may vary depending on the value of the`XmNlayoutDirection`resource and the locale.
In the C locale the default is > (right angle bracket).Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.

### Inherited Resources


Command inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmSelectionBox Resource Set``Name``Class``Type``Default``Access`XmNapplyCallbackXmCCallbackXtCallbackListNULLN/AXmNapplyLabelStringXmCApplyLabelStringXmStringdynamicN/AXmNcancelCallbackXmCCallbackXtCallbackListNULLN/AXmNcancelLabelStringXmCCancelLabelStringXmStringdynamicN/AXmNchildPlacementXmCChildPlacementunsigned charXmPLACE_ABOVE_SELECTIONCSGXmNdialogTypeXmCDialogTypeunsigned charXmDIALOG_COMMANDGXmNhelpLabelStringXmCHelpLabelStringXmStringdynamicN/AXmNlistItemCountXmCItemCountint0CSGXmNlistItemsXmCItemsXmStringTableNULLCSGXmNlistLabelStringXmCListLabelStringXmStringNULLN/AXmNlistVisibleItemCountXmCVisibleItemCountintdynamicCSGXmNminimizeButtonsXmCMinimizeButtonsBooleanFalseN/AXmNmustMatchXmCMustMatchBooleanFalseN/AXmNnoMatchCallbackXmCCallbackXtCallbackListNULLN/AXmNokCallbackXmCCallbackXtCallbackListNULLN/AXmNokLabelStringXmCOkLabelStringXmStringdynamicN/AXmNselectionLabelStringXmCSelectionLabelStringXmStringdynamicCSGXmNtextAcceleratorsXmCTextAcceleratorsXtAcceleratorsdefaultCXmNtextColumnsXmCColumnsshortdynamicCSGXmNtextStringXmCTextStringXmString""CSG

`XmBulletinBoard Resource Set``Name``Class``Type``Default``Access`XmNallowOverlapXmCAllowOverlapBooleanTrueCSGXmNautoUnmanageXmCAutoUnmanageBooleanFalseN/AXmNbuttonFontListXmCButtonFontListXmFontListdynamicN/AXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNcancelButtonXmCWidgetWidgetNULLN/AXmNdefaultButtonXmCWidgetWidgetNULLN/AXmNdefaultPositionXmCDefaultPositionBooleanFalseCSGXmNdialogStyleXmCDialogStyleunsigned chardynamicCSGXmNdialogTitleXmCDialogTitleXmStringNULLCSGXmNfocusCallbackXmCCallbackXtCallbackListNULLCXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTableXmRenderTabledynamicCSGXmNmapCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension10CSGXmNmarginWidthXmCMarginWidthDimension10CSGXmNnoResizeXmCNoResizeBooleanFalseCSGXmNresizePolicyXmCResizePolicyunsigned charXmRESIZE_NONECSGXmNshadowTypeXmCShadowTypeunsigned charXmSHADOW_OUTCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNtextTranslationsXmCTranslationsXtTranslationsNULLCXmNunmapCallbackXmCCallbackXtCallbackListNULLC

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
} XmCommandCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback
* **`value`** 

Specifies theXmStringin the CommandArea
* **`length`** 

Specifies the size in bytes of theXmStringvalue.
This member is obsolete and exists for compatibility with
earlier releases.

### Translations


`XmCommand`inherits translations from`XmSelectionBox`.
### Accelerators


The`XmNtextAccelerators`from`XmSelectionBox`are added to the Text
descendant of`XmCommand`.
### Action Routines


The`XmCommand`action routines are:

* **SelectionBoxUpOrDown(`Previous|Next|First|Last`):** 

When called with an argument of`Previous`, or 0 (zero) for
compatibility,
selects the previous item in the history
list and replaces the text with that item.

When called with an argument of`Next`, or 1 for
compatibility, selects the next item in the history
list and replaces the text with that item.

When called with an argument of`First`, or 2 for
compatibility, selects the first item in the history
list and replaces the text with that item.

When called with an argument of`Last`, or 3 for
compatibility, selects the last item in the history
list and replaces the text with that item.

Calls the callbacks for`XmNcommandChangedCallback`.

### Additional Behavior


The Command widget has the following additional behavior:

* **KeyosfCancel:** 

If the parent of the Command is a manager, the event is passed to the parent.
* **KeyosfActivate&ensp;in&ensp;Text:** 

Calls the Text widget's`XmNactivateCallback`callbacks.
If the text is empty, this action then returns.
Otherwise, if the history list has`XmNhistoryMaxItems`items, it
removes the first item in
the list.
It adds the text to the history list as the last item, clears the text,
and calls the`XmNcommandEnteredCallback`callbacks.
* **Key&ensp;in&ensp;Text:** 

When any change is made to the text edit widget, this action
calls the callbacks for`XmNcommandChangedCallback`.
* **BtnDown`(2+)`&ensp;or&ensp;KeyosfActivate&ensp;in&ensp;List:** 

Calls the List widget's`XmNdefaultActionCallback`callbacks.
If the history list has`XmNhistoryMaxItems`items, this action
removes the first item in the list.
It adds the selected List item to the history list as the last item,
clears the text, and calls the`XmNcommandEnteredCallback`callbacks.
* **FocusIn:** 

Calls the callbacks for`XmNfocusCallback`.
* **MapWindow:** 

When a Command that is the child of a DialogShell is mapped, this action
calls the callbacks for`XmNmapCallback`.
* **UnmapWindow:** 

When a Command that is the child of a DialogShell is unmapped, this
action calls the
callbacks for`XmNunmapCallback`.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;, &cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmBulletinBoard;,
&cdeman.XmCommandAppendValue;,
&cdeman.XmCommandError;,
&cdeman.XmCommandGetChild;,
&cdeman.XmCommandSetValue;,
&cdeman.XmCreateCommand;,
&cdeman.XmManager;, and
&cdeman.XmSelectionBox;.