# TransientShell
library call`TransientShell`The TransientShell widget classTransientShellwidget classTransientShell&npzwc;#include &lt;Xm/Xm.h>
&npzwc;#include &lt;X11/Shell.h>
## DESCRIPTION


TransientShell is used for shell windows that can be manipulated by the
window manager, but are not allowed to be iconified separately. For example,
DialogBoxes make no sense without their associated application.
They are iconified by the window manager only if the main application
shell is iconified.
### Classes


TransientShell inherits behavior and resources from`Core`,`Composite`,`Shell`,`WMShell`, and`VendorShell`.

The class pointer is`transientShellWidgetClass`.

The class name is`TransientShell`.
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

In addition to these new resources,`TransientShell`overrides the`XmNsaveUnder`resource in`Shell`and the`XmNtransient`resource in`WMShell`.

`TransientShell Resource Set``Name``Class``Type``Default``Access`XmNtransientForXmCTransientForWidgetNULLCSG

* **`XmNtransientFor`** 

Specifies a widget that the shell acts as a pop-up for.
If this resource is NULL or is a widget that has not been realized, the`XmNwindowGroup`is used instead.

### Inherited Resources


TransientShell inherits behavior and resources from the superclasses
described in the following tables,
which define sets of widget resources used by the programmer
to specify data.
For a complete description of each resource, refer to the reference
page for that superclass.

The programmer can also set the resource values for the
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G), or is not applicable (N/A).

`VendorShell Resource Set``Name``Class``Type``Default``Access`XmNaudibleWarningXmCAudibleWarningunsigned charXmBELLCSGXmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTabledynamicCSGXmNdefaultFontListXmCDefaultFontListXmFontListdynamicCGXmNdeleteResponseXmCDeleteResponseunsigned charXmDESTROYCSGXmNinputMethodXmCInputMethodStringNULLCSGXmNinputPolicyXmCInputPolicyXmInputPolicyXmPER_SHELLCSGXmNkeyboardFocusPolicyXmCKeyboardFocusPolicyunsigned charXmEXPLICITCSGXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTabelXmRenderTabledynamicCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectionXmLEFT_TO_RIGHTCGXmNmwmDecorationsXmCMwmDecorationsint-1CGXmNmwmFunctionsXmCMwmFunctionsint-1CGXmNmwmInputModeXmCMwmInputModeint-1CGXmNmwmMenuXmCMwmMenuStringNULLCGXmNpreeditTypeXmCPreeditTypeStringdynamicCSGXmNshellUnitTypeXmCShellUnitTypeunsigned charXmPIXELSCSGXmNtextFontListXmCTextFontListXmFontListdynamicCSGXmNtextRenderTableXmCTextRenderTableXmRenderTabledynamicCSGXmNunitTypeXmCUnitTypeunsigned charXmPIXELSCSGXmNuseAsyncGeometryXmCUseAsyncGeometryBooleanFalseCSG

`WMShell Resource Set``Name``Class``Type``Default``Access`XmNbaseHeightXmCBaseHeightintXtUnspecifiedShellIntCSGXmNbaseWidthXmCBaseWidthintXtUnspecifiedShellIntCSGXmNheightIncXmCHeightIncintXtUnspecifiedShellIntCSGXmNiconMaskXmCIconMaskPixmapNULLCSGXmNiconPixmapXmCIconPixmapPixmapNULLCSGXmNiconWindowXmCIconWindowWindowNULLCSGXmNiconXXmCIconXintXtUnspecifiedShellIntCSGXmNiconYXmCIconYintXtUnspecifiedShellIntCSGXmNinitialStateXmCInitialStateintNormalStateCSGXmNinputXmCInputBooleanTrueCSGXmNmaxAspectXXmCMaxAspectXintXtUnspecifiedShellIntCSGXmNmaxAspectYXmCMaxAspectYintXtUnspecifiedShellIntCSGXmNmaxHeightXmCMaxHeightintXtUnspecifiedShellIntCSGXmNmaxWidthXmCMaxWidthintXtUnspecifiedShellIntCSGXmNminAspectXXmCMinAspectXintXtUnspecifiedShellIntCSGXmNminAspectYXmCMinAspectYintXtUnspecifiedShellIntCSGXmNminHeightXmCMinHeightintXtUnspecifiedShellIntCSGXmNminWidthXmCMinWidthintXtUnspecifiedShellIntCSGXmNtitleXmCTitleStringdynamicCSGXmNtitleEncodingXmCTitleEncodingAtomdynamicCSGXmNtransientXmCTransientBooleanTrueCSGXmNwaitForWmXmCWaitForWmBooleanTrueCSGXmNwidthIncXmCWidthIncintXtUnspecifiedShellIntCSGXmNwindowGroupXmCWindowGroupWindowdynamicCSGXmNwinGravityXmCWinGravityintdynamicCSGXmNwmTimeoutXmCWmTimeoutint5000 msCSG

`Shell Resource Set``Name``Class``Type``Default``Access`XmNallowShellResizeXmCAllowShellResizeBooleanFalseCGXmNcreatePopupChildProcXmCCreatePopupChildProcXtCreatePopupChildProcNULLCSGXmNgeometryXmCGeometryStringNULLCSGXmNoverrideRedirectXmCOverrideRedirectBooleanFalseCSGXmNpopdownCallbackXmCCallbackXtCallbackListNULLCXmNpopupCallbackXmCCallbackXtCallbackListNULLCXmNsaveUnderXmCSaveUnderBooleanTrueCSGXmNvisualXmCVisualVisual *CopyFromParentCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for TransientShell.
## RELATED


&cdeman.Composite;,
&cdeman.Core;, &cdeman.Shell;, &cdeman.VendorShell;, and &cdeman.WMShell;.