# csa_query_configuration
library call`csa_query_configuration`Determine information about the installed CSA configuration#include <xcmc.h>CSA_return_code`csa_query_configuration`CSA_session_handlesessionCSA_enumitemCSA_buffer *referenceCSA_extension *query_configuration_extensions
## DESCRIPTION


The`csa_query_configuration`function queries the underlying implementation's
configuration, and returns the information requested about
it, allocating memory when necessary.
The underlying
configuration file format is implementation dependent.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.

Session may be`NULL`to indicate that there is no session and that session
independent information should be returned.
This will
provide default logon information.

If this value is set to a valid Session Handle, session
dependent configuration information will be returned.

If the session handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Item (Enum)


This argument indicates which configuration information
should be returned.
If the specified item is not
appropriate for the implementation, the error`CSA_E_UNSUPPORTED_ENUM`is returned.
The possible values include:

* **CSA_CONFIG_CHARACTER_SET** 

The reference argument will be a pointer to the array of
character set FPI strings for the implementation.
The
array will be terminated with a`NULL`string.
The first character set FPI in the array is the
default character set used if the caller does not specify
one explicitly.
This pointer to the array should be freed
using
&cdeman.csa.free;. This FPI is used by the caller at logon to specify to the
implementation to use a different character set than the
default.
* **CSA_CONFIG_LINE_TERM** 

The reference argument will be a pointer to aCSA_enumvariable, which will be set to a value of
* **CSA_LINE_TERM_CRLF** 

if the line delimiter is a carriage return followed by a line feed,`CSA_LINE_TERM_LF`if the line delimiter is a line feed, or`CSA_LINE_TERM_CR`if the line delimiter is a carriage return.
* **CSA_CONFIG_DEFAULT_SERVICE** 

The returned reference argument will be a pointer to aCSA_stringinto which the default service name will be returned.
A pointer value of`NULL`will be written if no default service name is available.
This pointer should be freed using
&cdeman.csa.free;. This string, along with the one returned by`CSA_CONFIG_DEFAULT_USER,`can be used as defaults in user dialogs when asking for the
service name, user name, and password.
This will be
returned in the implementation default character set.
* **CSA_CONFIG_DEFAULT_USER** 

The reference argument will be a pointer to a`CSA_string,`into which the default user name will be returned.
A
pointer value of`NULL`will be written if no default user name is available.
This
pointer should be freed using
&cdeman.csa.free;. This string, along with the one returned by`CSA_CONFIG_DEFAULT_SERVICE,`can be used as defaults in user dialogs when asking for the
provider name, user name, and password.
This will be
returned in the implementation default character set.
* **CSA_CONFIG_REQ_PASSWORD** 

The reference argument will be a pointer to aCSA_enumvariable, which will be set to a value of`CSA_REQUIRED_NO`if the password is not required to logon,`CSA_REQUIRED_OPT`if the password is optional to logon, or`CSA_REQUIRED_YES`if the password is required to logon.
* **CSA_CONFIG_REQ_SERVICE** 

The reference argument will be a pointer to aCSA_enumvariable, which will be set to a value of`CSA_REQUIRED_NO`if the service name is not required to logon,`CSA_REQUIRED_OPT`if the service name is optional to logon, or`CSA_REQUIRED_YES`if the service name is required to logon.
* **CSA_CONFIG_REQ_USER** 

The reference argument will be a pointer to aCSA_enumvariable, which will be set to a value of`CSA_REQUIRED_NO`if the user name is not required to logon,`CSA_REQUIRED_OPT`if the user name is optional to logon, or`CSA_REQUIRED_YES`if the user name is required to logon.
* **CSA_CONFIG_UI_AVAIL** 

The reference argument will be a pointer to aCSA_booleanvariable, which will be set to a true value if there is UI
provided by the CSA implementation.
* **CSA_CONFIG_VER_IMPLEM** 

The reference argument will be a pointer to aCSA_stringvariable, which will be set to the CSA formal public
identifier for the version number for the implementation.
This pointer should be freed using
&cdeman.csa.free;.
* **CSA_CONFIG_VER_SPEC** 

The reference argument will be a pointer to aCSA_stringvariable, which will be set to the CSA formal public
identifier for the CSA specification version number
supported by this implementation.
This pointer should be
freed using
&cdeman.csa.free;.


The error`CSA_E_UNSUPPORTED_ENUM`is returned if the specified value is not supported by the
implementation.
### Query Configuration Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.

Through extensions, the application can find out which
extensions are available.
The extension to do this is`CSA_X_COM_SUPPORT_EXT.`Any CSA implementation that supports extensions must
support this extension.
For more information on this
extension, see the common extensions section of the
extensions appendix in this document
## RETURN VALUE

### Reference (Buffer)


This argument points to the buffer in which to receive the
configuration information.
The type of the variable or
buffer depends on the item argument.
### Query Configuration Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_query_configuration`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
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
* **CSA_E_NOT_SUPPORTED** 

The operation requested is not supported by this implementation.
* **CSA_E_SERVICE_UNAVAILABLE** 

The requested calendar service is unavailable.
* **CSA_E_UNSUPPORTED_ENUM** 

The specified enumerated value is not valid.
* **CSA_E_UNSUPPORTED_FLAG** 

The flag requested is not supported.
* **CSA_E_UNSUPPORTED_FUNCTION_EXT** 

The specified function extension is not supported or`CSA_EXT_REQUIRED`is set.

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.free.time.search;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,