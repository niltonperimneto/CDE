# XmImMbLookupString
library call`XmImMbLookupString`An input manager function that retrieves a composed string from an input methodXmImMbLookupStringinput manager functionsXmImMbLookupString#include <Xm/XmIm.h>int`XmImMbLookupString`WidgetwidgetXKeyPressedEvent *eventchar *buffer_returnintbytes_bufferKeySym *keysym_returnint *status_return
## DESCRIPTION


`XmImMbLookupString`returns a string composed in the
locale associated with the widget's input method and a
KeySym that is currently mapped to the keycode in a KeyPress
event. The KeySym is obtained by using the standard
interpretation of Shift, Lock and Group modifiers as
defined in the X Protocol specification.

An XIM will be created, but an XIC will not be created. One of the functions,`XmImSetValues`,`XmImVaSetValues`, or`XmImGetXIC`,
needs to be called to create an XIC.

* **`widget`** 

Specifies the ID of the widget registered with the input manager
* **`event`** 

Specifies the key press event
* **`buffer_return`** 

Specifies the buffer in which the string is returned
* **`bytes_buffer`** 

Specifies the size of the buffer in bytes
* **`keysym_return`** 

Specifies a pointer to the KeySym returned if one exists
* **`status_return`** 

Specifies the status values returned by the function. These status
values are the same as those for the`XmbLookupString`function. The
possible status values are:

* **`XBufferOverflow`** 

The size of the buffer was insufficient to handle
the returned string. The contents of`buffer_return`and`keysym_return`are not modified. The required
buffer size is returned as a value of the function. The
client should repeat the call with a larger buffer size to
receive the string.
* **`XLookupNone`** 

No consistent input was composed. The contents of`buffer_return`and`keysym_return`are not modified
and the function returns a value of 0.
* **`XLookupChars`** 

Some input characters were composed and returned in`buffer_return`. The content of`keysym_return`is not modified. The function returns the length of
the string in bytes.
* **`XLookupKeysym`** 

A keysym value was returned instead of a string. The content of`buffer_return`is not modified and the function returns
a value of 0.
* **`XLookupBoth`** 

A keysym value and a string were returned. The keysym value may
not necessarily correspond to the string returned. The function
returns the length of the string in bytes.


## RETURN


Return values depend on the status returned by the function. Refer
to the description of status values above.
## RELATED


&cdeman.XmImGetXIM;,
&cdeman.XmImGetXIC;,
&cdeman.XmImRegister;,
&cdeman.XmImSetValues;,
and
&cdeman.XmImUnregister;.