# csa_read_next_reminder
library call`csa_read_next_reminder`reads the next reminder of the given type in the specified calendar relative to a given time#include <xcsa.h>CSA_return_code`csa_read_next_reminder`CSA_session_handlesessionCSA_uint32number_namesCSA_attribute_reference *reminder_namesCSA_date_timegiven_timeCSA_uint32 *number_remindersCSA_reminder_reference **reminder_referencesCSA_extension *read_next_reminder_extensions
## DESCRIPTION


The`csa_read_next_reminder`function reads the next reminder of the specified type in
the specified calendar relative to a given time.
More than
one type of reminder may be specified.
For each reminder
type specified, the next reminder of that type after the
given time will be returned.
The owner of the calendar or
users with`CSA_OWNER_RIGHTS`access right can read the next reminder for a calendar.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Number Names (Uint32)


Specifies the size of reminder_names.
### Reminder Names (Attribute Reference)


A pointer to an array of attribute references.
This is an
array of reminder attribute names.
The names are used as
search criteria to return the next reminder of each type.
If`NULL`, then the first reminder after the given time will be
returned, no matter what type it is.
If an invalid
reminder type is specified, then the error`CSA_E_INVALID_ATTRIBUTE`is returned.
### Given Time (Date Time)


The given date and time after which the search for the next
reminder is to begin.
If the date and time value is
incorrect, then the error`CSA_E_INVALID_DATE_TIME`will be returned.
### Read Next Reminder Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Number Reminders (Uint32)


The number of reminder reference structures returned in
reminder_references.
If no reminders were found a value of
zero is returned.
### Reminder References (Reminder Reference)


A pointer to an array of reminder reference structures.
This array is allocated by the service, and should be freed
with a single call to
&cdeman.csa.free;.
### Read Next Reminder Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_read_next_reminder`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_ATTRIBUTE** 

An attribute was specified that was not defined by this
specification and the implementation does not support the
attribute as an application specific attribute.
* **CSA_E_INVALID_DATE_TIME** 

An invalid date and time combination was specified.
* **CSA_E_INVALID_FLAG** 

A flag value in the`flags`argument was invalid.
* **CSA_E_INVALID_FUNCTION_EXT** 

The function extension requested is invalid.
* **CSA_E_INVALID_PARAMETER** 

A function parameter was invalid.
* **CSA_E_INVALID_SESSION_HANDLE** 

The specified Session Handle is invalid or no longer valid
(e.g., after logging off).
* **CSA_E_NO_AUTHORITY** 

The user has insufficient authority for this function.
* **CSA_E_NOT_SUPPORTED** 

The operation requested is not supported by this implementation.
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_UNSUPPORTED_ATTRIBUTE** 

An attribute was encountered that is unsupported by the calendar service.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.register.callback;,