# XmInternAtom
library call`XmInternAtom`A macro that returns an atom for a given nameXmInternAtomatoms#include <Xm/AtomMgr.h>Atom`XmInternAtom`Display* displayStringnameBooleanonly_if_exists
## DESCRIPTION


`XmInternAtom`returns an atom for a given name.
The returned atom remains defined even after the client's
connection closes. The returned atom becomes undefined when
the last connection to the X server closes.

* **`display`** 

Specifies the connection to the X server
* **`name`** 

Specifies the name associated with the atom you want returned.
The value of`name`is case dependent.
* **`only_if_exists`** 

Specifies a Boolean value.
If False, the atom is created even if it does not exist.
(If it does not exist, the returned atom will be`None`.)
If True, the atom is created only if it exists.

## RETURN


Returns an atom.