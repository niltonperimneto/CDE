# ksh
user cmdkshKornShell, a
standard/restricted command and programming languageksh&plusmn;abcefhikmnoprstuvxCP-Ifile&plusmn;ooption...-arg ...rksh&plusmn;abcefhikmnoprstuvxCP-Ifile&plusmn;ooption...-arg ...
## DESCRIPTION


Kshis a command and programming language
that executes commands read from a terminal
or a file.Rkshis a restricted version of the
command interpreterksh; it is used to set up login names and execution environments whose
capabilities are more controlled than those of the standard shell.
SeeInvocationbelow
for the meaning of arguments to the shell.
### Definitions.


Ametacharacteris one of the following characters:

`; &amp; &lpar; &rpar; &verbar; &lt; &gt; new-line space tab`

Ablankis a`tab`or a`space`. Anidentifieris a sequence of letters, digits, or underscores
starting with a letter or underscore.
Identifiers are used as components ofvariablenames.
Avnameis a sequence of one or more identifiers
separated by a. and optionally preceded
by a ..
Vnames are used as function and variable names.
Awordis a sequence ofcharactersexcluding non-quotedmetacharacters.

Acommandis a sequence of characters in the syntax
of the shell language.
The shell reads each command and
carries out the desired action either directly or by invoking
separate utilities.
A built-in command is a command that is carried out by the
shell itself without creating a separate process.
Some commands are built-in purely for convenience
and are not documented here.
Built-ins that cause
side effects in the shell environment and
built-ins that are found before performing a
path search (seeExecutionbelow)
are documented here.
For historical reasons, some of
these built-ins behave differently than
other built-ins and are calledspecial built-ins.
### Commands.


Asimple-commandis a list of variable assignments
(seeVariableAssignmentsbelow)
or a sequence ofblankseparated words
which may be preceded by a list of variable assignments
(seeEnvironmentbelow).
The first word specifies the name of the command to
be executed.
Except as specified below,
the remaining words are passed as arguments
to the invoked command.
The command name is passed as argument 0
(seeexec(2)). Thevalueof a simple-command is its exit status; 0-255
if it terminates normally; 256+signumif
it terminates abnormally (the name of the signal corresponding
to the exit status can be
obtained via the`-l $?`option of the`kill`built-in utility).

Anarithmetic commandbegins with a`((`, and consists of an arithmetic expression formed by
all the characters until a matching`))`. (SeeArithmetic evaluationbelow.)
The exit status of an arithmetic command is 0 when the
arithmetic expression evaluates to a non-zero value and
is 1 when the arithmetic expression evaluates to 0.

Apipelineis a sequence of one or morecommandsseparated by`&verbar;`. The standard output of each command but the last
is connected by apipe(2) to the standard input of the next command.
Each command,
except possibly the last,
is run as a separate process;
the shell waits for the last command to terminate.
The exit status of a pipeline is the exit
status of the last command.
Each pipeline can be preceded by thereserved word`!`which negates the exit status of the pipeline.

Alistis a sequence of one or more
pipelines
separated by`;`,`&`,`&verbar;&amp;`,`&&`, or`&verbar;&verbar;`, and optionally terminated by`;`,`&`, or`&verbar;&amp;`. Of these five symbols,`;`,`&`, and`&verbar;&amp;`have equal precedence,
which is lower than that of`&&`and`&verbar;&verbar;`. The symbols`&&`and`&verbar;&verbar;`also have equal precedence.
A semicolon
(`;`) causes sequential execution of the preceding pipeline; an ampersand
(`&`) causes asynchronous execution of the preceding pipeline (i.e., the shell doesnotwait for that pipeline to finish).
The symbol`&verbar;&amp;`causes asynchronous execution of the preceding pipeline
with a two-way pipe established to the parent shell;
the standard input and output of the spawned pipeline
can be written to and read from by the parent shell
by applying
the redirection operators`<&`and`>&`with arg`p`to commands and by using`-p`option of
the built-in commands`read`and`print`described later.
The symbol`&&`(`&verbar;&verbar;`) causes thelistfollowing it to be executed only if the preceding
pipeline
returns a zero (non-zero) value.
An arbitrary number of new-lines may appear in alistinstead of a semicolon,
to delimit a command.

Acommandis either a simple-command
or one of the following.
Unless otherwise stated,
the value returned by a command is that of the
last simple-command executed in the command.

* **for vname in word ... ;do list ;done** 

Each time a`for`command is executed,nameis set to the nextwordtaken from the`in`wordlist.
If`in`word...
is omitted, then
the`for`command executes the`do`listonce for each positional parameter
that is set
(seeParameter Expansionbelow).
Execution ends when there are no more words in the list.
* **for (( expr1; expr2; expr3 )) ;do list ;done** 

The arithmetic expressionexpr1is evaluated first.
(SeeArithmetic evaluationbelow.)
The arithmetic expressionexpr2is repeatedly evaluated until it evalues to zero and when non-zero,`list`is executed and the arithmetic expressionexpr3evaluated.
If any expression
is omitted, then it behaves as if it evaluated to 1.
* **select vname in word ... ;do list ;done** 

A`select`command prints on standard error (file descriptor 2) the set ofwords, each preceded by a number.
If`in`word...
is omitted, then
the
positional parameters
are used instead
(seeParameter Expansionbelow).
The`PS3`prompt is printed
and a line is read from the standard input.
If this line consists of the number
of one of the listedwords, then the value of the variablevnameis set to thewordcorresponding to this number.
If this line is empty the selection list is
printed again.
Otherwise the value of the variablevnameis set to`null`. The contents of the line read from standard input is
saved in
the variable`REPLY`. Thelistis executed for each selection until a`break`orend-of-fileis encountered.
If the`REPLY`variable is set tonullby the execution of`list`, then the selection list is printed before
displaying the`PS3`prompt for the next selection.
* **case word in (pattern &verbar; pattern ...) list ;; ... esac** 

A`case`command executes thelistassociated with the firstpatternthat matchesword. The form of the patterns is
the same as that used for
file-name generation (seeFile Name Generationbelow).
The`;;`operator causes execution of`case`to terminate.
If`;&`is used in place of`;;`the next subsequent list, if any, is executed.
* **if list ;then list** 

`elif`list`;then`list&str.CK; ...
&str.OK;`;else`list&str.CK;`;fi`Thelistfollowing`if`is executed and,
if it
returns a zero exit status, thelistfollowing
the first`then`is executed.
Otherwise, thelistfollowing`elif`is executed and, if its value is zero,
thelistfollowing
the next`then`is executed.
Failing that, the`else`listis executed.
If the`if`listhas non-zero exit status
and there is no`else``list`, then the`if`command returns a zero exit status.
* **while list ;do list ;done** 


* **until list ;do list ;done** 

A`while`command repeatedly executes the`while`listand, if the exit status of the last command in the list is zero, executes
the`do``list`; otherwise the loop terminates.
If no commands in the`do`listare executed, then the`while`command returns a zero exit status;`until`may be used in place of`while`to negate
the loop termination test.
* **(list)** 

Executelistin a separate environment.
Note, that if two adjacent open parentheses are
needed for nesting, a space must be inserted to avoid
evaluation as an arithmetic command as described above.
* **{ list;}** 

listis simply executed.
Note that unlike the metacharacters`(`and`)`,`{`and`}`arereserved words and must occur
at the beginning of a line or after a`;`in order to be recognized.
* **[[ expression ]]** 

Evaluatesexpressionand returns a zero exit status whenexpressionis true.
SeeConditional Expressionsbelow, for a description ofexpression.
* **function varname { list ;}** 


* **varname () { list ;}** 

Define a function which is referenced byvarname. A function whosevarnamecontains a
. is called a discipline function and the portion
of thevarnamepreceding the last
. must refer to an existing variable.
The body of the function is thelistof commands between`{`and`}`. A function defined with the`function`varnamesyntax can also be used as an argument to the.
special built-in command to get the equivalent behavior
as if thevarname`()`syntax were used to define it.
(SeeFunctionsbelow.)
* **time pipeline** 

Ifpipelineis omitted the user and system time for
the current shell and completed child processes is printed
on standard error.
Otherwise,pipelineis executed and the elapsed time as well as
the user and system time are printed on standard error.

The following reserved words
are only recognized as such when they are the first word of a command
and are not quoted:

`if then else elif fi case esac for
while until do done { } function
select time [[ ]] !`

### Variable Assignments.


One or more variable assignments can start a simple command
or can be an arguments to the`typeset`,`export`, or`readonly`special built-in commands.
The syntax for anassignmentis of the form:

* **varname=word** 


* **varname[word]=word** 

No space is permitted betweenvarnameand the`=`or
between`=`andword.
* **varname=(assign_list)** 

No space is permitted betweenvarnameand the`=`.
Anassign_listcan be one of the following:

* **word ...** 

Indexed array assignment.
* **[word]=word ...** 

Associative array assignment.
* **assignment ...** 

Nested variable assignment.
* **typeset options assignment ...** 

Nested variable assignment. Multiple assignments
can be specified by separating each of them with a`;`.


### Comments.


A word beginning with`#`causes that word and all the following characters up to a new-line
to be ignored.
### Aliasing.


The first word of each command is replaced by the text of an`alias`if an`alias`for this word has been defined.
An`alias`name consists of any number of characters excluding metacharacters,
quoting characters,
file expansion characters,
parameter expansion and command substitution
characters,
and`=`. The replacement string can contain any
valid shell script
including the metacharacters listed above.
The first word of each command in the
replaced text,
other than
any that are in the process of being replaced,
will be tested for aliases.
If the last character of the alias value is ablankthen the word following the alias will also be checked for alias
substitution.
Aliases can be used to redefine
built-in commands but cannot be used to redefine
the reserved words listed above.
Aliases can be created and listed with the`alias`command and can be removed with the`unalias`command.

Aliasingis performed when
scripts are read,
not while they are executed.
Therefore,
for an alias to take effect
the`alias
definition command has to be executed before
the command which references the alias is read.`

The following`exported`aliasesare compiled into the shell
but can be unset or redefined:

autoload=&acute;typeset -fu&acute;

command=&acute;command &acute;

fc=hist

float=&acute;typeset -E&acute;

functions=&acute;typeset -f&acute;

hash=&acute;alias -t --&acute;

history=&acute;hist -l&acute;

integer=&acute;typeset -i&acute;

nameref=&acute;typeset -n&acute;

nohup=&acute;nohup &acute;

r=&acute;hist -s&acute;

redirect=&acute;command exec&acute;

stop=&acute;kill -s STOP&acute;

times=&acute;{ {time;} 2>&amp;1;}&acute;

type=&acute;whence -v&acute;
### Tilde Substitution.


After alias substitution is performed, each word
is checked to see if it begins with an unquoted`&ap;`. For tilde substitution,wordalso refers to thewordportion of parameter expansion
( seeParameter Expansionbelow.)
If it does, then the word up to a/is checked to see if it matches a user name in the
password database (
often the/etc/passwdfile).
If a match is found, the`&ap;`and the matched login name are replaced by the
login directory of the matched user.
If no match is found, the original text is left unchanged.
A`&ap;`by itself, or in front of a/, is replaced by$HOME. A`&ap;`followed by a`+`or`-`is replaced by the value of
$PWDand
$OLDPWDrespectively.

In addition,
when expanding avariable assignment,tildesubstitution is attempted when
the value of the assignment
begins with a`&ap;`, and when a`&ap;`appears after a`:`. The`:`also terminates a`&ap;`login name.
### Command Substitution.


The standard output from a command enclosed in
parentheses preceded by a dollar sign (`$()`)
or a pair of grave accents (`&grave;&grave;`)
may be used as part or all
of a word;
trailing new-lines are removed.
In the second (obsolete) form, the string between the quotes is processed
for special quoting characters before the command is executed (seeQuotingbelow).
The command substitution`$(cat file)`can be replaced by the equivalent but faster`$(<file)`.
### Arithmetic Substitution.


An arithmetic expression enclosed in double
parentheses preceded by a dollar sign (`$(())`)
is replaced by the value of the arithmetic expression
within the double parentheses.
### Process Substitution.


This feature is only available on
versions of the UNIX operating system that support the/dev/fddirectory for naming open files.
Each command argument of the form`<(`list`)`or`>(`list`)`will run process`list`asynchronously connected to some file in/dev/fd. The name of this file will become the argument to the command.
If the form with`>`is selected then writing on this file will provide input for`list`. If`<`is used,
then the file passed as an argument will contain the output of the`list`process.
For example,

`paste <(cut -f1`file1`) <(cut -f3`file2`) | tee >(`process1`) >(`process2`)`

cutsfields 1 and 3 from
the filesfile1andfile2respectively,pastesthe results together, and
sends it
to the processesprocess1andprocess2, as well as putting it onto the standard output.
Note that the file, which is passed as an argument to the command,
is a UNIXpipe(2) so programs that expect tolseek(2) on the file will not work.
### Parameter Expansion.


Aparameteris an`variable`, one or more digits,
or any of the characters`&ast;`,`@`,`#`,`?`,`-`,`$`, and`!\&caret;`. Avariableis denoted by avname.
To create a variable whosevnamecontains a &period;,
a variable whosevnameconsists of everything before the last &period; must already exist.
Avariablehas avalueand zero or moreattributes.Variablescan be assignedvaluesandattributesby using the`typeset`special built-in command.
The attributes supported by the shell are described
later with the`typeset`special built-in command.
Exported variables pass values and attributes to
the environment.

The shell supports both indexed and associative arrays.
An element of an array variable is referenced by asubscript. Asubscriptfor an indexed array is denoted by
anarithmeticexpression(seeArithmetic evaluationbelow)
between a`[`and a`]`. To assign values to an indexed array, use`set -A`vname`value`....
The value of all
subscripts must be in the
range of
0 through 4095.
Indexed arrays need not be declared.
Any reference to a variable
with a valid subscript is
legal and an array will be created if necessary.

An associative array is created with the`-A`option to`typeset.`Asubscriptfor an associative array is denoted by
a string enclosed between`[`and`]`.

Referencing any array without a subscript
is equivalent to referencing the array with subscript 0.

Thevalueof avariablemay be assigned by writing:

vname`=`value&str.OK;vname`=`value&str.CK; ...

or

vname`[`subscript`]=`value&str.OK;vname`[`subscript`]=`value&str.CK; ...

Note that no space is allowed before or after the`=`.

Anamerefis a variable that is a reference to another variable.
A nameref is created with the`-n`attribute of`typeset`. The value of the variable at the time of the`typeset`command becomes the variable that will be referenced whenever
the nameref variable is used.
The name of a nameref variable cannot contain a ..
When a variable or function name contains a ., and the portion
of the name up to the first &period; matches the
name of a nameref, the variable referred to is obtained by
replacing the nameref portion with the name of the variable
referenced by the nameref.
A nameref provides a convenient way to refer to the variable
inside a function whose name is passed as an argument to a function.
For example, if the name of a variable is passed as the first
argument to a function, the command

typeset -n var=$1

inside the function causes references and assignments to`var`to be references and assignments to the variable whose
name has been passed to the function.

If either of the floating point attributes,`-E`, or`-F`, or the integer attribute,`-i`, is set forvname, then thevalueis subject to arithmetic evaluation as described below.

Positional parameters,
parameters denoted by a number,
may be assigned values with the`set`special built-in command.
Parameter`$0`is set from argument zero when the shell
is invoked.

The character`$`is used to introduce substitutableparameters.

* **${parameter}** 

The shell
reads all the characters from`${`to the matching`}`as part of the same word even if it contains
braces or metacharacters.
The value, if any, of the parameter is substituted.
The braces are required whenparameteris followed by a letter, digit, or underscore
that is not to be interpreted as part of its name,
when the variable name contains a .,
or when a variable is subscripted.
Ifparameteris one or more digits then it is a positional parameter.
A positional parameter of more than one digit must be
enclosed in braces.
Ifparameteris`&ast;`or`@`, then all the positional
parameters, starting with`$1`, are substituted
(separated by a field separator character).
If an arrayvnamewith subscript`&ast;`or`@`is used,
then the value
for each of the
elements
is substituted
(separated by a field separator character).
* **${#parameter}** 

Ifparameteris`&ast;`or`@`, the number of positional parameters is substituted.
Otherwise, the length of the value of theparameteris substituted.
* **${#vname[*]}** 

The number of elements in the arrayvnameis substituted.
* **${!vname}** 

Expands to the name of the variable referred to byvname. This will bevnameexcept whenvnameis a name reference.
* **${!vname[subscript]}** 

Expands to name of the subscript unlesssubscriptis`*`, or`@`. Whensubscriptis`*`, the list of array subscripts forvnameis generated.
For a variable that is not an array, the value is 0 if the variable
is set. Otherwise it is null.
Whensubscriptis`@`, same as above, except that when used in double quotes,
each array subscript yields a separate
argument.
* **${!prefix*}** 

Expands to the names of the variables whose names begin withprefix.
* **${parameter:-word}** 

Ifparameteris set and is non-null then substitute its value;
otherwise substituteword.
* **${parameter:=word}** 

Ifparameteris not set or is null then set it toword; the value of the parameter is then substituted.
Positional parameters may not be assigned to
in this way.
* **${parameter:?word}** 

Ifparameteris set and is non-null then substitute its value;
otherwise, printwordand exit from the shell.
Ifwordis omitted then a standard message is printed.
* **${parameter:+word}** 

Ifparameteris set and is non-null then substituteword; otherwise substitute nothing.
* **${parameter:offset:length}** 


* **${parameter:offset}** 

Expands to the portion of the value ofparameterstarting at the character determined by expandingoffsetas an arithmetic expression and consisting of the
number of characters determined by the arithmetic expression
defined bylength.In the second form, the remainder of the value is used.
Ifparameteris`&ast;`or`@`, or is an array name indexed by`&ast;`or`@`, thenoffsetandlengthrefer to the array index and number
of elements respectively.
* **${parameter#pattern}** 


* **${parameter##pattern}** 

If
the shellpatternmatches the beginning of the value ofparameter, then the value of
this expansion is the value of theparameterwith the matched portion deleted;
otherwise the value of thisparameteris substituted.
In the first form the smallest matching pattern is deleted and in the
second form the largest matching pattern is deleted.
Whenparameteris`@`,`*`, or an array variable with subscript`@`, or`*`, the substring operation is applied to each element in turn.
* **${parameter%pattern}** 


* **${parameter%%pattern}** 

If
the shellpatternmatches the end of the value ofparameter, then the value of
this expansion is the value of theparameterwith the matched part deleted;
otherwise substitute the value ofparameter. In the first form the smallest matching pattern is deleted and in the
second form the largest matching pattern is deleted.
Whenparameteris`@`,`*`, or an array variable with subscript`@`, or`*`, the substring operation is applied to each element in turn.
* **${parameter/pattern/string}** 


* **${parameter//pattern/string}** 

Expandsparameterand replacespatternwith the givenstring.In the first form,
only the first occurrence ofpatternis replaced.
In the second form,
each match forpatternis replaced by the givenstring.Whenstringis null, thepatternwill be deleted and the/in front ofstringmay be omitted.
Whenparameteris`@`,`*`, or an array variable with subscript`@`, or`*`, the substitution operation is applied to each element in turn.


In the above,wordis not evaluated unless it is
to be used as the substituted string,
so that, in the following example,`pwd`is executed only if`d`is not set or is null:

print ${d:-$(pwd)}

If the colon (`:``)`is omitted from the above expressions,
then the shell only checks whetherparameteris set or not.

The following
parameters
are automatically set by the shell:

* **#** 

The number of positional parameters in decimal.
* **-** 

Options supplied to the shell on invocation or by
the`set`command.
* **?** 

The decimal value returned by the last executed command.
* **$** 

The process number of this shell.
* **_** 

Initially, the value of`_`is an absolute pathname of the shell or script being executed
as passed in theenvironment. Subsequently it is assigned the last argument of the previous command.
This parameter is not set for commands which are asynchronous.
This parameter is also used to hold the name of the matching
MAILfile when checking for mail.
* **!** 

The process number of the last background command invoked.
* **.sh.edchar** 

This variable contains the value of the keyboard character
(or sequence of characters if the first character is an ESC, ascii`033``)`that has
been entered when processing a
KEYBDtrap.
If the value is changed as part of the trap action, then the new
value replaces the key (or key sequence) that caused the trap.
* **.sh.edcol** 

The character position of the cursor at the time of the most recent
KEYBDtrap.
* **.sh.edmode** 

The value is set to ESC when processing a
KEYBDtrap while in`vi`insert mode. (SeeVi Editing Modebelow.)
Otherwise,`.sh.edmode`is null when processing a
KEYBDtrap.
* **.sh.edtext** 

The characters in the input buffer at the time of the most recent
KEYBDtrap.
The value is null when not processing a
KEYBDtrap.
* **.sh.name** 

Set to the name of the variable at the time of a`set`or`get`discipline is invoked.
* **.sh.subscript** 

Set to the name subscript of the variable at the time of a`set`or`get`discipline is invoked.
* **.sh.value** 

Set to the value of the variable at the time of a`set`discipline.
* **.sh.version** 

Set to a value that identifies the version of this shell.
* **LINENO** 

The line number of the current line within the script or
function being executed.
* **OLDPWD** 

The previous working directory set by the`cd`command.
* **OPTARG** 

The value of the last option argument processed by the`getopts`built-in command.
* **OPTIND** 

The index of the last option argument processed by the`getopts`built-in command.
* **PPID** 

The process number of the parent of the shell.
* **PWD** 

The present working directory set by the`cd`command.
* **RANDOM** 

Each time this variable is referenced, a random integer,
uniformly distributed between 0 and 32767, is generated.
The sequence of random numbers can be initialized by assigning
a numeric value to`RANDOM`.
* **REPLY** 

This variable is set by the`select`statement and by
the`read`built-in command when no arguments are supplied.
* **SECONDS** 

Each time this variable is referenced, the number of
seconds since shell invocation is returned.
If this variable is
assigned a value, then the value returned upon reference will
be the value that was assigned plus the number of seconds since the assignment.


The following
variables
are used by the shell:

* **CDPATH** 

The search path for the`cd`command.
* **COLUMNS** 

If this variable is set,
the value is used to define the width of the edit window
for the shell edit modes and for printing`select`lists.
* **EDITOR** 

If the value of this variable ends inemacs,gmacs, orviand the
VISUALvariable is not set,
then the corresponding option
(see Special Command`set`below)
will be turned on.
* **ENV** 

If this variable is set, then
parameter expansion, command substitution, and arithmetic substitution,
are performed on
the value to generate
the pathname of the script that will be
executed when the shell
is invoked.
(SeeInvocationbelow.)
This file is typically used for`alias`and`function`definitions.
* **FCEDIT** 

Obsolete name for
the default editor name for the`hist`command.
FCEDITis not used when
HISTEDITis set.
* **FIGNORE** 

A pattern that defines the set of filenames that will be
ignored when performing filename matching.
* **FPATH** 

The search path for function definitions.
This path is searched when a function with the`-u`attribute is referenced and when a command is not found.
If an executable file is found, then it is read and executed
in the current environment.
* **IFS** 

Internal field separators,
normally`space`,`tab`, and`new-line`that are used to separate the results of
command substitution or parameter expansion
and to separate fields with the built-in command`read`. The first character of the`IFS`variable is used to separate arguments for the`"$&ast;"
substitution. (See`Quotingbelow.)
Each single occurrence of
an`IFS`character in the string to be split,
except`space`,`tab`, and`new-line`, separates a field.
One or more`space`,`tab`, or`new-line`characters separate a field.
* **HISTEDIT** 

Name for
the default editor name for the`hist`command.
* **HISTFILE** 

If this variable is set when the shell is invoked, then
the value is the pathname of the file that will be
used to store the command history.
(SeeCommand re-entrybelow.)
* **HISTSIZE** 

If this variable is set when the shell is invoked, then
the number of previously entered commands that
are accessible by this shell
will be greater than or equal to this number.
The default is 128.
* **HOME** 

The default argument (home directory) for the`cd`command.
* **LINES** 

If this variable is set,
the value is used to determine the column length for printing`select`lists.
Select lists will print vertically until about two-thirds of
LINESlines are filled.
* **MAIL** 

If this variable is set to the name of a mail fileandthe
MAILPATHvariable is not set,
then the shell informs the user of arrival of mail
in the specified file.
* **MAILCHECK** 

This variable specifies how often (in seconds) the
shell will check for changes in the modification time
of any of the files specified by the
MAILPATHor
MAILvariables.
The default value is 600 seconds.
When the time has elapsed
the shell will check before issuing the next prompt.
* **MAILPATH** 

A colon (`:`)
separated list of file names.
If this variable is set
then the shell informs the user of
any modifications to the specified files
that have occurred within the last
MAILCHECKseconds.
Each file name can be followed by a`?`and a message that will be printed.
The message will undergo parameter expansion, command substitution,
and arithmetic substitution
with the variable`$_`defined as the name of the file that has changed.
The default message isyouhavemail`in`$_.
* **PATH** 

The search path for commands (seeExecutionbelow).
The user may not change`PATH`if executing under`rksh`(except in`.profile).`
* **PS1** 

The value of this variable is expanded for parameter
expansion, command substitution, and arithmetic substitution to define the
primary prompt string which by default is
```$`''. The character`!`in the primary prompt string is replaced by thecommandnumber (seeCommand Re-entrybelow).
Two successive occurrences of`!`will produce a single`!`when the prompt string is printed.
* **PS2** 

Secondary prompt string, by default
```>`''.
* **PS3** 

Selection prompt string
used within a`select`loop, by default
```#?`''.
* **PS4** 

The value of this variable is expanded for parameter evaluation,
command substitution, and arithmetic substitution
and precedes each line of an execution trace.
If omitted, the execution trace prompt is
```+`''.
* **SHELL** 

The pathname of theshellis kept in the environment.
At invocation, if the basename of this variable is`rsh`,`rksh`, or`krsh`, then the shell becomes restricted.
* **TMOUT** 

If set to a value greater than zero,
the`read`built-in command terminates after
TMOUTseconds when input is from a terminal.
Otherwise,
the shell will terminate if a line is not entered within
the prescribed number of seconds while reading from a terminal.
(Note that the shell can be compiled with a maximum bound
for this value which cannot be exceeded.)
* **VISUAL** 

If the value of this variable ends inemacs,gmacs, orvithen the corresponding option
(see Special Command`set`below)
will be turned on.


The shell gives default values to`PATH`,`PS1`,`PS2`,`PS3`,`PS4`,`MAILCHECK`,`HISTEDIT`,`TMOUT`and`IFS`,
while`HOME`,`SHELL`,`ENV`, and`MAIL`are
not set at all by the shell (although`HOME`isset bylogin(1)). On some systems`MAIL`and`SHELL`are also
set bylogin(1).
### Field Splitting


After parameter expansion and command substitution,
the results of substitutions are scanned for the field separator
characters (those found in`IFS`)
and split into distinct fields where such characters are found.
Explicit null fields (`""`or`(fm(fm`) are retained.
Implicit null fields
(those resulting fromparametersthat have no values or command substitutions with no output) are removed.
### File Name Generation.


Following splitting, each field is scanned for the characters`&ast;`,`?`,`(`, and`&str.OK;`unless the`-f`option has been set.
If one of these characters appears
then the word is regarded as a`pattern`. Each file name component that contains any pattern character
is replaced with a lexicographically sorted set of names
that matches the pattern
from
that directory.
If no file name is found that matches the pattern, then
that component of the filename is left unchanged.
If`FIGNORE`is set,
then each file name component
that matches the pattern defined by the value of`FIGNORE`is ignored when generating the matching filenames.
The names
. and`..`are also ignored.
If`FIGNORE`is not set,
the character
. at the start of each file name component
will be ignored unless the first character of the pattern
corresponding to this component is the character
. itself.
Note, that for other
uses of pattern matching the/and
. are not treated specially.

* **&ast;** 

Matches any string, including the null string.
* **?** 

Matches any single character.
* **...** 

Matches any one of the enclosed characters.
A pair of characters separated by`-`matches any
character lexically between the pair, inclusive.
If the first character following the opening`&str.OK;`is a`!`then any character not enclosed is matched.
A`-`can be included in the character set by putting it as the
first or last character.
Within`&str.OK;`and`&str.CK;`character classes can be specified with the syntax`[:``class``:]`where class is one of the following:

`alnum alpha cntrl digit graph lower print punct space upper xdigit`


Apattern-listis a list of one or more patterns separated from each other
with a`&verbar;`. Composite patterns can be formed with one or more of the following:

* **?(pattern-list)** 

Optionally matches any one of the given patterns.
* ***(pattern-list)** 

Matches zero or more occurrences of the given patterns.
* **+(pattern-list)** 

Matches one or more occurrences of the given patterns.
* **@(pattern-list)** 

Matches exactly one of the given patterns.
* **!(pattern-list)** 

Matches anything except one of the given patterns.

### Quoting.


Each of themetacharacterslisted earlier (seeDefinitionsabove)
has a special meaning to the shell
and causes termination of a word unless quoted.
A character may bequoted(i.e., made to stand for itself)
by preceding
it with a`&bsol;`. The pair`&bsol;new-line`is removed.
All characters enclosed between a pair of single quote marks
(`&acute;&acute;`)
that is not preceded by a`$`are quoted.
A single quote cannot appear within the single quotes.
A single quoted string preceded an unquoted`$`is processed as an ANSI-C string
except that`&bsol;0`within the string causes the remainder of the
string to be ignored and`&bsol;E`is equivalent to the escape character
(ascii`033`). Inside double quote marks
(`""`),
parameter and command substitution occur and`&bsol;`quotes the characters`&bsol;`,`&grave;`,`"`,
and`$`. The meaning of`$&ast;`and`$@`is identical when not quoted or when used as a variable assignment value
or as a file name.
However, when used as a command argument,`"$&ast;"
is equivalent to``"$1``d``$2``d`...`"`,
where`d`is the first character of the`IFS`variable, whereas`"$@"
is equivalent to``"$1"``"$2"
....
Inside grave quote marks
(``&grave;&grave;`),`&bsol;`quotes the characters`&bsol;`,`&grave;`, and`$`. If the grave quotes occur within double quotes then`&bsol;`also quotes the character`"`.

The special meaning of reserved words or aliases can be removed by quoting any
character of the reserved word.
The recognition of function names or built-in command names listed below
cannot be altered by quoting them.
### Arithmetic Evaluation.


The shell performs arithmetic evaluation for
arithmetic substitution, to evaluate an arithmetic command,
to evaluate an indexed array subscript,
and to evaluate arguments to
the built-in commands`shift`and`let`. Evaluations are performed using
double precision floating point
arithmetic.
Floating point constants follow the ANSI-C programming language
conventions.
Integer constants are of the form
&str.OK;base`#`&str.CK;nwherebaseis a decimal number between two and sixty-four
representing the arithmetic base
andnis a number in that base.
The digits above 9 are represented
by the lower case characters, the upper case characters,`@`, and`_`respectively.
For bases less than 36, upper and lower case
character can be used interchangeably.
Ifbaseis omitted
then base 10 is used.

An arithmetic expression uses the same syntax, precedence, and
associativity of
expression as the C language.
All the C language operators
that apply to floating point quantities can be used.
In addition, when the value of an arithmetic variable
or sub-expression can be represented as a long integer,
all C language integer arithmetic operations can be performed.
Variables can be referenced by name within an arithmetic expression
without using the parameter expansion syntax.
When a variable is referenced, its value is evaluated as
an arithmetic expression.
The following math library functions can be used with an arithmetic
expression:

`abs acos asin atan cos cosh exp int log sin sinh sqrt tan tanh`

An internal representation of avariableas a double precision floating point can be specified with the`-E`&str.OK;n&str.CK;
or`-F`&str.OK;n&str.CK;
option of the`typeset`special built-in command.
The`-E`option causes the expansion of the value to be represented using
scientific notation when it is expanded.
The optional option argument`n`defines the number of significant figures.
The`-F`option causes the expansion to be represented as a floating decimal number
when it is expanded.
The optional option argument`n`defines the number of places after the decimal point in this case.

An internal integer representation of avariablecan be specified with the`-i`&str.OK;n&str.CK;
option of the`typeset`special built-in command.
The optional option argument`n`specifies an arithmetic base to be used when expanding the variable.
If you do not specify an arithmetic base,
the first assignment to the
variable determines the arithmetic base.

Arithmetic evaluation is performed on the value of each
assignment to a variable with the`-E`,`-F`, or`-i`attribute.
Assigning a floating point number to a
variable whose type is an integer causes the fractional
part to be truncated.
### Prompting.


When used interactively,
the shell prompts with the value of`PS1`after expanding it for parameter expansion, command substitution, and
arithmetic substitution,
before reading a command.
In addition, each single`!`in the prompt is replaced by the command number.
A`!!`is required to place`!`in the prompt.
If at any time a new-line is typed and further input is needed
to complete a command, then the secondary prompt
(i.e., the value of`PS2`) is issued.
### Conditional Expressions.


Aconditional expressionis used with the`[[`compound command to test attributes of files and to compare
strings.
Field splitting and file name generation are
not performed on the words between`[[`and`]]`. Each expression can be constructed from one or more
of the following unary or binary expressions:

* **string** 

True, if`string`is not null.
* **-a file** 

Same is`-e`below.
This is obsolete.
* **-b file** 

True, if`file`exists and is a block special file.
* **-c file** 

True, if`file`exists and is a character special file.
* **-d file** 

True, if`file`exists and is a directory.
* **-e file** 

True, if`file`exists.
* **-f file** 

True, if`file`exists and is an ordinary file.
* **-g file** 

True, if`file`exists and is has its setgid bit set.
* **-k file** 

True, if`file`exists and is has its sticky bit set.
* **-n string** 

True, if length of`string`is non-zero.
* **-o option** 

True, if option namedoptionis on.
* **-p file** 

True, if`file`exists and is a fifo special file or a pipe.
* **-r file** 

True, if`file`exists and is readable by current process.
* **-s file** 

True, if`file`exists and has size greater than zero.
* **-t fildes** 

True, if file descriptor numberfildesis open and associated with a terminal device.
* **-u file** 

True, if`file`exists and is has its setuid bit set.
* **-w file** 

True, if`file`exists and is writable by current process.
* **-x file** 

True, if`file`exists and is executable by current process.
If`file`exists and is a directory, then true if the current process
has permission to search in the directory.
* **-z string** 

True, if length of`string`is zero.
* **-L file** 

True, if`file`exists and is a symbolic link.
* **-O file** 

True, if`file`exists and is owned by the effective user id of this process.
* **-G file** 

True, if`file`exists and its group matches the effective group id of this process.
* **-S file** 

True, if`file`exists and is a socket.
* **file1 -nt file2** 

True, iffile1exists and is newer thanfile2.
* **file1 -ot file2** 

True, iffile1exists and is older thanfile2.
* **file1 -ef file2** 

True, iffile1andfile2exist and refer to the same file.
* **string == pattern** 

True, if`string`matches`pattern`. Any part ofpatterncan be quoted to cause it to be matched as a string.
* **string = pattern** 

Same as`==`above, but is obsolete.
* **string != pattern** 

True, if`string`does not match`pattern`.
* **string1 < string2** 

True, ifstring1comes beforestring2based on ASCII value of their characters.
* **string1 > string2** 

True, ifstring1comes afterstring2based on ASCII value of their characters.

The following obsolete arithmetic comparisons are also permitted:
* **exp1 -eq exp2** 

True, ifexp1is equal toexp2.
* **exp1 -ne exp2** 

True, ifexp1is not equal toexp2.
* **exp1 -lt exp2** 

True, ifexp1is less thanexp2.
* **exp1 -gt exp2** 

True, ifexp1is greater thanexp2.
* **exp1 -le exp2** 

True, ifexp1is less than or equal toexp2.
* **exp1 -ge exp2** 

True, ifexp1is greater than or equal toexp2.


In each of the above expressions, if`file`is of the form/dev/fd/`n`,
where`n`is an integer,
then the test is applied to the open file whose
descriptor number is`n`.

A compound expression can be constructed from these primitives by
using any of the following, listed in decreasing order of precedence.

* **(expression)** 

True, ifexpressionis true.
Used to group expressions.
* **! expression** 

True ifexpressionis false.
* **expression1 && expression2** 

True, ifexpression1andexpression2are both true.
* **expression1 &verbar;&verbar; expression2** 

True, if eitherexpression1orexpression2is true.

### Input/Output.


Before a command is executed, its input and output
may be redirected using a special notation interpreted by the shell.
The following may appear anywhere in a simple-command
or may precede or follow acommandand arenotpassed on to the invoked command.
Command substitution, parameter expansion,
and arithmetic substitution occur beforewordordigitis used except as noted below.
File name generation
occurs only if the shell is interactive and
the pattern matches a single file,
Field splitting is not performed.

* **< word** 

Use filewordas standard input (file descriptor 0).
* **> word** 

Use filewordas standard output (file descriptor 1).
If the file does not exist then it is created.
If the file exists, and the`noclobber`option is on,
this causes an error;
otherwise, it is truncated to zero length.
* **>| word** 

Sames as`>`, except that it overrides the`noclobber`option.
* **>> word** 

Use filewordas standard output.
If the file exists then output is appended to it (by first seeking to the end-of-file);
otherwise, the file is created.
* **<> word** 

Open filewordfor reading and writing
as standard input.
* **<<[-]word** 

The shell input is read up to a line that is the same aswordafter any quoting has been removed remove,
or to an end-of-file.
No parameter substitution, command substitution, arithmetic substitution or
file name generation is performed onword. The resulting document,
called ahere-document, becomes
the standard input.
If any character ofwordis quoted, then no interpretation
is placed upon the characters of the document;
otherwise, parameter expansion, command substitution, and arithmetic
substitution occur,`&bsol;new-line`is ignored,
and`&bsol;`must be used to quote the characters`&bsol;`,`$`,`&grave;`. If`-`is appended to`<<`, then all leading tabs are stripped fromwordand from the document.
* **<& digit** 

The standard input is duplicated from file descriptordigit(seedup(2)). Similarly for the standard output using`>&`digit.
* **<&-** 

The standard input is closed.
Similarly for the standard output using`>&-`.
* **<&p** 

The input from the co-process is moved to standard input.
* **>&p** 

The output to the co-process is moved to standard output.

If one of the above is preceded by a digit,
then the
file descriptor number referred to is that specified
by the digit
(instead of the default 0 or 1).
For example:

... 2>&1

means file descriptor 2 is to be opened
for writing as a duplicate
of file descriptor 1.

The order in which redirections are specified is significant.
The shell evaluates each redirection in terms of the
(file descriptor,`file`) association at the time of evaluation.
For example:

... 1>fname2>&1

first associates file descriptor 1 with filefname. It then associates file descriptor 2 with the file associated with file
descriptor 1 (i.e.fname). If the order of redirections were reversed, file descriptor 2 would be associated
with the terminal (assuming file descriptor 1 had been) and then file descriptor
1 would be associated with filefname.

If a command is followed by`&`and job control is not active,
then the default standard input
for the command
is the empty file/dev/null. Otherwise, the environment for the execution of a command contains the
file descriptors of the invoking shell as modified by
input/output specifications.

### Environment.


Theenvironment(seeenviron(7)) is a list of name-value pairs that is passed to
an executed program in the same way as a normal argument list.
The names must beidentifiersand the values are character strings.
The shell interacts with the environment in several ways.
On invocation, the shell scans the environment
and creates a
variable
for each name found,
giving it the corresponding value and attributes and marking itexport. Executed commands inherit the environment.
If the user modifies the values of these
variables
or creates new ones,
using the`export`or`typeset``-x`commands they become part of the
environment.
The environment seen by any executed command is thus composed
of any name-value pairs originally inherited by the shell,
whose values may be modified by the current shell,
plus any additions
which must be noted in`export`or`typeset``-x`commands.

The environment for anysimple-commandor function
may be augmented by prefixing it with one or more variable assignments.
A variable assignment argument is a word of the formidentifier=value. Thus:TERM=450 cmd args

and(export TERM; TERM=450; cmd args)

are equivalent (as far as the above execution ofcmdis concerned except for special built-in commands listed below -
those that are
preceded with a dagger).

If the obsolete`-k`option is set,allvariable assignment arguments are placed in the environment,
even if they occur after the command name.
The following
first prints`a=b c`and then`c`:echo a=b c
set ;-k
echo a=b c

This feature is intended for use with scripts written
for early versions of the shell and its use in new scripts
is strongly discouraged.
It is likely to disappear someday.
### Functions.


For historical reasons, there are two
ways to define functions,
the`name``()`syntax and
the`function`namesyntax, described in theCommandssection above.
Shell functions are read in and stored internally.
Alias names are resolved when the function is read.
Functions are executed like commands with the arguments
passed as positional parameters.
(SeeExecutionbelow.)

Functions defined by the`function``name`syntax and called by name execute in the same process as the caller and
share all files
and present working directory with the
caller.
Traps caught by the caller are reset to their default action
inside the function.
A trap condition that is not caught or ignored by the
function causes the function to terminate and the condition
to be passed on to the caller.
A trap on`EXIT`set inside a function
is executed after the function completes in the environment
of the caller.
Ordinarily,
variables are shared between the calling program
and the function.
However,
the`typeset`special built-in command used within a function
defines local variables whose scope includes
the current function and
all functions it calls.
Errors within functions return control to the caller.

Functions defined with the`name``()`syntax and functions defined with the`function``name`syntax that are invoked with the &period;
special built-in
are executed in the caller's
environment and share all variables
and traps with the caller.
Errors within these function executions cause the script that contains
them to abort.

The special built-in command`return`is used to return
from function calls.

Function names
can be listed with the`-f`or`+f`option of the`typeset`special built-in command.
The text of functions, when available, will also
be listed with`-f`. Functions can be undefined with the`-f`option of the`unset`special built-in command.

Ordinarily, functions are unset when the shell executes a shell script.
Functions that need to be defined across separate
invocations of the shell should
be placed in a directory and the
FPATH
variable should contains the name of this directory.
They may also
be specified in the
ENV
file.
### Jobs.


If the`monitor`option of the`set`command is turned on,
an interactive shell associates ajobwith each pipeline.
It keeps
a table of current jobs, printed by the`jobs`command, and assigns them small integer numbers.
When a job is started asynchronously with`&`, the shell prints a line which looks
like:

[1] 1234

indicating that the job which was started asynchronously was job number
1 and had one (top-level) process, whose process id was 1234.

This paragraph and the next require features that are
not in all versions of UNIX and may not apply.
If you are running a job and wish to do something else you may hit the key`&caret;Z`(control-Z) which sends a STOP signal to the current job.
The shell will then normally indicate that the job has been `Stopped',
and print another prompt.
You can then manipulate the state of this job,
putting it in the background with the`bg`command, or run some other
commands and then eventually bring the job back into the foreground with
the foreground command`fg`. A`&caret;Z`takes effect immediately and
is like an interrupt in that pending output and unread input are discarded
when it is typed.

A job being run in the background will stop if it tries to read
from the terminal.
Background jobs are normally allowed to produce output,
but this can be disabled by giving the command ``stty tostop''.
If you set this
tty option, then background jobs will stop when they try to produce
output like they do when they try to read input.

There are several ways to refer to jobs in the shell.
A job can be referred to by the process id of any process of the job
or by one of the following:

* **% number** 

The job with the given number.
* **% string** 

Any job whose command line begins with`string`.
* **%? string** 

Any job whose command line contains`string`.
* **%%** 

Current job.
* **%+** 

Equivalent to`%%`.
* **%-** 

Previous job.


The shell learns immediately whenever a process changes state.
It normally informs you whenever a job becomes blocked so that
no further progress is possible, but only just before it prints
a prompt.
This is done so that it does not otherwise disturb your work.
The`notify`option causes
the shell to print these job change messages
as soon as they occur.

When the`monitor`option is on, each background job that completes
triggers any trap set for`CHLD`.

When you try to leave the shell while jobs are running or stopped, you will
be warned that `You have stopped(running) jobs.'
You may use the`jobs`command to see what they are.
If you immediately try to
exit again, the shell will not warn you a second time, and the stopped
jobs will be terminated.
When a login shell receives a HUP signal, it sends
a HUP signal to each job that has not been disowned with a the`disown`built-in command described below.
### Signals.


The INT and QUIT signals for an invoked
command are ignored if the command is followed by`&`and the`monitor`option is not active.
Otherwise, signals have the values
inherited by the shell from its parent
(but see also
the`trap`built-in command below).
### Execution.


Each time a command is read, the above substitutions
are carried out.
If the command name matches one
of theSpecial built-in Commandslisted below,
it is executed within the
current shell process.
Next, the command name is checked to see if
it matches a user defined function.
If it does,
the positional parameters are saved
and then reset to the arguments of thefunctioncall.
When thefunctioncompletes or issues a`return`, the positional parameter list is restored.
For functions defined with the`function`namesyntax,
any trap set on`EXIT`within the function is executed.
The value of afunctionis the value of the last command executed.
A function is also executed in the
current shell process.
If a command name is not aspecial built-in commandor a user defined`function`, but it is one of the built-in commands listed below
it is executed in the current shell process.

The shell variable
PATHdefines the search path for
the directory containing the command.
Alternative directory names are separated by
a colon
(`:`). The default path is`/bin:/usr/bin:`(specifying/bin,/usr/bin, and the current directory
in that order).
The current directory can be specified by
two or more adjacent colons, or by a colon
at the beginning or end of the path list.
If the command name contains a/then the search path
is not used.
Otherwise, each directory in the path is
searched for an executable file that is not a directory.
If the shell
determines that there is a built-in version
of a command corresponding to a given pathname,
this built-in is invoked in the current process.
A process is created and
an attempt is made to execute the command viaexec(2). If the file has execute permission but is not an`a.out`file,
it is assumed to be a file containing shell commands.
A separate shell is spawned to read it.
All non-exported variables are removed in this case.
If the shell command
file doesn't have read permission,
or if thesetuidand/orsetgidbits are set on the file,
then the shell executes an agent whose job it is to
set up the permissions and execute the shell with the
shell command file passed down as an open file.
A parenthesized command is executed in
a sub-shell without removing non-exported variables.
### Command Re-entry.


The text of the last
HISTSIZE
(default 128)
commands entered from a terminal device
is saved in ahistoryfile.
The file$HOME/.sh_historyis used if the
HISTFILE
variable is not set
or if the file it names is not writable.
A shell can access the commands of
allinteractiveshells which use the same named`HISTFILE`. The built-in command`hist`is used to list or
edit a portion of this file.
The portion of the file to be edited or listed can be selected by
number or by giving the first character or
characters of the command.
A single command or range of commands can be specified.
If you do not specify an editor program as
an argument to`hist`then the value of the variable`HISTEDIT`is used.
If`HISTEDIT`is unset, the obsolete variable`FCEDIT`is used.
If`FCEDIT`is not defined then/bin/edis used.
The edited command(s) is printed and re-executed upon
leaving the editor unless you quit without writing.
The`-s`option
(
an in obsolete versions, the editor name`-`)
is used to skip the editing phase and
to re-execute the command.
In this case a substitution parameter of the formold`=``new`can be used to modify the command before execution.
For example, with the preset alias`r`, which is aliased to`&acute;hist -s&acute;`, typing
``r bad=good c`'
will re-execute the most recent command which starts with the letter`c`, replacing the first occurrence of the string`bad`with the string`good`.
### In-line Editing Options


Normally, each command line entered from a terminal device is simply
typed followed by a`new-line`(`RETURN' or `LINE FEED').
If either the`emacs`,`gmacs`, or`vi`option is active, the user can edit the command line.
To be in either of these edit modes`set`the corresponding
option.
An editing option is automatically selected each time the`VISUAL`or`EDITOR`variable is assigned a value ending in either of these
option names.

The editing features require that the user's terminal
accept `RETURN' as carriage return without line feed
and that a space (` ') must overwrite the current character on
the screen.

The editing modes implement a concept where the user is looking through a
window at the current line.
The window width is the value of`COLUMNS`if it is defined, otherwise 80.
If the window width is too small to display the prompt and leave
at least 8 columns to enter input, the prompt is truncated from the
left.
If the line is longer than the window width minus two, a mark is
displayed at the end of the window to notify the user.
As the cursor moves and reaches the window boundaries the window will be
centered about the cursor.
The mark is a`>`(<,`*`) if the line extends on the
right (left, both) side(s) of the window.

The search commands in each edit mode provide access to the history file.
Only strings are matched, not patterns, although a leading`&caret;`in the string restricts the match
to begin at the first character in the line.

Each of the edit modes has an operation to list the files
or commands that match a partially entered word.
When applied to the first word on the line,
or the first word after a`;`,`&verbar;`,`&`, or`(`, and the word does not begin with`&ap;`or contain a/, the list of aliases, functions, and executable commands
defined by the
PATHvariable that could match the partial word is displayed.
Otherwise, the list of files that match the given
word is displayed.
If the partially entered word does not contain any
file expansion characters, a`*`is appended before generating these lists.
After displaying the generated list, the input line
is redrawn.
These operations are called command name listing and file name listing,
respectively.
There are additional operations, referred to as command name
completion and file name completion, which compute the list
of matching commands or files, but instead of printing the list,
replace
the current word with a complete or partial match.
For file name completion,
if the match is unique, a/is appended if the file is a directory and a space is
appended if the file is not a directory.
Otherwise, the longest common prefix for all the matching
files replaces the word.
For command name completion, only the portion of the file names
after the last/are used to find the longest command prefix.
If only a single name matches this prefix, then the
word is replaced with the command name followed by a space.
### Key Bindings.


The
KEYBDtrap can be used to intercept keys as they are typed
and change the characters that are actually seen by
the shell.
This trap is executed after each character
( or sequence of characters when the first character is`ESC`) is entered while reading from a terminal.
The variable`.sh.edchar`contains the character or character sequence which
generated the trap.
Changing the value of`.sh.edchar`in the trap action causes the shell to behave as if the
new value were entered from the keyboard rather than
the original value.

The variable`.sh.edcol`is set to the input column number of the cursor at the time
of the input.
The variable`.sh.edmode`is set to`ESC`when in vi insert mode (see below) and is null otherwise.
By prepending`${.sh.editmode}`to a value assigned to`.sh.edchar`it will cause the shell
to change to control mode if it is not already in this mode.

This trap is not invoked for characters entered as arguments to
editing directives, or while reading input for a character search.
### Emacs Editing Mode.


This mode is entered by enabling either theemacsorgmacsoption.
The only difference between these two modes is the way
they handle`&caret;T`. To edit, the user
moves the cursor to the point needing correction and
then inserts or deletes characters or words as needed.
All the editing commands are control characters or escape
sequences.
The notation for control characters is caret (`&caret;`) followed
by the character.
For example,`&caret;F`is the notation for control`F`. This is entered by depressing `f' while holding down the
`CTRL' (control) key.
The `SHIFT' key is`not`depressed.
(The notation`&caret;?`indicates the DEL (delete) key.)

The notation for escape sequences is`M-`followed by a
character.
For example,`M-f`(pronounced Meta f)
is entered by depressing ESC
(ascii`033`) followed by `f'.
(`M-F`would be the notation for ESC followed by `SHIFT' (capital) `F'.)

All edit commands
operate from any place on the line
(not just at the beginning).
Neither the "RETURN" nor the "LINE FEED" key is
entered after edit commands except when noted.

* **&caret;F** 

Move cursor forward (right) one character.
* **M-f** 

Move cursor forward one word.
(The`emacs`editor's idea of a word is a string of characters
consisting of only letters, digits and underscores.)
* **&caret;B** 

Move cursor backward (left) one character.
* **M-b** 

Move cursor backward one word.
* **&caret;A** 

Move cursor to start of line.
* **&caret;E** 

Move cursor to end of line.
* **&caret;] char** 

Move cursor forward to character`char`on current line.
* **M-&caret;] char** 

Move cursor backward to character`char`on current line.
* **&caret;X&caret;X** 

Interchange the cursor and mark.
* **erase** 

(User defined erase character as defined
by thestty(1) command, usually`&caret;H`or`#`.) Delete previous character.
* **&caret;D** 

Delete current character.
* **M-d** 

Delete current word.
* **M-&caret;H** 

(Meta-backspace) Delete previous word.
* **M-h** 

Delete previous word.
* **M-&caret;?** 

(Meta-DEL) Delete previous word (if your interrupt character is`&caret;?`(DEL, the default) then this command will not work).
* **&caret;T** 

Transpose current character with next character inemacsmode.
Transpose two previous characters ingmacsmode.
* **&caret;C** 

Capitalize current character.
* **M-c** 

Capitalize current word.
* **M-l** 

Change the current word to lower case.
* **&caret;K** 

Delete from the cursor to the end of the line.
If preceded by a numerical parameter whose value is less than the
current cursor position, then delete from given position
up to the cursor.
If preceded by a numerical parameter whose value is greater than the
current cursor position, then delete from cursor up to
given cursor position.
* **&caret;W** 

Kill from the cursor to the mark.
* **M-p** 

Push the region from the cursor to the mark on the stack.
* **kill** 

(User defined kill character as defined
by the stty command, usually`&caret;G`or`@`.) Kill the entire current line.
If two`kill`characters are entered in succession, all
kill characters from then on cause a line feed
(useful when using paper terminals).
* **&caret;Y** 

Restore last item removed from line. (Yank item back to the line.)
* **&caret;L** 

Line feed and print current line.
* **&caret;@** 

(Null character) Set mark.
* **M- space** 

(Meta space) Set mark.
* **&caret;J** 

(New line) Execute the current line.
* **&caret;M** 

(Return) Execute the current line.
* **eof** 

End-of-file character,
normally`&caret;D`, is processed as an End-of-file only
if the current line is null.
* **&caret;P** 

Fetch previous command.
Each time`&caret;P`is entered
the previous command back in time is accessed.
Moves back one line when not on the first line of a multi-line command.
* **M-<** 

Fetch the least recent (oldest) history line.
* **M->** 

Fetch the most recent (youngest) history line.
* **&caret;N** 

Fetch next command line.
Each time`&caret;N`is entered
the next command line forward in time is accessed.
* **&caret;R string** 

Reverse search history for a previous command line containing`string`. If a parameter of zero is given, the search is forward.Stringis terminated by a "RETURN" or "NEW LINE".
If string is preceded by a`&caret;`, the matched line must begin with`string`. If`string`is omitted,
then the next command line containing the most recent`string`is accessed.
In this case a parameter of zero
reverses the direction of the search.
* **&caret;O** 

Operate - Execute the current line and fetch
the next line relative to current line from the
history file.
* **M- digits** 

(Escape) Define numeric parameter, the digits
are taken as a parameter to the next command.
The commands that accept a parameter are`&caret;F`,`&caret;B`,erase,`&caret;C`,`&caret;D`,`&caret;K`,`&caret;R`,`&caret;P`,`&caret;N`,`&caret;]`,`M-.`,`M-&caret;]`,`M-_`,`M-b`,`M-c`,`M-d`,`M-f`,`M-h`,`M-l`and`M-&caret;H`.
* **M- letter** 

Soft-key - Your alias list is searched for an
alias by the name`_`letterand if an alias of this name is defined, its
value will be inserted on the input queue.
Thelettermust not be one of the above meta-functions.
* **M-[ letter** 

Soft-key - Your alias list is searched for an
alias by the name`__`letterand if an alias of this name is defined, its
value will be inserted on the input queue.
The can be used to program functions keys on many terminals.
* **M-.** 

The last word of the previous command is inserted
on the line.
If preceded by a numeric parameter, the value
of this parameter determines which word to insert rather than
the last word.
* **M-_** 

Same as`M-.`.
* **M-*** 

Attempt file name generation on the current word.
An asterisk is appended if the word doesn't match any file
or contain any special
pattern characters.
* **Command or file name completion as described above.** 


* **M-=** 

Command or file name listing as described above.
* **&caret;U** 

Multiply parameter of next command by 4.
* **&bsol;** 

Escape next character.
Editing characters, the user's erase, kill and
interrupt (normally`&caret;?`) characters
may be entered
in a command line or in a search string if preceded by a`&bsol;`. The`&bsol;`removes the next character's
editing features (if any).
* **&caret;V** 

Display version of the shell.
* **M-#** 

If the line does not begin with a`#`, a`#`is inserted
at the beginning of the line
and after each new-line,
and the line is entered.
This causes a comment to be inserted in the history file.
If the line begins with a`#`, the`#`is deleted and one`#`after each new-line is also deleted.

### Vi Editing Mode.


There are two typing modes.
Initially, when you enter a command you are in theinputmode.
To edit, the user enterscontrolmode by typing ESC
(`033`) and moves the cursor to the point needing correction and
then inserts or deletes characters or words as needed.
Most control commands accept an optional repeat`count`prior to the command.

When in`vi`mode on most systems,
canonical processing is initially enabled and the
command will be echoed again if the speed is 1200 baud or greater and it
contains any control characters or less than one second has elapsed
since the prompt was printed.
The ESC character terminates canonical processing for the remainder of the command
and the user can then modify the command line.
This scheme has the advantages of canonical processing with the type-ahead
echoing of raw mode.

If the option`viraw`is also set, the terminal will always have canonical processing
disabled.
This mode is implicit for systems that do not support two
alternate end of line delimiters,
and may be helpful for certain terminals.
### Input Edit Commands


By default the editor is in input mode.

* **erase** 

(User defined erase character as defined
by the stty command, usually`&caret;H`or`#`.) Delete previous character.
* **&caret;W** 

Delete the previous blank separated word.
One some systems the`viraw`option
may be required for this to work.
* **eof** 

As the first character of the line causes
the shell to terminate unless the`ignoreeof`option is set.
Otherwise this character is ignored.
* **&caret;V** 

Escape next character.
Editing characters and the user's erase or kill
characters may be entered
in a command line or in a search string if preceded by a`&caret;V`. The`&caret;V`removes the next character's
editing features (if any).
One some systems the`viraw`option
may be required for this to work.
* **&bsol;** 

Escape the nexteraseor`kill`character.

### Motion Edit Commands


These commands will move the cursor.

* **[count]l** 

Cursor forward (right) one character.
* **[count]w** 

Cursor forward one alpha-numeric word.
* **[count]W** 

Cursor to the beginning of the next word that follows a blank.
* **[count]e** 

Cursor to end of word.
* **[count]E** 

Cursor to end of the current blank delimited word.
* **[count]h** 

Cursor backward (left) one character.
* **[count]b** 

Cursor backward one word.
* **[count]B** 

Cursor to preceding blank separated word.
* **[count]&verbar;** 

Cursor to column`count`.
* **[count]fc** 

Find the next charactercin the current line.
* **[count]Fc** 

Find the previous charactercin the current line.
* **[count]tc** 

Equivalent to`f`followed by`h`.
* **[count]Tc** 

Equivalent to`F`followed by`l`.
* **[count];** 

Repeats`count`times,
the last single character find command,`f`,`F`,`t`, or`T`.
* **[count],** 

Reverses the last single character find command`count`times.
* **0** 

Cursor to start of line.
* **&caret;** 

Cursor to first non-blank character in line.
* **$** 

Cursor to end of line.
* **%** 

Moves to balancing`(`,`)`,`{`,`}`,`[`, or`]`. If cursor is not on one of the above characters,
the remainder of the line is searched for the first
occurrence of one of the above characters first.

### Search Edit Commands


These commands access your command history.

* **[count]k** 

Fetch previous command.
Each time`k`is entered
the previous command back in time is accessed.
* **[count]-** 

Equivalent to`k`.
* **[count]j** 

Fetch next command.
Each time`j`is entered
the next command forward in time is accessed.
* **[count]+** 

Equivalent to`j`.
* **[count]G** 

The command number`count`is fetched.
The default is the least recent history command.
* **/ string** 

Search backward through history for a previous command containing`string`.Stringis terminated by a "RETURN" or "NEW LINE".
If string is preceded by a`&caret;`, the matched line must begin with`string`. If`string`is null the previous string will be used.
* **? string** 

Same as/except that search will be in the forward direction.
* **n** 

Search for next match of the last pattern to/or`?`commands.
* **N** 

Search for next match of the last pattern to/or`?`, but in reverse direction.

### Text Modification Edit Commands


These commands will modify the line.

* **a** 

Enter input mode and enter text after the current character.
* **A** 

Append text to the end of the line.
Equivalent to`$a`.
* **[count]cmotion** 


* **c[count]motion** 

Delete current character through the character thatmotionwould move the cursor to and enter input mode.
Ifmotionis`c`, the entire line will be deleted and
input mode entered.
* **C** 

Delete the current character through the end of line and enter input mode.
Equivalent to`c$`.
* **S** 

Equivalent to`cc`.
* **D** 

Delete the current character through the end of line.
Equivalent to`d$`.
* **[count]dmotion** 


* **d[count]motion** 

Delete current character through the character thatmotionwould move to.
Ifmotionis`d`, the entire line will be deleted.
* **i** 

Enter input mode and insert text before the current character.
* **I** 

Insert text before the beginning of the line.
Equivalent to`0i`.
* **[count]P** 

Place the previous text modification before the cursor.
* **[count]p** 

Place the previous text modification after the cursor.
* **R** 

Enter input mode and
replace characters on the screen with characters you type overlay fashion.
* **[count]rc** 

Replace the`count`character(s) starting at the current cursor position withc, and advance the cursor.
* **[count]x** 

Delete current character.
* **[count]X** 

Delete preceding character.
* **[count].** 

Repeat the previous text modification command.
* **[count]&ap;** 

Invert the case of the`count`character(s) starting at the current cursor position and advance the cursor.
* **[count]_** 

Causes thecountword of the previous command to be appended and
input mode entered.
The last word is used
ifcountis omitted.
* ***** 

Causes an`*`to be appended to the current word and file name generation attempted.
If no match is found,
it rings the bell.
Otherwise, the word is replaced
by the matching pattern and input mode is entered.
* **&bsol;** 

Command or file name completion as described above.

### Other Edit Commands


Miscellaneous commands.

* **[count]ymotion** 


* **y[count]motion** 

Yank current character through character thatmotionwould move the cursor to and puts them into the delete buffer.
The text and cursor are unchanged.
* **Y** 

Yanks from current position to end of line.
Equivalent to`y$`.
* **u** 

Undo the last text modifying command.
* **U** 

Undo all the text modifying commands performed on the line.
* **[count]v** 

Returns the command`hist -e ${VISUAL:-${EDITOR:-vi}}`countin the input buffer.
Ifcountis omitted, then the current line is used.
* **&caret;L** 

Line feed and print current line.
Has effect only in control mode.
* **&caret;J** 

(New line) Execute the current line, regardless of mode.
* **&caret;M** 

(Return) Execute the current line, regardless of mode.
* **#** 

If the first character of the command is a`#`, then this command deletes this`#`and each`#`that follows a newline.
Otherwise,
sends the line after
inserting a`#`in front of each line in the command.
Useful for causing the current line to be
inserted in the history as a comment and
uncommenting previously commented commands
in the history file.
* **=** 

Command or file name listing as described above.
* **@ letter** 

Your alias list is searched for an
alias by the name`_`letterand if an alias of this name is defined, its
value will be inserted on the input queue for processing.

### Built-in Commands.


The following simple-commands are executed in the shell process.
Input/Output redirection is permitted.
Unless otherwise indicated, the output is written on file descriptor 1
and the exit status, when there is no syntax error, is zero.
Except for`:`,`true`,`false`,`echo`,`command`,`newgrp`, and`login`, all built-in commands accept`--`to indicate end of options.
They also interpret the option`-?`as a help request and print ausagemessage
on standard error.
Commands that are preceded by one or two -
are special built-in commands and
are treated specially in the following ways:

Variable assignment lists preceding the command
remain in effect when the command completes.

I/O redirections are processed after variable assignments.

Errors
cause a script
that contains them to abort.

They are not valid function names.

Words,
following a command preceded by --
that are in the format of a variable assignment,
are expanded with the same rules as a variable assignment.
This means that
tilde substitution is performed after the`=`sign and field splitting and file name generation are not
performed.

* **-: arg ...** 

The command only expands parameters.
* **- . name arg ...** 

Ifnameis a function defined with the`function`namereserved word syntax,
the function is executed in the current environment
( as if it had been defined with the`name``()`syntax.)
Otherwise ifnamerefers to a file, the
file is read in its entirety and the commands are
executed in the current shell environment.
The search path
specified by
PATHis used to find the directory containing file.
If any argumentsargare given,
they become the positional parameters while processing
the
. command and are restored upon completion.
Otherwise the positional parameters are unchanged.
The exit status is the exit status of the last command executed.
* **-- alias -ptx name =value ...** 

`alias`with no arguments prints the list of aliases
in the formname=valueon standard output.
The`-p`option
causes the word`alias`to be inserted before each one.
When one or more arguments are given
analiasis defined
for eachnamewhosevalueis given.
A trailing space invaluecauses the next word to be checked for
alias substitution.
The obsolete`-t`option is used to set and list tracked aliases.
The value of a tracked alias is the full pathname
corresponding to the given`name`. The value becomes undefined when the value of`PATH`is reset but the alias remains tracked.
Without the`-t`option,
for eachnamein the argument list
for which novalueis given, the name
and value of the alias is printed.
The obsolete`-x`option has no effect.
The exit status is non-zero if anameis given, but no value, and no alias has been defined for thename.
* **bg job...** 

This command is only on systems that support job control.
Puts each specifiedjobinto the background.
The current job is put in the background
ifjobis not specified.
SeeJobsfor a description of the format ofjob.
* **- break n** 

Exit from the enclosing`for`,`while`,`until`, or`select`loop, if any.
Ifnis specified then breaknlevels.
* **builtin -ds -f file name ...** 

Ifnameis not specified, the built-ins are printed on standard output.
The`-s`option prints only the special built-ins.
Otherwise, eachnamerepresents the pathname whose basename is the name of the built-in.
The entry point function name is determined by prepending`b_`to the built-in name.
Special built-ins cannot be bound to a pathname or deleted.
The`-d`option deletes each of the given built-ins.
On systems that support dynamic loading, the`-f`option names a shared library containing the code for built-ins.
Once a library is loaded, its symbols become available
for subsequent invocations of`builtin`. Multiple libraries can be specified with separate invocations
of the`builtin`command.
Libraries are searched in the reverse order in which they are specified.Each command that is to be built-in must be written as a C function whose name is of the formb_funcName, wherefuncNameis the name of the built-in
to be added. This function has the same argument calling convention as 'main' (i.e.argcandargv-- a list of strings).  Theb_funcNameparses and
error-checks the incoming arguments and then calls the realfuncNamefunction.
* **cd -LP arg** 


* **cd -LP old new** 

This command can be in either of two forms.
In the first form it
changes the current directory toarg. Ifargis`-`the directory is changed to the previous
directory.
The shell
variable
HOMEis the defaultarg. The variable`PWD`is set to the current directory.
The shell variable
CDPATHdefines the search path for
the directory containingarg. Alternative directory names are separated by
a colon
(`:`). The default path is`<null>`(specifying the current directory).
Note that the current directory is specified by a null path name,
which can appear immediately after the equal sign
or between the colon delimiters anywhere else in the path list.
Ifargbegins with a/then the search path
is not used.
Otherwise, each directory in the path is
searched forarg.

The obsolete second form of`cd`substitutes the string`new`for the stringoldin the current directory name,`PWD`and tries to change to this new directory.

By default, symbolic links are not followed when
finding the directory name.
This is equivalent to the`-L`option.
The`-P`option causes
symbolic links to be followed when determining the directory.
The last instance of`-L`or`-P`determines which method is used.

The`cd`command may not be executed by`rksh.`
* **command -pvV name arg ...** 

Without the`-v`or`-V`options,`command`executesnamewith the arguments given byarg. The`-p`option causes
a default path to be searched
rather than the one defined by the value of`PATH`. Functions will not be searched for when finding`name`. In addition, ifnamerefers to a special built-in,
none of the special properties associated with the leading
daggers will be honored.
(
For example, the predefined alias`redirect=&acute;command exec&acute;`prevents a script from terminating when an invalid
redirection is given.)
With the`-v`option,`command`is equivalent to the built-in`whence`command described below.
The`-V`options, causes`command`to`whence -v`.
* **- continue n** 

Resume the next iteration of the enclosing`for`,`while`,`until`, or`select`loop.
Ifnis specified then resume at the`n`-th enclosing loop.
* **disown job...** 

Causes the shell not to send a HUP signal to
each givenjob, or all active jobs ifjobis omitted,
when a login shell terminates.
* **echo arg ...** 

When the firstargdoes not begin with a -, and
none of the arguments contain a &bsol;,
then`echo`prints each of its arguments separated by a space
and terminated by a new-line.
Otherwise, the behavior of`echo`is system dependent
and`print`or`printf`described below should be used.
Seeecho(1) for usage and description.
* **- eval arg ...** 

The arguments are read as input
to the shell
and the resulting command(s) executed.
* **- exec -c -a name arg ...** 

Ifargis given,
the command specified by
the arguments is executed in place of this shell
without creating a new process.
The`-c`option causes the environment to be cleared before applying
variable assignments associated with the`exec`invocation.
The`-a`option
causesnamerather than the firstarg, to become`argv[0]`for the new process.
Input/output arguments may appear and
affect the current process.
Ifargis not given
the effect of this command is to
modify file descriptors
as prescribed by the input/output redirection list.
In this case,
any file descriptor numbers greater than 2 that are
opened with this mechanism are closed when invoking
another program.
* **- exit n** 

Causes the shell to exit
with the exit status specified by`n`. The value will be the least significant 8 bits of the specified status.
Ifnis omitted then the exit status is that of the last command executed.
An end-of-file will also cause the shell to exit
except for a
shell which has theignoreeofoption (see`set`below) turned on.
* **-- export -p name=value ...** 

Ifnameis not given,
the names and values of each variable with
the export attribute are printed with the values
quoted in a manner that allows them to be re-inputed.
The`-p`option
causes the word`export`to be inserted before each one.
Otherwise, the given`name`s are marked for automatic
export to theenvironmentof subsequently-executed commands.
* **fg job...** 

This command is only on systems that support job control.
Eachjobspecified is brought to the foreground and waited for in
the specified order.
Otherwise, the current job is
brought into the foreground.
SeeJobsfor a description of the format ofjob.
* **getconf name pathname** 

Prints the current value of the configuration parameter given by`name`.
* **getopts optstring vname arg ...** 

Checksargfor legal options.
Ifargis omitted,
the positional parameters are used.
An option argument begins with a`+`or a`-`. An option not beginning with`+`or`-`or the argument`--`ends the options.optstringcontains the letters thatgetoptsrecognizes.
If a letter is followed by a`:`, that option is expected to have an argument.
The options can be separated from the argument by blanks.

`getopts
places the next option letter it finds inside variable`vnameeach time it is invoked with a`+`prepended whenargbegins with a`+`. The index of the nextargis stored in`OPTIND`. The option argument,
if any,
gets stored in`OPTARG`.

A leading`:`inoptstringcauses`getopts`to store the letter of an invalid
option in`OPTARG`, and to setvnameto`?`for an unknown option and to`:`when a required option is missing.
Otherwise,`getopts`prints an error message.
The exit status is non-zero when there are no more options.

There is no way to specify any of the options`:`,`+`,`-`,`?`,`[`, and`]`. The option`#`can only be specified as the first option.
* **hist -e ename   -nlr first last** 


* **hist -s old=new command** 

In the first form,
a range of commands fromfirsttolastis selected from the last`HISTSIZE`commands that were typed at the terminal.
The argumentsfirstandlastmay be specified as a number or as a string.
A string is used to locate the most recent command starting with
the given string.
A negative number is used as an offset to the current command number.
If the`-l`option
is selected,
the commands are listed on standard output.
Otherwise, the editor programenameis invoked on a file containing these
keyboard commands.
Ifenameis not supplied, then the value of the variable`HISTEDIT`is used.
If`HISTEDIT`is not set then`FCEDIT`(default`/bin/ed`) is used as the editor.
When editing is complete, the edited command(s)
is executed if the changes have been saved.
Iflastis not specified
then it will be set to`first`. Iffirstis not specified
the default is the previous command
for editing and -16 for listing.
The option`-r`reverses the order of the commands and
the option`-n`suppresses command numbers when listing.
In the second form thecommandis re-executed after the substitutionold`=`newis performed.
* **jobs -lnp job ...** 

Lists information about each given job; or all active jobs ifjobis omitted.
The`-l`option lists process ids in addition to the normal information.
The`-n`option only displays jobs that have stopped or exited since last
notified.
The`-p`option causes only the process group to be listed.
SeeJobsfor a description of the format ofjob.
* **kill -s signame job ...** 


* **kill -n signum job ...** 


* **kill -l sig ...** 

Sends either the TERM (terminate) signal or the
specified signal to the specified jobs or processes.
Signals are either given by number with the`-n`option or by name with the`-s`option
(as given in`<signal.h>`, stripped of the prefix ``SIG'' with
the exception that SIGCLD is named CHLD).
For backward compatibility, the`n`and`s`can be omitted and the number or name placed immediately
after the`-`. If the signal being sent is TERM (terminate) or HUP (hangup),
then the job or process will be sent a CONT (continue) signal
if it is stopped.
The argumentjobcan be the process id of a process that is not a member of one of the
active jobs.
SeeJobsfor a description of the format ofjob. In the third form,`kill -l`, ifsigis not specified,
the signal names are listed.
Otherwise, for eachsigthat is a name, the corresponding signal number is listed.
For eachsigthat is a number, the signal name corresponding to the
least significant 8 bits ofsigis listed.
* **let arg ...** 

Eachargis a separatearithmetic expressionto be evaluated.
SeeArithmetic Evaluationabove, for a description of arithmetic expression evaluation.

The exit status is
0 if the value of the last expression
is non-zero, and 1 otherwise.
* **- newgrp arg ...** 

Equivalent to`exec /bin/newgrp`arg....
* **print -Rnprs -u unit -f format arg ...** 

With no options or with option`-`or`--`, Eachargis printed
on standard output.
The`-f`option causes the arguments to be printed as
described by`printf`. In this case any`n`,`r`,`R`options are ignored.
Otherwise,
unless the`-R`or`-r`, are specified, the following
escape conventions will be applied:

* **&bsol;a** 

The alert character (ascii`07`).
* **&bsol;b** 

The backspace character (ascii`010`).
* **&bsol;c** 

Causes`print`to end without processing more arguments and
not adding a new-line.
* **&bsol;f** 

The formfeed character (ascii`014`).
* **&bsol;n** 

The new-line character (ascii`012`).
* **&bsol;r** 

The carriage return character (ascii`015`).
* **&bsol;t** 

The tab character (ascii`011`).
* **&bsol;v** 

The vertical tab character (ascii`013`).
* **&bsol;E** 

The escape character (ascii`033`).
* **&bsol;&bsol;** 

The backslash character &bsol;.


The`-R`option will print all subsequent arguments and options
other than`-n`. The`-p`option causes the
arguments to be written onto the pipe
of the process spawned with`&verbar;&amp;`instead of standard output.
The`-s`option causes the
arguments to be written onto the history file
instead of standard output.
The`-u`option can be used to specify a one digit
file descriptor unit numberuniton which the
output will be placed.
The default is 1.
If the option`-n`is used, no`new-line`is added to the output.
* **printf format arg ...** 

The argumentsargare printed on standard output
in accordance with the ANSI-C
formatting rules associated with the format string`format`. The following extensions can also be used:

A`%b`format can be used instead of`%s`to cause escape sequences in the correspondingargto be expanded as described in`print.`

A`%P`format can be used instead of`%s`to causeargto be interpreted as an extended regular
expression and be printed as a shell pattern.

A`%q`format can be used instead of`%s`to cause the resulting string to be quoted in a manner than can
be reinput to the shell.

The precision field of the`%d`format can be followed by a
. and the output base.
* **pwd -LP** 

Outputs the value of the current working
directory.
If the`-P`option is given,
all symbolic links are resolved from the name.
* **read -Aprs -d delim -t timeout -u unit vname?prompt vname ...** 

The shell input mechanism.
One line is read and
is broken up into fields using the characters in
IFSas separators.
The escape character,`&bsol;`, is used to remove any special meaning for the next
character and for line continuation.
The`-d`option
causes the read to continue to the first character ofdelimrather than new-line.
In raw mode,`-r,`the`&bsol;`character is not treated specially.
The first
field is assigned to the firstvname, the second field
to the secondvname, etc., with leftover fields assigned to the lastvname. The`-A`option causes the variablevnameto be unset and each field that is read to be stored in
successive elements of the indexed arrayvname.The`-p`option causes the input line
to be taken from the input pipe
of a process spawned by the shell
using`&verbar;&amp;`. If the`-s`option is present,
the input will be saved as a command in the history file.
The option`-u`can be used to specify a one digit file
descriptor unitunitto read from.
The file descriptor can be opened with the`exec`special built-in command.
The default value of unitnis 0.
The option`-t`is used to specify a timeout in decimal
seconds when reading from a terminal or pipe.
Ifvnameis omitted then`REPLY`is used as the defaultvname. An end-of-file with the`-p`option causes cleanup for this process
so that another can be spawned.
If the first argument contains a`?`, the remainder of this word is used as aprompton standard error
when the shell is interactive.
The exit status is 0 unless an end-of-file is encountered
or read has timed out.
* **-- readonly -p vname=value ...** 

Ifvnameis not given
the names and values of each variable with
the readonly attribute is printed with the values
quoted in a manner that allows them to be re-inputed.
The`-p`option
causes the word`readonly`to be inserted before each one.
Otherwise, the givenvnames are marked
readonly and these
names cannot be changed
by subsequent assignment.
* **- return n** 

Causes a shell`function`or`.`script to return
to the invoking script
with the return status specified by`n`. The value will be the least significant 8 bits of the specified status.
Ifnis omitted then the return status is that of the last command executed.
If`return`is invoked while not in a`function`or a`.`script,
then it behaves the same as`exit`.
* **- set &plusmn;CPabefhkmnopstuvx &plusmn;o option ... &plusmn;A vname arg ...** 

The options for this command have meaning as follows:

* **-A** 

Array assignment.
Unset the variablevnameand assign values sequentially from the
listarg. If`+A`is used, the variablevnameis not unset first.
* **-C** 

Prevents redirection`>`from truncating existing files.
Files that are created are opened with the O_EXCL mode.
Require`>&verbar;`to truncate a file when turned on.
* **-P** 

Causes the`cd`and`pwd`built-in commands to default to physical mode.
* **-a** 

All subsequent variables that are defined are automatically exported.
* **-b** 

Prints job completion messages as soon as a background job changes
state rather than waiting for the next prompt.
* **-e** 

If a command has a non-zero exit status,
execute the`ERR`trap, if set,
and exit.
This mode is disabled while reading profiles.
* **-f** 

Disables file name generation.
* **-h** 

Each command
becomes a tracked alias when first encountered.
* **-k** 

(Obsolete). All variable assignment arguments are placed in the environment for a command,
not just those that precede the command name.
* **-m** 

Background jobs will run in a separate process group
and a line will print upon completion.
The exit status of background jobs is reported in a completion message.
On systems with job control,
this option is turned on automatically for
interactive shells.
* **-n** 

Read commands and check them for syntax errors, but do not execute them.
Ignored for interactive shells.
* **-o** 

The following argument can be one of the following option names:

* **allexport** 

Same as`-a`.
* **errexit** 

Same as`-e`.
* **bgnice** 

All background jobs are run at a lower priority.
This is the default mode.
* **emacs** 

Puts you in anemacsstyle in-line editor for command entry.
* **gmacs** 

Puts you in agmacsstyle in-line editor for command entry.
* **ignoreeof** 

The shell will not exit on end-of-file.
The command`exit`must be used.
* **keyword** 

Same as`-k`.
* **markdirs** 

All directory names resulting from file name generation have a trailing/appended.
* **monitor** 

Same as`-m`.
* **noclobber** 

Same as`-C`.
* **noexec** 

Same as`-n`.
* **noglob** 

Same as`-f`.
* **nolog** 

Do not save function definitions in history file.
* **notify** 

Same as`-b`.
* **nounset** 

Same as`-u`.
* **physical** 

Same as`-P`.
* **privileged** 

Same as`-p`.
* **verbose** 

Same as`-v`.
* **trackall** 

Same as`-h`.
* **vi** 

Puts you in insert mode of avistyle in-line editor
until you hit escape character`033`. This puts you in control mode.
A return sends the line.
* **viraw** 

Each character is processed as it is typed
invimode.
* **xtrace** 

Same as`-x`.
* **If no option name is supplied then the current option settings are printed.** 



* **-p** 

Disables processing of the$HOME/.profilefile and uses the file/etc/suid_profileinstead of the`ENV`file.
This mode is on whenever the effective uid (gid)
is not equal to the real uid (gid).
Turning this off causes the effective uid and gid to be
set to the real uid and gid.
* **-s** 

Sort the positional parameters lexicographically.
* **-t** 

(Obsolete). Exit after reading and executing one command.
* **-u** 

Treat unset parameters as an error when substituting.
* **-v** 

Print shell input lines as they are read.
* **-x** 

Print commands and their arguments as they are executed.
* **--** 

Do not change any of the options; useful in setting`$1`to a value beginning with`-`. If no arguments follow this option then the positional parameters are unset.

As an obsolete feature,
if the firstargis`-`then the`-x`and`-v`options are turned off and the nextargis treated as the first argument.
Using`+`rather than`-`causes these options to be turned off.
These options can also be used upon invocation of the shell.
The current set of options may be found in`$-`. Unless`-A`is specified,
the remaining arguments are positional
parameters and are assigned, in order, to`$1``$2`....
If no arguments are given then the names and values
of all variables are printed on the standard output.

* **- shift n** 

The positional parameters from`$``n``+1`...
are renamed`$1`...
, defaultnis 1.
The parameterncan be any arithmetic expression that evaluates to a non-negative
number less than or equal to`$#`.
* **sleep seconds** 

Suspends execution for the number of decimal seconds or fractions of a
second given byseconds.
* **- trap -p action sig ...** 

The -p
option causes the trap
action associated with each trap as specified by the arguments
to be printed with appropriate quoting.
Otherwise,actionwill be processed as if it were an argument to`eval`when the shell
receives signal(s)sig. Eachsigcan be given as a number or as the name of the signal.
Trap commands are executed in order of signal number.
Any attempt to set a trap on a signal that
was ignored on entry to the current shell
is ineffective.
Ifactionis omitted and the firstsigis a number, or ifactionis`-`, then the trap(s) for eachsigare reset
to their original values.
Ifactionis the null
string then this signal is ignored by the shell and by the commands
it invokes.
Ifsigis`ERR`thenactionwill be executed whenever a command has a non-zero exit status.
Ifsigis`DEBUG`thenactionwill be executed before each command.
Ifsigis`0`or`EXIT`and the`trap`statement is executed inside the body of a function,
then the commandactionis executed
after the function completes.
Ifsigis`0`or`EXIT`for a`trap`set outside any function
then the commandactionis executed
on exit from the shell.
Ifsigis`KEYBD`thenactionwill be executed whenever a key is read
while in`emacs`,`gmacs`, or`vi`mode.
The`trap`command
with no arguments prints a list
of commands associated with each signal number.
* **-- typeset &plusmn;AHflnprtux &plusmn;EFLRZin vname=value  ...** 

Sets attributes and values for shell variables and functions.
When invoked inside a function,
a new instance of the variablesvnameis created.
The variables' value and type are restored
when the function completes.
The following list of attributes may be specified:

* **-A** 

Declaresvnameto be an associate array.
Subscripts are strings rather than arithmetic
expressions.
* **-E** 

Declaresvnameto be a double precision floating point number.
Ifnis non-zero it defines the number of significant figures
that are used when expandingvname. Otherwise ten significant figures will be used.
* **-F** 

Declaresvnameto be a double precision floating point number.
Ifnis non-zero it defines the number of places after the
decimal point that are used when expandingvname. Otherwise ten places after the decimal point will be used.
* **-H** 

This option provides UNIX to host-name file mapping on non-UNIX
machines.
* **-L** 

Left justify and remove leading blanks from`value`. Ifnis non-zero it defines the width
of the field,
otherwise it is determined by the width of the value of
first assignment.
When the variable is assigned to, it is
filled on the right with blanks or truncated, if necessary, to
fit into the field.
The`-R`option is turned off.
* **-R** 

Right justify and fill with leading blanks.
Ifnis non-zero it defines the width
of the field,
otherwise it is determined by the width of the value of
first assignment.
The field is left filled with blanks or
truncated from the end if the
variable is reassigned.
The`-L`option is turned off.
* **-Z** 

Right justify and fill with leading zeros if
the first non-blank character is a digit and the`-L`option has not been set.
Remove leading zeros if the`-L`option is also set.
Ifnis non-zero it defines the width
of the field,
otherwise it is determined by the width of the value of
first assignment.
* **-f** 

The names refer to function names rather than
variable names.
No assignments can be made and the only other
valid options are`-t`,`-u`and`-x`. The option`-t`turns on execution tracing for this function.
The option`-u`causes this function to be marked undefined.
The`FPATH`variable will be searched to find the function definition
when the function is referenced.
* **-i** 

Declaresvnameto be represented internally as integer.
The right hand side of an assignment is evaluated as an
arithmetic expression when assigning to an integer.
Ifnis non-zero it defines the output arithmetic base,
otherwise the the output base will be ten.
* **-l** 

All upper-case characters are
converted to lower-case.
The upper-case option,`-u`is turned off.
* **-n** 

Declaresvnameto be a reference to the variable whose name is
defined by the value of variablevname. This is usually used to reference a variable inside
a function whose name has been passed as an argument.
* **-r** 

The givenvnames are marked
readonly and these
names cannot be changed
by subsequent assignment.
* **-t** 

Tags the variables.
Tags are user definable and have no special
meaning to the shell.
* **-u** 

All lower-case characters are converted
to upper-case characters.
The lower-case option,`-l`, is turned off.
* **-x** 

The givenvnames are marked for automatic
export to theenvironmentof subsequently-executed commands.
Variables whose names contain a &period;
can not be exported.

The`-i`attribute can not be specified along with`-R`,`-L`,`-Z`, or`-f`.

Using`+`rather than`-`causes these options to be turned off.
If novnamearguments are given
a list ofvnames(and optionally thevalues) of thevariablesis printed.
(Using`+`rather than`-`keeps the
values from being printed.)
The`-p`option causes`typeset`followed by the the option letters
to be printed before each name
rather than the names of the options.
If any option other than`-p`is given,
only those variables
which have all of the given
options are printed.
Otherwise, thevnames andattributesof allvariablesare printed.

* **ulimit -HSacdfmnpstv limit** 

Set or display a resource limit.
The available resources limits are listed below.
Many systems do not contain one or more of these limits.
The limit for a specified resource is set whenlimitis specified.
The value oflimitcan be a number in the unit specified below with each resource,
or the value`unlimited`. The`-H`and`-S`options specify whether the hard limit or the
soft limit for the given resource is set.
A hard limit cannot be increased once it is set. A soft
limit can be increased up to the value of the hard limit.
If neither the`H`or`S`options is specified, the limit applies to both.
The current resource limit is printed whenlimitis omitted.
In this case the soft limit is printed unless`H`is specified.
When more that one resource is specified, then the limit
name and unit is printed before the value.

* **-a** 

Lists all of the current resource limits.
* **-c** 

The number of 512-byte blocks on the size of core dumps.
* **-d** 

The number of K-bytes on the size of the data area.
* **-f** 

The number of 512-byte blocks on files that can be written the
current process or by child processes (files of any size may be read).
* **-m** 

The number of K-bytes on the size of physical memory.
* **-n** 

The number of file descriptors plus 1.
* **-p** 

The number of 512-byte blocks for pipe buffering.
* **-s** 

The number of K-bytes on the size of the stack area.
* **-t** 

The number of seconds to be used by each process.
* **-v** 

The number of K-bytes for virtual memory.

If no option is given,`-f`is assumed.

* **umask -S mask** 

The user file-creation mask is set tomask(seeumask(2)).`mask`can either be an octal number or
a symbolic value as described inchmod(1). If a symbolic value is given,
the new
umask value is the complement of the result of
applyingmaskto the complement of the previous umask value.
Ifmaskis omitted, the current value of the mask is printed.
The`-S`option causes the mode to be printed as a symbolic
value. Otherwise, the
mask is printed in octal.
* **- unalias -a name ...** 

Thealiasesgiven by the list of`name`s are removed from thealiaslist.
The`-a`option causes all the
aliases to be unset.
* **-unset -fv vname ...** 

The variables given by the list ofvnames are unassigned,
i.e.,
their values and attributes are erased.
Readonly variables cannot be unset.
If the`-f`option
is set, then the names refer tofunctionnames.
If the`-v`option is set, then the names refer tovariablenames.
The default is equivalent to`-v`. Unsetting`ERRNO`,`LINENO`,`MAILCHECK`,`OPTARG`,`OPTIND`,`RANDOM`,`SECONDS`,`TMOUT`, and`_`removes their special meaning even if they are
subsequently assigned to.
* **wait job ...** 

Wait for the specifiedjoband
report its termination status.
Ifjobis not given then all currently active child processes are waited for.
The exit status from this command is that of
the last process waited for.
SeeJobsfor a description of the format ofjob.
* **whence -apv name ...** 

For each`name`, indicate how it
would be interpreted if used as a command name.

The`-v`option
produces a more verbose report.

The`-p`option
does a path search fornameeven if name is an alias, a function, or a reserved word.
The`-a`is similar to the`-v`option but causes
all interpretations of the given name to be reported.

### Invocation.


If the shell is invoked byexec(2), and the first character of argument zero
(`$0`) is`-`, then the shell is assumed to be aloginshell and
commands are read from/etc/profileand then from either.profilein the current directory or$HOME/.profile, if either file exists.
Next, commands are read from
the file named by
performing parameter expansion, command substitution,
and arithmetic substitution on
the value of the environment variable`ENV`if the file exists.
If the`-s`option is not present andargis, then a path search is performed on the firstargto determine the name of the script to execute.
The scriptargmust have read permission and anysetuidandgetgidsettings will be ignored.
If the script is not found on the path,argis processed as if it named a built-in command or function.
Commands are then read as described below;
the following option are interpreted by the shell
when it is invoked:

* **-c** 

If the`-c`option is present then
commands are read from the firstarg. Any remaining arguments become
position parameters starting at`0`.
* **-s** 

If the`-s`option is present or if no
arguments remain
then commands are read from the standard input.
Shell output,
except for the output of theSpecialCommandslisted above,
is written to
file descriptor 2.
* **-i** 

If the`-i`option is present or
if the shell input and output are attached to a terminal (as told byioctl(2)) then this shell isinteractive. In this case TERM is ignored (so that`kill 0`does not kill an interactive shell) and INTR is caught and ignored
(so that`wait`is interruptible).
In all cases, QUIT is ignored by the shell.
* **-r** 

If the`-r`option is present the shell is a restricted shell.


The`-I`filenameoption is used
to generate a cross reference database
that can be used by a separate utility
to find definitions and references for variables and commands.

The remaining options and arguments are described under the`set`command above.
An optional`-`as the first argument is ignored.
### Rksh Only


Rkshis used to set up login names and execution environments whose
capabilities are more controlled than those of the standard shell.
The actions of`rksh`are identical to those of`ksh`, except that the following are disallowed:changing directory (seecd(1))setting or unsetting the value or attributes of`SHELL`,`ENV`, or`PATH`specifying path or
command names containing/redirecting output
(`>`,`>|`,`<>`, and`>>`)

The restrictions above are enforced
after.profileand the`ENV`files are interpreted.

When a command to be executed is found to be a shell procedure,
invokessh}{.B rksh
invokesksh}to execute it.
Thus, it is possible to provide to the end-user shell procedures
that have access to the full power of
the standard shell,
while imposing a limited menu of commands;
this scheme assumes that the end-user does not have write and
execute permissions in the same directory.

The net effect of these rules is that the writer of the.profilehas complete control over user actions,
by performing guaranteed setup actions
and leaving the user in an appropriate directory
(probablynotthe login directory).

The system administrator often sets up a directory
of commands
(i.e.,/usr/rbin) that can be safely invoked by`rksh`.
## EXIT


Errors detected by the shell, such as syntax errors,
cause the shell
to return a non-zero exit status.
Otherwise, the shell returns the exit status of
the last command executed (see also the`exit`command above).
If the shell is being used non-interactively
then execution of the shell file is abandoned.
Run time errors detected by the shell are reported by
printing the command or function name and the error condition.
If the line number that the error occurred on is greater than one,
then the line number is also printed in square brackets
(`[]`) after the command or function name.
## FILES
/etc/passwd/etc/profile/etc/suid_profile$HOME/.profile/tmp/sh&ast;/dev/null
## SEE


cat(1),
cd(1),
chmod(1),
cut(1),
echo(1),
emacs(1),
env(1),
gmacs(1),
newgrp(1),
stty(1),
test(1),
umask(1),
vi(1),
dup(2),
exec(2),
fork(2),
ioctl(2),
lseek(2),
paste(1),
pipe(2),
umask(2),
ulimit(2),
wait(2),
rand(3),
a.out(5),
profile(5),
environ(7).

Morris I. Bolsky and David G. Korn,The KornShell Command and Programming Language, Prentice Hall, 1989, ISBN 0-13-516972-0.
## CAVEATS


If a command
is executed, and then a command with the same name is
installed in a directory in the search path before the directory where the
original command was found, the shell will continue toexecthe original command.
Use the`-t`option of the`alias`command to correct this situation.

Some very old shell scripts contain a`&caret;`as a synonym for the pipe character`&verbar;`.

Using the`hist`built-in command within a compound command will cause the whole
command to disappear from the history file.

The built-in command`.`filereads the whole file before any commands are executed.
Therefore,`alias`and`unalias`commands in the file
will not apply to any commands defined in the file.

Traps are not processed while a job is waiting for a foreground process.
Thus, a trap on`CHLD`won't be executed until the foreground job terminates.