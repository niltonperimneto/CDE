dtdtsfilespecial filedtdtsfileformat and location of desktop data type and action filesset DtDbVersion=version_numbersetVariableName=variable_valueRecordType record_name{
        #CommentFieldName field_valueFieldName field_value...
}DESCRIPTIONThe &str-XZ; data types database provides definitions for the data types
and actions recognized by &str-XZ; clients.The general syntax of the data types files is as described above in
the SYNOPSIS section of this man page. The set of general constructs composing
the database entries is as follows:CommentsAny line whose first non-space character is # is treated as a comment
line, and is ignored during the reading of the database file.Database VersionThe database loader supports a version number, which indicates the version
of the database syntax used by a particular database file. If a database version
number is not specified, then the database loader assumes that the file uses
the version 1.0 syntax, described here. If a database file specifies a version
number, then it must be the first non-blank, non-comment line in the database
file; if the version is specified anywhere else in the file, then an error
message is generated, and the remainder of that database file is ignored.
The database version number is specified using the following syntax:set DtDbVersion=version_numberString VariablesDatabase records can reference string variables that are set within
the database file. The scope of a string variable is restricted to only those
record definitions within the database file defining the string variable.
A string variable is defined using the following syntax:setVariableName=variable_valueString variables are referenced using either of the standard shell variable
referencing syntaxes: $variable_nameor ${variable_name}. A variable name can be made up of any of the alphanumeric
characters and the underscore.Environment VariablesDatabase records may refer to environment variables, using either of
the standard shell variable referencing syntaxes: $environment_variableor ${environment_variable}. If the environment
variable name conflicts with a string variable name, the string variable takes
precedence.Line ContinuationAny field within a record can be continued onto another line by ending
the line with a \ character. The \ and anyblanks following
the \ and preceding the newline are discarded; leadingblanks
on the following line are preserved in the continued field.Record NameThe first line of a record is made up of the record type,RecordType(one of:DATA_ATTRIBUTES,DATA_CRITERIAorACTION), followed by the record name,record_name, which is henceforth used to identify this record.
Therecord_namestring must be coded in ASCII and must
be uniquely named across the data attributes, data criteria and actions tables.Record DelimitersAfter the record name has been located, the set of corresponding fields
is delimited by the { and } characters. Each of these characters must appear
on a line by itself.FieldsThe fields are all of the non-comment lines found between the record
delimiters. They are composed of keyword/value pairs. TheFieldNamestring must be coded in ASCII. Thefield_valuemay be coded in additional, implementation-dependent, code sets, except that
any literal string values shown in Data Criteria Format, below, string must
be coded in ASCII.Record TypesThere are three recognized record types in database files used for data
types (and actions):DATA_CRITERIADATA_ATTRIBUTESACTIONThese three kinds of database record can appear together in the same
file or they can be segregated into separate files. See &cdeman.dtaction; for the file format ofACTIONrecords.DATA_CRITERIA RECORDSThe first seven subsections of this section describe theFieldNamessupported for data
criteria records. The remaining subsections describe formatting and sorting
information for data criteria records.NAME_PATTERN FieldA shell pattern-matching expression describing the file names that could
match this data. See Pattern Matching Notation. The default is an empty string,
which means to ignore file patterns in matching.If the data to be matched is in a buffer, rather than a file, theNAME_PATTERNexpression is evaluated against
theopt_namevalue given to &cdeman.DtDtsBufferToDataType; and related functions.PATH_PATTERN FieldA shell pattern-matching expression describing the absolute pathnames
that could match this data. See Pattern Matching Notation. The default is
an empty string, which means to ignore path patterns in matching.ThePATH_PATTERNexpression
is used only for matching data in files; it does not affect matching of data
in buffers.CONTENT FieldStrings that match on the contents of a file, buffer or directory:offset type value(s)Theoffsetstring is a positive decimal
integer number of octets from the beginning of the file or buffer, where the
firstvalueis tested. Theoffsetvalue is ignored for thefilenametype.Thetypestring is one of the following:stringThevalueis a single string that is
compared against the data starting at theoffsetlocation.byteshortlongEachblank-separatedvalueis an unsigned integer: decimal, octal (leading0)
or hexadecimal (leading0xor0X). Multiple
values are matched against multiple byte (octet), short (two octets) or long
(four octets) locations starting atoffsetoctets from the beginning of the file or data.filenameThevalueis a string that is compared
against the filenames located anywhere in a directory. The use offilenameon non-directory data produces undefined results.The defaultCONTENTis an
empty field, which means to ignore contents in matching.TheCONTENTfield applies
to data in both files and buffers.Examples of two data criteria records withCONTENTfields are:DATA_CRITERIA PCL1
{
        DATA_ATTRIBUTES_NAME    PCL
        CONTENT         0 byte 033 0105
        MODE            f&!x
}
DATA_CRITERIA POSTSCRIPT3
{
        DATA_ATTRIBUTES_NAME    POSTSCRIPT
        CONTENT         0 string %!
        MODE            f&!x
}MODE FieldA string of zero to four characters that match the mode field of astatstructure (seestat(2)). The first character
indicates:dmatch a directorysmatch a socketlmatch a symbolic linkfmatch a regular filebmatch a block filecmatch a character special fileThe first, or subsequent characters, can also be:rmatch any file with any of its user, group, or other read permission
bits setwmatch any file with any of its user, group, or other write permission
bits setxmatch any file with any of its user, group, or other execute or directory-search
permission bits setFor example, theMODEfield
offrwmatches any regular file that is readable or writable;xmatches any file with any of its executable or search bits set.The default is an empty field, which means to ignore the mode in matching.If the data to be matched is in a buffer, rather than a file, the buffer
is processed as if it had a mode offr.LINK_NAME FieldA shell pattern-matching expression describing the filename component
(basename) of the filename the symbolic link points to that could match this
data. See Pattern Matching Notation. The default is an empty expression, which
means to ignore symbolic link names in matching.LINK_NAMEpoints to the file itself, not to the name of the file.TheLINK_NAMEexpression is
used only for matching data in files; it does not affect matching of data
in buffers.LINK_PATH FieldA shell pattern-matching expression describing the absolute pathname
of the file pointed to by the symbolic link that could match this data. See
Pattern Matching Notation. The default is an empty expression, which means
to ignore symbolic link name in matching.TheLINK_PATHexpression is
used only for matching data in files; it does not affect matching of data
in buffers.DATA_ATTRIBUTES_NAME FieldThe name of this type of data. This value is arecord_namein the data attributes table.Pattern Matching NotationThe pattern-matching text field permits use of the shell pattern-matching
characters *, ?, and []. The asterisk (*) matches any set of characters,
the question mark (?) matches a single character, and the square brackets
([]) match any one of a set of characters enclosed in the square brackets.
The full definition of shell pattern matching is in the X/OpenCAE
Specification, Commands and Utilities, Issue 4.Logical ExpressionsThe logical operators AND (&), OR (|) and NOT (!)
can be used in any of the data criteria fields, except forDATA_ATTRIBUTES_NAME, as shown in Data Criteria Format, below.
The resultant expressions are evaluated from left to right.White SpaceWhite space is used to delimit tokens, as shown by theblanksandnewlineterminals in Data
Criteria Format, below. Within thepatternterminal, however, leading and trailing white space not explicitly shown in
the grammar is significant to the expression. For example,NAME_PATTERN   abc | defis matched by either ``abc'' (with a trailingspace) or ``def'' (with a leadingspace).Escape CharacterShell pattern-matching and logical expression characters can be escaped
and used as literal characters by preceding the character with a backslash
(\). For example, \* is interpreted as an asterisk, \? as a question mark
and \[\] as square brackets. Backslash itself can be escaped by preceding
it with a backslash (\\).Data Criteria FormatThe following pseudo-BNF describes the data criteria variable definition: