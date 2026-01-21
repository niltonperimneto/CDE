# csa_read_entry_attributes
library call`csa_read_entry_attributes`read and return the calendar entry attribute values for a specified calendar entry#include <xcsa.h>CSA_return_code`csa_read_entry_attributes`CSA_session_handlesessionCSA_entry_handleentryCSA_uint32number_namesCSA_attribute_reference *attribute_namesCSA_uint32 *number_attributesCSA_attribute **entry_attributesCSA_extension *read_entry_attributes_extensions
## DESCRIPTION


The`csa_read_entry_attributes`function returns an array of attribute structures
containing the values of the attributes of the specified
calendar entry.
The function will return all of the
attributes ifnumber_namesargument is zero andattribute_namesargument is`NULL`.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Entry (Entry Handle)


The handle of the calendar entry to be read.
If the entry
handle is invalid, then the error`CSA_E_INVALID_ENTRY_HANDLE`is returned.
### Number Names (Uint32)


The number of names inattribute_names.Ifattribute_namesargument is`NULL`, then this must be zero.
### Attribute Names (Attribute Reference)


A pointer to an array of attribute names that reference the
attributes that are to be read.
Ifnumber_namesargument is zero, then this must be`NULL`.
### Read EntryAttribute Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Number Attributes (Uint32)


The number of attributes returned inentry_attributes.If none of the specified attributes have values, a value of
zero is returned.
### Entry Attributes (Attribute)


A pointer to an array of attributes.
This pointer is
allocated by the service and should be freed with a single
call to
&cdeman.csa.free;.
### Read Entry Attribute Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_read_entry_attributes`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_ATTRIBUTE** 

An attribute was specified that was not defined by this
specification and the implementation does not support the
attribute as an application specific attribute.
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
* **CSA_E_UNSUPPORTED_ATTRIBUTE** 

An attribute was encountered that is unsupported by the calendar service.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,