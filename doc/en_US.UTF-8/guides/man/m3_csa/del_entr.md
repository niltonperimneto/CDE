# csa_delete_entry
library call`csa_delete_entry`delete an entry from a calendar#include <xcsa.h>CSA_return_code`csa_delete_entry`CSA_session_handlesessionCSA_entry_handleentryCSA_enumdelete_scopeCSA_extension *delete_entry_extensions
## DESCRIPTION


The`csa_delete_entry`function will delete an entry in a calendar.
The
delete_scope indicates the scope of the deletion if the
entry has any associated recurring entries.
All of the
recurring entries can be deleted, only the specified entry
can be deleted, or only the recurring entries that follow
the specified entry can be deleted.
Only the calendar
owner, users with`CSA_OWNER_RIGHTS,`users with`CSA_UPDATE_PUBLIC_ENTRIES,`users with`CSA_UPDATE_CONFIDENTIAL_ENTRIES,`or users with`CSA_UPDATE_PRIVATE_ENTRIES`access rights can delete entries on the calendar.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Entry (Entry Handle)


The handle of the calendar entry to be deleted.
If the
entry handle is invalid, then the error`CSA_E_INVALID_ENTRY_HANDLE`is returned.
### Delete Scope (Enum)


The scope for the delete operation.
Operator settings
include:

* **CSA_SCOPE_ALL** 

If set and the entry is associated with recurring entries,
it specifies that all of the recurring entries are to be
deleted.
* **CSA_SCOPE_ONE** 

If set and the entry is associated with recurring entries,
it specifies that only the specified entry is to be
deleted.
* **CSA_SCOPE_FORWARD** 

If set and the entry is associated with recurring entries,
it specifies that only the recurring entries following the
specified entry are to be deleted.
The scope also includes
the specified entry.

### Delete Entry Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Delete Entry Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_delete_entry`function returns the following error values:

* **CSA_E_DISK_FULL** 

Insufficient disk space was available to complete
the requested operation (this may refer to local or shared disk space).
* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INVALID_ENTRY_HANDLE** 

An invalid calendar entry handle was specified.
* **CSA_E_INVALID_ENUM** 

ACSA_enumvalue is invalid.
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
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_UNSUPPORTED_ENUM** 

The specified enumerated value is not valid.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,