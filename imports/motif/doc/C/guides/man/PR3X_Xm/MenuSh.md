# XmMenuShell
library call`XmMenuShell`The MenuShell widget classXmMenuShellwidget classMenuShell&npzwc;#include &lt;Xm/MenuShell.h>
## DESCRIPTION


The MenuShell widget is a custom OverrideShell widget. An OverrideShell
widget bypassesmwmwhen displaying itself. It is designed
specifically to contain Popup or Pulldown menu panes.

Most application writers never encounter this widget if they use the
menu-system convenience functions,`XmCreatePopupMenu`or`XmCreatePulldown Menu`,
to create a Popup or Pulldown menu pane.
The convenience functions automatically create a MenuShell widget as the
parent of the menu pane. However, if the convenience functions are
not used, the application programmer must create
the required MenuShell. In this case, it is important to note that the
parent of the MenuShell depends on the type of menu system
being built.

If the MenuShell is for the top-level Popup menu pane, the MenuShell's
parent must be the widget from which the Popup menu pane is popped up.

If the MenuShell is for a menu pane that is pulled down from a Popup or
another Pulldown menu pane, the MenuShell's parent must be the Popup or
Pulldown menu pane.

If the MenuShell is for a menu pane that is pulled down from a MenuBar,
the MenuShell's parent must be the MenuBar.

If the MenuShell is for a Pulldown menu pane in an OptionMenu, the
MenuShell's parent must be the OptionMenu's parent.

Setting`XmNheight`,`XmNwidth`, or`XmNborderWidth`for
either a MenuShell or its child sets that resource to the same value
in both the parent and the child.
An application should always specify these resources for the child, not
the parent.

For the managed child of a MenuShell, regardless of the value of
the shell's`XmNallowShellResize`, setting`XmNx`or`XmNy`sets the corresponding resource of the parent but does not change the
child's position relative to the parent.`XtGetValues`for the child's`XmNx`or`XmNy`yields the
value of the corresponding resource in the parent.
The x and y-coordinates of the child's upper left outside
corner relative to the parent's upper left inside corner are both 0 (zero)
minus the value of`XmNborderWidth`.

MenuShell uses the`XmQTmenuSystem`trait and holds the`XmQTspecifyRenderTable`trait.
### Classes


MenuShell inherits behavior, resources, and traits from`Core`,`Composite`,`Shell`, and`OverrideShell`.

The class pointer is`xmMenuShellWidgetClass`.

The class name is`XmMenuShell`.
### New Resources


MenuShell overrides the`XmNallowShellResize`resource in Shell.
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

`XmMenuShell Resource Set``Name``Class``Type``Default``Access`XmNbuttonFontListXmCButtonFontListXmFontListdynamicCSGXmNbuttonRenderTableXmCButtonRenderTableXmRenderTableNULLCSGXmNdefaultFontListXmCDefaultFontListXmFontListdynamicCGXmNlabelFontListXmCLabelFontListXmFontListdynamicCSGXmNlabelRenderTableXmCLabelRenderTableXmRenderTableNULLCSGXmNlayoutDirectionXmCLayoutDirectionXmDirectionXmLEFT_TO_RIGHTCG

* **`XmNbuttonFontList`** 

Specifies the font list used for button descendants. See the`XmNbuttonRenderTable`resource.
* **`XmNbuttonRenderTable`** 

Specifies the render table used for MenuShell's button descendants.
If this value is NULL at initialization and if the value of`XmNdefaultFontList`is not NULL,`XmNbuttonRenderTable`is initialized to the value of`XmNdefaultFontList`. If
the value of`XmNdefaultFontList`is NULL,`XmNbuttonRenderTable`is initialized by looking up the parent hierarchy of the widget for
an ancestor that holds the`XmQTspecifyRenderTable`trait.
If such an ancestor is found,`XmNbuttonRenderTable`is initialized to the`XmBUTTON_RENDER_TABLE`value
of the ancestor widget. If no such ancestor is found, the default
is implementation dependent.
Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNdefaultFontList`** 

Specifies a default font list for MenuShell's descendants.
This resource is obsolete and exists for compatibility with
earlier releases. It has been replaced by`XmNbuttonFontList`and`XmNlabelFontList`.
* **`XmNlabelFontList`** 

Specifies the font list used for label descendants. See the`XmNlabelRenderTable`resource.
* **`XmNlabelRenderTable`** 

Specifies the render table used for MenuShell's label descendants
(Labels and LabelGadgets). If this value is NULL at initialization
and if the value of`XmNdefaultFontList`is not NULL,`XmNlabelRenderTable`is initialized to the value of`XmNdefaultFontList`. If the value of`XmNdefaultFontList`is
NULL, the parent hierarchy of the widget is searched
for an ancestor that holds the`XmQTspecifyRenderTable`trait.
If such
an ancestor is found,`XmNlabelRenderTable`is initialized to the`XmLABEL_RENDER_TABLE`value of the ancestor widget. If no such ancestor
is found, the default is implementation dependent. Refer to
&cdeman.XmRenderTable; for more information on the creation and structure
of a render table.
* **`XmNlayoutDirection`** 

Specifies the direction in which the subwidgets, children of a
widget, or other visual components are to be laid out. This policy
will apply as the default layout policy for all descendants of this
MenuShell.

### Inherited Resources


MenuShell inherits behavior and resources from the
superclasses described in the following tables.
For a complete description of each resource, refer to the
reference page for that superclass.
The programmer can set the resource values for these
inherited classes to set attributes for this widget. To reference a
resource by name or by class in a.Xdefaultsfile, remove the`XmN`or`XmC`prefix and use the remaining letters. To specify one of the defined
values for a resource in a.Xdefaultsfile, remove the`Xm`prefix and use
the remaining letters (in either lowercase or uppercase, but include any
underscores between words).
The codes in the access column indicate if the given resource can be
set at creation time (C),
set by using`XtSetValues`(S),
retrieved by using`XtGetValues`(G),
or is not applicable (N/A).

`Shell Resource Set``Name``Class``Type``Default``Access`XmNallowShellResizeXmCAllowShellResizeBooleanTrueGXmNcreatePopupChildProcXmCCreatePopupChildProcXtCreatePopupChildProcNULLCSGXmNgeometryXmCGeometryStringNULLCSGXmNoverrideRedirectXmCOverrideRedirectBooleanTrueCSGXmNpopdownCallbackXmCCallbackXtCallbackListNULLCXmNpopupCallbackXmCCallbackXtCallbackListNULLCXmNsaveUnderXmCSaveUnderBooleanTrueCSGXmNvisualXmCVisualVisual *CopyFromParentCSG

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension0CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


The XmMenuShell translations are described in the following list.

The following key names are listed in the
X standard key event translation table syntax.
This format is the one used by Motif to
specify the widget actions corresponding to a given key.
A brief overview of the format is provided under
&cdeman.VirtualBindings;.
For a complete description of the format, please refer to the
X Toolkit Instrinsics Documentation.

* **BtnDown:** 

ClearTraversal()
* **BtnUp:** 

MenuShellPopdownDone()

### Action Routines


The`XmMenuShell`action routines are

* **ClearTraversal():** 

Disables keyboard traversal for the menu, enables mouse traversal, and
unposts any menus posted by this menu.
* **MenuShellPopdownDone():** 

Unposts the menu hierarchy and, when the shell's keyboard focus policy is`XmEXPLICIT`,
restores focus to the widget that had
the focus before the menu system was entered.
* **MenuShellPopdownOne():** 

In a top-level Pulldown MenuPane from a MenuBar, this action unposts the menu,
disarms the MenuBar CascadeButton and the MenuBar, and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard
focus to the widget that had the focus before the MenuBar was
entered.
In other Pulldown MenuPanes, this action unposts the menu.

In a Popup MenuPane, this action unposts the menu, and,
when the shell's keyboard focus policy is`XmEXPLICIT`,
restores keyboard focus to the
widget from which the menu was posted.

### Virtual Bindings


The bindings for virtual keys are vendor specific.
For information about bindings for virtual buttons and keys, see &cdeman.VirtualBindings;.
## RELATED


&cdeman.Composite;, &cdeman.Core;, &cdeman.OverrideShell;, &cdeman.Shell;,
&cdeman.XmCreateMenuShell;,
&cdeman.XmCreatePopupMenu;, &cdeman.XmCreatePulldownMenu;, and
&cdeman.XmRowColumn;.