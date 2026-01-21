# XmMessageBox
library call`XmMessageBox`The MessageBox widget classXmMessageBoxwidget classMessageBox&npzwc;#include &lt;Xm/MessageB.h>
## DESCRIPTION


MessageBox is a dialog class used for creating simple message dialogs.
Convenience dialogs based on MessageBox are provided for several common
interaction tasks, which include giving information, asking questions, and
reporting errors.

A MessageBox dialog is typically transient in nature, displayed for the
duration of a single interaction.
MessageBox is a subclass of BulletinBoard and
depends on it for much of its general dialog behavior.

The default value for`XmNinitialFocus`is the value of`XmNdefaultButton`.

A typical MessageBox contains a message symbol, a message, and up to
three standard default PushButtons:`OK, Cancel`, and`Help`.
It is laid out with the symbol and message on top and the
PushButtons on the bottom. The`Help`button is positioned to the side
of the other push buttons.
You can localize the default symbols and button labels for MessageBox
convenience dialogs.

The user can specify resources in a resource file for the gadgets
created automatically that contain the MessageBox symbol pixmap
and separator. The gadget names are`Symbol`and`Separator`.

A MessageBox can also be customized by creating and managing new
children that are added to the MessageBox children created
automatically by the convenience dialogs.
In the case of
TemplateDialog, only the separator child is created by default.
If the callback, string, or pixmap symbol resources are specified,
the appropriate child will be created.

Additional children are laid out in the following manner:

The first MenuBar child is placed at the top of the window.
The`XmQTmenuSystem`trait is used to check that it is the first
MenuBar child.

All widgets or gadgets
are placed after the`OK`button in the order of their creation
(this order is checked using the`XmQTactivatable`trait).

A child that is not in the above categories is placed above
the row of buttons. If a message label exists, the child is placed below
the label. If a message pixmap exists, but a message label is absent, the
child is placed on the same row as the pixmap. The child behaves as a
work area and grows or shrinks to fill the space above the
row of buttons. The layout of multiple work area children is
undefined.

At initialization, MessageBox looks for the following bitmap files:

`xm_error`

`xm_information`

`xm_question`

`xm_working`

`xm_warning`

See &cdeman.XmGetPixmap; for a list of the paths that are searched for
these files.

MessageBox uses the`XmQTactivatable`and`XmQTmenuSystem`traits.
### Descendants


MessageBox automatically creates the descendants shown in the
following table.
An application can use`XtNameToWidget`to gain access
to the named descendant. In addition, a user or an application
can use the named descendant when specifying resource values.`Named Descendant``Class``Identity`=`Cancel``XmPushButtonGadget`Cancel button`Help``XmPushButtonGadget`Help button`Message``XmLabelGadget`displayed message`OK``XmPushButtonGadget`OK button`Separator``XmSeparatorGadget`dividing line between message and buttons`Symbol``XmLabelGadget`icon symbolizing message type
### Classes


MessageBox inherits behavior, resources, and traits from`Core`,`Composite`,`Constraint`,`XmManager`, and`XmBulletinBoard`.

The class pointer is`xmMessageBoxWidgetClass`.

The class name is`XmMessageBox`.
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

`XmMessageBox Resource Set``Name``Class``Type``Default``Access`XmNcancelCallbackXmCCallbackXtCallbackListNULLCXmNcancelLabelStringXmCCancelLabelStringXmStringdynamicCSGXmNdefaultButtonTypeXmCDefaultButtonTypeunsigned charXmDIALOG_OK_BUTTONCSGXmNdialogTypeXmCDialogTypeunsigned charXmDIALOG_MESSAGECSGXmNhelpLabelStringXmCHelpLabelStringXmStringdynamicCSGXmNmessageAlignmentXmCAlignmentunsigned charXmALIGNMENT_BEGINNINGCSGXmNmessageStringXmCMessageStringXmString""CSGXmNminimizeButtonsXmCMinimizeButtonsBooleanFalseCSGXmNokCallbackXmCCallbackXtCallbackListNULLCXmNokLabelStringXmCOkLabelStringXmStringdynamicCSGXmNsymbolPixmapXmCPixmapPixmapdynamicCSG

* **`XmNcancelCallback`** 

Specifies the list of callbacks that is called when
the user clicks on the cancel button.
The reason sent by the callback is`XmCR_CANCEL`.
* **`XmNcancelLabelString`** 

Specifies the string label for the cancel button.
The default for this resource depends on the locale.
In the C locale the default is`Cancel`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNdefaultButtonType`** 

Specifies the default PushButton.
A value of`XmDIALOG_NONE`means that there should be no default
PushButton.
The following types are valid:

`XmDIALOG_CANCEL_BUTTON`

`XmDIALOG_OK_BUTTON`

`XmDIALOG_HELP_BUTTON`

`XmDIALOG_NONE`
* **`XmNdialogType`** 

Specifies the type of MessageBox dialog, which determines
the default message symbol.
The following are the possible values for this resource:

* **`XmDIALOG_ERROR`** 

Indicates an ErrorDialog.
* **`XmDIALOG_INFORMATION`** 

Indicates an InformationDialog.
* **`XmDIALOG_MESSAGE`** 

Indicates a MessageDialog.
This is the default MessageBox dialog type.
It does not have an associated message symbol.
* **`XmDIALOG_QUESTION`** 

Indicates a QuestionDialog.
* **`XmDIALOG_TEMPLATE`** 

Indicates a TemplateDialog.
The TemplateDialog contains only a separator child. It does not
have an associated message symbol.
* **`XmDIALOG_WARNING`** 

Indicates a WarningDialog.
* **`XmDIALOG_WORKING`** 

Indicates a WorkingDialog.


If this resource is changed with`XtSetValues`, the symbol bitmap is
modified to the new`XmNdialogType`bitmap unless`XmNsymbolPixmap`is also being set in the call to`XtSetValues`.
If the dialog type does not have an associated message symbol, then no
bitmap will be displayed.
* **`XmNhelpLabelString`** 

Specifies the string label for the help button.
The default for this resource depends on the locale.
In the C locale the default is`Help`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNmessageAlignment`** 

Controls the alignment of the message Label.
Possible values include the following:

`XmALIGNMENT_BEGINNING`(default)

`XmALIGNMENT_CENTER`

`XmALIGNMENT_END`

See the description of`XmNalignment`in the`XmLabel`reference page for an explanation of these values.
* **`XmNmessageString`** 

Specifies the string to be used as the message.
* **`XmNminimizeButtons`** 

Sets the buttons to the width of the widest button and height of the
tallest button if False. If this resource is True,
button width and height are
set to the preferred size of each button.
* **`XmNokCallback`** 

Specifies the list of callbacks that is called when
the user clicks on the OK button.
The reason sent by the callback is`XmCR_OK`.
* **`XmNokLabelString`** 

Specifies the string label for the OK button.
The default for this resource depends on the locale.
In the C locale the default is`OK`.Now that some default localized label strings are provided through
message catalogs for the children of composite widgets, thelabelStringresources
cannot be set on the child through default resource files.
Instead, the resource provided at the parent level must be used.
* **`XmNsymbolPixmap`** 

Specifies the pixmap label to be used as the message symbol.

### Inherited Resources


MessageBox inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`XmBulletinBoard Resource Set``Name``Class``Type``Default``Access`XmNallowOverlapXmCAllowOverlapBooleanTrueCSGXmNautoUnmanageXmCAutoUnmanageBooleanTrueCGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNcancelButtonXmCWidgetWidgetCancel buttonSGXmNdefaultButtonXmCWidgetWidgetdynamicSGXmNdefaultPositionXmCDefaultPositionBooleanTrueCSGXmNdialogStyleXmCDialogStyleunsigned chardynamicCSGXmNdialogTitleXmCDialogTitleXmStringNULLCSGXmNfocusCallbackXmCCallbackXtCallbackListNULLCXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTableXmRenderTabledynamicCSGXmNmapCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension10CSGXmNmarginWidthXmCMarginWidthDimension10CSGXmNnoResizeXmCNoResizeBooleanFalseCSGXmNresizePolicyXmCResizePolicyunsigned charXmRESIZE_ANYCSGXmNshadowTypeXmCShadowTypeunsigned charXmSHADOW_OUTCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNtextTranslationsXmCTranslationsXtTranslationsNULLCXmNunmapCallbackXmCCallbackXtCallbackListNULLC

`XmManager Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNinitialFocusXmCInitialFocusWidgetdynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimensiondynamicCSGXmNstringDirectionXmCStringDirectionXmStringDirectiondynamicCGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicN/AXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent *`event`;
} XmAnyCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback

### Translations


`XmMessageBox`includes the translations from`XmManager`.
### Additional Behavior


The`XmMessageBox`widget has the following additional behavior:

* **KeyosfCancel:** 

Calls the activate callbacks for the cancel button if it is sensitive.
* **KeyosfActivate:** 

Calls the activate callbacks for the button with the keyboard focus.
If no button has the keyboard focus, calls the activate callbacks
for the default button if it is sensitive.
* **Ok&ensp;Button&ensp;Activated:** 

Calls the callbacks for`XmNokCallback`.
* **Cancel&ensp;Button&ensp;Activated:** 

Calls the callbacks for`XmNcancelCallback`.
* **Help&ensp;Button&ensp;Activated:** 

Calls the callbacks for`XmNhelpCallback`.
* **FocusIn:** 

Calls the callbacks for`XmNfocusCallback`.
* **Map:** 

Calls the callbacks for`XmNmapCallback`if the parent is a
DialogShell.
* **Unmap:** 

Calls the callbacks for`XmNunmapCallback`if the parent is a
DialogShell.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;,
&cdeman.Constraint;,
&cdeman.Core;,
&cdeman.XmBulletinBoard;,
&cdeman.XmCreateErrorDialog;,
&cdeman.XmCreateInformationDialog;,
&cdeman.XmCreateMessageBox;,
&cdeman.XmCreateMessageDialog;,
&cdeman.XmCreateQuestionDialog;,
&cdeman.XmCreateTemplateDialog;,
&cdeman.XmCreateWarningDialog;,
&cdeman.XmCreateWorkingDialog;,
&cdeman.XmManager;, and
&cdeman.XmMessageBoxGetChild;.