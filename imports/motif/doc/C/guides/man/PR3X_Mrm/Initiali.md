# MrmInitialize
library call`MrmInitialize`Prepares an application to use MRM widget-fetching facilitiesMrmInitializeuil functionsMrmInitializevoid`MrmInitialize`MRM functionMrmInitializeMrmInitializedefinition
## DESCRIPTION


The`MrmInitialize`function must be called to prepare an application to use MRM
widget-fetching facilities.
You must call this function prior to fetching a widget.
However, it is good programming practice to call`MrmInitialize`prior to performing any
MRM operations.

`MrmInitialize`initializesMrmInitializedescriptionthe internal data structures that MRM needs to
successfully perform type
conversion on arguments and to successfully access widget creation
facilities.
An application must call`MrmInitialize`before it uses other
MRM functions.