# XmRenditionRetrieve
library call`XmRenditionRetrieve`A convenience function that retrieves rendition resourcesXmRenditionRetrieve#include <Xm/Xm.h>void`XmRenditionRetrieve`XmRenditionrenditionArgListarglistCardinalargcount
## DESCRIPTION


`XmRenditionRetrieve`extracts values for the given resources
(`arglist`) from the specified rendition.
Note that the function returns the actual values of the resources, not
copies. Therefore it is necessary to copy before modifying any
resource whose value is an address. This will include such resources
as`XmNfontName`,`XmNfont`, and`XmNtabList`.

* **`rendition`** 

Specifies the rendition.
* **`arglist`** 

Specifies the argument list.
* **`argcount`** 

Specifies the number of attribute/value pairs in the argument list
(`arglist`).

## RELATED


&cdeman.XmRendition;
and
&cdeman.XmTabListCopy;.