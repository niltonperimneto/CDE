# ttsnoop
demosttsnoopsend and receive ToolTalk messages interactivelyttsnoop-t
## DESCRIPTION


Thenttsnooputility allows
its user to create and send custom constructed ToolTalk messages, and to
selectively monitor any or all ToolTalk messages on your system.ttsnoopmenu options are as follows:Snoop -- Turn on/off tracing; get version informationMessage -- Create, open, receive, and destroy messagesPattern -- Create, open, and destroy patternsFile -- Numerous tasks, including joining a fileSession -- Join a specific session; set the default sessionPtype -- Declare and undeclare aptype; determine whether
aptypeexistsTypes -- Generate a list of declared types; generate a list of ToolTalk-based actionsProcid -- Open, close, suspend, and resume a procidlibc -- Callsystem(),putenv(),chdir(),pause(), andexit(); useexit()to exitttsnoop
## OPTIONS


The following command-line options are available:

* **`-t`** 

Print trace output. Of particular interest is that ttsnoop
will print example ToolTalk API code in the invoking OpenWindows
cmdtool (or console if ttsnoop is invoked via a menu choice)
showing what ToolTalk API calls are being used to construct
a particular pattern or message.

## SEE ALSO


&cdeman.ttsession;