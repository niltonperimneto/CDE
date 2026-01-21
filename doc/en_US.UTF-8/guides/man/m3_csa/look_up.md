# csa_look_up
library call`csa_look_up`Looks up calendar information#include <xcsa.h>CSA_return_code`csa_look_up`CSA_session_handlesessionCSA_calendar_user *usersCSA_flagslook_up_flagsCSA_uint32 *number_usersCSA_calendar_user **user_listCSA_extension *look_up_extensions
## DESCRIPTION


The`csa_look_up`function looks up calendar addressing information in the
directory provided by the CSA calendaring service.
It
primarily is used to resolves a user's friendly name to a
calendar address.

Multiple addresses may be returned.
An array of calendar
user descriptors is allocated and returned containing fully
resolved information about each entry.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.

If the session handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Users(Calendar User)


A pointer to a calendar user structure containing the
user-friendly names whose calendar address is to be looked
up.
For name resolution, the name field in the structure
contains the user name to be resolved.
The user type can
be set to provide information on desired resolution of the
name.
See the calendar user structure documentation for possible types.
For displaying calendar user details, the calendar
user structure must contain an entry that resolves to only
one user.
If not, the error`CSA_E_AMBIGUOUS_USER`will be returned.
For both name resolution and displaying
user details, all user structures except the first will be
ignored.
### Look Up Flags (Flags)


Bit mask of flags.
Unspecified flags should always be
passed as 0.
Undocumented flags are reserved.
Flag
settings include:

* **CSA_LOOKUP_RESOLVE_PREFIX_SEARCH** 

If set, the search method should be prefix.
Prefix search
means that all names matching the prefix string, beginning
at the first character of the name, will be matched.
If
not set, the search method should be exact match.
CSA
implementations are required to support simple prefix
searching.
The availability of wild-card or substring
searches is optional.
* **CSA_LOOKUP_RESOLVE_IDENTITY** 

If set, the function will return a user record for the
identity of the user in the calendar system.
If this
cannot be uniquely determined, ambiguous name resolution
will be carried out.
This allows the application to find
out the address of the current user.

### Look Up Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Number Users (Uint32)


A pointer to the number of elementsuser_list.If no names match the criteria, a value of zero is
returned, and the error`CSA_E_USER_NOT_FOUND`is returned.
### User List (Calendar User)


A pointer to an array of one or more calendar user
structures allocated by
&cdeman.csa.look.up;. The structure may then be used to fill in an attendee list
structure.
This pointer is allocated by the service, and
should be freed with a single call to
&cdeman.csa.free;.
### Look Up Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_look_up`function returns the following error values:

* **CSA_E_AMBIGUOUS_USER** 

The calendar user's name is ambiguous; multiple matches have been found.
* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_DATA_EXT** 

The data extension requested is invalid.
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
* **CSA_E_UNSUPPORTED_DATA_EXT** 

The data extension requested is not supported.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.
* **CSA_E_USER_NOT_FOUND** 

One or more of the specified calendar users were not found.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,