# XmImSetValues
library call`XmImSetValues`An input manager function that updates attributes of
an input contextXmImSetValuesinput manager functionsXmImSetValues#include <Xm/XmIm.h>void`XmImSetValues`WidgetwidgetArgListarglistCardinalargcount
## DESCRIPTION


`XmImSetValues`updates attributes of the input context associated with
the specified widget. The`arglist`argument is a list of
attribute/value pairs for the input context. This function passes the
attributes and values to`XICSetValues`. The initial call to
this routine should pass in all of the input context attributes.
Thereafter, the application programmer calls`XmImSetValues`, for
an XIC,
only if a value has changed.

If the previous parameters for the widget's XIC do not allow
the previously registered XIC to be reused, that XIC will be
unregistered, and a new one will be created and registered with the
widget. Note that sharing of data is preserved.

Note that the Text and TextField widgets call the`XmImSetValues`function when they receive focus. Therefore,
further calls to the`XmImSetValues`function for these
widgets are unnecessary.

* **`widget`** 

Specifies the ID of the widget registered with the input manager
* **`arglist`** 

Specifies the list of attribute/value pairs to be passed to`XICSetValues`;
the following attributes are accepted:`XmNpreeditStartCallback``XmNpreeditDoneCallback``XmNpreeditDrawCallback`and`XmNpreeditCaretCallback`. These attributes accept an accompanying
value of type pointer
to structure of type`XIMCallback`.

These callbacks are used only when the`XmNpreeditType`resource of the relevant`VendorShell`has the "onthespot" value,
and that the XIM supports`XIMPreeditCallbacks`input style. These
values are ignored if the condition is not met.

For each of these callbacks, if the callback value is not set by this
function, no action will be taken when the Input Method tries to call this callback.
Refer to the "Xlib - C Language X Interface, X Version 11, Release 6,"
Chapter 13 for the detail of these callbacks.
* **`argcount`** 

Specifies the number of attribute/values pairs in the argument
list (`arglist)`


Resources that can be set for the input context include:

* **`XmNbackground`** 

Specifies the pixel value for the background color.
* **`XmNbackgroundPixmap`** 

Specifies a pixmap for tiling the background.
* **`XmNfontList`** 

Specifies the font list used by the widget. The input method
uses the first occurrence of a font set tagged with`XmFONTLIST_DEFAULT_TAG`. If no such instance is
found, the first font set in the font list is used. If
the font list does not contain a font set, a value is not
passed to`XICSetValues`.
* **`XmNforeground`** 

Specifies the pixel value for the foreground color.
* **`XmNlineSpace`** 

Specifies the line spacing used in the pre-edit window.
* **`XmNrenderTable`** 

Specifies the render table used by the widget.
* **`XmNspotLocation`** 

Specifies thexandycoordinates of the position
where text will be inserted in the widget handling
input, whose input method style is`"OverTheSpot"`. Theycoordinate is the position of the baseline used by the current text line.


The caller may also pass any other vendor-defined resources to this
function. For additional information on the internationalization
interface, see the Xlib documentation.
## RELATED


&cdeman.XmImSetFocusValues;,
&cdeman.XmImVaSetFocusValues;, and
&cdeman.XmImVaSetValues;.