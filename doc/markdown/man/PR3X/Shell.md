# Shell
library call`Shell`The Shell widget classShellwidget classShell&npzwc;#include &lt;Xm/Xm.h>
&npzwc;#include &lt;X11/Shell.h>
## DESCRIPTION


Shell is a top-level widget (with only one managed child) that encapsulates
the interaction with the window manager.

At the time the shell's child is managed, the child's width is used
for both widgets if the shell is unrealized and no width has been
specified for the shell. Otherwise, the shell's width is used for
both widgets. The same relations hold for the height of the shell and
its child.
### Classes


Shell inherits behavior and resources from`Composite`and`Core`.

The class pointer is`shellWidgetClass`.

The class name is`Shell`.
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

`Shell Resource Set``Name``Class``Type``Default``Access`XmNallowShellResizeXmCAllowShellResizeBooleanFalseCGXmNcreatePopupChildProcXmCCreatePopupChildProcXtCreatePopupChildProcNULLCSGXmNgeometryXmCGeometryStringNULLCSGXmNoverrideRedirectXmCOverrideRedirectBooleanFalseCSGXmNpopdownCallbackXmCCallbackXtCallbackListNULLCXmNpopupCallbackXmCCallbackXtCallbackListNULLCXmNsaveUnderXmCSaveUnderBooleanFalseCSGXmNvisualXmCVisualVisual *CopyFromParentCSG

* **`XmNallowShellResize`** 

Specifies that if this resource is False, the Shell widget instance
returns`XtGeometryNo`to all geometry requests from its children.
All Motif convenience create dialog functions override this default
value and set`XmNallowShellResize`to True.
* **`XmNcreatePopupChildProc`** 

Specifies the pointer to a function that is called when the Shell
widget instance is popped up by`XtPopup`.
The function creates the child widget when the shell is popped up
instead of when the application starts up.
This can be used if the child needs to be reconfigured each time the
shell is popped up.
The function takes one argument, the popup shell, and returns no result.
It is called after the popup callbacks specified by`XmNpopupCallback`.
* **`XmNgeometry`** 

Specifies the desired geometry for the widget instance. This resource
is examined only when the widget instance is unrealized and the number
of its managed children is changed. It is used to change the values
of the`XmNx`,`XmNy`,`XmNwidth`, and`XmNheight`resources.
When`XtGetValues`is called on this resource, the returned value
is a pointer to the actual resource value and
should not be freed. In addition, this resource is not copied on
creation or by`XtSetValues`. The application must ensure that
the string remains valid until the shell is realized.
* **`XmNoverrideRedirect`** 

If True, specifies that the widget instance is a temporary
window that should be ignored by the window manager. Applications and
users should not normally alter this resource.
* **`XmNpopdownCallback`** 

Specifies a list of callbacks that is called when the widget instance
is popped down by`XtPopdown`.
* **`XmNpopupCallback`** 

Specifies a list of callbacks that is called when the widget instance
is popped up by`XtPopup`.
The second argument to`XtPopup`must be`XtGrabNone`.
* **`XmNsaveUnder`** 

If True, specifies that it is desirable to save the contents of the
screen beneath this widget instance, avoiding expose events when the
instance is unmapped. This is a hint, and an implementation may save
contents whenever it desires, including always or never.
* **`XmNvisual`** 

Specifies the visual used in creating the widget.

### Inherited Resources


Shell inherits behavior and resources from the
superclass described in the following table.
For a complete description of each resource, refer to the
reference page for that superclass.

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for Shell.
## RELATED


&cdeman.Composite; and &cdeman.Core;.