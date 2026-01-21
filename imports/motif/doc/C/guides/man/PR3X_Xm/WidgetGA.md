# XmWidgetGetBaselines
library call`XmWidgetGetBaselines`Retrieves baseline information for
a widgetXmWidgetGetBaselines#include <Xm/Xm.h>Boolean`XmWidgetGetBaselines`WidgetwidgetDimension **baselinesint *line_count
## DESCRIPTION


`XmWidgetGetBaselines`returns an array that
contains one or more baseline values associated with the
specified widget. The baseline of any given line of text
is a vertical offset in pixels from the origin of the
widget's bounding box to the given baseline.

* **`widget`** 

Specifies the ID of the widget for which baseline values
are requested
* **`baselines`** 

Returns an array that contains the value of each
baseline of text in the widget.
The function allocates space to hold the returned array.
The application is responsible for managing the allocated space.
The application can recover this allocated space by calling`XtFree`.
* **`line_count`** 

Returns the number of lines in the widget

## RETURN


Returns a Boolean value that indicates whether the
widget contains a baseline. If the value is True, the function returns
a value for each baseline of text. If it is False, the function
was unable to return a baseline value.
## RELATED


&cdeman.XmWidgetGetDisplayRect;.