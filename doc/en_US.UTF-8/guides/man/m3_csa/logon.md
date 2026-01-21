# csa_logon
library call`csa_logon`log on to the calendar service and establish a session with a calendar#include <xcsa.h>CSA_return_code`csa_logon`CSA_service_referencecalendar_serviceCSA_calendar_user *userCSA_stringpasswordCSA_stringcharacter_setCSA_stringrequired_csa_versionCSA_session_handle *sessionCSA_extension *logon_extensions
## DESCRIPTION


The`csa_logon`function allows the calling application to logon to the
calendar service.
If the specified calendar does not
exist, then the error`CSA_E_CALENDAR_NOT_EXIST`is returned.

The function returns a Session Handle that the application
will use in subsequent CSA calls.
## ARGUMENTS

### Calendar Service (Service Reference)


A calendar service name specifying the requested
calendaring service, (e.g., the path to a calendar store or
a remote server node name).
This value may be`NULL`if the underlying calendaring service does not require a
service name or if UI is allowed.
This may be necessary on
some implementations and ignored on others.
### User (Calendar User)


A pointer to the calendar user structure that identifies
the user and calendar to the calendaring service.
This
value may be`NULL`.
### Password (String)


A string containing the password required for access to the
CSA service.
This value may be`NULL`if the underlying calendaring service does not require a
password or if UI is allowed.
### Character Set (String)


A CSA formal public identifier string for the character set
of strings used by the CSA caller.
The client may pass`NULL`in which case the character set used is determined by the
CSA service.
The supported values are implementation
specific.
### Required CSA Version (String)


The formal public identifier for the CSA version number
required by the application.
For this version of the
specification this string must be
``-//XAPIA/CSA/VERSION1/NONSGML CSA Version 1//EN''.
### Logon Extensions (Extensions)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.

Through extensions, the application can find out which
extensions are available and set which data extensions will
be active for the session.
## RETURN VALUE

### Session (Session Handle)


Opaque session handle that represents a session with the
CSA calendar.
### Logon Extensions (Extensions)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_logon`function returns the following error values:

* **CSA_E_CALENDAR_NOT_EXIST** 

The specified calendar does not exist.
* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_CALENDAR_SERVICE** 

The underlying calendar service is invalid, so logging on cannot be completed.
* **CSA_E_INVALID_CONFIGURATION** 

The specified logon configuration is inconsistent with the
selected calendar service.
* **CSA_E_INVALID_DATA_EXT** 

The data extension requested is invalid.
* **CSA_E_INVALID_FLAG** 

A flag value in the`flags`argument was invalid.
* **CSA_E_INVALID_FUNCTION_EXT** 

The function extension requested is invalid.
* **CSA_E_INVALID_PARAMETER** 

A function parameter was invalid.
* **CSA_E_INVALID_PASSWORD** 

The password is incorrect.
* **CSA_E_INVALID_USER** 

The specified calendar user is invalid.
* **CSA_E_NO_AUTHORITY** 

The user has insufficient authority for this function.
* **CSA_E_PASSWORD_REQUIRED** 

A password is required on this calendar service.
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_TOO_MANY_USERS** 

The implementation cannot support the additional logon of
another calendar user at this time.
* **CSA_E_UNSUPPORTED_CHARACTER_SET** 

The character set requested is not supported.
* **CSA_E_UNSUPPORTED_DATA_EXT** 

The data extension requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.
* **CSA_E_UNSUPPORTED_VERSION** 

The specification version specified in the call cannot be
supported by this CSA implementation.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,