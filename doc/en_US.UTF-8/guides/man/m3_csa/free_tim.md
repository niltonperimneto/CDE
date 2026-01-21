# csa_free_time_search
library call`csa_free_time_search`searches one or more calendars for available free time#include <xcsa.h>CSA_return_code`csa_free_time_search`CSA_session_handlesessionCSA_date_time_rangedate_time_rangeCSA_time_durationtime_durationCSA_uint32number_usersCSA_calendar_user *calendar_usersCSA_free_time **free_timeCSA_extension *free_time_search_extensions
## DESCRIPTION


The`csa_free_time_search`function searches for available free time on one or more
calendars and returns a list of free time intervals found.
Free time is an interval of time that is not currently
scheduled on one or more calendars.
The free time search
is based on a date and time range and the minimum time
duration required of the free time interval.
Only the
owner of the calendar, users with`CSA_FREE_TIME_SEARCH,`users with`CSA_VIEW_PUBLIC_ENTRIES,`users with`CSA_VIEW_CONFIDENTIAL_ENTRIES,`or users with`CSA_VIEW_PRIVATE_ENTRIES`can search the calendar for free time.
## ARGUMENTS

### Session (Session Handle)


Opaque session handle that represents a session with the
calendaring service.

Session handles are created by a logon function call and
invalidated with a logoff function call.
If the session
handle is invalid, then the error`CSA_E_INVALID_SESSION_HANDLE`is returned.
### Date Time Range (Date Time Range)


Specifies the start and stop date and time range for the
free time search.
If the date_time_range is invalid, then
the error`CSA_E_INVALID_DATE_TIME`is returned.
### Time Duration (Time Duration)


Specifies the minimum time range for the free time
intervals to be sought.
If the time duration is invalid,
then the error`CSA_E_INVALID_DATE_TIME`is returned.
### Number Users (Uint32)


The number of elements in thecalendar_usersargument.
### Calendar Users (Calendar User)


A pointer to an array of calendar users.
This specifies
the calendars to be searched for available free time.
If
the user does not have authority to access any of the
attendee's calendars, then the error`CSA_E_NO_AUTHORITY`is returned.
If an invalid attendee is specified, then the
error`CSA_E_INVALID_USER`is returned.
It is implementation specific whether the
seach function will proceed in any of these cases.
### Free Time Search Extensions (Extension)


A pointer to an array ofCSA_extensionstructures for this function.
The array may contain both
input extensions for providing additional information to
the function and output extensions for receiving
information from the function.
A value of`NULL`indicates that the caller is not using any extensions.
See the extensions structure for more information.
## RETURN VALUE

### Free Time (Free Time)


A pointer to a free time structure that specifies a set of
free time intervals.
If more free time intervals are found than can be
represented in the available memory, the`CSA_E_INSUFFICIENT_MEMORY`error message will be returned.
If no available free time
is found, then a`NULL`pointer will be returned.
This structure is allocated by
the service, and should be freed with a single call to
&cdeman.csa.free;.
### Free Time Search Extensions (Extension)


If output extensions were passed to the function in the
extensions list, the results from the service will be
available in the extension.
See the extensions structure for more information.
Whether the function succeeded or
not, and, if not, why.
It may be success or one of the
values listed under ERRORS below.
## ERRORS


The`csa_free_time_search`function returns the following error values:

* **CSA_E_FAILURE** 

There was a general failure that does not
fit the description of any other error code.
* **CSA_E_INSUFFICIENT_MEMORY** 

Insufficient memory was available to complete the requested operation.
* **CSA_E_INVALID_DATA_EXT** 

The data extension requested is invalid.
* **CSA_E_INVALID_DATE_TIME** 

An invalid date and time combination was specified.
* **CSA_E_INVALID_FLAG** 

A flag value in the`flags`argument was invalid.
* **CSA_E_INVALID_FUNCTION_EXT** 

The function extension requested is invalid.
* **CSA_E_INVALID_PARAMETER** 

A function parameter was invalid.
* **CSA_E_INVALID_SESSION_HANDLE** 

The specified Session Handle is invalid or no longer valid
(e.g., after logging off).
* **CSA_E_INVALID_USER** 

The specified calendar user is invalid.
* **CSA_E_NO_AUTHORITY** 

The user has insufficient authority for this function.
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

## SEE ALSO


&cdeman.csa.csa.h;, &cdeman.csa.add.calendar;, &cdeman.csa.add.entry;, &cdeman.csa.call.callbacks;, &cdeman.csa.delete.calendar;, &cdeman.csa.delete.entry;, &cdeman.csa.free;, &cdeman.csa.list.calendar.attributes;, &cdeman.csa.list.calendars;, &cdeman.csa.list.entries;, &cdeman.csa.list.entry.attributes;, &cdeman.csa.list.entry.sequence;, &cdeman.csa.logoff;, &cdeman.csa.logon;, &cdeman.csa.look.up;, &cdeman.csa.query.configuration;, &cdeman.csa.read.calendar.attributes;, &cdeman.csa.read.entry.attributes;, &cdeman.csa.read.next.reminder;, &cdeman.csa.register.callback;,