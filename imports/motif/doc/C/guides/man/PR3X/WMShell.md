# WMShell
library call`WMShell`The WMShell widget classWMShellwidget classWMShell&npzwc;#include &lt;Xm/Xm.h>
&npzwc;#include &lt;X11/Shell.h>
## DESCRIPTION


WMShell is a top-level widget that encapsulates the interaction with the
window manager.
### Classes


WMShell inherits behavior and resources from the`Core`,`Composite`,
and`Shell`classes.

The class pointer is`wmShellWidgetClass`.

The class name is`WMShell`.
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

`WMShell Resource Set``Name``Class``Type``Default``Access`XmNbaseHeightXmCBaseHeightintXtUnspecifiedShellIntCSGXmNbaseWidthXmCBaseWidthintXtUnspecifiedShellIntCSGXmNheightIncXmCHeightIncintXtUnspecifiedShellIntCSGXmNiconMaskXmCIconMaskPixmapNULLCSGXmNiconPixmapXmCIconPixmapPixmapNULLCSGXmNiconWindowXmCIconWindowWindowNULLCSGXmNiconXXmCIconXintXtUnspecifiedShellIntCSGXmNiconYXmCIconYintXtUnspecifiedShellIntCSGXmNinitialStateXmCInitialStateintNormalStateCSGXmNinputXmCInputBooleanFalseCSGXmNmaxAspectXXmCMaxAspectXintXtUnspecifiedShellIntCSGXmNmaxAspectYXmCMaxAspectYintXtUnspecifiedShellIntCSGXmNmaxHeightXmCMaxHeightintXtUnspecifiedShellIntCSGXmNmaxWidthXmCMaxWidthintXtUnspecifiedShellIntCSGXmNminAspectXXmCMinAspectXintXtUnspecifiedShellIntCSGXmNminAspectYXmCMinAspectYintXtUnspecifiedShellIntCSGXmNminHeightXmCMinHeightintXtUnspecifiedShellIntCSGXmNminWidthXmCMinWidthintXtUnspecifiedShellIntCSGXmNtitleXmCTitleStringdynamicCSGXmNtitleEncodingXmCTitleEncodingAtomdynamicCSGXmNtransientXmCTransientBooleanFalseCSGXmNwaitForWmXmCWaitForWmBooleanTrueCSGXmNwidthIncXmCWidthIncintXtUnspecifiedShellIntCSGXmNwindowGroupXmCWindowGroupWindowdynamicCSGXmNwinGravityXmCWinGravityintdynamicCSGXmNwmTimeoutXmCWmTimeoutint5000 msCSG

* **`XmNbaseHeight`** 

Specifies the base for a progression of preferred heights for the
window manager to use in sizing the widget.
The preferred heights are`XmNbaseHeight`plus integral multiples of`XmNheightInc`, with a minimum of`XmNminHeight`and a maximum
of`XmNmaxHeight`.
If an initial value is not supplied for`XmNbaseHeight`but is
supplied for`XmNbaseWidth`, the value of`XmNbaseHeight`is set to 0 (zero) when the widget is realized.
* **`XmNbaseWidth`** 

Specifies the base for a progression of preferred widths for the
window manager to use in sizing the widget.
The preferred widths are`XmNbaseWidth`plus integral multiples of`XmNwidthInc`, with a minimum of`XmNminWidth`and a maximum of`XmNmaxWidth`.
If an initial value is not supplied for`XmNbaseWidth`but is
supplied for`XmNbaseHeight`, the value of`XmNbaseWidth`is set to 0 (zero) when the widget is realized.
* **`XmNheightInc`** 

Specifies the increment for a progression of preferred heights for the
window manager to use in sizing the widget.
The preferred heights are`XmNbaseHeight`plus integral multiples of`XmNheightInc`, with a minimum of`XmNminHeight`and a maximum
of`XmNmaxHeight`.
If an initial value is not supplied for`XmNheightInc`but is
supplied for`XmNwidthInc`, the value of`XmNheightInc`is set to 1 when the widget is realized.
* **`XmNiconMask`** 

Specifies a bitmap that could be used by the window manager to clip the`XmNiconPixmap`bitmap to make the icon nonrectangular.
* **`XmNiconPixmap`** 

Specifies a bitmap that could be used by the window manager as the
application's icon.
* **`XmNiconWindow`** 

Specifies the ID of a window that could be used by the window manager
as the application's icon.
* **`XmNiconX`** 

Specifies a suitable place to put the application's icon; this is a hint
to the window manager in root window coordinates. Because the window manager
controls icon placement policy, this resource may be ignored.
* **`XmNiconY`** 

Specifies a suitable place to put the application's icon; this is a hint
to the window manager in root window coordinates.
Because the window manager
controls icon placement policy, this resource may be ignored.
* **`XmNinitialState`** 

Specifies the state the application wants the widget instance to
start in.
It must be one of the constants`NormalState`or`IconicState`.
* **`XmNinput`** 

Specifies the application's input model for this widget and its
descendants.
The meaning of a True or False value for this resource depends on the
presence or absence of a WM_TAKE_FOCUS atom in the
WM_PROTOCOLS property:`Input Model``XmNinput`WM_TAKE_FOCUSNo inputFalseAbsentPassiveTrueAbsentLocally activeTruePresentGlobally activeFalsePresent

For more information on input models, see the X Consortium StandardInter-Client Communication Conventions Manual(ICCCM).
* **`XmNmaxAspectX`** 

Specifies the numerator of the maximum aspect ratio (X/Y) that the
application wants the widget instance to have.
* **`XmNmaxAspectY`** 

Specifies the denominator of the maximum aspect ratio (X/Y) that the
application wants the widget instance to have.
* **`XmNmaxHeight`** 

Specifies the maximum height that the application wants the widget
instance to have.
* **`XmNmaxWidth`** 

Specifies the maximum width that the application wants the widget
instance to have.
* **`XmNminAspectX`** 

Specifies the numerator of the minimum aspect ratio (X/Y) that the
application wants the widget instance to have.
* **`XmNminAspectY`** 

Specifies the denominator of the minimum aspect ratio (X/Y) that the
application wants the widget instance to have.
* **`XmNminHeight`** 

Specifies the minimum height that the application wants the widget
instance to have.
* **`XmNminWidth`** 

Specifies the minimum width that the application wants the widget
instance to have.
* **`XmNtitle`** 

Specifies the application name to be displayed by the window manager.
The default is the icon name, if specified; otherwise, it is the name of the
application.
When`XtGetValues`is called on this resource, the returned value
is a pointer to the actual resource value and
should not be freed.
* **`XmNtitleEncoding`** 

Specifies a property type that represents the encoding of the`XmNtitle`string.
If a language procedure has been set, the default is None; otherwise,
the default is`XA_STRING`. When the widget is realized, if the value
is None, the corresponding name is assumed to be in the current locale.
The name is passed to`XmbTextListToTextProperty`with an encoding
style of`XStdICCTextStyle`. The resulting encoding is`STRING`if the name is fully convertible to`STRING`; otherwise it is`COMPOUND_TEXT`.
The values of the encoding resources are not changed; they remain None.
* **`XmNtransient`** 

Specifies a Boolean value that is True if the widget instance is
transient, typically a popup on behalf of another widget.
The window manager may treat a transient widget's window differently
from other windows.
For example, a window manager may
not iconify a transient window
separately from its associated application.
Applications and users should not normally alter this resource.
* **`XmNwaitForWm`** 

When True, specifies that the Intrinsics waits the length of time given
by the`XmNwmTimeout`resource for the window manager to respond to
certain actions before assuming that there is no window manager present.
This resource is altered by the Intrinsics as it receives, or fails
to receive, responses from the window manager.
* **`XmNwidthInc`** 

Specifies the base for a progression of preferred widths for the
window manager to use in sizing the widget.
The preferred widths are`XmNbaseWidth`plus integral multiples of`XmNwidthInc`, with a minimum of`XmNminWidth`and a maximum of`XmNmaxWidth`.
If an initial value is not supplied for`XmNwidthInc`but is
supplied for`XmNheightInc`, the value of`XmNwidthInc`is set to 1 when the widget is realized.
* **`XmNwindowGroup`** 

Specifies the ID of a window with which this widget instance is
associated.
By convention, this window is the "leader" of a group of windows.
A window manager may treat all windows in a group in some way; for
example, it may always move or iconify them together.

If no initial value is specified, the value is set to the window of the
first realized ancestor widget in the parent hierarchy when the widget
is realized.
If a value of`XtUnspecifiedWindowGroup`is specified, no window
group is set.
* **`XmNwinGravity`** 

Specifies the window gravity for use by the window manager in
positioning the widget.
If no initial value is specified, the value is set when the widget is
realized.
If`XmNgeometry`is not NULL,`XmNwinGravity`is set to the
window gravity returned by`XWMGeometry`.
Otherwise,`XmNwinGravity`is set to`NorthWestGravity`.
* **`XmNwmTimeout`** 

Specifies the length of time that the Intrinsics waits for the window
manager to respond to certain actions before assuming that there is no
window manager present.
The value is in milliseconds and must not be negative.

### Inherited Resources


WMShell inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.

`Shell Resource Set``Name``Class``Type``Default``Access`XmNallowShellResizeXmCAllowShellResizeBooleanFalseCGXmNcreatePopupChildProcXmCCreatePopupChildProcXtCreatePopupChildProcNULLCSGXmNgeometryXmCGeometryStringNULLCSGXmNoverrideRedirectXmCOverrideRedirectBooleanFalseCSGXmNpopdownCallbackXmCCallbackXtCallbackListNULLCXmNpopupCallbackXmCCallbackXtCallbackListNULLCXmNsaveUnderXmCSaveUnderBooleanFalseCSGXmNvisualXmCVisualVisual *CopyFromParentCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for WMShell.
## RELATED


&cdeman.Composite;, &cdeman.Core;, and &cdeman.Shell;.