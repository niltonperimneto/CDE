# XmScrollBarSetValues
library call`XmScrollBarSetValues`A ScrollBar function that changes ScrollBar's increment values and the slider's size and positionXmScrollBarSetValuesScrollBar functionsXmScrollBarSetValues#include <Xm/ScrollBar.h>
void XmScrollBarSetValues (widget, value, slider_size, increment, page_increment,
notify)
        Widgetwidget;
        intvalue;
        intslider_size;
        intincrement;
        intpage_increment;
        Booleannotify;
## DESCRIPTION


`XmSetScrollBarValues`changes the ScrollBar's increment values and the
slider's size and position.
The scroll region is overlaid with a
slider bar that is adjusted in size and position using the main ScrollBar or
set slider function attributes.

* **`widget`** 

Specifies the ScrollBar widget ID.
* **`value`** 

Specifies the ScrollBar's slider
position. Refer to the`XmNvalue`resource described on
&cdeman.XmScrollBar;.
* **`slider_size`** 

Specifies the size of the
slider. Refer to the`XmNsliderSize`resource described on
&cdeman.XmScrollBar;. This argument sets that resource.
Specify a
value of 0 (zero) if you do not want to change the value.
* **`increment`** 

Specifies the amount of button increment and decrement.
Refer to the`XmNincrement`resource described on
&cdeman.XmScrollBar;. This argument sets that resource.
Specify a
value of 0 (zero) if you do not want to change the value.
* **`page_increment`** 

Specifies the amount of page increment and decrement.
Refer to the`XmNpageIncrement`resource described on
&cdeman.XmScrollBar;. This argument sets that resource.
Specify a
value of 0 (zero) if you do not want to change the value.
* **`notify`** 

Specifies a Boolean value that, when True, indicates a change in
the ScrollBar value and also specifies that the ScrollBar widget
automatically activates
the`XmNvalueChangedCallback`with the recent change.
If it is set to False,
it specifies any change that has occurred in the ScrollBar's value, but
does not activate`XmNvalueChangedCallback`.


For a complete definition of ScrollBar and its associated resources, see
&cdeman.XmScrollBar;.
## RELATED


&cdeman.XmScrollBar;.