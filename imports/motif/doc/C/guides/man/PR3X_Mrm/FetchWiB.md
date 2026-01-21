# MrmFetchWidgetOverride
library call`MrmFetchWidgetOverride`Fetches any indexed (UIL named) application widget.  It overrides the arguments specified for this application widget in UILMrmFetchWidgetOverrideuil functionsMrmFetchWidgetOverride#include <Mrm/MrmPublic.h>Cardinal`MrmFetchWidgetOverride`MrmHierarchyhierarchy_idStringindexWidgetparent_widgetStringoverride_nameArgListoverride_argsCardinaloverride_num_argsWidget *widgetMrmType *classMRM functionMrmFetchWidgetOverrideMrmFetchWidgetOverridedefinition
## DESCRIPTION


The`MrmFetchWidgetOverride`functionMrmFetchWidgetOverridedescriptionis the extended version of`MrmFetchWidget`.
It is identical to`MrmFetchWidget`, except that it
allows the caller to override the widget's name and any
arguments that`MrmFetchWidget`would otherwise retrieve from the UID file or
one of the defaulting mechanisms.
That is, the override argument list is not
limited to those arguments in the UID file.

The override arguments apply only to the widget fetched and
returned
by this function.
Its children (subtree) do not receive any
override
parameters.

* **`hierarchy_id`** 

Specifies the ID of the UID hierarchy that contains
the interface definition.
The value of`hierarchy_id`was returned in a previous call to`MrmOpenHierarchyPerDisplay`.
* **`index`** 

Specifies the UIL name of the widget to fetch.
* **`parent_widget`** 

Specifies the parent widget ID.
* **`override_name`** 

Specifies the name to override the widget name.
Use a NULL value if you do not want to override the widget name.
* **`override_args`** 

Specifies the override argument list, exactly as given to`XtCreateWidget`(conversion complete and so forth).
Use a
NULL value if you do not want to override the argument list.
* **`override_num_args`** 

Specifies the number of arguments in`override_args`.
* **`widget`** 

Returns the widget ID of the created widget.
* **`class`** 

Returns the class code identifying MRM's widget class.
Literals identifying MRM widget class codes are defined in
the include fileMrm/MrmPublic.h.

## RETURN VALUE
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
&cdeman.MrmFetchWidget;.