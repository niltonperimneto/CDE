Message GuidelinesRefer to the information in this appendix to write messages that are
easily internationlized.File-Namingfile, naming conventionsConventionsThe conventionsmessagesfile-naming
conventionsused in naming files with user messages
are discussed here. Usually, the message source file has the suffix.msg; the generated message catalog has the suffix.cat. There may be other such files related to messages.
The following criteria must be met for a file to have these suffixes:It is X/Open-compliant.It becomes a*.catfile through the use of thegencatcommand.Cause and Recoverymessagescause and recovery informationInformationWhenever possible, explain to users exactly what has happened and what
they can do to remedy the situation.The messageBad argis not very helpful. However,
the following message tells users exactly what to do to make the command
work:Do not specify more than 2 files on the command lineSimilarly, the messageLine too longdoes not give
users recovery information. However, the following message gives users more
specific recovery information:Line cannot exceed 20 charactersIf detailed recovery information is necessary for a given error message,
add it to the appropriate place in online information or help.Seefor samples of original
and rewritten messages.Comment Lines for TranslatorsA messagemessagescomment
lines for translatorssource file should contain comments
to help the translator in the process of translation. These comments will
not be part of the message catalog generated. The comments are similar to
C language comments to help document a program. A dollar sign ($) followed
by a space will be interpreted by the translation tool and thegencatcommand as comments. The following is an example
of a comment line in a message source file.$ This is a commentUse comment lines to tell translators and writers what variables, such
as%s,%c, and%d,represent. For example, note whether the variable refers to such things as
a user, file, directory, or flag.Place the comment line directly beneath the message to which it refers,
rather than at the bottom of the message catalog. Global comments for an
entire set can be placed directly below the $set directive in the source
file.Specify in a comment line any messages within the message catalog that
are obsolete.Programming Formatmessagesprogramming formatFor the programming format of messages, see the following list.Do not construct messages from
clauses. Use flags or other means within the program to pass information
so that a complete message can be issued at the proper time.Do not use hardcoded English text as a variable
for a%sstring in an existing message. This is also
the construction of messages and is not translatable.Capitalize the first word of the sentence, and
use a period at the end of the sentence or phrase.End the last line of the message with
(backslash followed by a lowercase n, indicating a new line). This also applies
to one-line messages.Begin the second and remaining lines of a message
with\t(backslash
followed by a lowercase t, indicating a tab).End all other lines with\n\(backslash followed by a lowercase n, followed by another backslash, indicating
a new line).If, for some reason, the message should not end
with a new line, use a comment to tell the writers.Precede each message with the name of the command
that called the message, followed by a colon. The command name should precede
the component number in error messages. The command name is shown in the
following example as it should appear in a message:>OPIE &ldquo;foo: Opening the file.&rdquo;Writing StyleThe following guidelines on the writing style of messages include terminology,
punctuation, mood, voice, tense, capitalization, and other usage questions.messageswriting style inUse sentence format. One-line
and one-sentence messages are preferable.Add articles (a,an,the) when necessary to eliminate ambiguity.Capitalize the first word of the sentence and use
a period at the end.Use the present tense. Do not allow future tense
in a message. For example, use the sentence:The foo command displays a calendar.Instead of:The foo command will display a calendar.Do not use the first person (Iorwe) anywhere in messages.Avoid using the second person.Do not use the wordyouexcept in help and interactive
text.Use active voice. The first line is the original
message. The second line is the preferred wording.MYNUM &ldquo;Month and year must
be entered as numbers.&rdquo;MYNUM &ldquo;foo: 7777-222 Enter month and year
as numbers.\n&rdquo;7777-222 is the message ID.Use the imperative mood (command phrase) and active
verbs:specify,use,check,choose, andwaitare examples.State messages in a positive tone. The first line
is the original message. The second line is the preferred wording.BADL &ldquo;Don't use the f option
more than once.&rdquo;BADL &ldquo;foo: 7777-009 Use the -f flag only once.\n&rdquo;Do not use nouns as verbs. Use words only in the
grammatical categories shown in the dictionary. If a word is shown only as
a noun, do not use it as a verb. For example, do notsolutiona problem (or, for that matter,architecta system).Do not use prefixes or suffixes. Translators may
not understand words beginning withre-,un-,in-, ornon-, and
the translations of messages that use these prefixes or suffixes may not
have the meaning you intended. Exceptions to this rule occur when the prefix
is an integral part of a commonly used word. The wordspreviousandprematureare acceptable; the wordnonexistent,is not.Do not use plurals. Do not use parentheses to show
singular or plural, as inerror(s),which cannot be
translated. If you must show singular and plural, writeerror or
errors. A better way is to condition the code so that two different
messages are issued depending on whether the singular or plural of a word
is required.Do not use contractions. Use the single wordcannotto denote something the system is unable to do.Do not use quotation marks. This includes both
single and double quotation marks. For example, do not use quotation marks
around variables such as%s,%c,
and%dor around commands. Users may take the quotation
marks literally.Do not hyphenate words at the end of lines.Do not use the standard highlighting guidelines
in messages, and do not substitute initial or all caps for other highlighting
practices.Do not useand/or. This construction
does not exist in other languages. Usually it is better to sayorto indicate that it is not necessary to do both.Use the 24-hour clock. Do not usea.m.orp.m.to specify time. For example, write1:00 p.m.as1300.Avoid acronyms. Only use acronyms that are better
known to your audience than their spelled-out versions. To make a plural
of an acronym, add a lowercases, without
an apostrophe. Verify that it is not a trademark before using it.Avoid the &ldquo;no-no&rdquo; words. Examples areabort,argument,
andexecute. See the project glossary.Retain meaningful terminology. Keep as much of
the original message text as possible while ensuring that the message is
meaningful and translatable.Usage StatementsThemessagesusage statements
inusage statement is generated by commands when at
least one flag that is not valid has been included in the command line. The
usage statement must not be used if only the data associated with a flag
is missing or incorrect. If this occurs, an error message unique to the problem
is used.Show the command syntax in the
usage statement. For example, a possible usage statement for thedelcommand reads:Usage: del {File ...|-}Clauses defining the purpose of a command are to
be removed.Capitalize the first letter of such words (parameters)
asFile, Directory, String, Number,and so on only when
used in a usage statement.Do not abbreviate parameters on the command line.
It may be perfectly obvious to experienced users thatNummeansNumber, but spell it out to ensure correct translation.usage statements, delimitersUse only the following delimiters in usage statements:DelimiterDescription[]Parameter is optional.{ }There is more than one parameter choice, but one of the parameters
is required. (See the following text.)|Choose one parameter only. [a|b] indicates that you can chooseaorbor neitheranorb. {a|b} indicates that you must choose eitheraorb...Parameter can be repeated on the command line. (Note that there
is a space before the ellipsis.)-Standard input.A usage statement parameter does not require square
brackets or braces if it is required and is the only choice, as in the following:banner StringIn usage statements, put a space between flags
that must be separated on the command line. For example:unget [-n] [-rSID] [-s] {File|-}If flags can be used together without a separating
space, do not separate them with a space on the command line. For example:wc [-cwl] {File ...|-}When the order of flags on the command line does
not make a difference, put them in alphabetical order. If the case is mixed,
put lowercase versions first:get -aAijlmMSome usage statements can be long and involved.
Use your best judgment to determine where you should end lines in the usage
statement. The following example shows an old-style usage statement for thegetcommand:Usage: get [-e|-k] [-cCutoff] [-iList]
[-rSID] [-wString] [xList] [-b] [-gmnpst] [-l[p]] File ... Retrieves a specified
version of a Source Code Control System (SCCS) file.Standard Messagesmessagespunctuation and wording
guidelinesCertain commands have standard errors defined
in POSIX.2 documentation. Follow the guidelines set up in POSIX.2, if applicable.Tell the user toPress the ------ keyto select
a key on the keyboard, including the specific key to press (such as,Press Ctrl-D).Unless the system is overloaded, there is no need
to tell the user toTry againlater. That should be obvious from the
message.When writing message text, use the wordparameterto describe text on the command line; use the wordvalueto indicate numeric data.Use the wordflagrather than the wordscommand option.Do not use commas to set off the one-thousandth
place in values.Do not use 1,000. Use 1000.If a message must be set off with an asterisk,
use two asterisks at the beginning of the message and two asterisks at the
end of the message.** Total **Uselog inandlog offas verbs.Log in to the system; enter the
data; then log off.Useuser name,group name, andloginas nouns.The user name is sam. The group
name is staff. The login directory is /u/sam.User number and group number refer to the number
associated with the user's name and group.Do not use the termsuperuser.
Theroot usermay not have all privileges.Use the wordscommand stringto describe the command with its parameters.Many of the same messages occur frequently.messagesoptionTable
A-1 lists the new standard message that replaces the old message.New Standard MessagesUse the Following
Standard MessagesInstead of These MessagesCannot find or open
the file.Can't open filename.Cannot find or access
the file.Can't accessThe syntax of a parameter
is not valid.syntax errorRegular Expression Standard MessagesTable A-2 lists the standard regular expression error messages, including
the message number associated with each regular expression error:Regular Expression Standard MessagesNumberUse These Standard MessagesInstead of These Messages11Specify a range end point that is less than 256.Range end point too large.16The character or characters between \{ and \} must be numeric.Bad number.25Specify a \digit between 1 and 9 that is not
greater than the number of subpatterns.\digit out of range.36A delimiter is not correct or is missing.Illegal or missing delimiter.41There is no remembered search string.No remembered search string.42There is a missing \( or \).\(\) imbalance.43Do not use \( more than 9 times.Too many \(.44Do not specify more than 2 numbers between \{ and \}.More than two numbers given in \{ and \}.45An opening \{ must have a closing \}.} expected after \.46The first number cannot exceed the second number between \{
and \}.First number exceeds second in \{ and \}.48Specify a valid end point to the range.Invalid end point in range expression.49For each [ there must be a ].[ ] imbalance.50The regular expression is too large for internal memory storage.
Simplify the regular expression.Regular expression overflow.Sample MessagesThese are examplesmessagessamplesof original messages and rewritten messages.
The rewritten message follows each original message.AFLGKEYLTRS &ldquo;Too Many -a Keyletters (Ad9)&rdquo;

AFLGKEYLTRS &ldquo;foo: 7777-007 Use the -a flag less than 11 times.\n&rdquo;
FLGTWICE &ldquo;Flag %c Twice (Ad4)&rdquo;

FLGTWICE &ldquo;foo: 7777-004 Use the %c header flag once.\n&rdquo;
ESTAT &ldquo;can't access %s.\n&rdquo;

ESTAT &ldquo;foo: 7777-031 Cannot find or access %s.\n&rdquo;
EMODE &ldquo;foo: invalid mode\n&rdquo;

EMODE &ldquo;foo: 7777-033 A mode flag or value is not correct.\n&rdquo;
DNORG &ldquo;-d has no argument (ad1)&rdquo;

DNORG &ldquo;foo: 7777-001 Specify a parameter after the -d flag.\n&rdquo;
FLOORRNG &ldquo;floor out of range (ad23)&rdquo;

FLOORRNG &ldquo;foo: 7777-021 Specify a floor value greater than 0\n\

\tand less than 10000.\n&ldquo;
AFLGARG &ldquo;bad -a argument (ad8)&rdquo;

AFLGARG &ldquo;foo: 7777-006 Specify a user name, group name, or\n\

\tgroup number after the -a flag.\n&ldquo;
BADLISTFMT &ldquo;bad list format (ad27)&rdquo;

BADLISTFMT &ldquo;foo: 7777-025 Use numeric version and release\
\tnumbers.\n&rdquo;