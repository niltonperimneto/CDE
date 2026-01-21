# tttracefile
special filetttracefilescript of settings for ToolTalk tracing`tttracefile`
## DESCRIPTION


Atttracescript contains settings that control ToolTalk calls and messages.
Atttracescript consists of commands separated by semicolons or newlines.
The first command must be the`version`command.
### Commands


If conflicting values are given for a setting, the last value wins.

* **version n** 

The version of the`tttracefile`command syntax used. The current version is 1.
* **follow [off | on]** 

Sets whether to follow all children forked by the traced client or
subsequently started in the traced session. Default is`off`.
* **[> | >>] outfile** 

File to be used for the trace output. By default, trace output
goes to standard error. Normalsh(1) interpretation of>and>>applies.
* **functions [all | none | func...]** 

ToolTalk API functions to trace.funcmay includesh(1) wildcard characters. Default is`all`.
* **attributes [all | none]** 

`none`means use only a single line when printing a message on the trace
output.`all`means print all attributes, arguments, and context slots of traced
messages. Default is`none`.
* **states [none | edge | deliver | dispatch | Tt_state]...** 

State(s) through which to trace messages. In addition to
theTt_states defined intt_c.h, valid`state`s
are:

none- disable all message tracing

edge- messages entering initial (TT_SENT) and final
(TT_HANDLED,TT_FAILED) states.

deliver- all state changes and all client deliveries.

dispatch-deliver+ all patterns considered for
matching. (default)
* **`ops`toTrace...** 
* **`sender_ptypes`toTrace...** 
* **handler_ptypes toTrace...** 

Trace messages that havetoTraceas a value for the indicated message attribute.toTracemay includesh(1) wildcard characters.
If notoTraceargument is included for a given message attribute, then
no value of that attribute excludes a message from tracing.

### Comments


A word beginning with`#`causes that word and all the following characters up to a newline to
be ignored.
## EXAMPLES


To trace all attribute-getting and -setting messages sent by ptype starting
with "Dt",

* **** 

`version`1`ops`Get* Set*`sender_ptypes`Dt*

To trace only ToolTalk function calls (but not messages) in a
process tree,
* **** 

`version 1; follow on; states none`

## SEE ALSO


&cdeman.ttsession;, &cdeman.tttrace;, the`Session_Trace`() ToolTalk request