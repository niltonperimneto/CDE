# XmScrollBarGetValues
library call`XmScrollBarGetValues`A ScrollBar function that returns the ScrollBar's increment valuesXmScrollBarGetValuesScrollBar functionsXmScrollBarGetValues#include <Xm/ScrollBar.h>
void XmScrollBarGetValues (widget, value_return, slider_size_return,
increment_return, page_increment_return)
        Widgetwidget;
        int* value_return;
        int* slider_size_return;
        int* increment_return;
        int* page_increment_return;
## DESCRIPTION


`XmScrollBarGetValues`returns the
the ScrollBar's increment values.
The scroll region is overlaid with a
slider bar that is adjusted in size and position using the main ScrollBar or
set slider function attributes.

* **`widget`** 

Specifies the ScrollBar widget ID.
* **`value_return`** 

Returns the ScrollBar's slider position between the`XmNminimum`and`XmNmaximum`resources.
Specify NULL to prevent the return of a particular value.
* **`slider_size_return`** 

Returns the size of the slider as a value between 0 (zero) and the absolute
value of`XmNmaximum`minus`XmNminimum`. The size of the slider
varies, depending on how much of the slider scroll area it represents.
* **`increment_return`** 

Returns the amount of increment and decrement.
* **`page_increment_return`** 

Returns the amount of page increment and decrement.


For a complete definition of ScrollBar and its associated resources, see
&cdeman.XmScrollBar;.
## RETURN


Returns the ScrollBar's increment values.
## RELATED


&cdeman.XmScrollBar;.