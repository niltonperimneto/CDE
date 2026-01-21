dtactionfilespecial filedtactionfiledefine the content of desktop action definition recordsset DtDbVersion=version_numbersetVariableName=variable_valueACTIONaction_name{
        #CommentFieldNamefield_valueFieldNamefield_value...
}DESCRIPTION&str-XZ; actions define the behavior of icons, front panel controls
and operations on data objects. Actions are defined in a set of text files
with the.dtsuffix. Each action definition consists of
the wordACTIONfollowed by an action
name and a list of Field and Value pairs (one per line) on lines by themselves
and enclosed in brackets.These text files may also contain data typing information as described
in &cdeman.dtdtsfile;. (See &cdeman.dtdtfile; for
the general syntax and location of the actions and data types database.)Actions are of one of the following classes: command actions, ToolTalk
message actions or map actions. These action classes are described in the
following sections.Command ActionsCommand actions are identified by aTYPECOMMANDfield. This
field defines an execution string to invoke and a set of related information,
such as the current working directory for the command and the host where the
command should be executed. The following field names are unique to command
actions:EXEC_STRING,EXEC_HOST,CWD,WINDOW_TYPEandTERM_OPTS.ToolTalk Message ActionsToolTalk message actions are identified by aTYPETT_MSGfield.
This field defines a ToolTalk message to be sent. The following fields are
unique to ToolTalk message actions:TT_CLASS,TT_SCOPE,TT_OPERATION,TT_FILE,TT_ARGn_MODE,TT_ARGn_VTYPE,TT_ARGn_REP_TYPEandTT_ARGn_VALUE.Map ActionsMap actions are identified by aTYPEMAPfield. This field does not define any specific
behavior; instead, this field specifies a different action name that should
be invoked in place of the original action. Multiple map actions can be chained
together, but the chain must ultimately terminate in a non-map action. The
following field is unique to map actions:MAP_ACTION.Common FieldsIn addition to the unique action fields listed above, all actions support
the following fields:LABEL,ICON,DESCRIPTION,ARG_CLASS,ARG_MODE,ARG_TYPE,ARG_COUNTandTYPE.KeywordsThe value string for certain action fields may reference special keywords
enclosed within percentage character (%) delimiters. These keywords are evaluated
when the action is invoked and replaced with the appropriate value. In fields
that do not evaluate keywords, the keyword is taken literally. The valid keywords
are:%DatabaseHost%The name of the host where the action definition file is located. This
hostname is specified by the host portion of thehost:/pathsearchpath used to find the action.%DisplayHost%The name of the host where the X server displaying the &str-XZ; session
is running.%LocalHost%The name of the host where the application invoking the action is executing.%SessionHost%The name of the host where the controlling login manager ( &cdeman.dtlogin;) runs.Argument ReferencesArguments passed to actions can be referenced in certain action fields
using special argument keywords enclosed within percent character (%) delimiters.
These argument keywords are evaluated when the action is invoked and replaced
with the appropriate value. In fields that do not evaluate keywords, the keyword
is taken literally. The valid argument keywords are:%Arg_n%Thenth (starting with 1) argument
of the action. If the action was invoked with fewer thannarguments, the value of the keyword isNULL.%Args%All remaining arguments of the action. If any arguments of the action
have already been referenced within this field by an %Arg_n% keyword, those arguments are not referenced a second time by %Args%.%"prompt"%Prompt the user for a value, usingpromptas the
label of a text field.%Arg_n"prompt"%Thenth (starting with 1) argument
of the action. If the action was invoked with fewer thannarguments, prompt the user for a value usingpromptas the label of a text field.If a keyword references the name of a file argument, the value of the
keyword is expanded to an absolute pathname prior to substitution. In addition,
if the file name is to be passed to a remote system, the file name is first
mapped appropriately (see &cdeman.tt.file.netfile; and &cdeman.tt.netfile.file;).If the keyword references a buffer argument, the buffer data is placed
in a temporary file and the name of the temporary file is substituted, as
described above. Some action fields provide direct support for data buffers
and do not require use of a temporary file. This behavior is noted in the
description of the appropriate fields.If the keyword references a string obtained from the user, it is treated
as a simple string and the value substituted without any transformation.Argument references can be forced to be treated as file names or simple
strings by using the(File)or(String)qualifier immediately after the opening % of the keyword. For example:%(String)Arg_n%
%(File)"prompt"%If an action is invoked with more than one argument, and the action
definition only references one or zero arguments, the action is iteratively
invoked with each of the supplied arguments. If the action definition references
more than one argument, any extra arguments are ignored.Action SelectionMultiple actions can be defined with the same name. When the action
is invoked, the appropriate action definition is chosen based on the number
and class of arguments supplied. For example, theOpenaction may invoke &cdeman.dtpad; if a text file is supplied as
an argument, or it may invoke &cdeman.dticon; if a bitmap file
argument is supplied. TheARG_COUNT,ARG_CLASS,ARG_MODEandARG_TYPEfields
specify the number, mode and types of arguments that are accepted by a particular
action. Because these fields can have shell pattern-matching values such as
*, it is possible that the action database contains multiple actions that
have the same name and are all capable of accepting the arguments that are
supplied. In this case, the following precedence rules are used to choose
a single action definition to invoke:Actions with more specific attribute values take precedence
over more general attribute values.For theARG_COUNTfield, an exact numerical value (N) is
more specific than a less-than range (<N). A less-than
range (<N) is more specific than a greater-than range
(>N). And a greater-than range (>N)
is more specific than a shell pattern-matching character (*).For theARG_CLASSandARG_TYPEfields,
a single item is more specific than a list of items and a list of items is
more specific than a shell pattern-matching *.For theARG_MODEfield,w(writable) and!w(not writable)
are more specific than a shell pattern-matching *.The fields have the following precedence, from
high to low:ARG_CLASS,ARG_TYPE,ARG_MODE,ARG_COUNT.If two action definitions have equal specificity,
the action definition appearing first in the database load order takes precedence.
Database directories are loaded in the order specified by theDTDATABASESEARCHPATHenvironment variable,
and are loaded in the collation order of their file names.ARG_CLASS FieldTheARG_CLASSfield is optional
for all types of actions. This field specifies the class of arguments the
action accepts. If an action is invoked with more than one argument, the class
of only the first argument is checked against the value of theARG_CLASSfield. The valid values for this field
are:BUFFERThe action accepts arguments that are blocks of data held in memory.FILEThe action accepts arguments that are file names.*The action is defined for all classes of arguments.A comma-separated list of valid values is also allowed and specifies
that the action accepts arguments of any of the listed classes.If an action is defined to accept a buffer argument, yet the implementation
of the action requires a file name, the buffer is automatically converted
into a temporary file for the action to use. See the description of theDtTmpDirresource for information on configuring the location of
these temporary files.Keywords are not evaluated in theARG_CLASSfield. The default value of theARG_CLASSfield is*.ARG_COUNT FieldTheARG_COUNTfield is optional
for all types of actions. TheARG_COUNTfield specifies the number of arguments that the action accepts. The valid
values for this field (whereNdenotes any non-negative
integer) are:NThe action accepts exactlyNarguments.<NThe action accepts any number of arguments less thanN.>NThe action accepts any number of arguments greater thanN.*The action accepts any number of arguments.Keywords are not evaluated in theARG_COUNTfield. The default value of theARG_COUNTfield is*.ARG_MODE FieldTheARG_MODEfield is optional
for all types of actions. This field specifies the mode of arguments the action
accepts. If an action is invoked with more than one argument, the mode of
only the first argument is checked against the value of theARG_MODEfield. The valid values for this field are:wThe action accepts arguments that writable by the user.!wThe action accepts arguments that are not writable by the user.*The action is defined for all classes of arguments.Keywords are not evaluated in theARG_MODEfield. The default value of theARG_MODEfield is*.ARG_TYPE FieldTheARG_TYPEfield is optional
for all types of actions. This field specifies the types of arguments the
action accepts. If the action is invoked with more than one argument, the
type of only the first argument is checked against the value of this field.
Valid values for this field are * (all data types are accepted), a single
data type name or a comma-separated list of data types. The set of valid data
types are those defined byDATA_ATTRIBUTErecords in the data typing database. (See &cdeman.dtdtsfile;
for more information.)Keywords are not evaluated in theARG_TYPEfield. The default value of theARG_TYPEfield is*.CWD FieldTheCWDfield is optional
for all types of actions. This field specifies the current working directory
to be used when the execution string is invoked. Valid values include any
absolute pathname. If this field is not specified, the current working directory
for the execution string is determined by the following:If the application invoking the action specifies a current
working directory, that directory is used.If arguments are supplied to the action and the
first argument is a directory, that directory is used.If arguments are supplied to the action and the
first argument is a file, the directory where the file is located is used.The current working directory of the application
invoking the action is used.Keywords are not evaluated in theCWDfield.DESCRIPTION FieldTheDESCRIPTIONfield is optional
forCOMMANDactions. This field
specifies a textual description of the action that is suitable for presentation
to a user requesting information about the action. The description should
contain no formatting information such as tab or newline characters. The application
that presents the information to the user formats the information.Keywords are not evaluated in theDESCRIPTIONfield. There is no default value for theDESCRIPTIONfield.EXEC_HOST FieldTheEXEC_HOSTfield is optional
forCOMMANDactions. This field
specifies the host where the execution string should be invoked. Valid values
for this field include actual hostnames, as well as any of the hostname keywords.
If a comma-separated list of hostnames is provided, execution is attempted
on each of the hosts in the order specified until execution succeeds.Keywords are evaluated in theEXEC_HOSTfield. The default value of theEXEC_HOSTfield is%DatabaseHost%,%LocalHost%.
(See the description of theExecutionHostsresource for
information on how to change this default value.)EXEC_STRING FieldTheEXEC_STRINGfield is required
forCOMMANDactions. This field
specifies an execution string to be invoked. The string is parsed using the
same quoting rules as defined bysh(1); however, the execution
string is not automatically passed to any shell. Therefore, if the execution
string requires shell features such as redirection of standard input, redirection
of standard output, or pipes, the appropriate shell must be specified explicitly
in the execution string. For example:EXEC_STRING       sh -c 'ls -l | more'Keywords are evaluated in theEXEC_STRINGfield. There is no default value for theEXEC_STRINGfield.ICON FieldTheICONfield is optional
for all types of actions. This field specifies the name of an icon that represents
the action.Icons are found by using the standard &str-XZ; icon search path, so
the value can be either an absolute pathname (for example,/foo/icons/myicon.bm), a relative pathname (for example,icons/myicon.bm)
or a partial filename (for example,myicon). Partial filenames
are preferred because they allow the &str-XZ; icon search path to find the
icon with the optimum size and depth for the current environment.Keywords are not evaluated in theICONfield. The default value of theICONfield isDtactn. (See the description
of theActionIconresource for information on how to change
this default value.)LABEL FieldTheLABELfield is optional
for all types of actions. This field specifies a user-visible label for the
action. When actions are presented to the user, the localizedLABELfield is used to identify the action instead
of the non-localized action name.Keywords are not evaluated in theLABELfield. The default value of theLABELfield is the name of the action.MAP_ACTION FieldTheMAP_ACTIONfield is required
forMAPactions. This field specifies
the name of an action that should be invoked in place of the current action.
The specified action is invoked with the same set of arguments that were passed
to the original action.Keywords are not evaluated in theMAP_ACTIONfield. There is no default value for theMAP_ACTIONfield.TERM_OPTS FieldTheTERM_OPTSfield is optional
forCOMMANDactions. This field
specifies command-line options that are passed to the terminal emulator for
allCOMMANDactions that are terminal
based. (That is, anyCOMMANDaction
other than those that specifyWINDOW_TYPENO_STDIO.) These command-line options are typically
used to specify a unique terminal-window geometry, font, color or title.The value of theTERM_OPTSfield must be an option string in a form the terminal emulator supports and
it must only affect the appearance of the terminal window. For example, options
such as-e, which affect the behavior of the terminal
window, must not be used.Keywords are evaluated in theTERM_OPTSfield. The default value of theTERM_OPTSfield is-title action_labelwhereaction_labelis theLABELfield for the action. See &cdeman.dtterm; (orxterm(1)) for the meaning of-title.TT_ARGn_MODE FieldTheTT_ARGn_MODEfield is
optional forTT_MSGactions. This
field specifies the value of the ToolTalk mode attribute for thenth message argument, wherenis
zero for the first message argument. The valid values for this field are:TT_IN,TT_INOUTandTT_OUT.(SeeTt/tt_c.hfor a description
of these values.)Keywords are not evaluated in theTT_ARGn_MODEfield. There is no default value for theTT_ARGn_MODEfield.TT_ARGn_REP_TYPE FieldTheTT_ARGn_REP_TYPEfield
is optional forTT_MSGactions.
This field specifies the representation type of thenth ToolTalk message argument, wherenis zero for the first message argument. The valid values for this field are:TT_REP_UNDEFINEDIfTT_ARGn_VALUEreferences
a buffer argument, the representation type is a buffer; otherwise, it is a
string.TT_REP_INTEGERThe representation type is an integer.TT_REP_BUFFERThe representation type is a buffer.TT_REP_STRINGThe representation type is string.Keywords are not evaluated in theTT_ARGn_REP_TYPEfield. The default value of theTT_ARGn_REP_TYPEfield isTT_REP_UNDEFINED.TT_ARGn_VALUE FieldTheTT_ARGn_VALUEfield is
optional forTT_MSGactions. If
there is no correspondingTT_ARGn_MODEfield, theTT_ARGn_VALUEfield is
ignored. If there is aTT_ARGn_MODEfield, theTT_ARGn_VALUEfield specifies
the value of thenth ToolTalk message argument,
wherenis zero for the first message argument.
If there is aTT_ARGn_MODEfield
with no correspondingTT_ARGn_VALUEfield, the value of thenth ToolTalk message
argument is set toNULL.The value of theTT_ARGn_VALUEfield must be a single string or action argument. Keywords that reference
a single action argument are evaluated in theTT_ARGn_VALUEfield, however %Args% is not allowed as it references
multiple action arguments. There is no default value for theTT_ARGn_VALUEfield.TT_ARGn_VTYPE FieldTheTT_ARGn_VTYPEfield is
required to accompany anyTT_ARGn_MODEfields inTT_MSGactions. This field
specifies the value of the ToolTalk vtype attribute of thenth message argument, wherenis
zero for the first message argument. If this field references an argument
keyword, theMEDIAattribute of the specified argument
is used. If theMEDIAattribute is not defined, theDATA_ATTRIBUTEname of the data type is used.Keywords are evaluated in theTT_ARGn_VTYPEfield. There is no default value for theTT_ARGn_VTYPEfield.TT_CLASS FieldTheTT_CLASSfield is required
forTT_MSGactions. This field specifies
the value of the ToolTalk class message field. The valid values for this field
are:TT_NOTICEThe action defines a ToolTalk notification message.TT_REQUESTThe action defines a ToolTalk request message.Keywords are not evaluated in theTT_CLASSfield. There is no default value for theTT_CLASSfield.TT_FILE FieldTheTT_FILEfield is optional
forTT_MSGactions. This field specifies
the value of the ToolTalk file message field. The value of this field must
be a single file name and can either be a specific file name (for example,/tmp/foo) or an argument keyword (for example, %Arg_1%). %Args% is not allowed because it references multiple action
arguments. If an argument keyword is specified and the corresponding argument
is not a file (that is, it is a buffer), the action invocation fails.Keywords are evaluated in theTT_FILEfield. There is no default value for theTT_FILEfield; if it is not set, the file attribute of the ToolTalk
message is set toNULL.TT_OPERATION FieldTheTT_OPERATIONfield is
required forTT_MSGactions. This
field specifies the value of the ToolTalk operation message field. Typical
values are operations such asDisplayorEditthat are defined by the Media Exchange Message
Set.Keywords are not evaluated in theTT_OPERATIONfield. There is no default value for theTT_OPERATIONfield.TT_SCOPE FieldTheTT_SCOPEfield is required
forTT_MSGactions. This field specifies
the value of the ToolTalk scope message field. (SeeTt/tt_c.hfor a description of these values.) The valid values
for this field are:TT_BOTH,TT_FILE,TT_FILE_IN_SESSIONandTT_SESSION.Keywords are not evaluated in theTT_SCOPEfield. There is no default value for theTT_SCOPEfield.TYPE FieldTheTYPEfield is optional
forCOMMANDactions and required
forMAPorTT_MSGactions. This field specifies the type of behavior defined
by the action. Valid values for this field are:COMMANDThe action invokes a command.MAPThe action specifies a different action name to invoke in place of the
current action.TT_MSGThe action defines a ToolTalk message to be sent.Keywords are not evaluated in theTYPEfield. The default value of theTYPEfield isCOMMAND.WINDOW_TYPE FieldTheWINDOW_TYPEfield is optional
forCOMMANDactions. This field
specifies the type of windowing support the execution string requires. Valid
values for this field are:NO_STDIONo windowing support is required. This value is appropriate for execution
strings that have no output or are X Windows applications.PERM_TERMINALThe execution string requires a terminal window. When the execution
string exits, the terminal window is left open until the user explicitly closes
it. This value is appropriate for applications that write their output to
standard output and then terminate, such asls(1).TERMINALThe execution string requires a terminal window. When the execution
string exits, the terminal window is closed. If the execution string exits
``quickly'' (see the description of thewaitTimeresource),
the terminal window is left open to allow the user to view any error messages
that were written to standard output or standard error. This value is appropriate
for full-screen terminal applications such as thevi(1) editor.Keywords are not evaluated in theWINDOW_TYPEfield. The default value of theWINDOW_TYPEfield isPERM_TERMINAL.RESOURCESThe following resources are available to control the behavior of actions.
These resources must be set for the application that is invoking the action.
They can be set for all applications that invoke actions by omitting the application
name or class name.