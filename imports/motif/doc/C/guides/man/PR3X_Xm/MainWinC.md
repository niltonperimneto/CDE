# XmMainWindowSep2
library call`XmMainWindowSep2`A MainWindow function that returns the widget ID of the second Separator widgetXmMainWindowSep2MainWindow functionsXmMainWindowSep2#include <Xm/MainW.h>Widget`XmMainWindowSep2`Widgetwidget
## DESCRIPTION


`XmMainWindowSep2`returns the widget ID of the second Separator in the
MainWindow. The second Separator is located between the
Command widget and the ScrolledWindow.
This Separator is visible only when`XmNshowSeparator`is True.

`NOTE:``XmMainWindowSep2`is obsolete and exists for compatibility
with previous releases. Use`XtNameToWidget`instead. Pass
a MainWindow variable as the first argument to`XtNameToWidget`and pass`Separator2`as the second argument.

* **`widget`** 

Specifies the MainWindow widget ID


For a complete definition of MainWindow and its associated resources, see
&cdeman.XmMainWindow;.
## RETURN


Returns the widget ID of the second Separator.
## RELATED


&cdeman.XmMainWindow;.