# Composite
library call`Composite`The Composite widget classCompositewidget classComposite&npzwc;&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


Composite widgets are intended to be containers for other widgets and can
have an arbitrary number of children. Their responsibilities (implemented
either directly by the widget class or indirectly by Intrinsics
functions) include:

Overall management of children from creation to destruction.

Destruction of descendants when the composite widget is destroyed.

Physical arrangement (geometry management) of a displayable subset of
managed children.

Mapping and unmapping of a subset of the managed children.
Instances of composite widgets need to specify the order in which
their children are kept. For example, an application may want a set of
command buttons in some logical order grouped by function, and it may want
buttons that represent filenames to be kept in alphabetical order.
### Classes


Composite inherits behavior and resources from`Core`.

The class pointer is`compositeWidgetClass`.

The class name is`Composite`.
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

`Composite Resource Set``Name``Class``Type``Default``Access`XmNchildrenXmCReadOnlyWidgetListNULLGXmNinsertPositionXmCInsertPositionXtOrderProcNULLCSGXmNnumChildrenXmCReadOnlyCardinal0G

* **`XmNchildren`** 

A read-only list of the children of the widget.
* **`XmNinsertPosition`** 

Points to the`XtOrderProc`function described below.
* **`XmNnumChildren`** 

A read-only resource specifying the length of the list of children in`XmNchildren`.


The following procedure pointer in a composite widget instance is of type`XtOrderProc`:Cardinal (* XtOrderProc) (Widget`w`);

* **`w`** 

Specifies the widget.


Composite widgets that allow clients to order their children (usually
homogeneous boxes) can call their widget instance's`XmNinsertPosition`procedure from the class's`insert_child`procedure to determine
where a new
child should go in its children array. Thus, a client of a composite class
can apply different sorting criteria to widget instances of the class,
passing in a different`XmNinsertPosition`procedure when it creates each
composite widget instance.

The return value of the`XmNinsertPosition`procedure indicates
how many children
should go before the widget. A value of 0 (zero) indicates that the
widget should go before all other children; returning
the value of`XmNumChildren`indicates
that it should go after all other children.
By default, unless a subclass or an application provides an`XmNinsertPosition`procedure, each child is inserted at the end
of the`XmNchildren`list.
The`XmNinsertPosition`procedure can be overridden by a specific
composite
widget's resource list or by the argument list provided when the composite
widget is created.
### Inherited Resources


Composite inherits behavior and resources from the superclass described in
the following table.
For a complete description of each resource, refer to the reference page for
that superclass.

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension1CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for Composite.
## RELATED


&cdeman.Core;.