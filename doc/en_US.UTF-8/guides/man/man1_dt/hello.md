dthellouser cmddthelloCDE login
transitional greetingdthello-display<display>-fground<color>-bground<color>-font<fontname>-string<message>-file<filename>-timeout<seconds>DESCRIPTIONThe dthello client provides transition visuals from the end of login
to the start of the window manager in the user's session.Upon invocation, the dthello client will create an override-redirect
window the size of the screen and draw a specified message on it. At the same
time, a 1x1 window is created that will be picked up by the window manager.
When the window manager reparents the little window (an indication that the
window manager has started), this program exits.The message may be specified on the command line, or in a text file.OPTIONSThedthelloclient is designed to be started by the system and
is not intended to be started directly by users.-displaydisplayDisplay id.-fgroundcolorForeground color.-bgroundcolorBackground color.-fontfontnameFont.-stringmessageString to be displayed in window.-filefilenameText file name whose contents will be displayed in window.
This option may be specified up to five times.-timeoutsecondsNumber of seconds before giving up on the window manager
and terminating.RETURNExit values are:0Successful completion.>0Error condition occurred.EXAMPLESdthello-stringWelcome
to the DesktopTransition window will contain this message.RESOURCESNOTE: Resources should be prefaced with the string "Dthello*" when specified.
Resources should be specified in the Dthello app-defaults file.NameClassTypeDefaultvbackgroundVbackgroundPixeldynamicvforegroundVforegroundPixeldynamicvfontVfontFontListdynamicstringStringString.........fileFileStringNULLtimeoutTimeoutInteger240