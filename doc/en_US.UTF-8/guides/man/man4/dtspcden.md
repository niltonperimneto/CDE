# dtspcdenv
special filedtspcdenvenvironment Variable File for the CDE Subprocess Control Service
## DESCRIPTION


The CDE Subprocess Control service provides the
ability for a process running on one system to
invoke another process on a different system.
By default, the environment variables of the
parent process are passed unmodified to the
child process (on the remote host);
however, there are some environment
variables that are given special handling.
In
addition, the user or system administrator can
specify environment variables that should be
modified when they are passed to the child process.

The system-wide environment files are:/etc/dt/config/dtspcdenvand/usr/dt/config/dtspcdenvand the user-specific environment file is:$HOME/.dt/dtspcdenv.
The file/usr/dt/config/dtspcdenvis created
when CDE is installed.
This file may be overwritten
by subsequent CDE installations.
Consequently, local
variable definitions should be placed in the file/etc/dt/config/dtspcdenvbecause this file is not
overwritten during installation.

The environment variable files are evaluated in the following order:local host:/usr/dt/config/dtspcdenvlocal host:/etc/dt/config/dtspcdenvlocal host:$HOME/.dt/dtspcdenvremote host:/usr/dt/config/dtspcdenvremote host:/etc/dt/config/dtspcdenvremote host:$HOME/.dt/dtspcdenv

The precedence occurs in the reverse order of
evaluation.
Thus, variables in the remote
host's$HOME/.dt/dtspcdenvfile have the highest
precedence and variables in the local host's/etc/dt/config/dtspcdenvfile have the lowest
precedence.
In this context, the local host is the
host where a CDE client is running and
the remote host is the host where a remote
process will be started (on behalf of
the local client).

Lines beginning with a numer symbol (#) are considered
comments and are not processed.

The syntax for a non-commented line is:VAR_NAME=some_value

whereVAR_NAMEis the name of an
environment variable andsome_valueis the value assigned to the variable.

Ifsome_valuecontains a variable reference,
the reference will be replaced by the variable's
value.
For example, if a CDE client has the
following definition in its environment:PATH=/bin:/sbin:/usr/local/bin

and the following definition occurs in one of the
environment files:PATH=/opt/foo/bin:$PATH

then before the remote process is executed,`PATH`will be expanded to:PATH=/opt/foo/bin:/bin:/sbin:/usr/local/bin

The environment variable names may consist of
letters, digits or underscores and may be
enclosed in curly braces.
The environment variable files may contain
"unset <variable_name>" commands to prevent
an environment variable from being propagated
to the remote process.
For example, the following line would
prevent the variableLIB_PATHfrom being
propagated to the remote process:unset LIB_PATH
## ENVIRONMENT VARIABLES


Thedtspcddaemon treats the following variables specially:

* **`DISPLAY`** 

If`DISPLAY`is set to "unix:<n>", "local:<n>" or ":<n>"
(where <n> is the screen number), then before the
remote process is executed,`DISPLAY`is changed to the name of the local host and
the screen number is preserved.
* **`HOME`** 

Before the remote process is executed,`HOME`is set
to the value of the user's home directory in
the password file on the remote host.
* **`SHELL`** 

If`SHELL`is not in the parent processes environment,`SHELL`is set to the value of the user's shell in
the password file on the remote host.
* **`PWD`** 

The variable`PWD`is not propagated to
the remote host.
Note that the above variables will be
overridden by their corresponding definitions
in an environment file.

## EXAMPLES


The following definition sets the variableFOO_BIN_PATHto the value/var/foo/bin:`FOO_BIN_PATH=/var/foo/bin`
## FILES


* **/usr/dt/config/dtspcdenv** 

System-wide, installed environment variable definitions
used when a process is executed
* **/etc/dt/config/dtspcdenv** 

System-wide, locally defined environment variable definitions
used when a process is executed
* **$HOME/.dt/dtspcdenv** 

User-specific environment variable definitions used
when a process is executed

## SEE ALSO


&cdeman.dtspcd;.