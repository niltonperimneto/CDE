# MrmFetchWidget
library call`MrmFetchWidget`Fetches and creates an indexed (UIL named) application widget and its childrenMrmFetchWidgetuil functionsMrmFetchWidget#include <Mrm/MrmPublic.h>Cardinal`MrmFetchWidget`MrmHierarchyhierarchy_idStringindexWidgetparent_widgetWidget *widgetMrmType *classMRM functionMrmFetchWidgetMrmFetchWidgetdefinition
## DESCRIPTION


The`MrmFetchWidget`functionMrmFetchWidgetdescriptionfetches and creates an
indexed application widget and its children.
The indexed application widget is any widget that is named in UIL.
In
fetch operations, the fetched widget's subtree is also
fetched and created.
This widget must not appear as the child of a widget within its own
subtree.`MrmFetchWidget`does not execute`XtManageChild`for the newly created widget.

All widgets fetched by a call to`MrmFetchWidget`are not managed
at the time of their creation callbacks.

* **`hierarchy_id`** 

Specifies the ID of the`UID`hierarchy that contains the
interface definition.
The value of`hierarchy_id`was returned in a previous call to`MrmOpenHierarchyPerDisplay`.
* **`index`** 

Specifies the UIL name of the widget to fetch.
* **`parent_widget`** 

Specifies the parent widget ID.
* **`widget`** 

Returns the widget ID of the created widget.
* **`class`** 

This argument must be set to an actual pointer; it
cannot be a NULL pointer.`MrmFetchWidget`sets this argument to
an implementation dependent value.


An application can fetch
any named widget in the`UID`hierarchy using`MrmFetchWidget`.`MrmFetchWidget`can be called at any time to fetch a widget that was not fetched at
application startup.`MrmFetchWidget`can be used to defer fetching pop-up
widgets until they are first
referenced (presumably in a callback), and then used to fetch them
once.

`MrmFetchWidget`can also create multiple instances of a widget (and its subtree).
In this case, the`UID`definition functions as a template;
a widget definition can be fetched any number of times.
An application can use
this template to make multiple instances of a widget, for example, in a
dialog box box or menu.

The index (UIL name) that identifies the widget must be
known to the application.
## RETURN
MrmSUCCESSMrmBAD_HIERARCHYMrmNOT_FOUNDMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmBAD_HIERARCHY`** 

The hierarchy ID was invalid.
* **`MrmNOT_FOUND`** 

The widget was not found in UID hierarchy.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmOpenHierarchyPerDisplay;,
&cdeman.MrmFetchWidgetOverride;.