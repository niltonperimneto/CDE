# XmTextField
library call`XmTextField`The TextField class#include <Xm/Xm.h>
## DESCRIPTION


The TextField widget provides a single line text editor for customizing
both user and programmatic interfaces. It is used for single-line
string entry, and forms entry with verification procedures.
It provides an application with a consistent editing system for
textual data.

TextField provides separate callback lists to verify
movement of the insert cursor, modification of the text, and
changes in input focus. Each of
these callbacks provides the verification function with the
widget instance, the event that caused the callback, and a
data structure specific to the verification type. From this
information, the function can verify if the application considers
this to be a legitimate state change and can signal the widget
whether to continue with the action.

The user interface tailors a new set of actions.
The key bindings have been added for insert cursor movement, deletion,
insertion, and selection of text.

TextField allows the user to select regions of text.
Selection is based on the model specified in theInter-Client
Communication Conventions Manual(ICCCM).
TextField supports primary and secondary selection.

TextField uses the`XmQTnavigator`,`XmQTspecifyRenderTable`, and`XmQTscrollFrame`traits,
and holds the`XmQTaccessTextual``XmQTtransfer`traits.

If an application or widget calls the`setValue`trait method
of`XmQTaccessTextual`, then`XmTextField`will call`XmTextFieldSetString`to set the string value.
### Data Transfer Behavior


TextField supports transfer of the primary, secondary, and clipboard
selections and dragging of selected text from the widget.
TextField can also be the destination for the primary, secondary, and
clipboard selections, and it supports dropping of data being dragged
onto the widget.

When the`XmNconvertCallback`procedures are called, the`location_data`member of theXmConvertCallbackStructmember
is NULL if the selected text is being transferred.
If the entire text, not the selected text, is being transferred, the
value of this member is the widget ID of the TextField widget.

As a source of data, TextField supports the following targets and
associated conversions of data to these targets:

* **`locale`** 

If the`locale`target matches the widget's locale, the widget
transfers the selected text in the encoding of the locale.
* **`COMPOUND_TEXT`** 

The widget transfers the selected text as type`COMPOUND_TEXT`.
* **`STRING`** 

The widget transfers the selected text as type`STRING`.
* **`TEXT`** 

If the selected text is fully convertible to the encoding of the locale,
the widget transfers the selected text in the encoding of the locale.
Otherwise, the widget transfers the selected text as type`COMPOUND_TEXT`.
* **`DELETE`** 

The widget deletes the selected text.
* **_MOTIF_CLIPBOARD_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets to which
the widget can convert data to be placed on the clipboard immediately.
These include the following targets:

`COMPOUND_TEXT`

The encoding of the locale, if the selected text is fully convertible to
the encoding of the locale

`STRING`, if the selected text is fully convertible to`STRING`
* **_MOTIF_EXPORT_TARGETS** 

The widget transfers, as type`ATOM`, a list of the targets to be
used as the value of the DragContext's`XmNexportTargets`in a
drag-and-drop transfer.
These include`COMPOUND_TEXT`, the encoding of the locale,`STRING`,`TEXT`,`BACKGROUND`, and`FOREGROUND`.
* **_MOTIF_LOSE_SELECTION** 

The widget takes the following actions:

When losing the`PRIMARY`selection, it unhighlights the selected
text and calls the`XmNlosePrimaryCallback`procedures.

When losing the`SECONDARY`selection, it removes the secondary
selection highlight.

When losing the_MOTIF_DESTINATIONselection, if the widget does
not have focus it changes the cursor to indicate that the widget is no
longer the destination.


As a source of data, TextField also supports the following standard
Motif targets:

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
These also include`COMPOUND_TEXT`, the encoding of the locale,`STRING`, and`TEXT`.
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


As a destination for data, TextField chooses a target and requests
conversion of the selection to that target.
If the encoding of the locale is present in the list of available
targets, TextField chooses a requested target from the available targets
in the following order of preference:

`TEXT`

`COMPOUND_TEXT`

The encoding of the locale

`STRING`

If the encoding of the locale is not present in the list of available
targets, TextField chooses a requested target from the available targets
in the following order of preference:

`COMPOUND_TEXT`

`STRING`
### Classes


TextField widget inherits behavior, resources, and traits from`Core`andPrimitive.

The class pointer is`xmTextFieldWidgetClass`.

The class name is`XmTextField`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lower case or upper case, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G),
or is not applicable (N/A).

`XmTextFieldResource Set``Name``Class``Type``Default``Access`XmNactivateCallbackXmCCallbackXtCallbackListNULLCXmNblinkRateXmCBlinkRateint500CSGXmNcolumnsXmCColumnsshortdynamicCSGXmNcursorPositionXmCCursorPositionXmTextPosition0CSGXmNcursorPositionVisibleXmCCursorPositionVisibleBooleandynamicCSGXmNdestinationCallbackXmCCallbackXtCallbackListNULLCXmNeditableXmCEditableBooleanTrueCSGXmNfocusCallbackXmCCallbackXtCallbackListNULLCXmNfontListXmCFontListXmFontListdynamicCSGXmNgainPrimaryCallbackXmCCallbackXtCallbackListNULLCXmNlosePrimaryCallbackXmCCallbackXtCallbackListNULLCXmNlosingFocusCallbackXmCCallbackXtCallbackListNULLCXmNmarginHeightXmCMarginHeightDimension5CSGXmNmarginWidthXmCMarginWidthDimension5CSGXmNmaxLengthXmCMaxLengthintlargest integerCSGXmNmodifyVerifyCallbackXmCCallbackXtCallbackListNULLCXmNmodifyVerifyCallbackWcsXmCCallbackXtCallbackListNULLCXmNmotionVerifyCallbackXmCCallbackXtCallbackListNULLCXmNpendingDeleteXmCPendingDeleteBooleanTrueCSGXmNrenderTableXmCRenderTableXmRenderTabledynamicCSGXmNresizeWidthXmCResizeWidthBooleanFalseCSGXmNselectionArrayXmCSelectionArrayXtPointerdefault arrayCSGXmNselectionArrayCountXmCSelectionArrayCountint3CSGXmNselectThresholdXmCSelectThresholdint5CSGXmNvalueXmCValueString""CSGXmNvalueChangedCallbackXmCCallbackXtCallbackListNULLCXmNvalueWcsXmCValueWcswchar_t *(wchar_t *)""CSG1XmNverifyBellXmCVerifyBellBooleandynamicCSG

1This resource cannot be specified in a resource file.

* **`XmNactivateCallback`** 

Specifies the list of callbacks that is called when the user invokes an
event that calls theactivate()action.
The type of the structure whose address is passed to this callback isXmAnyCallbackStruct.
The reason sent by the callback is`XmCR_ACTIVATE`.
* **`XmNblinkRate`** 

Specifies the blink rate of the text cursor in milliseconds.
The time indicated in the blink rate relates to the
length of time the cursor is visible and the time the
cursor is invisible (that is, the time it will take to blink
the insertion cursor on and off will be two times the blink
rate). The cursor will not blink when the blink rate
is set to 0 (zero).
The value must not be negative.
* **`XmNcolumns`** 

Specifies the initial width of the text window as an integer number of
characters. The width equals the number of characters specified by
this resource multiplied by the width as derived from the specified
font. If the em-space
value is available,
it is used. If not, the width of the numeral "0" is used. If this is
not available, the maximum width is used.
For proportionate fonts, the actual number of characters that fit
on a given line may be greater than the value specified.
* **`XmNcursorPosition`** 

Indicates the position in the text where the current insert cursor is to
be located. Position is determined by the number of characters from
the beginning of the text.
* **`XmNcursorPositionVisible`** 

If the text widget has anXmPrintShellas one of its ancestors
(that is, the widget was created on a print server connection)
then the default value isFalse; otherwise, it isTrue.
* **`XmNdestinationCallback`** 

Specifies a list of callbacks called when the widget is the destination
of a transfer operation.
The type of the structure whose address is passed to these callbacks isXmDestinationCallbackStruct.
The reason is`XmCR_OK`.
* **`XmNeditable`** 

When set to True, indicates that the user can edit the text string.
A false value will prohibit the user from editing the text.When`XmNeditable`is used on a widget
it sets the dropsite to`XmDROP_SITE_ACTIVE`.
* **`XmNfocusCallback`** 

Specifies the list of callbacks called when TextField accepts
input focus. The type of the structure whose address is passed to this
callback isXmAnyCallbackStruct. The reason sent by the
callback is`XmCR_FOCUS`.
* **`XmNfontList`** 

Specifies the font list to be used for TextField. The font list is an
obsolete structure, and is retained only for compatibility with
earlier releases of Motif. Use the render table (`XmNrenderTable`)
instead of font lists wherever possible. If both are specified, the
render table will take precedence. If this value is NULL at
initialization, the parent hierarchy of the widget is searched for an
ancestor that holds the`XmQTspecifyRenderTable`trait. If such
an ancestor is found, the font list is initialized to the`XmTEXT_RENDER_TABLE`value of the ancestor widget. If no such
ancestor is found, the default is implementation dependent.

TextField searches the font list for the first occurrence of a font
set that has an`XmFONTLIST_DEFAULT_TAG`. If a default element is
not found, the first font set in the font list is used. If the list
contains no font sets, the first font in the font list will be used.
Refer to &cdeman.XmFontList; for more information on a font list
structure.
* **`XmNgainPrimaryCallback`** 

Specifies the list of callbacks that are called when the user invokes
an event that causes the text widget to gain ownership of the primary
selection. The callback reason for this callback is`XmCR_GAIN_PRIMARY`.
* **`XmNlosePrimaryCallback`** 

Specifies the list of callbacks that are called when the user invokes
an event that cause the text widget to lose ownership of the primary
selection. The callback reason for this callback is`XmCR_LOSE_PRIMARY`.
* **`XmNlosingFocusCallback`** 

Specifies the list of callbacks that are called
before TextField widget loses input focus.
The type of the structure whose address is passed to this callback isXmTextVerifyCallbackStruct.
The reason sent by the callback is`XmCR_LOSING_FOCUS`.
* **`XmNmarginHeight`** 

Specifies the distance between the top edge of the widget
window and the text, and the bottom edge of the widget
window and the text.
* **`XmNmarginWidth`** 

Specifies the distance between the left edge of the widget
window and the text, and the right edge of the widget
window and the text.
* **`XmNmaxLength`** 

Specifies the maximum length of the text string that can be entered into
text from the keyboard.
This value must be nonnegative.
Strings that are entered using the`XmNvalue`resource or the`XmTextFieldSetString`function ignore this resource.
* **`XmNmodifyVerifyCallback`** 

Specifies the list of callbacks that is called
before text is deleted from or inserted into
TextField.
The type of the structure whose address is passed to this callback isXmTextVerifyCallbackStruct.
The reason sent by the callback is`XmCR_MODIFYING_TEXT_VALUE`.
When multiple TextField widgets share the same
source, only the widget that initiates the source change will
generate the`XmNmodifyVerifyCallback`.

If both`XmNmodifyVerifyCallback`and`XmNmodifyVerifyCallbackWcs`are registered callback lists, the procedure(s) in the`XmNmodifyVerifyCallback`list is always executed first; and the
resulting data, which may have been modified, is passed to the`XmNmodifyVerifyCallbackWcs`callback routines.
* **`XmNmodifyVerifyCallbackWcs`** 

Specifies the list of callbacks called before text is deleted from
or inserted into Text. The type of the structure whose address is
passed to this callback isXmTextVerifyCallbackStructWcs. The
reason sent by the callback is`XmCR_MODIFYING_TEXT_VALUE`.
When multiple TextField widgets share the same
source, only the widget that initiates the source change will
generate the`XmNmodifyVerifyCallbackWcs`.

If both`XmNmodifyVerifyCallback`and`XmNmodifyVerifyCallbackWcs`are registered callback lists, the procedure(s) in the`XmNmodifyVerifyCallback`list is always executed first; and the
resulting data, which may have been modified, is passed to the`XmNmodifyVerifyCallbackWcs`callback routines.
* **`XmNmotionVerifyCallback`** 

Specifies the list of callbacks that is called
before the insert cursor is moved to a new position.
The type of the structure whose address is passed to this callback isXmTextVerifyCallbackStruct.
The reason sent by the callback is`XmCR_MOVING_INSERT_CURSOR`.
It is possible for more than one`XmNmotionVerifyCallback`s to be
generated from a single action.
* **`XmNpendingDelete`** 

Indicates that pending delete mode is on when the Boolean is True.
Pending deletion is defined as deletion
of the selected text when an insertion is made.
* **`XmNrenderTable`** 

Specifies the render table to be used in deriving a font set or font
for displaying text. If both a render table and a font list are
specified, the render table will take precedence. If the value of`XmNrenderTable`is NULL at initialization, the parent hierarchy
of the widget is searched for an ancestor that holds the`XmQTspecifyRenderTable`trait. If such an ancestor is found, the
font list is initialized to the`XmTEXT_RENDER_TABLE`value of the
ancestor widget. If no such ancestor is found, the default is
implementation dependent.

TextField searches the render table for the first occurrence of a
rendition that has the tag`_MOTIF_DEFAULT_LOCALE`. If a default
element is not found, the first rendition in the table is used. Refer
to &cdeman.XmRenderTable; for more information on the render table
structure.
* **`XmNresizeWidth`** 

Indicates that the TextField widget will attempt to resize its
width to accommodate all
the text contained in the widget when Boolean is True.
* **`XmNselectionArray`** 

Defines the actions for multiple mouse clicks. Each mouse click
performed within some time of the previous mouse click will increment
the index into this array and perform the defined action for that
index. (This "multi-click" time is specified by the operating
environment, and varies among different systems. In general, it is
usually set to some fraction of a second.)
The possible actions are

* **`XmSELECT_POSITION`** 

Resets the insert cursor position
* **`XmSELECT_WORD`** 

Selects a word
* **`XmSELECT_LINE`** 

Selects a line of text

* **`XmNselectionArrayCount`** 

Specifies the number of actions that are defined in the`XmNselectionArray`resource.
The value must not be negative.
* **`XmNselectThreshold`** 

Specifies the number of pixels of motion that is required to select the
next character when selection is performed using the click-drag
mode of selection.
The value must not be negative.
This resource also specifies whether a drag should be started and the
number of pixels to start a drag when`Btn2Down`and`Btn1Down`are integrated.
* **`XmNvalue`** 

Specifies the string value of the TextField widget as a`char*`data value.
Moves the cursor to position 0 unless a value of`XmNcursorPosition`was explicitly supplied in the argument list.
If`XmNvalue`and`XmNvalueWcs`are both
defined, the value of`XmNvalueWcs`supersedes that of`XmNvalue`.`XtGetValues`returns a copy of the value of
the internal buffer and`XtSetValues`copies the string values
into the internal buffer.
* **`XmNvalueChangedCallback`** 

Specifies the list of callbacks that is called
after text is deleted from or inserted into
TextField.
The type of the structure whose address is passed to this callback isXmAnyCallbackStruct.
The reason sent by the callback is`XmCR_VALUE_CHANGED`.
The`XmNvalueChangedCallback`should occur only in pairs
with a`XmNmodifyVerifyCallback`, assuming that the`doit`flag in the callback structure of the`XmNmodifyVerifyCallback`is
not set to False.
* **`XmNvalueWcs`** 

Specifies the string value of the TextField widget as a`wchar_t*`data
value.
Moves the cursor to position 0 unless a value of`XmNcursorPosition`was explicitly supplied in the argument list.

This resource cannot be specified in a resource file.

If`XmNvalue`and`XmNvalueWcs`are both defined,
the value of`XmNvalueWcs`supersedes that of`XmNvalue`.`XtGetValues`returns a copy of the value of the internal buffer
encoded as a wide character string.`XtSetValues`copies the
value of the wide character string into the internal buffer.
* **`XmNverifyBell`** 

Specifies whether a bell will sound when an action is reversed
during a verification callback.
The default depends on the value of the ancestor VendorShell's`XmNaudibleWarning`resource.

### Inherited Resources


TextField widget inherits behavior and resources from the
superclasses in the following tables.
For a complete description of these resources, refer to the
reference page for that superclass.

`XmPrimitive Resource Set``Name``Class``Type``Default``Access`XmNbottomShadowColorXmCBottomShadowColorPixeldynamicCSGXmNbottomShadowPixmapXmCBottomShadowPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNconvertCallbackXmCCallbackXtCallbackListNULLCXmNforegroundXmCForegroundPixeldynamicCSGXmNhelpCallbackXmCCallbackXtCallbackListNULLCXmNhighlightColorXmCHighlightColorPixeldynamicCSGXmNhighlightOnEnterXmCHighlightOnEnterBooleanFalseCSGXmNhighlightPixmapXmCHighlightPixmapPixmapdynamicCSGXmNhighlightThicknessXmCHighlightThicknessDimension2CSGXmNlayoutDirectionXmCLayoutDirectionXmDirectiondynamicCGXmNnavigationTypeXmCNavigationTypeXmNavigationTypeXmTAB_GROUPCSGXmNpopupHandlerCallbackXmCCallbackXtCallbackListNULLCXmNshadowThicknessXmCShadowThicknessDimension2CSGXmNtopShadowColorXmCTopShadowColorPixeldynamicCSGXmNtopShadowPixmapXmCTopShadowPixmapPixmapdynamicCSGXmNtraversalOnXmCTraversalOnBooleanTrueCSGXmNunitTypeXmCUnitTypeunsigned chardynamicCSGXmNuserDataXmCUserDataXtPointerNULLCSG

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Callback Information


A pointer to the following structure is passed to each callback:typedef struct
{
        int`reason`;
        XEvent`* event`;
} XmAnyCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked
* **`event`** 

Points to the`XEvent`that triggered the callback


The TextField widget defines a new callback structure
for use with verification callbacks. Note that
not all of the fields are relevant for every
callback reason. The application must first
look at the`reason`field and use only the structure
members that are valid for the particular reason.
The values`startPos`,`endPos`, andtextin the
callback structureXmTextVerifyCallbackStructmay be modified upon
receiving the callback, and these changes will be reflected as the
change made to the source of the TextField widget. (For example, all
keystrokes can be converted to spaces or NULL characters when a
password is entered into a TextField widget.) The application
programmer should not overwrite thetextfield, but should
attach data to that pointer.

A pointer to the following structure is passed to the
callbacks for`XmNlosingFocusCallback`,`XmNmodifyVerifyCallback`,
and`XmNmotionVerifyCallback`.typedef struct
{
        int`reason`;
        XEvent *`event`;
        Boolean`doit`;
        XmTextPosition`currInsert, newInsert`;
        XmTextPosition`startPos, endPos`;
        XmTextBlocktext;
} XmTextVerifyCallbackStruct, *XmTextVerifyPtr;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`the triggered the callback.
It can be NULL. For example, changes made to the Text widget
programmatically do not have an event that can be
passed to the associated callback.
* **`doit`** 

Indicates whether the action that invoked the callback will be performed.
Setting`doit`to False negates the action.
Note that not all actions may be negated. For example,`XmCR_LOSING_FOCUS`callbacks may be beyond the control of the
widget if they are produced by mouse clicks.
* **`currInsert`** 

Indicates the current position of the insert cursor.
* **`newInsert`** 

Indicates the position at which the user attempts to position the insert
cursor.
* **`startPos`** 

Indicates the starting position of the text to modify. If the callback is
not a modify verification callback, this value is the same as`currInsert`.
* **`endPos`** 

Indicates the ending position of the text to modify. If no text is replaced or
deleted, then the value is the same as`startPos`. If the callback is not
a modify verification callback, this value is the same as`currInsert`.
* **text** 

Points to the following structure of typeXmTextBlockRec. This structure holds
the textual information to be inserted.typedef struct
{
        char *`ptr`;
        int`length`;
        XmTextFormat`format`} XmTextBlockRec, *XmTextBlock;

* **`ptr`** 

The text to be inserted.`ptr`points to a temporary
storage space that is reused after the callback is finished. Therefore,
if an application needs to save the text to be inserted, it should copy
the text into its own data space.
* **`length`** 

Specifies the length of the text to be inserted.
* **`format`** 

Specifies the format of the text,
either`XmFMT_8_BIT`or`XmFMT_16_BIT`.



A pointer to the following structure is passed to callbacks
for`XmNmodifyVerifyCallbackWcs`.typedef struct
{
        int`reason`;
        XEvent *`event`;
        Boolean`doit`;
        XmTextPosition`currInsert, newInsert`;
        XmTextPosition`startPos, endPos`;
        XmWcsTextBlocktext;
} XmTextVerifyCallbackStructWcs, *XmTextVerifyPtrWcs;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
It can be NULL. For example, changes made to the Text widget
programmatically do not have an event that can be
passed to the associated callback.
* **`doit`** 

Indicates whether the action that invoked the callback is performed.
Setting`doit`to False negates the action.
Note that not all actions may be negated. For example,`XmCR_LOSING_FOCUS`callbacks may be beyond the control of the
widget if they are produced by mouse clicks.
* **`currInsert`** 

Indicates the current position of the insert cursor.
* **`newInsert`** 

Indicates the position at which the user attempts to position the insert
cursor.
* **`startPos`** 

Indicates the starting position of the text to modify. If the callback is
not a modify verification callback, this value is the same
as`currInsert`.
* **`endPos`** 

Indicates the ending position of the text to modify. If no text is
replaced or
deleted, the value is the same as`startPos`. If the callback is not
a modify verification callback, this value is the same as`currInsert`.
* **text** 

Points to the following structure of typeXmTextBlockRecWcs.
This structure holds
the textual information to be inserted.typedef struct
{
        wchar_t *`wcsptr`;
        int`length`;
} XmTextBlockRecWcs, *XmTextBlockWcs;

* **`wcsptr`** 

Points to the wide character text to be inserted
* **`length`** 

Specifies the number of characters to be inserted



The following table describes the reasons for which the individual
verification callback structure fields are valid. Note that the`event`field will never be valid for`XmCR_MOVING_INSERT_CURSOR`.`Reason``Valid Fields`XmCR_LOSING_FOCUSreason, event, doitXmCR_MODIFYING_TEXT_VALUEreason, event, doit, currInsert, newInsert, startPos, endPos, textXmCR_MOVING_INSERT_CURSORreason, doit, currInsert, newInsert

A pointer to the following callback structure is passed to the`XmNdestinationCallback`procedures:typedef struct
{
        int`reason`;
        XEvent *`event`;
        Atom`selection`;
        XtEnum`operation`;
        int`flags`;
        XtPointer`transfer_id`;
        XtPointer`destination_data`;
        XtPointer`location_data`;
        Time`time`;
} XmDestinationCallbackStruct;

* **`reason`** 

Indicates why the callback was invoked.
* **`event`** 

Points to the`XEvent`that triggered the callback.
It can be NULL.
* **`selection`** 

Indicates the selection for which data transfer is being requested.
Possible values are`CLIPBOARD`,`PRIMARY`,`SECONDARY`, and_MOTIF_DROP.
* **`operation`** 

Indicates the type of transfer operation requested.

When the selection is`PRIMARY`or`SECONDARY`, possible
values are`XmMOVE`,`XmCOPY`, and`XmLINK`.

When the selection is`CLIPBOARD`, possible
values are`XmCOPY`and`XmLINK`.

When the selection is_MOTIF_DROP, possible values are`XmMOVE`,`XmCOPY`,`XmLINK`, and`XmOTHER`.
A value of`XmOTHER`means that the callback procedure must get
further information from theXmDropProcCallbackStructin the`destination_data`member.
* **`flags`** 

Indicates whether or not the destination widget is also the source of
the data to be transferred.
Following are the possible values:

* **`XmCONVERTING_NONE`** 

The destination widget is not the source of the data to be transferred.
* **`XmCONVERTING_SAME`** 

The destination widget is the source of the data to be transferred.

* **`transfer_id`** 

Serves as a unique ID to identify the transfer transaction.
* **`destination_data`** 

Contains information about the destination.
When the selection is_MOTIF_DROP, the callback procedures are
called by the drop site's`XmNdropProc`, and`destination_data`is a pointer to theXmDropProcCallbackStructpassed to the`XmNdropProc`procedure.
When the selection is`SECONDARY`,`destination_data`is an Atom
representing a target recommmended by the selection owner for use in
converting the selection.
Otherwise,`destination_data`is NULL.
* **`location_data`** 

Contains information about the location where data is to be transferred.
The value is always NULL when the selection is`CLIPBOARD`.
If the value is NULL, the data is to be inserted at the widget's cursor
position.
Otherwise, the value is a pointer to an`XPoint`structure
containing the x and y coordinates at the location where the data is to
be transferred. Once`XmTransferDone`procedures start to be called,`location_data`will no longer be stable.
* **`time`** 

Indicates the time when the transfer operation began.

### Translations


The`XmTextField`translations are described in the following list.The actions represent the effective behavior of the associated events,
and they may differ in a right-to-left language environment.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **`&ap;c s &ap;m &ap;a`Btn1Down:** 

extend-start()
* **`c &ap;s &ap;m &ap;a`Btn1Down:** 

move-destination()
* **`&ap;c &ap;s &ap;m &ap;a`Btn1Down:** 

grab-focus()
* **`&ap;c &ap;m &ap;a`Btn1Motion:** 

extend-adjust()
* **`&ap;c &ap;m &ap;a`Btn1Up:** 

extend-end()
* **Btn2Down:** 

process-bdrag()
* **`m &ap;a`Btn2Motion:** 

secondary-adjust()
* **`&ap;m a`Btn2Motion:** 

secondary-adjust()
* **`s c <Btn2Up>`:** 

link-to()
* **`&ap;s`Btn2Up:** 

copy-to()
* **`&ap;c`Btn2Up:** 

move-to()
* **`:m`KeyosfPrimaryPaste:** 

cut-primary()
* **`:a`KeyosfPrimaryPaste:** 

cut-primary()
* **`:`KeyosfPrimaryPaste:** 

copy-primary()
* **`:m`KeyosfCut:** 

cut-primary()
* **`:a`KeyosfCut:** 

cut-primary()
* **`:`KeyosfCut:** 

cut-clipboard()
* **`:`KeyosfPaste:** 

paste-clipboard()
* **`:m`KeyosfCopy:** 

copy-primary()
* **`:a`KeyosfCopy:** 

copy-primary()
* **`:`KeyosfCopy:** 

copy-clipboard()
* **`:s`KeyosfBeginLine:** 

beginning-of-line(`extend`)
* **`:`KeyosfBeginLine:** 

beginning-of-line()
* **`:s`KeyosfEndLine:** 

end-of-line(`extend`)
* **`:`KeyosfEndLine:** 

end-of-line()
* **`:s`KeyosfPageLeft:** 

page-left(`extend`)
* **`:`KeyosfPageLeft:** 

page-left()
* **`:s c`KeyosfPageUp:** 

page-left(`extend`)
* **`:c`KeyosfPageUp:** 

page-left()
* **`:s`KeyosfPageRight:** 

page-right(`extend`)
* **`:`KeyosfPageRight:** 

page-right()
* **`s c`KeyosfPageDown:** 

page-right(`extend`)
* **`:c`KeyosfPageDown:** 

page-right()
* **`:`KeyosfClear:** 

clear-selection()
* **`:`KeyosfBackSpace:** 

delete-previous-character()
* **`:s m`KeyosfDelete:** 

cut-primary()
* **`:s a`KeyosfDelete:** 

cut-primary()
* **`:s`KeyosfDelete:** 

cut-clipboard()
* **`:c`KeyosfDelete:** 

delete-to-end-of-line()
* **`:`KeyosfDelete:** 

delete-next-character()
* **`:c m`KeyosfInsert:** 

copy-primary()
* **`:c a`KeyosfInsert:** 

copy-primary()
* **`:s`KeyosfInsert:** 

paste-clipboard()
* **`:c`KeyosfInsert:** 

copy-clipboard()
* **`:`KeyosfInsert:** 

toggle-overstrike()
* **`:s`KeyosfSelect:** 

key-select()
* **`:`KeyosfSelect:** 

set-anchor()
* **`:`KeyosfSelectAll:** 

select-all()
* **`:`KeyosfDeselectAll:** 

deselect-all()
* **`:`KeyosfActivate:** 

activate()
* **`:`KeyosfAddMode:** 

toggle-add-mode()
* **`:`KeyosfHelp:** 

Help()
* **`:`KeyosfCancel:** 

process-cancel()
* **`:s c`KeyosfLeft:** 

backward-word(`extend`)
* **`:c`KeyosfLeft:** 

backward-word()
* **`:s`KeyosfLeft:** 

key-select(`left`)
* **`:`KeyosfLeft:** 

backward-character()
* **`:s c`KeyosfRight:** 

forward-word(`extend`)
* **`:c`KeyosfRight:** 

forward-word()
* **`:s`KeyosfRight:** 

key-select(`right`)
* **`:`KeyosfRight:** 

forward-character()
* **`:`KeyosfUp:** 

traverse-prev()
* **`:`KeyosfDown:** 

traverse-next()
* **`c &ap;m &ap;a`Key`slash`:** 

select-all()
* **`c &ap;m &ap;a`Key`backslash`:** 

deselect-all()
* **`s &ap;m &ap;a`Key`Tab`:** 

prev-tab-group()
* **`&ap;m &ap;a`Key`Tab`:** 

next-tab-group()
* **`&ap;s &ap;m &ap;a`Key`Return`:** 

activate()
* **`c &ap;s &ap;m &ap;a`Key`space`:** 

set-anchor()
* **`c s &ap;m &ap;a`Key`space`:** 

key-select()
* **`s &ap;c &ap;m &ap;a`Key`space`:** 

self-insert()
* **Key:** 

self-insert()


The TextField button event translations are modified when Display's`XmNenableBtn1Transfer`resource does not have a value of`XmOFF`(in other words, it is either`XmBUTTON2_TRANSFER`or`XmBUTTON2_ADJUST`). This
option allows the
actions for selection and transfer to be integrated onBtn1.
The actions forBtn1that are defined above
still apply when theBtn1event occurs over text that is not
selected. The following actions apply when theBtn1event
occurs over text that is selected:

* **Btn1Down:** 

process-bdrag().
* **ShiftBtn1Down:** 

process-bdrag().
* **Btn1DownShiftBtn1Up:** 

grab-focus(),`extend-end`.
* **ShiftBtn1DownShiftBtn1Up:** 

extend-start(),extend-end().
* **CtrlBtn1DownShiftBtn1Up:** 

move-destination().
* **CtrlBtn1Down:** 

process-bdrag().


When Display's`XmNenableBtn1Transfer`resource has a value of`XmBUTTON2_ADJUST`, the following actions apply:

* **Btn2Down:** 

extend-start().
* **Btn2Motion:** 

extend-adjust().
* **Btn2Up:** 

extend-end().

### Action Routines


The`XmTextField`action routines are

* **activate():** 

Calls the callbacks for`XmNactivateCallback`.
If the parent is a manager, passes the event to the parent.
* **backward-character(`extend`):** 

Moves the insertion cursor one character to the left.This action may have different behavior in a right-to-left language
environment.

If called with an argument of`extend`, moves the insertion cursor as
in the case of no argument and extends the current selection.

Thebackward-character()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
If called with the`extend`argument, thebackward-character()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **backward-word(`extend`):** 

If this action is called with no argument,
moves the insertion cursor to the first non-whitespace character after the
first whitespace character to the left or after the beginning of the line.
If the insertion cursor is already at the beginning of a word,
moves the insertion cursor to the beginning of the previous word.This action may have different behavior in a locale other than the C locale.

If called with an argument of`extend`, moves the insertion cursor as
in the case of no argument and extends the current selection.

Thebackward-word()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
If called with the`extend`argument, thebackward-word()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **beginning-of-line(`extend`):** 

If this action is called with no argument,
moves the insertion cursor to the beginning of the line.

If called with an argument of`extend`, moves the insertion cursor as
in the case of no argument and extends the current selection.

Thebeginning-of-line()action produces calls to the`XmNmotionVerifyCallback`with reason value`XmCR_MOVING_INSERT_CURSOR`.
If called with the`extend`argument, thebeginning-of-line()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **clear-selection():** 

Clears the current selection by replacing each character exceptReturnwith aspacecharacter.

Theclear-selection()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`and the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`.
* **copy-clipboard():** 

If this widget owns the primary selection, this action copies the
selection to the clipboard.
This action calls the`XmNconvertCallback`procedures, possibly
multiple times, for the`CLIPBOARD`selection.
* **copy-primary():** 

Copies the primary selection to just before the insertion cursor.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmCOPY`operation.
It calls the selection owner's`XmNconvertCallback`procedures,
possibly multiple times, for the`PRIMARY`selection.

In addition, thecopy-primary()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
Thecopy-primary()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **copy-to():** 

If a secondary selection exists, this action copies the secondary
selection to the insertion position of the destination component.
If the primary selection is in the destination widget, it will
be deselected. Otherwise, there is no effect on the primary selection.

This action calls the destination's`XmNdestinationCallback`procedures for the`SECONDARY`selection and the`XmCOPY`operation.
The destination's`XmNdestinationCallback`procedures or the
destination component itself invokes the selection owner's`XmNconvertCallback`procedures, possibly multiple times, for the`SECONDARY`selection.

If no secondary selection exists, this action copies the primary
selection to the pointer position.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmCOPY`operation.
It calls the selection owner's`XmNconvertCallback`procedures,
possibly multiple times, for the`PRIMARY`selection.

In addition, thecopy-to()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
If there is no secondary selection, thecopy-to()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **cut-clipboard():** 

If this widget owns the primary selection, this action cuts the
selection to the clipboard.
This action calls the`XmNconvertCallback`procedures, possibly
multiple times, for the`CLIPBOARD`selection.
If the transfer is successful, this action then calls the`XmNconvertCallback`procedures for the`CLIPBOARD`selection
and the`DELETE`target.

In addition, thecut-clipboard()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, and the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`.
* **cut-primary():** 

Cuts the primary selection and pastes it just before the insertion cursor.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmMOVE`operation.
It calls the selection owner's`XmNconvertCallback`procedures,
possibly multiple times, for the`PRIMARY`selection.
If the transfer is successful, this action then calls the selection
owner's`XmNconvertCallback`procedures for the`PRIMARY`selection and the`DELETE`target.

In addition, thecut-primary()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`, the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, and the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`.
* **delete-next-character():** 

In normal mode, if there is a nonnull selection, deletes the selection;
otherwise,
deletes the character following the insertion cursor.
In add mode, if there is a nonnull selection, the cursor is not disjoint
from the selection and`XmNpendingDelete`is set to True,
deletes the selection;
otherwise,
deletes the character following the insertion cursor.
This may impact the selection.

Thedelete-next-character()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, and the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`.
* **delete-next-word():** 

In normal mode, if there is a nonnull selection, deletes the selection; otherwise,
deletes the characters following the insertion cursor to the next space, tab
or end-of-line character.
In add mode, if there is a nonnull selection, the cursor is not disjoint
from the selection and`XmNpendingDelete`is set to True,
deletes the selection; otherwise,
deletes the characters following the insertion cursor to the next space, tab
or end-of-line character.
This may impact the selection.
This action may have different behavior in a locale other than the C locale.

Thedelete-next-word()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, and the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`.
* **delete-previous-character():** 

In normal mode, if there is a nonnull selection, deletes the selection; otherwise,
deletes the character of text immediately preceding the insertion cursor.
In add mode, if there is a nonnull selection, the cursor is not disjoint
from the selection and`XmNpendingDelete`is set to True,
deletes the selection; otherwise,
deletes the character of text immediately preceding the insertion cursor.
This may impact the selection.

Thedelete-previous-character()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **delete-previous-word():** 

In normal mode, if there is a nonnull selection, deletes the selection; otherwise,
deletes the characters preceding the insertion cursor
to the next
space, tab or beginning-of-line character.
In add mode, if there is a nonnull selection, the cursor is not disjoint
from the selection and`XmNpendingDelete`is set to True,
deletes the selection; otherwise,
deletes the characters preceding the insertion cursor to the next space,
tab or beginning-of-line character.
This may impact the selection.
This action may have different behavior in a locale other than the C locale.

Thedelete-previous-word()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **delete-selection():** 

Deletes the current selection.

Thedelete-selection()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **delete-to-end-of-line():** 

In normal mode, if there is a nonnull selection, deletes the selection; otherwise,
deletes the characters following the insertion cursor to the next end of
line character.
In add mode, if there is a nonnull selection, the cursor is not disjoint
from the selection and`XmNpendingDelete`is set to True,
deletes the selection; otherwise,
deletes the characters following the insertion cursor to the next end
of line character.
This may impact the selection.

Thedelete-to-end-of-line()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, and the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`.
* **delete-to-start-of-line():** 

In normal mode, if there is a nonnull selection, deletes the selection; otherwise,
deletes the characters preceding the insertion cursor to the previous
beginning-of-line character.
In add mode, if there is a nonnull selection, the cursor is not disjoint
from the selection and`XmNpendingDelete`is set to True,
deletes the selection; otherwise,
deletes the characters preceding the insertion cursor
to the previous beginning-of-line character.
This may impact the selection.

Thedelete-to-start-of-line()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **deselect-all():** 

Deselects the current selection.

Thedeselect-all()action produces no callbacks.
* **end-of-line(`extend`):** 

If this action is called with no argument,
moves the insertion cursor to the end of the line.If called with an argument of`extend`, moves the insertion cursor as
in the case of no argument and extends the current selection.

Theend-of-line()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
If called with the`extend`argument, theend-of-line()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **extend-adjust():** 

Selects text from the anchor to the pointer position and deselects text
outside that range.

Theextend-adjust()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
Theextend-adjust()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **extend-end():** 

Moves the insertion cursor to the position of the pointer.
Theextend-end()action is used to commit the selection. After
this action has been done,process-cancel()will no longer
cancel the selection.

Theextend-end()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
Theextend-end()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **extend-start():** 

Adjusts the anchor using the balance-beam method.
Selects text from the anchor to the pointer position and deselects text
outside that range.

Theextend-start()action can produce no callbacks,
however, it may produce calls to the`XmNgainPrimaryCallback`and`XmNmotionVerifyCallback`procedures.
See callback description for more information.
* **forward-character(`extend`):** 

Moves the insertion cursor one character to the right.This action may have different behavior in a right-to-left language
environment.

If called with an argument of`extend`, moves the insertion cursor as
in the case of no argument and extends the current selection.

Theforward-character()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
If called with the`extend`argument, theforward-character()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **forward-word(`extend`):** 

If this action is called with no argument, moves the insertion cursor to
the first whitespace character or end-of-line following the next
non-whitespace character.
If the insertion cursor is already at the end of a word,
moves the insertion cursor to the end of the next word.This action may have different behavior in a locale other than the C locale.

If called with an argument of`extend`, moves the insertion cursor as
in the case of no argument and extends the current selection.

Theforward-word()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
If called with the`extend`argument, theforward-word()action may produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **grab-focus():** 

This key binding performs the action defined in the`XmNselectionArray`, depending on the number of multiple mouse
clicks.
The default selection array ordering is one click to move the
insertion cursor to the pointer position, two clicks to select a word,
and three
clicks to select a line of text.
A single click also deselects any selected text and sets the anchor at
the pointer position.
This action may have different behavior in a locale other than the C locale.

Thegrab-focus()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **Help():** 

Calls the callbacks for`XmNhelpCallback`if any exist.
If there are no help
callbacks for this widget, this action calls the help callbacks
for the nearest ancestor that has them.
* **key-select(`right|left`):** 

If called with an argument of`right`, moves the insertion cursor
one character to the right and extends the current selection.
If called with an argument of`left`, moves the insertion cursor
one character to the left and extends the current selection.
If called with no argument, extends the current selection.

Note that after a`key-select`action, the selection will still
begin at the original anchor, and will extend to the position
indicated in the action call. If this new position is on the opposite
side of the selection anchor from the previous selection boundary, the
original selection will be deselected.

Thekey-select()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
Thekey-select()action may also produce calls to the`XmNgainPrimaryCallback`procedures.
See callback description for more information.
* **link-primary():** 

Places a link to the primary selection just before the insertion cursor.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmLINK`operation.
The TextField widget itself performs no transfers; the`XmNdestinationCallback`procedures are responsible for inserting
the link to the primary selection and for taking any related actions.
* **link-to():** 

If a secondary selection exists, this action places a link to the
secondary selection at the insertion position of the destination
component.
This action calls the destination's`XmNdestinationCallback`procedures for the`SECONDARY`selection and the`XmLINK`operation.

If no secondary selection exists, this action places a link to the
primary selection at the pointer position.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmLINK`operation.

The TextField widget itself performs no transfers; the`XmNdestinationCallback`procedures are responsible for inserting
the link to the primary or secondary selection and for taking any
related actions.
* **move-destination():** 

Moves the insertion cursor to the pointer position without changing any
existing current selection.
If there is
a
current selection, sets the widget as the destination widget.
This also moves the widget focus to match the insertion cursor.

Themove-destination()action produces calls to the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **move-to():** 

If a secondary selection exists, this action moves the secondary
selection to the insertion position of the destination component.
If the secondary selection is in the destination widget, and the
secondary selection and the primary selection overlap, the result
is undefined.
This action calls the destination's`XmNdestinationCallback`procedures for the`SECONDARY`selection and the`XmCOPY`operation.
The destination's`XmNdestinationCallback`procedures or the
destination component itself invokes the selection owner's`XmNconvertCallback`procedures, possibly multiple times, for the`SECONDARY`selection.
If the transfer is successful, this action then calls the selection
owner's`XmNconvertCallback`procedures for the`SECONDARY`selection and the`DELETE`target.

If no secondary selection exists, this action moves the primary
selection to the pointer position.
This action calls the`XmNdestinationCallback`procedures for the`PRIMARY`selection and the`XmMOVE`operation.
It calls the selection owner's`XmNconvertCallback`procedures,
possibly multiple times, for the`PRIMARY`selection.
If the transfer is successful, this action then calls the selection
owner's`XmNconvertCallback`procedures for the`PRIMARY`selection and the`DELETE`target.

In addition, themove-to()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`. This action may also produce calls
to the`XmNgainPrimaryCallback`procedures.
* **next-tab-group():** 

Traverses to the next tab group.

Thenext-tab-group()action produces no callbacks, unless it
results in the widget losing focus, in which case, the`XmNlosingFocusCallback`procedures are called with reason value`XmCR_LOSING_FOCUS`.
* **page-left():** 

Scrolls the viewing window left one page of text.

Thepage-left()action produces no callbacks.
* **page-right():** 

Scrolls the viewing window right one page of text.

Thepage-right()action produces no callbacks.
* **paste-clipboard():** 

Pastes the contents of the clipboard before the insertion cursor.
This action calls the`XmNdestinationCallback`procedures for the`CLIPBOARD`selection and the`XmCOPY`operation.

Thepaste-clipboard()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **prev-tab-group():** 

Traverses to the previous tab group.

Theprev-tab-group()action produces no callbacks, unless it
results in the widget losing focus, in which case, the`XmNlosingFocusCallback`procedures are called with reason value`XmCR_LOSING_FOCUS`.
* **process-bdrag()** 

If the pointer is within the selection, this action starts a drag
operation for the selection.
This action sets the`XmNconvertProc`of the DragContext to a
function that calls the`XmNconvertCallback`procedures, possibly
multiple times, for the_MOTIF_DROPselection.

If no selection exists or the pointer is outside the selection, this
action prepares to start a secondary selection at the pointer position.
* **process-cancel():** 

Cancels the currentextend-adjust(),secondary-adjust()orprocess-bdrag()operation and leaves the selection state as it was before the operation;
otherwise, and
if
the parent is a manager, it passes the event to the parent.
* **secondary-adjust():** 

Extends the secondary selection to the pointer position.

Thesecondary-adjust()action produces no callbacks.
* **secondary-start():** 

Marks the beginning of a secondary selection.

Thesecondary-start()action produces no callbacks.
* **select-all():** 

Selects all text.

Theselect-all()action can produce no callbacks,
however, it may produce calls to the`XmNgainPrimaryCallback`and`XmNmotionVerifyCallback`procedures.
See callback description for more information.
* **self-insert():** 

If`XmNpendingDelete`is True and the cursor is not disjoint from the
current selection, deletes the entire selection.
Inserts the character associated with the key pressed
before the insertion cursor.

Theself-insert()action produces calls to the`XmNmodifyVerifyCallback`procedures with reason value`XmCR_MODIFYING_TEXT_VALUE`, the`XmNvalueChangedCallback`procedures with reason value`XmCR_VALUE_CHANGED`, and the`XmNmotionVerifyCallback`procedures with reason value`XmCR_MOVING_INSERT_CURSOR`.
* **set-anchor():** 

Resets the anchor point for extended selections.
Resets the destination of secondary selection actions.

Theset-anchor()action produces no callbacks.
* **toggle-add-mode():** 

Toggles the state of Add Mode.

Thetoggle-add-mode()action produces no callbacks.
* **toggle-overstrike():** 

Toggles the state of the text insertion mode. By default,
characters typed into the TextField widget are inserted before
the position of the insertion cursor. In overstrike
mode, characters entered into the TextField widget replace
the characters that directly follow the insertion cursor.
In overstrike mode, when the end of a line is reached,
characters are appended to the end of the line.

The following traversal actions generate no callbacks unless they
result in the loss of focus by the widget in question, as when`XmNnavigationType`is`XmNONE`. In this case,
they produce calls to the`XmNlosingFocusCallback`procedures,
with reason value`XmCR_FOCUS_MOVED`.
* **traverse-home():** 

Traverses to the first widget in the tab group.
* **traverse-next():** 

Traverses to the next widget in the tab group.
* **traverse-prev():** 

Traverses to the previous widget in the tab group.

### Additional Behavior


This widget has the following additional behavior:

* **FocusIn:** 

Draws the insertion cursor as solid and starts blinking the cursor.
* **FocusOut:** 

Displays the insertion cursor as a stippled I-beam unless it is the destination
widget.

### Virtual Bindings


The bindings for virtual keys are vendor specific.For information about bindings for virtual buttons and keys,
see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Core;,
&cdeman.XmCreateTextField;,
&cdeman.XmFontList;,
&cdeman.XmFontListAppendEntry;,
&cdeman.XmPrimitive;,
&cdeman.XmTextFieldClearSelection;,
&cdeman.XmTextFieldCopy;,
&cdeman.XmTextFieldCopyLink;,
&cdeman.XmTextFieldCut;,
&cdeman.XmTextFieldGetBaseline;,
&cdeman.XmTextFieldGetEditable;,
&cdeman.XmTextFieldGetInsertionPosition;,
&cdeman.XmTextFieldGetLastPosition;,
&cdeman.XmTextFieldGetMaxLength;,
&cdeman.XmTextFieldGetSelection;,
&cdeman.XmTextFieldGetSelectionPosition;,
&cdeman.XmTextFieldGetSelectionWcs;,
&cdeman.XmTextFieldGetString;,
&cdeman.XmTextFieldGetStringWcs;,
&cdeman.XmTextFieldGetSubstring;,
&cdeman.XmTextFieldGetSubstringWcs;,
&cdeman.XmTextFieldInsert;,
&cdeman.XmTextFieldInsertWcs;,
&cdeman.XmTextFieldPaste;,
&cdeman.XmTextFieldPasteLink;,
&cdeman.XmTextFieldPosToXY;,
&cdeman.XmTextFieldRemove;,
&cdeman.XmTextFieldReplace;,
&cdeman.XmTextFieldReplaceWcs;,
&cdeman.XmTextFieldSetAddMode;,
&cdeman.XmTextFieldSetEditable;,
&cdeman.XmTextFieldSetHighlight;,
&cdeman.XmTextFieldSetInsertionPosition;,
&cdeman.XmTextFieldSetMaxLength;,
&cdeman.XmTextFieldSetSelection;,
&cdeman.XmTextFieldSetString;,
&cdeman.XmTextFieldSetStringWcs;,
&cdeman.XmTextFieldShowPosition;, and
&cdeman.XmTextFieldXYToPos;.