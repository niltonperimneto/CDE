# csa_add_entry
library call`csa_add_entry`add an entry to the specified calendar#include <xcsa.h>CSA_return_code`csa_add_entry`CSA_session_handlesessionCSA_uint32number_attributesCSA_attribute *entry_attributesCSA_entry_handle *entryCSA_extension *add_entry_extensions
## DESCRIPTION


The`csa_add_entry`function adds a new entry to a calendar.
The handle for
the new entry is returned.
Only the owner of the calendar,
users with`CSA_OWNER_RIGHTS,`users with`CSA_INSERT_PUBLIC_ENTRIES,`user with`CSA_INSERT_CONFIDENTIAL_ENTRIES,`or users with`CSA_INSERT_PRIVATE_ENTRIES`access rights can add entries to the calendar.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Number Attributes (Uint32)


Specifies the number of attributes in theentry_attributesargument.
### Entry Attributes (Attribute)


A pointer to an array of entry attributes that will be used
to define the new entry.
### Add Entry Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Entry (Entry Handle)


A pointer to the handle for the new entry.
This handle is
allocated by the service, and should be freed with a single
call to
&cdeman.csa.free;.
### Add Entry Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_add_entry`function returns the following error values:

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
* **CSA_E_INVALID_DATA_EXT** 

The data extension requested is invalid.
* **CSA_E_INVALID_FLAG** 

A flag value in the`flags`argument was invalid.
* **CSA_E_INVALID_FUNCTION_EXT** 

The function extension requested is invalid.
* **CSA_E_INVALID_PARAMETER** 

A function parameter was invalid.
* **CSA_E_INVALID_RULE** 

The implementation does not understand the exception or recurrence rule.
* **CSA_E_INVALID_SESSION_HANDLE** 

The specified Session Handle is invalid or no longer valid
(e.g., after logging off).
* **CSA_E_NO_AUTHORITY** 

The user has insufficient authority for this function.
* **CSA_E_READ_ONLY** 

An attempt was made to update a readonly attribute.
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_TEXT_TOO_LARGE** 

The size of the text string passed to the implementation is too large.
* **CSA_E_UNSUPPORTED_ATTRIBUTE** 

An attribute was encountered that is unsupported by the calendar service.
* **CSA_E_UNSUPPORTED_DATA_EXT** 

The data extension requested is not supported.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,