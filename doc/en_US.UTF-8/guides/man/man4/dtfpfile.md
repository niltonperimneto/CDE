dtfpfilespecial filedtfpfileformat
and locations of front panel configuration filesset DtDbVersion=version_numbersetVariableName=variable_valueRecordTyperecord_name{
        #CommentKeywordValueKeywordValue...
}DESCRIPTIONThedtfpfilefront panel database provides definitions
for the components that define the content and functionality of the front
panel. Files containing front panel definitions must end with the.fpsuffix. Like the action and data type database, the front panel
database is constructed by reading all files ending in the.fpsuffix found in the search path specified by theDTDATABASESEARCHPATHenvironment variable.See &cdeman.dtdtfile; for a complete description of the
directory locations where these database files are found and for a description
of the specific syntax for the database files.File FormatThe general syntax of the front panel configuration files is as described
above in the SYNOPSIS section of this man page.TheComments,VersionsandVariablesfields are described in &cdeman.dtdtfile;.The front panel record types each have a set of Keyword and Value pairs.
There are six record types defined:PANEL,BOX,SUBPANEL,SWITCH,CONTROLandANIMATION. Each
record type has a set of keywords defined for it. Many of the keywords are
used for multiple record types.Record Types`PANEL`front panel nameThePANELrecord type defines
the outermost container of the front panel. It can contain one or moreBOXes and optionally repositioning handles,
a menu and a minimize button. The keywords defined forPANELare described in the following table.