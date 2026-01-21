# XmGetSecondaryResourceData
library call`XmGetSecondaryResourceData`A function that provides access to secondary widget resource dataXmGetSecondaryResourceData#include <Xm/Xm.h>Cardinal`XmGetSecondaryResourceData`WidgetClasswidget_classXmSecondaryResourceData **secondary_data_return
## DESCRIPTION


Some Motif widget classes (such as Gadget, Text, and VendorShell) have
resources that are not accessible through the functions`XtGetResourceList`and`XtGetConstraintResourceList`.
In order to retrieve the descriptions of these resources, an application
must use`XmGetSecondaryResourceData`.

When a widget class has such resources, this function provides
descriptions of the resources in one or more data structures.`XmGetSecondaryResourceData`takes a widget class argument and
returns the number of these data structures associated with the widget
class.
If the return value is greater than 0 (zero), the function allocates and fills
an array of pointers to the corresponding data structures.
It returns this array at the address that is the value of the`secondary_data_return`argument.

The typeXmSecondaryResourceDatais a pointer to a structure with
two members that are useful to an application:`resources`, of type`XtResourceList`, and`num_resources`, of typeCardinal.
The`resources`member is a list of the widget resources that are
not accessible using Xt functions.
The`num_resources`member is the length of the`resources`list.

If the return value is greater than 0 (zero),`XmGetSecondaryResourceData`allocates memory that the application must free.
Use`XtFree`to free the resource list in each structure (the value
of the`resources`member), the structures themselves, and the array
of pointers to the structures (the array whose address is`secondary_data_return`).

* **`widget_class`** 

Specifies the widget class for which secondary resource data is to be
retrieved.
* **`secondary_data_return`** 

Specifies a pointer to an array ofXmSecondaryResourceDatapointers to be returned by this function.
If the widget class has no secondary resource data, for example, if the value
returned by the function is 0 (zero), the function returns no meaningful value
for this argument.

## RETURN


Returns the number of secondary resource data structures associated with
this widget class.
## EXAMPLE


The following example uses`XmGetSecondaryResourceData`to print the
names of the secondary resources of the Motif Text widget and then frees
the data allocated by the function:XmSecondaryResourceData * block_array;
Cardinal num_blocks, i, j;
if (num_blocks = XmGetSecondaryResourceData (xmTextWidgetClass,
                                             &amp;block_array)) {
  for (i = 0; i &lt; num_blocks; i++) {
    for (j = 0; j &lt; block_array[i]->num_resources; j++) {
      printf("%s&bsol;n", block_array[i]->resources[j].resource_name);
    }
    XtFree((char*)block_array[i]->resources);
    XtFree((char*)block_array[i]);
  }
  XtFree((char*)block_array);
}