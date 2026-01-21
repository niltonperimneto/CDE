# csa_unregister_callback
library call`csa_unregister_callback`unregister the specified callback functions#include <xcsa.h>CSA_return_code`csa_unregister_callback`CSA_session_handlesessionCSA_flagsreasonCSA_callbackcallbackCSA_bufferclient_dataCSA_extension *unregister_callback_extensions
## DESCRIPTION


The`csa_unregister_callback`function removes the specified callback procedure to the
specified callback list.
Both the procedure and the client
data must match for this function to remove the procedure.
Specifying a value of`NULL`for both callback and client data will cause all callbacks
on the specified callback list(s) to be removed.
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
corresponds to a callback list.
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


Specifies the client procedure that should be removed from
the specified callback list(s).
### Client Data (Opaque Data)


A pointer to an application specific data structure that
was specified when the callback procedure was registered.
This must match for the function to succeed.
This
constraint allows the application to register the same
function on more than one callback list with different
client_data.
The instances of the callback function on the
different callback lists will be treated independently by
the calendar service.
### Unregister Callback Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Unregister Callback Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_unregister_callback`function returns the following error values:

* **CSA_E_CALLBACK_NOT_REGISTERED** 

The specified callback function was not registered.
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


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,