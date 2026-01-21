dtksh CommandscommandsThis appendix contains a list of the commands supported bydtksh. Many of these commands are almost identical to their Motif,
Xt Intrinsics, or Xlib counterparts. Commands that return a value must have
the return variable as an environment variable that is the first parameter
in the call. Some commands have more differences.The following subsections give a synopsis of each of thedtkshcommands. In general, parameter ordering and types are the same
as for corresponding C procedures; exceptions are noted. For more detail
on the functionality and parameters of a command, see the standard documentation
for the corresponding Xlib, Xt Intrinsics, or Motif procedure.In the command definitions, parameters namedvar,var2,var3, and so on, indicate that the
shell script should supply the name of an environment variable into which
some value will be returned. The wordvariableindicates an environment variable that accepts a return value.Commands that return a Boolean value (which can be used directly as
part of anifstatement), are noted as such.Parameters enclosed within[]are optional.Built-in Xlib CommandsXBelldisplay volumeXClearAreadisplay drawable[optional GC arguments]x y width heightexposuresXClearWindowdisplay drawableXCopyAreadisplay src dest
srcX srcY width height destX destY[optional GCarguments]XDefineCursordisplay window cursorXDrawArcdisplay drawable[optional GC arguments]x y width height
angle1angle2XDrawLinedisplay drawable[optional GC arguments]x1 y1 x2 y2XDrawLinesdisplay drawable[-coordinateMode] [optional GC arguments]x1 y1x2 y2 [x3 y3 ...]wherecoordinateModeis eitherCoordModeOriginorCoordModePrevious.XDrawPointdisplay drawable[optional GC arguments]x yXDrawPointsdisplay drawable[-coordinateMode] [optional GC
arguments]x1 y1[x2 y2 x3 y3 ...]wherecoordinateModeis eitherCoordModeOriginorCoordModePrevious.XDrawRectangledisplay drawable[optional GC arguments]x y width heightXDrawSegmentsdisplay drawable[optional GC arguments]x1 y1 x2 y2[x3 y3 x4y4 ...]XDrawStringdisplay drawable[optional GC arguments]x y stringXDrawImageStringdisplay
drawable[optional GC arguments]x y stringXFillArcdisplay drawable[optional GC arguments]x y width height
angle1angle2XFillPolygondisplay drawable[-shape] [-coordinateMode]
[optional GCarguments]x1 y1 x2 y2...whereshapeis eitherComplex,Convex,orNonconvex,andcoordinateModeis eitherCoordModeOriginorCoordModePrevious.XFillRectangledisplay drawable[optional GC arguments]x y width heightXFlushdisplayXHeightOfScreenvariablescreenXRaiseWindowdisplay windowXRootWindowOfScreenvariablescreenXSyncdisplaydiscardwherediscardis eithertrueorfalse.XTextWidthvariablefontNamestringTheXTextWidthcommand is different from the corresponding
Xlib procedure because it takes the name of a font instead of a pointer to
a font structure.XUndefineCursordisplay windowXWidthOfScreenvariablescreenBuilt-in Xt Intrinsic CommandscommandsXt IntrinsicsXt Intrinsics
commandsAll the Xt Intrinsics commands used to create a new widget require that
you specify a widget class for the new widget. The widget (or gadget) class
name is the standard class name provided by Motif. For example, the class
name for a Motif push button widget isXmPushButton, while
the class name for the Motif label gadget isXmLabelGadget.XtAddCallbackwidgetHandlecallbackNameksh-commandXtAddCallbackwherecallbackNameis one of the standard Motif
or Xt callback names, with theXtorXmprefix dropped. For example,activateCallback.XtAddEventHandlerwidgetHandleeventMasknonMaskableFlagksh-commandXtAddEventHandlerwhereeventMaskis of the formmask|mask|maskand themaskcomponents are
any of the standard set of X event masks, andnonMaskableFlagis eithertrueorfalse.XtAddInputvariable[-r]fileDescriptorksh-commandXtAddInputRegisters the indicated file descriptor with the X Toolkit as an alternate
input source. It is the responsibility of the shell script's input handler
to unregister the input source when it is no longer needed and to close the
file descriptor.If the-roption is specified (rawmode), thendtkshdoes not automatically read any of the
data available from the input source; it will be up to the specified kshell
command to read all data. If the-roption is not specified,
then the command specified inksh-commandis invoked
only when a full line is read (that is, a line terminated by either an unescaped
newline character or the end of the file) or when the end of the file is
reached. The raw mode is useful for handlers that expect to process nontextual
data, or for handlers that do not wantdtkshautomatically
reading in a line of data. When the end of file is detected, it is the shell
script's input handler's responsibility to useXtRemoveInputto remove the input source and to close the file descriptor,
if necessary.In all cases, several environment variables are set up, which can be
used by the handler. These include: