# dtdtfile
special filedtdtfiledefine the format and location of actions and data type database filesSee
&cdeman.dtactionfile; and
&cdeman.dtdtsfile;.
## DESCRIPTION


The actions and data types database provides definitions for the actions
and data types &str-XZ; clients recognize.
Files containing actions and data type definitions must end
with the.dtsuffix.
The database is constructed by reading
all files ending in the.dtsuffix that are found in the search path
specified by the`DTDATABASESEARCHPATH`environment variable.

The
&cdeman.dttypes; utility is the tool that allows users to examine and debug their database.

The`DTDATABASESEARCHPATH`environment variable contains a comma-separated
list of directories specified in`[``host``:]/`pathformat.
The`host`: portion is optional, but if specified,/pathis interpreted relative to`host`. In addition,`host`defines theDatabaseHostfor records defined by files in the/pathdirectory.
Otherwise, theDatabaseHostis the same as theLocalHost. To allow for localized action definitions, the data base
search path supports the string`%L`within the pathname string.
The logic that parses`DTDATABASESEARCHPATH`substitutes the
value of the current locale as stored in the`LANG`environment variable for the string`%L`(or no characters if`LANG`is not set).
Other uses of % within the`DTDATABASESEARCHPATH`pathnames produce unspecified results.
Directories can be set up for various locales.
Each directory contains localized action definitions for a
single locale.
For examples, see the default search path shown below.
The local system administrator or the user
(in$HOME/.dtprofile) can modify the actual value of the search path.
The default search path includes the following directories,
searched in the following sequence:

* **$HOME/.dt/types/** 

personal user-defined database files
* **`/etc/dt/appconfig/types/%L`** 

locally defined language-specific database files
* **/etc/dt/appconfig/types/C** 

locally defined default database files
* **`/usr/dt/appconfig/types/%L`** 

language-specific database files
* **/usr/dt/appconfig/types/C** 

implementation-default database files

### File Format


In addition to the version information, comments and variable references
described under other headings in this document, these files may contain action
and data type records, as described in
&cdeman.dtactionfile; and
&cdeman.dtdtsfile;.
### Comments


Any line whose first non-space character is # is treated as a
comment line, and is ignored during the reading of the database file.
### Database Version


The database loader supports a version number, which indicates
the version of the database syntax used by a particular database file.
If a database version number is not specified, then the database
loader assumes that the file uses the version 1.0 syntax, described here.
If a database file specifies a version number, then it
must be the first non-blank, non-comment line in the database file;
if the version is specified anywhere else in the file, then an error
message is generated, and the remainder of that database file is ignored.
The database version number is specified using the following syntax:set DtDbVersion=version_number
### String Variables


Database entries can reference string variables that can be set
within the database file.
The scope of a string variable is restricted
to only those record definitions within the database file defining the
string variable.
A string variable is defined using the following syntax:setVariableName=variable_value

String variables are referenced using either of the standard shell
variable referencing syntaxes:
$variable_nameor ${variable_name}.
A variable name can be made up of any of the alphanumeric characters
and the underscore.
### Environment Variables


Database records may refer to environment variables, using either
of the standard shell variable referencing syntaxes:
$environment_variableor ${environment_variable}.
If the environment variable name conflicts
with a string variable name, the string variable takes precedence.
### Line Continuation


Any field within a record can be continued onto another line by
ending the line with a &bsol; character.
The &bsol; and anyblanks following the &bsol; and preceding the newline are discarded;
leadingblanks on the following line are preserved in the continued field.
## SEE ALSO


&cdeman.dtactionfile;, &cdeman.dtdtsfile;, &cdeman.dttypes;.