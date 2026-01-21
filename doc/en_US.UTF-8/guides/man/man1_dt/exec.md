# dtexec
user cmddtexecexecute command-based actiondtexecdtexec-openopen_option-ttprocidprocid-tmptmpfilecmdcmd_arg ...
## DESCRIPTION


&str-XZ; uses thedtexecutility to execute`COMMAND`actions.
In normal
usage, &str-XZ; automatically invokesdtexecwhen it is needed.
&str-XZ; users
and administrators do not need to interact directly withdtexec.

The primary argument fordtexecis the name of a command to execute and
any command-line options or arguments for that command.
There are
several important services thatdtexecprovides for the commands that it executes.
These services are controlled via the options specified in the ``OPTIONS''
section in this document.
## OPTIONS


The following options are available:

* **`-open`open_option** 

The`-open`option tellsdtexecwhether it should continue to run or
exit after the command terminates.

The valid values foropen_optionare:

* **`-1`** 

(default) continue to execute aftercmdterminates.
* **`0`** 

exit as soon ascmdterminates.
* **`n`** 

continue to execute ifcmdterminates
within`n`seconds of starting.


The values`-1`and`n`are typically used when`COMMAND`actions are executed in a terminal emulator to control
the lifetime of the terminal window.
* **`-ttprocid`procid** 

Thedtexeccommand uses the`-ttprocid`option
to send ToolTalk messages
back to the application that invoked the`COMMAND`action.
These
messages are used to convey status information (for example, the command
has terminated).
* **`-tmp`tmpfile** 

The`-tmp`option names a temporary file thatdtexecremoves aftercmdterminates if either no-ttprocidoption is
supplied or contact is lost with the specifiedprocid.
In normal usage, the parent process that startsdtexec, performs this cleanup.
This option can be issued multiple times, once for each tmp file present.
Typically,tmpfilealso appears as acmd_arg.

## EXIT STATUS


The following exit values are returned:

* **`0`** 

Implies all system resources were available to fork and exec the requested
command, not that the requested command executed successfully.
* **`1`** 

An error occurred.

## APPLICATION USAGE


Normally, application programs do not directly invoke this program.
Normally, an application program links with the
action service, which then invokesdtexecas needed.
Applications should use the`waitTime`resource to configure the value of the-open_optionand the`DtexecPath`resource to configure
the location ofdtexec.

Ifcmdwrites to stderr, the error messages are time stamped
and redirected to the user's errorlog file ($HOME/.dt/errorlog)
whenopen_optionis`0`. Ifopen_optionis set
otherwise, the error messages are typically displayed in the
terminal window.
## SEE ALSO


&cdeman.dtaction;, &cdeman.dtactionfile;.