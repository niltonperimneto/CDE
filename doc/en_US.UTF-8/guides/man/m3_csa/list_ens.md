# csa_list_entry_sequence
library call`csa_list_entry_sequence`lists the recurring calendar entries that are associated with a calendar entry#include <xcsa.h>CSA_return_code`csa_list_entry_sequence`CSA_session_handlesessionCSA_entry_handleentryCSA_date_time_rangetime_rangeCSA_uint32 *number_entriesCSA_entry_handle **entry_listCSA_extension *list_entry_sequence_extensions
## DESCRIPTION


The`csa_list_entry_sequence`function returns an array of the entry handles for the
recurring entries associated with a specific calendar
entry.

The entry handles for the recurring calendar entries are
returned inentry_list.A`NULL`is returned if no recurring entries are associated with
this calendar entry.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Entry (Entry Handle)


Specifies the handle of the calendar entry to be accessed
for a list of associated recurring entries.
If the entry
handle is invalid, then the error`CSA_E_INVALID_ENTRY_HANDLE`is returned.
### Time Range (Date Time Range)


The date and time range to be used to filter or restrict
the listing of the entry sequence.
The date and time range
specificies a begin and end date and time.
If`NULL`, then all entries in the sequence will be returned.
If the
sequence is of an indefinite repitition, then the function
will return implementation specific results.
### List Entry Sequence Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Number Entries (Uint32)


The number of calendar entry handles actually returned.
### Entry List (Entry Handle)


A pointer to an array of calendar entry handles that
represent the recurring entries associated with the
specified calendar entry.
This array is allocated by the
service, and the entire array should be freed with a single
call to
&cdeman.csa.free;.
### List Entry Sequence Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_list_entry_sequence`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_DATE_TIME** 

An invalid date and time combination was specified.
* **CSA_E_INVALID_ENTRY_HANDLE** 

An invalid calendar entry handle was specified.
* **CSA_E_INVALID_FLAG** 

A flag value in the`flags`argument was invalid.
* **CSA_E_INVALID_FUNCTION_EXT** 

The function extension requested is invalid.
* **CSA_E_INVALID_PARAMETER** 

A function parameter was invalid.
* **CSA_E_INVALID_SESSION_HANDLE** 

The specified Session Handle is invalid or no longer valid
(e.g., after logging off).
* **CSA_E_NOT_SUPPORTED** 

The operation requested is not supported by this implementation.
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,