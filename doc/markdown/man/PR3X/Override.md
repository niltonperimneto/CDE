# OverrideShell
library call`OverrideShell`The OverrideShell widget classOverrideShellwidget classOverrideShell&npzwc;#include &lt;Xm/Xm.h>
&npzwc;#include &lt;X11/Shell.h>
## DESCRIPTION


OverrideShell is used for shell windows that completely bypass
the window manager, for example, PopupMenu shells.
### Classes


OverrideShell inherits behavior and resources from`Core`,`Composite`, and`Shell`.

The class pointer is`overrideShellWidgetClass`.

The class name is`OverrideShell`.
### New Resources


OverrideShell defines no new resources, but overrides
the`XmNoverrideRedirect`and`XmNsaveUnder`resources in the`Shell`class.
### Inherited Resources


OverrideShell inherits behavior and resources from the following
superclasses. For a complete description of each resource, refer to the
reference page for that superclass.

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

`Shell Resource Set``Name``Class``Type``Default``Access`XmNallowShellResizeXmCAllowShellResizeBooleanFalseCGXmNcreatePopupChildProcXmCCreatePopupChildProcXtCreatePopupChildProcNULLCSGXmNgeometryXmCGeometryStringNULLCSGXmNoverrideRedirectXmCOverrideRedirectBooleanTrueCSGXmNpopdownCallbackXmCCallbackXtCallbackListNULLCXmNpopupCallbackXmCCallbackXtCallbackListNULLCXmNsaveUnderXmCSaveUnderBooleanTrueCSGXmNvisualXmCVisualVisual *CopyFromParentCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for OverrideShell.
## RELATED


&cdeman.Composite;, &cdeman.Core;, and &cdeman.Shell;.