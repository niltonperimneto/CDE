# MrmFetchSetValues
library call`MrmFetchSetValues`Fetches the values to be set from literals stored in
UID filesMrmFetchSetValuesuil functionsMrmFetchSetValues#include <Mrm/MrmPublic.h>Cardinal`MrmFetchSetValues`MrmHierarchyhierarchy_idWidgetwidgetArgListargsCardinalnum_argsMRM functionMrmFetchSetValuesMrmFetchSetValuesdefinition
## DESCRIPTION


The`MrmFetchSetValues`function
is similar to`XtSetValues`,
except that the values to be set are defined by the UIL named
values that are stored in the UID hierarchy.MrmFetchSetValuesdescription`MrmFetchSetValues`fetches the values to be set from literals stored in UID files.

* **`hierarchy_id`** 

Specifies the ID of the UID hierarchy that contains the
specified literal.
The value of`hierarchy_id`was returned in a previous call to`MrmOpenHierarchyPerDisplay`.
* **`widget`** 

Specifies the widget that is modified.
* **`args`** 

Specifies an argument list that identifies the widget arguments to be
modified as well as the index (UIL name) of the literal that
defines the value for that argument.
The name part of each argument (`args[n].name`) must begin with the string`XmN`followed by the name that uniquely identifies this attribute tag.
For example,`XmNwidth`is the attribute name associated with the core argument`width`.
The value part (`args[n].value`) must be a string
that gives the index (UIL name) of the literal.
You must define all literals in UIL as exported values.
* **`num_args`** 

Specifies the number of entries in`args`.


This function
sets the values
on a widget, evaluating the
values as public literal resource references resolvable from
a UID hierarchy.
Each literal is fetched from the hierarchy, and
its value is modified and converted as required.
This value is
then placed in the argument list and used as the actual value for an`XtSetValues`call.`MrmFetchSetValues`allows a widget to be modified
after creation using UID file values the same way
creation values are used in`MrmFetchWidget`.

As in`MrmFetchWidget`,
each argument whose value can be evaluated
from
the UID hierarchy is set in the widget.
Values that are not
found
or values in which conversion errors occur are not modified.

Each entry in the argument list identifies an argument to be modified
in the widget.
The name part identifies the tag, which begins with`XmN`.
The value part must be a string
whose value is the index of
the literal.
Thus, the following code would modify the label resource of the widget
to have the value of the literal accessed by the index`OK_button_label`in the hierarchy:args[n].name = XmNlabel;
args[n].value = "OK_button_label";
## RETURN VALUE
MrmSUCCESSMrmPARTIAL_SUCCESSMrmBAD_HIERARCHYMrmFAILURE

This function returns one of the following status return constants:

* **`MrmSUCCESS`** 

The function executed successfully.
* **`MrmPARTIAL_SUCCESS`** 

At least one literal was successfully fetched.
* **`MrmBAD_HIERARCHY`** 

The hierarchy ID was invalid.
* **`MrmFAILURE`** 

The function failed.

## RELATED


&cdeman.MrmOpenHierarchyPerDisplay;,XtSetValues(3).