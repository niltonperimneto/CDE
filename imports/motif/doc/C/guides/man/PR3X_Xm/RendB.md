# XmRenditionCreate
library call`XmRenditionCreate`A convenience function that creates a renditionXmRenditionCreate#include <Xm/Xm.h>XmRendition`XmRenditionCreate`WidgetwidgetXmStringTagtagArgListarglistCardinalargcount
## DESCRIPTION


`XmRenditionCreate`creates a rendition whose resources are set
to the values specified in`arglist`. Default values are assigned
to resources that are not specified.

* **`widget`** 

Specifies the widget used for deriving any necessary information for
creating the rendition. In particular, the X display of`widget`will be used for loading fonts.
* **tag** 

Specifies the tag for the rendition.
(This will become the`XmNtag`resource for the rendition.)
* **`arglist`** 

Specifies the argument list.
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`).

## RETURN


Returns the created rendition.
The function allocates space to hold the returned rendition.
The application is responsible for managing this allocated space.
The application can recover this allocated space by calling`XmRenditionFree`.
## RELATED


&cdeman.XmRendition; and
&cdeman.XmRenditionFree;.