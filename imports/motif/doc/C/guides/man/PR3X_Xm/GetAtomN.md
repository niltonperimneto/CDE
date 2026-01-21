# XmGetAtomName
library call`XmGetAtomName`A function that returns the string representation for an atomXmGetAtomNameatoms#include <Xm/Xm.h>
#include <Xm/AtomMgr.h>String`XmGetAtomName`Display* displayAtomatom
## DESCRIPTION


`XmGetAtomName`returns the string representation for an atom.
It mirrors theXlibinterfaces for atom management but provides
client-side caching. When and where caching is provided inXlib, the
routines will become pseudonyms for theXlibroutines.

* **`display`** 

Specifies the connection to the X server
* **`atom`** 

Specifies the atom for the property name you want returned

## RETURN


Returns a string.
The function allocates space to hold the returned string.
The application is responsible for managing the allocated space.
The application can recover the allocated space by calling`XFree`.