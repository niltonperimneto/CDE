# dthelptag
user cmddthelptagcompile CDE Help source documents into runtime Help volumesdthelptagoptionsfileparser-options
## DESCRIPTION
This utility has been superseded by thedtdocbookutility,
which converts source documents that conform to the DocBook 2.2.1 DTD
(Document Type Definition) subelement PART to documents
that conform to the SDL 1.2 DTD. The converted DocBook documents
are readable by the DtHelp viewer.

Thedthelptagutility is the CDE Help System compiler for translating
HelpTag source markup into the online distribution format suitable for
runtime display.
See theCDE Help System Author's and Programmer's
Guidefor a description of the HelpTag markup language.
See
&cdeman.dthelpview; for more information on
previewing compiled Help volumes.

Thedthelptagutility accepts a single file name as an argument.
If the file name
contains a period (``.''), any characters after the last period are
considered to be the extension.
Thedthelptagutility removes all characters after the last period and uses the
resulting name as the base name for all intermediate files and for the
final output files.

If the`file`argument has no periods,dthelptaguses the argument as the base name for intermediate and output files
and assumes an extension of.htg,.ctgor.sdlfor the input file.
The.ctgextension is assumed when the`-formal`option described below is used.
The.sdlextension is assumed when the`-compress`or`-decompress`option (described later in this document) is used.

Several options todthelptagmay precede the file name.
Several arguments directing the
parsing phase of thedthelptagprocess may follow the file name.

The output file is:`file`.sdl - the compiled help volume file.
## OPTIONS


The following options are available:

* **`-verbose`** 

The`-verbose`option will causedthelptagto generate and display parser messages during processing.
* **`-formal`** 

The`-formal`option causesdthelptagto accept a subset of the HelpTag language that is strictly compliant
to canonical SGML.
(See theCDE Help System Author's and Programmer's
Guide.)
When this option is given, the default extension of the
input file becomes.ctg.
* **`-nooptimize`** 

The`-nooptimize`option eliminates certain optimizations that normally take place
during translation of HelpTag markup to the runtime format.
Using
this option speeds the translation process.
* **`-clean`** 

The`-clean`option causesdthelptagto simply remove any intermediate files from the current directory.
No translation takes place.
* **`-debug`** 

The`-debug`option causesdthelptagto leave all intermediate files in the current directory.
The`-debug`option also blocks the compression step ofdthelptag, leaving the resulting.sdloutput file in a human-readable form.
* **`-files`** 

The`-files`option causes a list of files referenced in the translation process to
be emitted to the standard output.
No translation takes place.
* **`-help`** 

The`-help`option causesdthelptagto emit a synopsis of thedthelptagcommand line and a list of options to the standard output.
* **`-decompress`** 

The`-decompress`option causesdthelptagto decompress a previously created.sdlfile.
When this option is specified, the default input extension is.sdl.
* **`-compress`** 

The`-compress`option causesdthelptagto compress a.sdlfile that either was created by translating a.htgor.ctgfile using the`-debug`option or was previously decompressed using the`-decompress`option.
When this option is specified, the default input extension is.sdl.

### Parser Options


Anyparser optionsfollow the`file`argument on the command line and take the form`option=value`for those options taking an argument and simply`option`for those options not taking an argument.
Parser options may also be set in the environment variable`DTTAGOPT`, in a`helptag.opt`file or in a file named`file`.optin the current directory.
The`helptag.opt`file may reside in the current directory or in the directory in whichdthelptagis placed.

The order of precedence of the option settings is:

The file`helptag.opt`in thedthelptaginstallation directory.
This directory defaults to/usr/dt/bin.

The environment variable`DTTAGOPT`.

The file`helptag.opt`in the current directory.

The file`file`.optin the current directory.

The command line.

Parser options set later in the list override options set earlier.

The parser options supported bydthelptagare:

* **`onerror=go`** 

Cause errors to be non-fatal.
That is, parsing continues and later
phases of thedthelptagprocess are run even if syntax errors were encountered.
* **`onerror=stop`** 

This is the default setting of the`onerror=`option.
It causes thedthelptagprocess to stop upon completion of the parser phase if syntax errors were
encountered during the parse.
* **`charset=name`** 

The default character set used by the
help system is ISO8859-1.
A different character set may be specified, for example,`name`,
using the`charset=`option.
The character set may also be set in the`helplang.ent`file described in theCDE Help System Author's and Programmer's Guide.
* **`search=path`** 

Specifies one or more directory`path(s)`to be searched when executingdthelptag. Bothdthelptaginput files and/or additional graphics or entity declaration
files referenced within the HelpTag markup can be made accessible by setting
this option.
The`search=`option may be specified more that once and the list of directories to
search is accumulated.
* **`clearsearch`** 

Clears the list of directories searched for file and
image entities.
* **`memo`** 

The`memo`option causes authors' comments to be included in the output.
* **`nomemo`** 

Specifies the inverse of the`memo`option.
Both`memo`and`nomemo`may be specified, but the last entry will override any previous setting.
* **`shortfiles`** 

Neither the`shortfiles`parser-optionnor any of its synonyms should be used.
Rather, the`-shortfiles`option should be given as anoptiontodthelptag. Thedthelptagdriver needs to know whether the user has requested short file names sincedthelptagmust know the names of the intermediate files.
* **`shortfile`** 

This is a synonym for`shortfiles`.
* **short** 

This is a synonym for`shortfiles`.
* **`longfiles`** 

This option and any of its synonyms should not be used for the same
reason that the`shortfiles`option should not be used.
* **`longfiles`** 

Long, untruncated file names are the default.
* **`longfile`** 

This is a synonym for`longfiles`.
* **long** 

This is a synonym for`longfiles`.

## ENVIRONMENT VARIABLES


`LANG`determines the language in which the input`file`is interpreted.
The`LANG`environment variable can be overridden in the`helplang.ent`file described in theCDE Help System Author's and Programmer's Guide.

`DTTAGOPT`may be used to set parser options.
### International Code Set Support


Single- and multi-byte character code sets are supported.
## INPUT FILES


Following are the input files used by thedthelptagparser:

* **file.htg** 

Default input file.
* **`file.ctg`** 

Default input file when the`-formal`option has been specified.
* **`file.st`** 

Status file and log.
* **`helplang.ent`** 

Character set information and localizable replacement text.
* **`helptag.opt`** 

Option file.

## OUTPUT FILES


Following are the input files used by thedthelptagparser:

* **file.sdl** 

Runtime help volume
* **`file.err`** 

Run log and error listing

## EXTENDED DESCRIPTION


Thedthelptagutility is a driver program that executes two phases of the
compilation process.
The first phase translates the source markup
into the distribution format.
The second phase enhances the
distribution file by precomputing information such as a list of
identifiers in the file and their locations.
These precomputations,
along with several optimizations, enable rapid runtime display of
the file.
The second phase of the translation process also compresses
the distribution file to reduce file system space required to store
the file.
## EXIT STATUS


The following exit values are returned:

* **0** 

Successful completion.
* **1** 

An error was detected in the source file.
* **2** 

An invocation error was detected.

## CONSEQUENCES OF ERRORS


Default.
## EXAMPLES


* **`dthelptag -clean myFile.htg`** 

Remove all files previously generated by processing a source file
ofmyFile.htg.
* **`dthelptag myFile.htg onerror=go`** 

Process the filemyFile.htg, not stopping even if there are syntax errors.
* **`dthelptag myFile.htg`** 

Process the filemyFile.htg.

## SEE ALSO


&cdeman.dthelpview;,CDE Help System Author's and Programmer's Guide.