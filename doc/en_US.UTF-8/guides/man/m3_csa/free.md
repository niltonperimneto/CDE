# csa_free
library call`csa_free`free memory allocated by the calendaring service#include <xcsa.h>CSA_return_code`csa_free`CSA_buffermemory
## DESCRIPTION


The`csa_free`function frees memory allocated by the calendaring
service.
After the call, the pointer memory will be
invalid and should not be referenced again.
When any CSA
function allocates and returns a buffer to the application,
the application will free that memory with this call when
it is finished with the memory.
When a CSA function
returns a base pointer to a complex structure containing
several levels of pointers, all the application will do to
free the entire structure or array of structures is call
this routine with the base pointer returned by the CSA
function.
The CSA functions that return structures of
this form are
&cdeman.csa.list.calendars;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.free.time.search;, &cdeman.csa.add.entry;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.read.entry.attributes;, and
&cdeman.csa.read.next.reminder;, &cdeman.csa.update.entry.attributes;. The behavior of
&cdeman.csa.free; is undefined when called with a pointer to a memory block
not allocated by the calendaring service, a pointer to a
memory block that has already been freed, or a pointer
contained in a structure returned by the CSA
implementation.
In some situations, the extensions
specified for a function may be a combination of input and
output extensions.
In this case, the memory returned in
the output extensions must be freed one at a time using
&cdeman.csa.free;.
## ARGUMENTS

### Memory (Buffer)


A pointer to memory allocated by the calendaring service.
A value of`NULL`will be ignored, but will return the return code`CSA_SUCCESS`.
## RETURN VALUE


Whether the function succeeded or not, and, if not, why.
It may be success or one of the values listed under ERRORS
below.
## ERRORS


The`csa_free`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INVALID_MEMORY** 

A function parameter was invalid.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,