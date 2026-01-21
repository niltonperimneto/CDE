# XmDialogShell
library call`XmDialogShell`The DialogShell widget classXmDialogShellwidget classDialogShell&npzwc;#include &lt;Xm/DialogS.h>
## DESCRIPTION


Modal and modeless dialogs use DialogShell as the Shell parent.
DialogShell widgets cannot
be iconified. Instead, all secondary DialogShell widgets
associated with an ApplicationShell widget are iconified and
de-iconified as a group with the primary widget.

The client indirectly manipulates DialogShell through the convenience
interfaces during creation, and it can directly manipulate its
BulletinBoard-derived child.
Much of the functionality of DialogShell assumes
that its child is a BulletinBoard subclass, although it
can potentially stand alone.

Setting`XmNheight`,`XmNwidth`,
or`XmNborderWidth`for
either a DialogShell or its managed child usually sets that resource to the
same value in both the parent and the child. When an off-the-spot input
method exists, the height and width of the shell may be greater than
those of the managed child in order to accommodate the input method.
In this case, setting`XmNheight`or`XmNwidth`for the
shell does not necessarily set that resource to the same value in
the managed child, and setting`XmNheight`or`XmNwidth`for the child does not necessarily set that resource to the same
value in the shell.

For the managed child of a DialogShell, regardless of the value of
the shell's`XmNallowShellResize`resource,
setting`XmNx`or`XmNy`sets the corresponding resource of the parent but does not change the
child's position relative to the parent.
The`XtGetValues`resource for the child's`XmNx`or`XmNy`yields the
value of the corresponding resource in the parent.
The x and y-coordinates of the child's upper left outside
corner relative to the parent's upper left inside corner are both 0 (zero)
minus the value of`XmNborderWidth`.

Note that theInter-Client Communication Conventions Manual(ICCCM)
allows a window manager to change or control the border width of a reparented
top-level window.

DialogShell uses the`XmQTdialogShellSavvy`trait.
### Classes


DialogShell inherits behavior, resources, and traits from the`Core`,`Composite`,`Shell`,`WMShell`,`VendorShell`, and`TransientShell`classes.

The class pointer is`xmDialogShellWidgetClass`.

The class name is`XmDialogShell`.
### New Resources


DialogShell defines no new resources but overrides the`XmNdeleteResponse`resource in the`VendorShell`class.
### Inherited Resources


DialogShell inherits behavior and resources from the
superclasses described in the following tables,
which define sets of widget resources used by the programmer
to specify data.

For a complete description of each resource, refer to the
reference page for that superclass.
The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile,
remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable
(N/A).

`TransientShell Resource Set``Name``Class``Type``Default``Access`XmNtransientForXmCTransientForWidgetNULLCSG

`VendorShell Resource Set``Name``Class``Type``Default``Access`XmNaudibleWarningXmCAudibleWarningunsigned charXmBELLCSGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNdefaultFontListXmCDefaultFontListXmFontListdynamicCGXmNdeleteResponseXmCDeleteResponseunsigned charXmUNMAPCSGXmNinputMethodXmCInputMethodStringNULLCSGXmNinputPolicyXmCInputPolicyXmInputPolicyXmPER_SHELLCSGXmNkeyboardFocusPolicyXmCKeyboardFocusPolicyunsigned charXmEXPLICITCSGXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTabelXmRenderTabledynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectionXmLEFT_TO_RIGHTCGXmNmwmDecorationsXmCMwmDecorationsint-1CGXmNmwmFunctionsXmCMwmFunctionsint-1CGXmNmwmInputModeXmCMwmInputModeint-1CGXmNmwmMenuXmCMwmMenuStringNULLCGXmNpreeditTypeXmCPreeditTypeStringdynamicCSGXmNshellUnitTypeXmCShellUnitTypeunsigned charXmPIXELSCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNunitTypeXmCUnitTypeunsigned charXmPIXELSCSGXmNuseAsyncGeometryXmCUseAsyncGeometryBooleanFalseCSG

`WMShell Resource Set``Name``Class``Type``Default``Access`XmNbaseHeightXmCBaseHeightintXtUnspecifiedShellIntCSGXmNbaseWidthXmCBaseWidthintXtUnspecifiedShellIntCSGXmNheightIncXmCHeightIncintXtUnspecifiedShellIntCSGXmNiconMaskXmCIconMaskPixmapNULLCSGXmNiconPixmapXmCIconPixmapPixmapNULLCSGXmNiconWindowXmCIconWindowWindowNULLCSGXmNiconXXmCIconXintXtUnspecifiedShellIntCSGXmNiconYXmCIconYintXtUnspecifiedShellIntCSGXmNinitialStateXmCInitialStateintNormalStateCSGXmNinputXmCInputBooleanTrueCSGXmNmaxAspectXXmCMaxAspectXintXtUnspecifiedShellIntCSGXmNmaxAspectYXmCMaxAspectYintXtUnspecifiedShellIntCSGXmNmaxHeightXmCMaxHeightintXtUnspecifiedShellIntCSGXmNmaxWidthXmCMaxWidthintXtUnspecifiedShellIntCSGXmNminAspectXXmCMinAspectXintXtUnspecifiedShellIntCSGXmNminAspectYXmCMinAspectYintXtUnspecifiedShellIntCSGXmNminHeightXmCMinHeightintXtUnspecifiedShellIntCSGXmNminWidthXmCMinWidthintXtUnspecifiedShellIntCSGXmNtitleXmCTitleStringdynamicCSGXmNtitleEncodingXmCTitleEncodingAtomdynamicCSGXmNtransientXmCTransientBooleanTrueCSGXmNwaitForWmXmCWaitForWmBooleanTrueCSGXmNwidthIncXmCWidthIncintXtUnspecifiedShellIntCSGXmNwindowGroupXmCWindowGroupWindowdynamicCSGXmNwinGravityXmCWinGravityintdynamicCSGXmNwmTimeoutXmCWmTimeoutint5000 msCSGIf values forXmNminWidthandXmNminHeightare present,
and values forXmNbaseWidthandXmNbaseHeightare absent,XmNminWidthandXmNminHeightwill be used as
default values forXmNbaseWidthandXmNbaseHeight,
and these values will be added to the shell size specified by the user.
To work around this, add arguments during widget
creation to explicitly setXmNbaseWidthandXmNbaseHeightto zero.

`Shell Resource Set``Name``Class``Type``Default``Access`XmNallowShellResizeXmCAllowShellResizeBooleanFalseCGXmNcreatePopupChildProcXmCCreatePopupChildProcXtCreatePopupChildProcNULLCSGXmNgeometryXmCGeometryStringNULLCSGXmNoverrideRedirectXmCOverrideRedirectBooleanFalseCSGXmNpopdownCallbackXmCCallbackXtCallbackListNULLCXmNpopupCallbackXmCCallbackXtCallbackListNULLCXmNsaveUnderXmCSaveUnderBooleanTrueCSGXmNvisualXmCVisualVisual *CopyFromParentCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for XmDialogShell.
## RELATED


&cdeman.Composite;, &cdeman.Core;, &cdeman.Shell;,
&cdeman.TransientShell;, &cdeman.WMShell;, &cdeman.VendorShell;,
and &cdeman.XmCreateDialogShell;.