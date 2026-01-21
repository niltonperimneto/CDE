# csa_register_callback
library call`csa_register_callback`register the callback functions to be invoked when the specified type of update occurs in the calendar#include <xcsa.h>CSA_return_code`csa_register_callback`CSA_session_handlesessionCSA_flagsreasonCSA_callbackcallbackCSA_bufferclient_dataCSA_extension *register_callback_extensions
## DESCRIPTION


The`csa_register_callback`function adds the callback procedure to the specified
callback list for the session.
The service maintains a set
of callback lists, logically one list per callback activity
type per session.
The service reports changes to the state
of the service or its calendars by invoking the appropriate
callbacks in sequence when the client calls the
csa_call_callbacks function or when the specified callback
activity occurs.
The callback is only active for the
duration of the calendar session.
### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Reason (Flags)


A bitmask of flags.
Unspecified flags should always be
passed as 0.
Undocumented flags are reserved.
Each flag
corresponds to a callback activity and the specified
procedure is added to the callback list for that activity
type.
Flag settings include:

* **CSA_CB_CALENDAR_LOGON** 

If set, the new callback will be invoked when a the
calendar is accessed by a user.
* **CSA_CB_CALENDAR_DELETED** 

If set, the new callback will be invoked when a user
requests that the calendar be deleted.
* **CSA_CB_CALENDAR_ATTRIBUTE_UPDATED** 

If set, the new callback will be invoked whenever a
calendar attribute is updated.
* **CSA_CB_ENTRY_ADDED** 

If set, the new callback will be invoked whenever an entry
is added to the calendar.
* **CSA_CB_ENTRY_DELETED** 

If set, the new callback will be invoked whenever an entry
is deleted from the calendar.
* **CSA_CB_ENTRY_UPDATED** 

If set, the new callback will be invoked whenever an entry
is updated on the calendar.
It is implementation specific
if the callback function is invoked before or after the
specified update type occurs.

### Callback (Callback)


Specifies the client procedure that should be called by the
service to handle the callback activity.
### Client Data (Opaque Data)


A pointer to a data structure that will be passed to the
callback function in its client_data argument.
This can be
used to pass client-specific data structure that will be
needed by the callback function to perform the task.
### Register Callback Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Register Callback Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_register_callback`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
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
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;,