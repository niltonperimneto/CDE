# XmMainWindowSep3
library call`XmMainWindowSep3`A MainWindow function that returns the widget ID of the third Separator widgetXmMainWindowSep3MainWindow functionsXmMainWindowSep3#include <Xm/MainW.h>Widget`XmMainWindowSep3`Widgetwidget
## DESCRIPTION


`XmMainWindowSep3`returns the widget ID of the third Separator in the
MainWindow. The third Separator is located between the
message window and the widget above it.
This Separator is visible only when`XmNshowSeparator`is True.

`NOTE:``XmMainWindowSep3`is obsolete and exists for compatibility
with previous releases. Use`XtNameToWidget`instead. Pass
a MainWindow variable as the first argument to`XtNameToWidget`and pass`Separator3`as the second argument.

* **`widget`** 

Specifies the MainWindow widget ID


For a complete definition of MainWindow and its associated resources, see
&cdeman.XmMainWindow;.
## RETURN


Returns the widget ID of the third Separator.
## RELATED


&cdeman.XmMainWindow;.