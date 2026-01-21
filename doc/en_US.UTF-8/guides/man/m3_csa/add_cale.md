# csa_add_calendar
library call`csa_add_calendar`add a calendar to the calendar service#include <xcsa.h>CSA_return_code`csa_add_calendar`CSA_session_handlesessionCSA_calendar_user *userCSA_uint32number_attributesCSA_attribute *calendar_attributesCSA_extension *add_calendar_extensions
## DESCRIPTION


The`csa_add_calendar`function creates a calendar on the calendar service.
If
the user does not have sufficient authority to add a
calendar to the service, the error`CSA_E_NO_AUTHORITY`is returned.
If the calendar already exists the error`CSA_E_CALENDAR_EXISTS`is returned.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
The session handle may be`NULL`if the calendar service does not require a session to be
established in order to add a calendar.
### User (Calendar User)


A pointer to a calendar user structure.
This specifies the
user and name of the calendar to be added to the calendar
service.
### Number Attributes (Uint32)


The number of elements incalendar_attributesargument.
### Calendar Attributes (Attributes)


A pointer to an array of attribute structures that specify
the calendar attributes for the new calendar.
### Add CalendarExtensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Add Calendar Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_add_calendar`function returns the following error values:

* **CSA_E_CALENDAR_EXISTS** 

The specified calendar already exists.
* **CSA_E_DISK_FULL** 

Insufficient disk space was available to complete
the requested operation (this may refer to local or shared disk space).
* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_ATTRIBUTE** 

An attribute was specified that was not defined by this
specification and the implementation does not support the
attribute as an application specific attribute.
* **CSA_E_INVALID_ATTRIBUTE_VALUE** 

An invalid attribute value was specified for an attribute.
* **CSA_E_INVALID_FLAG** 

A flag value in the`flags`argument was invalid.
* **CSA_E_INVALID_FUNCTION_EXT** 

The function extension requested is invalid.
* **CSA_E_INVALID_PARAMETER** 

A function parameter was invalid.
* **CSA_E_INAVLID_SESSION_HANDLE** 


* **CSA_E_INVALID_USER** 

The specified calendar user is invalid.
* **CSA_E_NO_AUTHORITY** 

The user has insufficient authority for this function.
* **CSA_E_NOT_SUPPORTED** 

The operation requested is not supported by this implementation.
* **CSA_E_READONLY** 


* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_TEXT_TOO_LARGE** 

The size of the text string passed to the implementation is too large.
* **CSA_E_UNSUPPORTED_ATTRIBUTE** 

An attribute was encountered that is unsupported by the calendar service.
* **CSA_E_UNSUPPORTED_CHARACTER_SET** 

The character set requested is not supported.
* **CSA_E_UNSUPPORTED_DATA_EXT** 

The data extension requested is not supported.
* **CSA_E_UNSUPPORTED_ENUM** 

The specified enumerated value is not valid.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,