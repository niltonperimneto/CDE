# XmImCloseXIM
library call`XmImCloseXIM`An input manager function that releases the input
method associated with a specified widgetXmImCloseXIMinput manager functionsXmImCloseXIM#include <Xm/XmIm.h>void`XmImCloseXIM`Widgetwidget
## DESCRIPTION


`XmImCloseXIM`closes all input contexts associated with the Input
Method (IM) of`widget`.`widget`is used to
identify the Display that specifies the Input Method opened for the
widget. Upon closure, all widgets registered with the input contexts
are unregistered. Also, the Input Method specified by Display is closed.

* **`widget`** 

Specifies the ID of a widget whose reference Input Method is to be closed.

## RELATED


&cdeman.XmImGetXIM; and
&cdeman.XmImRegister;.