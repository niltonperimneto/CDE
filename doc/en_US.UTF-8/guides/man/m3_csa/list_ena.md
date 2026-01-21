# csa_list_entry_attributes
library call`csa_list_entry_attributes`list the names of the entry attributes associated with the specified entry#include <xcsa.h>CSA_return_code`csa_list_entry_attributes`CSA_session_handlesessionCSA_entry_handleentryCSA_uint32 *number_namesCSA_attribute_reference **entry_attributes_namesCSA_extension *list_entry_attributes_extensions
## DESCRIPTION


The`csa_list_entry_attributes`function lists the names of the entry attributes associated
with a calendar entry.
Using the returned entry attributes
name(s), the attributes value(s) may be read using the
&cdeman.csa.read.entry.attributes; function.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Entry (Entry Handle)


Specifies the handle of the calendar entry that will have
its attributes listed.
If the entry handle is invalid,
then the error`CSA_E_INVALID_ENTRY_HANDLE`is returned.
### List Entry Attributes Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Number Names (Uint32)


A pointer to the number of attribute names returned inentry_attribute_names.
### Entry Attribute Names (Attribute Reference)


A pointer to an array of attribute references containing
the names of the entry attributes in the calendar entry.
This array is allocated by the service, and should be freed
with a single call to
&cdeman.csa.free;.
### List Entry Attributes Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_list_entry_attributes`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
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
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,