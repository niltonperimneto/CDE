# XmImGetXIM
library call`XmImGetXIM`An input manager function that retrieves the input
method associated with a specified widgetXmImGetXIMinput manager functionsXmImGetXIM#include <Xm/XmIm.h>XIM`XmImGetXIM`Widgetwidget
## DESCRIPTION


`XmImGetXIM`retrieves the XIM data structure representing
the input method that the input manager has opened for the
specified widget. If an input method has not been opened
by a previous call to`XmImRegister`, the first time this
routine is called it opens an input method using the`XmNinputMethod`resource for the VendorShell. If the`XmNinputMethod`is NULL, an input method is opened using the
current locale.
If it cannot open an input method, the
function returns NULL.

* **`widget`** 

Specifies the ID of a widget registered with the input manager

## RETURN


Returns the input method for the current locale associated with
the specified widget's input manager; otherwise, returns NULL.
The application is responsible for freeing the returned XIM by calling`XmImCloseXIM`.
## RELATED


&cdeman.XmImCloseXIM;,
&cdeman.XmImGetXIM;,
&cdeman.XmImMbLookupString;, and
&cdeman.XmImRegister;.