# XmImMbResetIC
library call`XmImMbResetIC`An input manager function that resets the input context for a widget#include <Xm/XmIm.h>void`XmImMbResetIC`Widgetwidgetchar**mb
## DESCRIPTION


XmImMbResetICgets the XIC of the widget
and resets it. It puts a pointer to a string
containing the current preedit string to`mb. The caller should free the returned
string after use by callingXfree.
`widget`Specifies the ID of the widget.`mb`Contains a pointer to the preedit string upon return.
RETURN VALUENoneSEE ALSO