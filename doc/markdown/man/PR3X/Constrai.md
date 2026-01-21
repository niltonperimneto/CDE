# Constraint
library call`Constraint`The Constraint widget classConstraintwidget classConstraint&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


Constraint widgets
maintain additional state data for each child. For example,
client-defined constraints on the child's geometry may be specified.

When a constrained composite widget defines constraint resources, all of that
widget's children inherit all of those resources as their own. These
constraint resources are set and read just the same as any other resources
defined for the child. This resource inheritance extends exactly one
generation down, which means only the first-generation children of a
constrained composite widget inherit the parent widget's constraint
resources.

Because constraint resources are defined by the parent widgets and not
the children, the child widgets never directly use the constraint resource
data. Instead, the parents use constraint resource data to
attach child-specific data to children.
### Classes


Constraint inherits behavior and resources from`Composite`and`Core`.

The class pointer is`constraintWidgetClass`.

The class name is`Constraint`.
### New Resources


Constraint defines no new resources.
### Inherited Resources


Constraint inherits behavior and resources from`Composite`and`Core`.
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
retrieved by using`XtGetValues`(G), or is not applicable
(N/A).

`Core Resource Set``Name``Class``Type``Default``Access`XmNacceleratorsXmCAcceleratorsXtAcceleratorsdynamicCSGXmNancestorSensitiveXmCSensitiveBooleandynamicGXmNbackgroundXmCBackgroundPixeldynamicCSGXmNbackgroundPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderColorXmCBorderColorPixelXtDefaultForegroundCSGXmNborderPixmapXmCPixmapPixmapXmUNSPECIFIED_PIXMAPCSGXmNborderWidthXmCBorderWidthDimension1CSGXmNcolormapXmCColormapColormapdynamicCGXmNdepthXmCDepthintdynamicCGXmNdestroyCallbackXmCCallbackXtCallbackListNULLCXmNheightXmCHeightDimensiondynamicCSGXmNinitialResourcesPersistentXmCInitialResourcesPersistentBooleanTrueCXmNmappedWhenManagedXmCMappedWhenManagedBooleanTrueCSGXmNscreenXmCScreenScreen *dynamicCGXmNsensitiveXmCSensitiveBooleanTrueCSGXmNtranslationsXmCTranslationsXtTranslationsdynamicCSGXmNwidthXmCWidthDimensiondynamicCSGXmNxXmCPositionPosition0CSGXmNyXmCPositionPosition0CSG
### Translations


There are no translations for Constraint.
## RELATED


&cdeman.Composite; and &cdeman.Core;.