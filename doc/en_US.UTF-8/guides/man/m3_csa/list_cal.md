# csa_list_calendars
library call`csa_list_calendars`list the calendars supported by a calendar service#include <xcsa.h>CSA_return_code`csa_list_calendars`CSA_service_referencecalendar_serviceCSA_uint32 *number_namesCSA_calendar_user **calendar_namesCSA_extension *list_calendars_extensions
## DESCRIPTION


The`csa_list_calendars`function lists all the calendars supported by the specified
calendar service.
The names of the calendars supported are
returned incalendar_names, which is an array of`CSA_calendar_users`structures withnumber_nameselements.
It is implementation specific what authority a calendar
user needs to invoke this function.
## ARGUMENTS

### Calendar Service (Service Reference)


Specifies the calendar service.
A`NULL`pointer will reference the default calendar service name.

If the calendar service name is invalid, then the error`CSA_E_INVALID_CALENDAR_SERVICE`is returned.
If the user is not sufficiently authorized to
list the calendars on the calendar service, then the error`CSA_E_NO_AUTHORITY`is returned.
### List Calendars Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Number Names (Uint32)


A pointer to the number of calendar names returned in
calendar_names.
A value of zero indicates that no
calendars are known to the calendar service.
The error`CSA_E_INSUFFICIENT_MEMORY`is returned if the service has insufficient memory to store
the complete set of requested information.
### Calendar Names (Calendar User)


A pointer to the array of calendar user structures,
representing the calendars supported by the specified
calendar service.
This pointer is allocated by the
service, and should be freed with a single call to
&cdeman.csa.free;.
### List Calendars Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extension structure for more information.
Indicates whether the function
succeeded or not, and, if not, why.
It may be success or
one of the values listed under ERRORS below.
## ERRORS


The`csa_list_calendars`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_CALENDAR_SERVICE** 

The underlying calendar service is invalid, so logging on cannot be completed.
* **CSA_E_INVALID_FLAG** 

A flag value in the`flags`argument was invalid.
* **CSA_E_INVALID_FUNCTION_EXT** 

The function extension requested is invalid.
* **CSA_E_INVALID_PARAMETER** 

A function parameter was invalid.
* **CSA_E_NO_AUTHORITY** 

The user has insufficient authority for this function.
* **CSA_E_NOT_SUPPORTED** 

The operation requested is not supported by this implementation.
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,