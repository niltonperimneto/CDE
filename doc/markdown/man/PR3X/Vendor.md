# VendorShell
library call`VendorShell`The VendorShell widget classVendorShellwidget classVendorShell#include <Xm/Xm.h>
#include <X11/Shell.h>
## DESCRIPTION


VendorShell is a Motif widget class used as a supporting superclass for
all shell classes that are visible to the window manager and that are
not override redirect.
It contains resources that describe the MWM-specific look and feel.
It also manages the MWM-specific communication needed by all VendorShell
subclasses.
See themwmreference page for more information.

If an application uses the`XmNmwmDecorations`,`XmNmwmFunctions`, or`XmNmwmInputMode`resource, it should
include the fileXm/MwmUtil.h.

Setting`XmNheight`,`XmNwidth`, or`XmNborderWidth`for
either a VendorShell or its managed child usually sets that resource to the
same value in both the parent and the child. When an off-the-spot input
method exists, the height and width of the shell may be greater than
those of the managed child in order to accommodate the input method.
In this case, setting`XmNheight`or`XmNwidth`for the
shell does not necessarily set that resource to the same value in
the managed child, and setting`XmNheight`or`XmNwidth`for the child does not necessarily set that resource to the same
value in the shell.

For the managed child of a VendorShell, regardless of the value
of the shell's`XmNallowShellResize`, setting`XmNx`or`XmNy`sets the corresponding resource of the parent but does
not change the child's position relative to the parent.`XtGetValues`for the child's`XmNx`or`XmNy`yields the
value of the corresponding resource in the parent.
The x and y-coordinates of the child's upper left outside
corner relative to the parent's upper left inside corner are both 0 (zero)
minus the value of`XmNborderWidth`.

Note that theInter-Client Communication Conventions Manual(ICCCM)
allows a window manager to change or control the border width of a reparented
top-level window.

VendorShell holds the`XmQTspecifyRenderTable`trait.
### Classes


VendorShell inherits behavior, resources, and traits from the`Core`,`Composite`,`Shell`, and`WMShell`classes.

The class pointer is`vendorShellWidgetClass`.

The class name is`VendorShell`.
### New Resources


The following table defines a set of widget resources used by the programmer
to specify data. The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
subresource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a subresource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given subresource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).`VendorShell Resource Set``Name``Class``Type``Default``Access`XmNaudibleWarningXmCAudibleWarningunsigned charXmBELLCSGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNdefaultFontListXmCDefaultFontListXmFontListdynamicCGXmNdeleteResponseXmCDeleteResponseunsigned charXmDESTROYCSGXmNinputMethodXmCInputMethodstringNULLCSGXmNinputPolicyXmCInputPolicyXmInputPolicyXmPER_SHELLCSGXmNkeyboardFocusPolicyXmCKeyboardFocusPolicyunsigned charXmEXPLICITCSGXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTabelXmRenderTabledynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectionXmLEFT_TO_RIGHTCGXmNmwmDecorationsXmCMwmDecorationsint-1CGXmNmwmFunctionsXmCMwmFunctionsint-1CGXmNmwmInputModeXmCMwmInputModeint-1CGXmNmwmMenuXmCMwmMenuStringNULLCGXmNpreeditTypeXmCPreeditTypeStringdynamicCSGXmNverifyPreeditXmCVerifyPreeditBooleanFalseCSGXmNshellUnitTypeXmCShellUnitTypeunsigned charXmPIXELSCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNunitTypeXmCUnitTypeunsigned charXmPIXELSCSGXmNuseAsyncGeometryXmCUseAsyncGeometryBooleanFalseCSG

* **`XmNaudibleWarning`** 

Determines whether an action activates its associated audible cue.
The possible values are`XmBELL`and`XmNONE`.
* **`XmNbuttonFontList`** 

Specifies the font list used for button descendants. See the`XmNbuttonRenderTable`resource.
* **`XmNbuttonRenderTable`** 

Specifies the render table used for VendorShell's button descendants.
If this value is NULL at initialization and if the value of`XmNdefaultFontList`is not NULL,`XmNbuttonRenderTable`is initialized to the value of`XmNdefaultFontList`. If
the value of`XmNdefaultFontList`is NULL,
the parent hierarchy of the widget is searched for
an ancestor that holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found,`XmNbuttonRenderTable`is initialized to the`XmBUTTON_RENDER_TABLE`value
of the ancestor widget. If no such ancestor is found, the default
is implementation dependent. Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNdefaultFontList`** 

Specifies a default font list for VendorShell's descendants.
This resource is obsolete and exists for compatibility with
earlier releases. It has been replaced by`XmNbuttonFontList`,`XmNlabelFontList`, and`XmNtextFontList`.
* **`XmNdeleteResponse`** 

Determines what action the shell takes in response to aWM_DELETE_WINDOWmessage.
The setting can be one of three values:`XmDESTROY`,`XmUNMAP`,
and`XmDO_NOTHING`.
The resource is scanned, and the appropriate action is taken after theWM_DELETE_WINDOWcallback list (if any) that is registered with
the Protocol manager has been called.
* **`XmNinputMethod`** 

Specifies the string that sets the locale modifier for the input
method.
When`XtGetValues`is called on this resource, the returned value
is a pointer to the actual resource value and
should not be freed.
* **`XmNinputPolicy`** 

Specifies the policy to follow for creating an
Input Context (IC) for this shell. This resource can have the
following values:

* **`XmPER_SHELL`** 

Specifies that only one XIC is created per shell.
* **`XmPER_WIDGET`** 

Specifies that one XIC is created for each widget.

* **`XmNkeyboardFocusPolicy`** 

Determines allocation of keyboard focus within the widget hierarchy rooted
at this shell. The X keyboard focus must be directed to somewhere in the
hierarchy for this client-side focus management to take effect.
Possible values are`XmEXPLICIT`, specifying a click-to-type policy,
and`XmPOINTER`, specifying a pointer-driven policy.
* **`XmNlabelFontList`** 

Specifies the font list used for label descendants. See the`XmNlabelRenderTable`resource.
* **`XmNlabelRenderTable`** 

Specifies the font list used for VendorShell's label descendants
(Labels and LabelGadgets). If this value is NULL at initialization
and if the value of`XmNdefaultFontList`is not NULL,`XmNlabelFontList`is initialized to the value of`XmNdefaultFontList`. If the value of`XmNdefaultFontList`is
NULL, the parent hierarchy of the widget is searched
for an ancestor that holds the`XmQTspecifyRenderTable`trait.
If such
an ancestor is found,`XmNlabelRenderTable`is initialized to the`XmLABEL_RENDER_TABLE`of the ancestor widget. If no such ancestor
is found, the default is implementation dependent. Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNlayoutDirection`** 

Specifies the direction in which the subwidgets, children of a
widget, or other visual components are to be laid out. This policy
will apply as the default layout policy for all descendants of this
VendorShell.
* **`XmNmwmDecorations`** 

Specifies the decoration flags (specific decorations to add or remove
from the window manager frame) for the_MOTIF_WM_HINTSproperty.
If any decoration flags are specified by the_MOTIF_WM_HINTSproperty, only decorations indicated by both that property and the MWM`clientDecoration`and`transientDecoration`resources are displayed.
If no decoration flags are specified by the_MOTIF_WM_HINTSproperty,
decorations indicated by the MWM`clientDecoration`and`transientDecoration`resources are displayed.
The default for the`XmNmwmDecorations`resource
is not to specify any decoration flags for the_MOTIF_WM_HINTSproperty.

The value of this resource is the bitwise inclusive OR of one or more
flag bits.
The possible flag bit constants, defined in the include
fileXm/MwmUtil.h, are

* **`MWM_DECOR_ALL`** 

All decorationsexceptthose specified by
other flag bits that are set
* **`MWM_DECOR_BORDER`** 

Client window border
* **`MWM_DECOR_RESIZEH`** 

Resize frame handles
* **`MWM_DECOR_TITLE`** 

Title bar
* **`MWM_DECOR_MENU`** 

Window menu button
* **`MWM_DECOR_MINIMIZE`** 

Minimize window button
* **`MWM_DECOR_MAXIMIZE`** 

Maximize window button

* **`XmNmwmFunctions`** 

Specifies the function flags (specific window manager functions to
apply or not apply to the client window) for the_MOTIF_WM_HINTSproperty.
If any function flags are specified by the_MOTIF_WM_HINTSproperty, only functions indicated by both that property and the MWM`clientFunctions`and`transientFunctions`resources are
applied.
If no function flags are specified by the_MOTIF_WM_HINTSproperty, functions indicated by the MWM`clientFunctions`and`transientFunctions`resources are applied.
The default for the`XmNmwmFunctions`resource is not to specify any
function flags for the_MOTIF_WM_HINTSproperty.

The value of this resource is the bitwise inclusive OR of one or more
flag bits.
The possible flag bit constants, defined in the include
fileXm/MwmUtil.h, are

* **`MWM_FUNC_ALL`** 

All functionsexceptthose specified by
other flag bits that are set
* **`MWM_FUNC_RESIZE`** 

`f.resize`
* **`MWM_FUNC_MOVE`** 

`f.move`
* **`MWM_FUNC_MINIMIZE`** 

`f.minimize`
* **`MWM_FUNC_MAXIMIZE`** 

`f.maximize`
* **`MWM_FUNC_CLOSE`** 

`f.kill`

* **`XmNmwmInputMode`** 

Specifies the input mode flag (application modal or system modal input
constraints) for the_MOTIF_WM_HINTSproperty.
If no input mode flag is specified by the_MOTIF_WM_HINTSproperty, no input constraints are applied, and input goes to any
window.
The default for the`XmNmwmInputMode`resource is not to specify any
input mode flag for the_MOTIF_WM_HINTSproperty.

An application that sets input constraints on a dialog usually uses the
BulletinBoard's`XmNdialogStyle`resource rather than the parent
DialogShell's`XmNmwmInputMode`resource.

The possible values for this resource, defined in the
include fileXm/MwmUtil.h, are

* **`MWM_INPUT_MODELESS`** 

Input goes to any window.
* **`MWM_INPUT_PRIMARY_APPLICATION_MODAL`** 

Input does not
go to ancestors of this window.
* **`MWM_INPUT_SYSTEM_MODAL`** 

Input goes only to this window.
* **`MWM_INPUT_FULL_APPLICATION_MODAL`** 

Input does not go to other
windows in this application.

* **`XmNmwmMenu`** 

Specifies the menu items that the Motif window manager should add to the end
of the window menu. The string contains a list of items separated
by`\n`with the following format:`label [mnemonic] [accelerator] function`

If more than one item is specified, the items should be separated by a
newline character.

When`XtGetValues`is called on this resource, the returned value
is a pointer to the actual resource value and
should not be freed.
* **`XmNpreeditType`** 

Specifies the input method style or styles available to the input
manager. The resource can be a comma-separated list of the following
values:`Preedit Values``Preedit Value``XIM Style`OffTheSpotXIMPreeditAreaRootXIMPreeditNothingNoneXIMPreeditNoneOverTheSpotXIMPreeditPositionOnTheSpotXIMPreeditCallbacks

When`XtGetValues`is called on this resource, the returned value
is a pointer to the actual resource value and
should not be freed.
* **`XmNshellUnitType`** 

This resource is obsolete, and is included only for compatibility with
earlier releases of Motif. Use the`XmNunitType`resource
instead.
* **`XmNtextFontList`** 

Specifies the font list used for text descendants. See the`XmNtextRenderTable`resource.
* **`XmNtextRenderTable`** 

Specifies the render table used for VendorShell's Text and List
descendants. If this value is NULL at initialization
and if the value of`XmNdefaultFontList`is not NULL,`XmNtextRenderTable`is initialized to the value of`XmNdefaultFontList`. If the value of`XmNdefaultFontList`is
NULL, the parent hierarchy of the widget is searched
for an ancestor that holds the`XmQTspecifyRenderTable`trait.
If such
an ancestor is found,`XmNtextRenderTable`is initialized to the`XmTEXT_RENDER_TABLE`value of the ancestor widget. If no such ancestor
is found, the default is implementation dependent.
Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNunitType`** 

Provides the basic support for resolution independence. It defines
the type of units a widget uses with sizing and positioning resources.
The resource has a default unit type of`XmPIXELS`.

The unit type can also be specified in resource files, with the
following format:`<floating value><unit>`

where:

* **`unit`** 

is <" ", pixels, inches, centimeters, millimeters, points, font units>
* **pixels** 

is <`pix`,`pixel`,pixels>
* **`inches`** 

is <`in`,`inch`,`inches`>
* **`centimeter`** 

is <`cm`,`centimeter`,`centimeters`>
* **`millimeters`** 

is <`mm`,`millimeter`,`millimeters`>
* **`points`** 

is <`pt`,`point`,`points`>
* **`font units`** 

is <`fu`,`font_unit`,`font_units`>
* **`float`** 

is {"+"|"-"}{{<"0"-"9">*}.}<"0"-"9">*

Note that the type Dimension must always be positive.


For example,xmfonts*XmMainWindow.height: 10.4cm
*PostIn.width: 3inches

`XmNunitType`can have the following values:

* **`XmPIXELS`** 

All values provided to the widget are treated as normal
pixel values.
* **`XmMILLIMETERS`** 

All values provided to the widget are treated as normal millimeter
values.
* **`Xm100TH_MILLIMETERS`** 

All values provided to the widget are treated
as 1/100 of a millimeter.
* **`XmCENTIMETERS`** 

All values provided to the widget are treated as normal centimeter
values.
* **`XmINCHES`** 

All values provided to the widget are treated as normal inch
values.
* **`Xm1000TH_INCHES`** 

All values provided to the widget are treated as
1/1000 of an inch.
* **`XmPOINTS`** 

All values provided to the widget are treated as normal point
values. A point is a unit used in text processing
applications and is defined as 1/72 of an inch.
* **`Xm100TH_POINTS`** 

All values provided to the widget are treated as
1/100 of a point. A point is a unit used in text processing
applications and is defined as 1/72 of an inch.
* **`XmFONT_UNITS`** 

All values provided to the widget are treated as normal font
units. A font unit has horizontal and vertical components.
These are the values of the XmScreen resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.
* **`Xm100TH_FONT_UNITS`** 

All values provided to the widget are treated as 1/100 of a font unit.
A font unit has horizontal and vertical components. These are the
values of the`XmScreen`resources`XmNhorizontalFontUnit`and`XmNverticalFontUnit`.


For more information about units, refer to the`XmConvertUnits`reference page.
* **`XmNuseAsyncGeometry`** 

Specifies whether the geometry manager should wait for confirmation of a
geometry request to the window manager.
When the value of this resource is True, the geometry manager forces`XmNwaitForWm`to False and`XmNwmTimeout`to 0, and it relies
on asynchronous notification.
When the value of this resource is False,`XmNwaitForWm`and`XmNwmTimeout`are unaffected.
The default is False.

### Inherited Resources


VendorShell inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.`WMShell Resource Set``Name``Class``Type``Default``Access`XmNbaseHeightXmCBaseHeightintXtUnspecifiedShellIntCSGXmNbaseWidthXmCBaseWidthintXtUnspecifiedShellIntCSGXmNheightIncXmCHeightIncintXtUnspecifiedShellIntCSGXmNiconMaskXmCIconMaskPixmapNULLCSGXmNiconPixmapXmCIconPixmapPixmapNULLCSGXmNiconWindowXmCIconWindowWindowNULLCSGXmNiconXXmCIconXintXtUnspecifiedShellIntCSGXmNiconYXmCIconYintXtUnspecifiedShellIntCSGXmNinitialStateXmCInitialStateintNormalStateCSGXmNinputXmCInputBooleanTrueCSGXmNmaxAspectXXmCMaxAspectXintXtUnspecifiedShellIntCSGXmNmaxAspectYXmCMaxAspectYintXtUnspecifiedShellIntCSGXmNmaxHeightXmCMaxHeightintXtUnspecifiedShellIntCSGXmNmaxWidthXmCMaxWidthintXtUnspecifiedShellIntCSGXmNminAspectXXmCMinAspectXintXtUnspecifiedShellIntCSGXmNminAspectYXmCMinAspectYintXtUnspecifiedShellIntCSGXmNminHeightXmCMinHeightintXtUnspecifiedShellIntCSGXmNminWidthXmCMinWidthintXtUnspecifiedShellIntCSGXmNtitleXmCTitleStringdynamicCSGXmNtitleEncodingXmCTitleEncodingAtomdynamicCSGXmNtransientXmCTransientBooleanFalseCSGXmNwaitForWmXmCWaitForWmBooleanTrueCSGXmNwidthIncXmCWidthIncintXtUnspecifiedShellIntCSGXmNwindowGroupXmCWindowGroupWindowdynamicCSGmNwinGravityXmCWinGravityintdynamicCSGXmNwmTimeoutXmCWmTimeoutint5000 msCSG`Shell Resource Set``Name``Class``Type``Default``Access`XmNallowShellResizeXmCAllowShellResizeBooleanFalseCGXmNcreatePopupChildProcXmCCreatePopupChildProcXtCreatePopupChildProcNULLCSGXmNgeometryXmCGeometryStringNULLCSGXmNoverrideRedirectXmCOverrideRedirectBooleanFalseCSGXmNpopdownCallbackXmCCallbackXtCallbackListNULLCXmNpopupCallbackXmCCallbackXtCallbackListNULLCXmNsaveUnderXmCSaveUnderBooleanFalseCSGXmNvisualXmCVisualVisual *CopyFromParentCSG`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for VendorShell.
## RELATED


&cdeman.Composite;,
&cdeman.Core;,
&cdeman.mwm;,
&cdeman.Shell;,
&cdeman.WMShell;,
&cdeman.XmActivateProtocol;,
&cdeman.XmActivateWMProtocol;,
&cdeman.XmAddProtocolCallback;,
&cdeman.XmAddWMProtocolCallback;,
&cdeman.XmAddProtocols;,
&cdeman.XmAddWMProtocols;,
&cdeman.XmDeactivateProtocol;,
&cdeman.XmDeactivateWMProtocol;,
&cdeman.XmGetAtomName;,
&cdeman.XmInternAtom;,
&cdeman.XmIsMotifWMRunning;,
&cdeman.XmRemoveProtocolCallback;,
&cdeman.XmRemoveWMProtocolCallback;,
&cdeman.XmRemoveProtocols;,
&cdeman.XmRemoveWMProtocols;,
&cdeman.XmScreen;,
&cdeman.XmSetProtocolHooks;,
and &cdeman.XmSetWMProtocolHooks;.