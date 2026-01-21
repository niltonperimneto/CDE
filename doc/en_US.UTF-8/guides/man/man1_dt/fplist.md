# dtfplist
user cmddtfplista general purpose utility for printing the hierarchy of the front panel componentsdtfplist
## DESCRIPTION


Thedtfplistutility provides a textual view of what the front panel will look like without
restarting the window manager.
Thedtfplistutility provides the ability to get an ASCII text version of the front panel
hierarchy.
This utility will print the hierarchy to standard out.
## OPTIONS


None.
## OPERANDS


None.
## RESOURCES


None.
## STDIN


Not used.
## INPUT FILES


None.
## ENVIRONMENT VARIABLES


`DTDATABASESEARCHPATH`
## ASYCHRONOUS EVENTS


None.
## STDOUT


The following is the format of hierachy to be written to standard out:`PANEL`<panel name>`BOX`<box name>`CONTROL`<control name>`SUBPANEL`<subpanel name>`CONTROL`<control name>
                                        ...`CONTROL`<control name>
                        ...`BOX`<box name>
                ...

The hierarchy that will be written to standard out, is the one that will be
used when the front panel is created during the invocation ofdtwm.
## STDERR


Not used.
## OUTPUT FILES


None.
## EXTENDED DESCRIPTION


None.
## EXIT STATUS


None.
## CONSEQUENCES OF ERRORS


If the`DTDATABASESEARCHPATH`is not specified, this utility will not generate
any output.
## APPLICATION USAGE


This can be used to debug changes to the front panel configuration files.
## EXAMPLES


None.
## SEE ALSO


&cdeman.dtwm;