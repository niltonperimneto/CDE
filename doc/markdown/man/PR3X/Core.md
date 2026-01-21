# Core
library call`Core`The Core widget classCorewidget classCore&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


Core is the Xt Intrinsic base class for windowed widgets. The`Object`and`RectObj`classes provide support for windowless
widgets.
### Classes


All widgets are built from`Core`.

The class pointer is`widgetClass`.

The class name is`Core`.
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

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension1CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG

* **`XmNaccelerators`** 

Specifies a translation table that is bound with its actions in the context
of a particular widget. The accelerator table can then be installed on some
destination widget.
Note that the default accelerators for any widget will always be
installed, no matter whether this resource is specified or not.
* **`XmNancestorSensitive`** 

Specifies whether the immediate parent of the widget receives input
events. Use the function`XtSetSensitive`to change the argument
to preserve data integrity (see`XmNsensitive`).
For shells, the default is copied from the parent's`XmNancestorSensitive`resource if there is a parent; otherwise, it
is True.
For other widgets, the default is the bitwise AND of the parent's`XmNsensitive`and`XmNancestorSensitive`resources.
* **`XmNbackground`** 

Specifies the background color for the widget.
* **`XmNbackgroundPixmap`** 

Specifies a pixmap for tiling the background. The first tile is
placed at the upper left corner of the widget's window.
* **`XmNborderColor`** 

Specifies the color of the border in a pixel value.
* **`XmNborderPixmap`** 

Specifies a pixmap to be used for tiling the border. The first tile is
placed at the upper left corner of the border.
* **`XmNborderWidth`** 

Specifies the width of the border that surrounds the widget's window on all
four sides. The width is specified in pixels. A width of 0 (zero) means that
no border shows.
Note that you should use resources like`XmNshadowThickness`and`XmNhighlightThickness`instead of`XmNborderWidth`to specify
border widths.
* **`XmNcolormap`** 

Specifies the colormap that is used for conversions to the type`Pixel`for this widget instance. When this resource
is changed, previously
generated pixel values are not affected, but newly generated
values are in the new colormap.
For shells without parents, the default is the default colormap of the
widget's screen.
Otherwise, the default is copied from the parent.
* **`XmNdepth`** 

Specifies the number of bits that can be used for each pixel in the
widget's window. Applications should not change or set the value of this
resource as it is set by the Xt Intrinsics when the widget is created.
For shells without parents, the default is the default depth of the
widget's screen.
Otherwise, the default is copied from the parent.
* **`XmNdestroyCallback`** 

Specifies a list of callbacks that is called when the widget
is destroyed.
* **`XmNheight`** 

Specifies the inside height (excluding the border) of the widget's
window.
* **`XmNinitialResourcesPersistent`** 

Specifies whether or not resources are reference counted.
If the value is True when the widget is created, the resources
referenced by the widget are not reference counted, regardless of how
the resource type converter is registered.
An application that expects to destroy the widget and wants to have
resources deallocated should specify a value of False.
The default is True, implying an assumption that the widget will not be
destroyed during the life of the application.
* **`XmNmappedWhenManaged`** 

If this resource is set to True,
it maps the widget (makes it visible) as soon as it is both
realized and managed.
If this resource is set to False, the client is responsible for
mapping and unmapping the widget. If the value is changed from True to
False after the widget has been realized and managed, the widget is
unmapped.
* **`XmNscreen`** 

Specifies the screen on which a widget instance resides.
It is read only.
When the Toolkit is initialized, the top-level widget obtains its
default value from the default screen of the display.
Otherwise, the default is copied from the parent.
* **`XmNsensitive`** 

Determines whether a widget receives input events. If a widget is
sensitive, the Xt Intrinsics' Event Manager dispatches to the widget all
keyboard, mouse button, motion, window enter/leave, and focus events.
Insensitive widgets do not receive these events. Use the function`XtSetSensitive`to change the sensitivity argument. Using`XtSetSensitive`ensures that if a parent widget has`XmNsensitive`set to False, the ancestor-sensitive flag of all its
children is appropriately set.
* **`XmNtranslations`** 

Points to a translations list. A translations list is a list of events and
actions that are to be performed when the events occur.
Note that the default translations for any widget will always be
installed, no matter whether this resource is specified or not.
* **`XmNwidth`** 

Specifies the inside width (excluding the border) of the widget's
window.
* **`XmNx`** 

Specifies the x-coordinate of the upper left outside corner of the
widget's window.
The value is relative to the upper left inside corner of the parent
window.
* **`XmNy`** 

Specifies the y-coordinate of the upper left outside corner of the
widget's window.
The value is relative to the upper left inside corner of the parent
window.

### Translations


There are no translations for Core.
## RELATED


&cdeman.Object;
and
&cdeman.RectObj;.