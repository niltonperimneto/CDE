# csa_list_entries
library call`csa_list_entries`list the calendar entries that match all the attribute search criteria#include <xcsa.h>CSA_return_code`csa_list_entries`CSA_session_handlesessionCSA_uint32number_attributesCSA_attribute *entry_attributesCSA_enum *list_operatorsCSA_uint32 *number_entriesCSA_entry_handle **entriesCSA_extension *list_entries_extensions
## DESCRIPTION


The`csa_list_entries`function lists the entry handles for the calendar entries
that match all the attribute search criteria.
Using the
returned entry handles, the entries can have their
attributes listed and read, or the entries can updated or
deleted.
Only the calendar owner, users with`CSA_OWNER_RIGHTS,`users with`CSA_VIEW_PUBLIC_ENTRIES,`user with`CSA_VIEW_CONFIDENTIAL_ENTRIES,`or users with`CSA_VIEW_PRIVATE_ENTRIES`access rights can list entries in the calendar.
Iflist_operatorsis`NULL`, this specifies a short hand for an array of operators of`CSA_MATCH_EQUAL_TO.`The criteria are specified by the array of attributes and
the array of operators.
Each operator in operators
specifies how the corresponding attribute value is to be
matched.
The following operators are supported:

* **CSA_MATCH_ANY** 

Matches an entry that contains the corresponding attribute
regardless of the value.
* **CSA_MATCH_EQUAL_TO** 

Matches an entry that contains an attribute with a value
equal to the corresponding value.
* **CSA_MATCH_NOT_EQUAL_TO** 

Matches an entry that contains an attribute with a value
not equal to the corresponding value.
* **CSA_MATCH_GREATER_THAN** 

Matches an entry that contains an attribute with a value
greater than the corresponding value.
* **CSA_MATCH_LESS_THAN** 

Matches an entry that contains an attribute with a value
less than the corresponding value.
* **CSA_MATCH_GREATHER_THAN_OR_EQUAL_TO** 

Matches an entry that contains an attribute with a value
greater than or equal to the corresponding value.
* **CSA_MATCH_LESS_THAN_OR_EQUAL_TO** 

Matches an entry that contains an attribute with a value
less than or equal to the corresponding value.
* **CSA_MATCH_CONTAIN** 

Applies to character string values only.
It matches an
entry that contains the corresponding substring value.
The
only operators supported for reminder type attributes are`CSA_MATCH_ANY`and`CSA_MATCH_EQUAL_TO.`Searching of attributes with opaque data type is not
supported.
It is implementation specific in what order the
array of entries will be returned.
If eithernumber_attributesis zero orentry_attributesis`NULL`, then all of the entries will be returned.

## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Number Attributes (Uint32)


Specifies the size of the arrays pointed to byentry_attributesandlist_operators. Ifentry_attributesis`NULL`, then this must be zero.
### Entry Attributes (Attribute)


A pointer to an array of attribute structures specifying
the matching criteria.
Ifnumber_attributesis zero, then this must be`NULL`.
### List Operators (Enum)


A pointer to an array of matching operators.
### List Entries Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Number Entries (Uint32)


The number of entry handles in entries.
If this value is`NULL`, then no entries were found to match the search criteria.
If more entries were found than can be represented in the
available memory, then the error`CSA_E_INSUFFICIENT_MEMORY`will be returned.
### Entries (Entry Handle)


A pointer to an array of entry handles that match all the
search criteria.
This array is allocated by the service
and should be freed with a single call to
&cdeman.csa.free;.
### List Entries Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_list_entries`function returns the following error values:

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
* **CSA_E_UNSUPPORTED_ATTRIBUTE** 

An attribute was encountered that is unsupported by the calendar service.
* **CSA_E_UNSUPPORTED_DATA_EXT** 

The data extension requested is not supported.
* **CSA_E_UNSUPPORTED_ENUM** 

The specified enumerated value is not valid.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,