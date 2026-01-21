# XmNotebookGetPageInfo
library call`XmNotebookGetPageInfo`A Notebook function that returns page informationXmNotebookGetPageInfoNotebook functionsXmNotebookGetPageInfo#include <Xm/Notebook.h>XmNotebookPageStatus`XmNotebookGetPageInfo`Widgetnotebookintpage_numberXmNotebookPageInfo*page_info
## DESCRIPTION


`XmNotebookGetPageInfo`returns status information for the specified
Notebook page.

* **`notebook`** 

Specifies the Notebook widget.
* **`page_number`** 

Specifies the page number to be queried.
* **`page_info`** 

Points to the structure containing the page information. The structure
has the following form:typedef struct
{
        intpage_number;
        Widgetpage_widget;
        Widgetstatus_area_widget;
        Widgetmajor_tab_widget;
        Widgetminor_tab_widget;
} XmNotebookPageInfo;

* **`page_number`** 

Specifies the`page_number`passed to the function.
* **`page_widget`** 

Specifies a child widget of the Notebook with a`XmNchildType`of`XmPAGE`and a`XmNpageNumber`equal to`page_number`if one
exists; otherwise set to NULL.
* **`status_area_widget`** 

Specifies a child widget of the Notebook with a`XmNchildType`of`XmSTATUS_AREA`and a`XmNpageNumber`equal to`page_number`if one
exists; otherwise set to NULL.
* **`major_tab_widget`** 

Specifies a child widget of the Notebook with a`XmNchildType`of`XmMAJOR_TAB`and the nearest`XmNpageNumber`equal to or less than`page_number`if one
exists; otherwise set to NULL.
* **`minor_tab_widget`** 

Specifies a child widget of the Notebook with a`XmNchildType`of`XmMINOR_TAB`and the nearest`XmNpageNumber`equal to or less than`page_number`if one
exists; otherwise set to NULL.



For a complete definition of Notebook and its associated resources, see
&cdeman.XmNotebook;.
## RETURN


Returns one of the following page status values:

* **`XmPAGE_FOUND`** 

The specified page was found.
* **`XmPAGE_INVALID`** 

The specified page number is out of the page number range.
* **`XmPAGE_EMPTY`** 

The specified page does not have a page widget.
* **`XmPAGE_DUPLICATED`** 

There is more than one page widget with the specified page number. The
more recently managed page widget is used for the page information structure.

## RELATED


&cdeman.XmNotebook;.