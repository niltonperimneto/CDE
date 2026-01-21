# XmScaleSetTicks
library call`XmScaleSetTicks`A Scale function that controls tick marksXmScaleSetTicksScale functionsXmScaleSetTicks#include <Xm/Scale.h>void`XmScaleSetTicks`Widgetscaleintbig_everyCardinalnum_mediumCardinalnum_smallDimensionsize_bigDimensionsize_mediumDimensionsize_small
## DESCRIPTION


`XmScaleSetTicks`controls the number, location, and size of the
tick marks on a Scale. Each tick mark is a SeparatorGadget oriented
perpendicular to the Scale's orientation. For example, if the Scale
is oriented horizontally, the tick marks will be oriented vertically.

If you specify tick marks for a Scale and then change the
Scale's orientation, you will have to do the following:

Remove all the tick marks. To remove tick marks from a Scale,
you must destroy (with`XtDestroyChildren`)
the SeparatorGadget tick marks.
The first two children of a Scale are its title and scroll bar, and
all additional children are tick marks.

Recreate the tick marks by calling`XmScaleSetTicks`.

* **`scale`** 

Specifies the Scale widget ID that is getting the tick marks.
* **`big_every`** 

Specifies the number of scale values between big ticks.
* **`num_medium`** 

Specifies the number of medium ticks between big values.
* **`num_small`** 

Specifies the number of small ticks between medium values.
* **`size_big`** 

Specifies the size (either width or height) of the big ticks.
* **`size_medium`** 

Specifies the size (either width or height) of the medium ticks.
* **`size_small`** 

Specifies the size (either width or height) of the small ticks.


For a complete definition of Scale and its associated resources, see
&cdeman.XmScale;.
## RELATED


&cdeman.XmScale;.