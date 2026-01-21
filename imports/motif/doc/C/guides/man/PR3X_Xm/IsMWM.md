# XmIsMotifWMRunning
library call`XmIsMotifWMRunning`A function that determines whether the window manager is runningXmIsMotifWMRunning#include <Xm/Xm.h>Boolean`XmIsMotifWMRunning`Widgetshell
## DESCRIPTION


`XmIsMotifWMRunning`lets a user know
whether the Motif Window Manager is
running on a screen that contains a specific widget hierarchy.
This
function first sees whether the_MOTIF_WM_INFOproperty is present on
the root window of the shell's screen. If it is, its window
field is used to query for the presence of the specified window as a
child of root.

* **`shell`** 

Specifies the shell whose screen will be tested formwm's presence.

## RETURN


Returns True if MWM is running.