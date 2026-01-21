dttermfile formatsdttermescape
sequences recognized by dtterm and the DtTerm widgetDESCRIPTIONReceived Escape SequencesThe &cdeman.dtterm; utility and the DtTerm widget support
the following list of received escape sequences. Spaces have been added for
readability and are not part of the escape sequence. The following indicate
parameters:pi,p1,label,fileandtext.Spaceindicates a required space, hexadecimal
code 0x20. A <control>-charindicates
a control code (such as <control>-G, which is hexadecimal code 0x07).Escindicates hexadecimal code 0x1b.Backslashindicates hexadecimal code 0x5c. Literals are indicated asliteraland must be included exactly as specified. All references to the &cdeman.dtterm; utility in this man page also apply to the DtTerm widget.<control>-G(BEL) Bell. The terminal
either issues an audible bell, or flashes the text window depending on the
state of the visual bell flag.<control>-H(BS) Backspace. The cursor
moves one cursor position to the left. If reverse-wrap mode is disabled and
the cursor is at the left-most column of the line when a backspace character
is received, the cursor remains at its current position. If reverse-wrap mode
is enabled and the cursor is at the left-most column of the line when a backspace
character is received, the cursor moves to the right-most column of the previous
line. If the cursor is also in the top-most row, the cursor moves to the right-most
column of the bottom-most row.<control>-I(HT) Horizontal Tab. The
cursor moves right to the next tab stop. If there are no further tab stops
set to the right of the cursor, the cursor moves to the right-most column
of the current line.<control>-J(LF) Line Feed or New Line.
The cursor moves to the same column of the next line. If the cursor is in
the bottom-most line of the scrolling region, the scrolling region scrolls
up one line. Lines scrolled off the top of the scrolling region are lost.
Blank lines with no visible character attributes are added at the bottom of
the scrolling region.<control>-K(VT) Vertical Tab. Same as
Line Feed.<control>-L(FF) Form Feed or New Page.
Same as Line Feed.<control>-M(CR) Carriage Return. The
cursor moves to the left-most column of the current line.Esc( B(SCS) Designate ASCII (base
font) as G0.Esc( 0(SCS) Designate DEC Special
Graphic (line draw) as G0.Esc) B(SCS) Designate ASCII (base
font) as G1.Esc) 0(SCS) Designate DEC Special
Graphic (line draw) as G1.Esc* B(SCS) Designate ASCII (base
font) as G2.Esc* 0(SCS) Designate DEC Special
Graphic (line draw) as G2.Esc+ B(SCS) Designate ASCII (base
font) as G3.Esc+ 0(SCS) Designate DEC Special
Graphic (line draw) as G3.<control>-N(LS1) Map G1 into GL.<control>-O(LS0) Map G0 into GL.Escn(LS2) Map G2 into GL.Esco(LS3) Map G3 into GL.EscN(SS2) Map G2 into GL for
the next character.EscO(SS3) Map G3 into GL for
the next character.Esc SpaceF(S7C1T) Select 7-bit C1 Control
Characters. In this mode, thedttermutility sends all
C1 Control Characters to the host as 7-bit escape sequences. That is,CSIis sent to the host as ``Esc[''.Esc SpaceG(C8C1T) Select 8-bit C1 Control
Characters. In this mode, thedttermutility sends all
C1 Control Characters to the host as 8-bit control codes. That is,CSIis sent back as the hexadecimal value 0x9B.Esc# 8(DECALN) DEC Screen Align
Test. The screen is filled with the character ``E''.Esc7(DECSC) Save cursor. The
following is saved:Cursor positionCharacter attributes set by theSGRcommandAny pending single shift 2 or 3 (SS2orSS3)State of the autowrap flagState of the reverse wrap flagState of origin mode (DECOM)State of selective eraseEsc8(DECRC) Restore cursor. The
terminal emulator is restored to the state saved by the save cursor (DECSC) function. If nothing was saved byDECSC, then the following actions are performed:Moves the cursor to the home positionResets the origin mode (DECOM)Turns off all character attributes (SGR)Maps the ASCII character set into GLEsc=(DECPAM) Application keypad.
In this mode, the numeric keypad sends application sequences. (See the ``Transmitted
Escape Sequences'' section later in this document for additional information.)Esc>(DECPNM) Normal keypad. In
this mode, the numeric keypad sends the characters shown on the keypad. KeysPF1toPF4,
inclusive, send application sequences. (See the ``Transmitted Escape Sequences''
section later in this document for additional information.)Esc`D`(IND) Index. The cursor moves
down to the same column of the next line. If the cursor is in the bottom-most
line of the scrolling region, the scrolling region is scrolled up one line.
The line scrolled off the top of the scrolling region is lost. A blank line
with no visible character attributes is added at the bottom of the scrolling
region.Esc`E`(NEL) Next line. The cursor
moves down to the first column of the next line. If the cursor is in the bottom-most
line of the scrolling region, the scrolling region is scrolled up one line.
The line scrolled off the top of the scrolling region is lost. A blank line
with no visible character attributes is added at the bottom of the scrolling
region.Esc`H`(HTS) Tab set. This function
sets a horizontal tab stop at the column where the cursor is located.Esc`M`(RI) Reverse index. The cursor
moves up to the same column of the previous line. If the cursor is in the
top-most line of the scrolling region, the scrolling region is scrolled down
one line. The line scrolled off the bottom of the scrolling region is lost.
A blank line with no visible character attributes is added at the top of the
scrolling region.EscPp1;p2|p3 Esc Backslash(DECUDK) User defined keysEsc`Z`(DECID) Return terminal ID.
This function is similar to a primary device attributes (DA) request. (See ``Esc[ c''
(DA) described later in this document.)Escc(RIS) Full reset. This function
performs a full (hard) reset. For additional information, see the ``Reset''
section in this man page.Esc[piq(DECSCA) Select character
protection attribute. The default value is 0. This escape sequence defines
the characters that come after it as erasable or not erasable from the screen.
The selective erase escape sequences, (DECSEDandDECSEL), can only
erase characters defined as erasable. Valid supported values ofpiare:0DECSEDandDECSELcan erase characters.1DECSEDandDECSELcannot erase characters.2Same as0.Esc[pi@(ICH) Insertpiblank characters. The default value is 1. A parameter value of
0 or 1 inserts a single blank character. A parameter value ofNinsertsNblank characters. Blank characters
with normal character attributes are inserted at the cursor position. Characters
to the right of the cursor move to the right. Characters scrolled past the
end of the line are lost.Esc[pi`A`(CUU) Cursor uppilines. The default value is 1. A parameter value 0 or 1 moves
the cursor up one line. A parameter value ofNmoves
the cursor upNlines. The cursor stops at the top margin.
If the cursor is already above the top margin, the cursor stops at the top
line.Esc[pi`B`(CUD) Cursor downpilines. The default value is 1. A parameter value 0 or 1 moves
the cursor down one line. A parameter value ofNmoves
the cursor downNlines. The cursor stops at the bottom
margin. If the cursor is already below the bottom margin, the cursor stops
at the bottom line.Esc[pi`C`(CUF) Cursor forwardpicharacters. The default value is 1. A parameter value 0 or 1
moves the cursor forward one character. A parameter value ofNmoves the cursor forwardNcharacters. The cursor stops
at the right-most column of the line.Esc[pi`D`(CUB) Cursor backwardpicharacters. The default value is 1. A parameter value 0 or 1
moves the cursor backward one character. A parameter value ofNmoves the cursor backwardNcharacters. The
cursor stops at the left-most column of the line.Esc[pi`F`(CPL) Cursor to the first
column of thepithprecedingline.The default value is 1. A parameter value 0 or 1 moves the cursor to the preceding
line. A parameter value ofNmoves the cursor to theNth preceding line. If the cursor is below the top margin, the
cursor stops at the top margin. If the cursor is already above the top margin,
the cursor stops at the top line.Esc[pi`G`(CHA) Cursor to columnpi. The default value is 1. A parameter value 0 or 1 moves the
cursor to the first column of the current line. A parameter value ofNmoves the cursor to theNth column of the
current line.Esc[p1;p2`H`(CUP) Cursor position. The
default value is 1. Ap1value 0 or 1 moves the cursor
to row one. Ap1value ofNmoves
the cursor to row N. Ap2value 0 or 1 moves the cursor
to column one. Ap2value ofNmoves
the cursor to column N. The starting point for lines and columns depends on
the setting of the origin mode (DECOM).Esc[pi`J`(ED) Erase in display. The
default value is 0. A parameter value of 0 erases from the cursor to the end
of the display. A parameter value of 1 erases from the beginning of the display
to the cursor position, inclusive. A parameter value of 2 erases the complete
display.Esc[pi`K`(EL) Erase in line. The default
value is 0. A parameter value of 0 erases from the cursor to the end of the
line. A parameter value of 1 erases from the beginning of the line to the
cursor position, inclusive. A parameter value of 2 erases the complete line.Esc[pi`L`(IL) Insert lines. The default
value is 1. A parameter value 0 or 1 inserts one line at the cursor. A parameter
value ofNinsertsNlines at the
cursor. As lines are inserted, lines below the cursor and in the scrolling
region move down. Lines scrolled off the page are lost. There is no effect
outside the scrolling region.Esc[pi`M`(DL) Delete lines. The default
value is 1. A parameter value 0 or 1 deletes one line at the cursor. A parameter
value ofNdeletesNlines at the
cursor. As lines are deleted, lines below the cursor and in the scrolling
region move up. Blank lines with no visible character attributes are added
at the bottom of the scrolling region. There is no effect outside the scrolling
region.Esc[pi`P`(DCH) Delete characters.
The default value is 1. A parameter value 0 or 1 deletes one character at
the cursor position. A parameter value ofNdeletesNcharacters at the cursor position. An parameter greater than
the number of characters between the cursor and the right margin only deletes
the remaining characters on the line. As characters are deleted, the remaining
characters move left and are replaced by blank spaces with no visual character
attributes.Esc[pi`S`(SU) Scroll uppilines. The default value is 1. A parameter value 0 or 1 scrolls
the display up one line. A parameter value ofNscrolls
the display upNlines. The scrolling region scrolls
up. Lines scrolled off the top of the scrolling region are lost. Blank lines
with no visible character attributes are added at the bottom of the scrolling
region.Esc[pi`T`(SD) Scroll downpilines. The default value is 1. A parameter value 0 or 1 scrolls
the display down one line. A parameter value ofNscrolls
the display downNlines. The scrolling region scrolls
down. Lines scrolled off the bottom of the scrolling region are lost. Blank
lines with no visible character attributes are added at the top of the scrolling
region.Esc[pi`X`(ECH) Erasepicharacters. The default value is 1. A parameter value 0 or 1 erases
a single character. A parameter value ofNerasesNcharacters. The character attributes of erased characters are
cleared. This escape sequences works inside or outside the scrolling margins.Esc[pic(DA) Send device attributes.
The default is 0. A parameter value 0 or 1 causes the terminal emulator to
respond with ``Esc[ ? 1; 2 c''.
This is the standard response for the DEC VT100 Terminal andxterm(1).Esc[p1;p2f(HVP) Horizontal and vertical
position. This escape sequence has been replaced byCUPand offers identical functionality. It is provided to maintain
backward compatibility.Esc[pig(TBC) Tab clear. The default
is 0. A parameter value of 0 clears the tab stop at the current cursor column.
A parameter value of 3 clears all tab stops.Esc[pih(SM) Set mode. This escape
sequence sets ANSI modes. Valid supported values ofpiare:2(KAM) Keyboard lock. In this
mode, &cdeman.dtterm; ignores all keystrokes from the keyboard.4(IRM) Insert mode. In this
mode, new characters move characters in display memory to the right. Characters
moved past the end of the line are lost.12(SRM) Local echo off. In
this mode, &cdeman.dtterm; sends keyboard characters to the host
only. The host must echo back characters for them to be displayed.20(LNM) New line. In this mode,
the cursor moves to the first column on the next line when &cdeman.dtterm; receives anLF,FForVTcharacter. When the Return key is pressed, &cdeman.dtterm; sends
a carriage-return (CR) followed
by a newline (NL).Esc[pil(RM) Reset mode. This escape
sequences resets ANSI modes. Valid supported values ofpiare:2(KAM) Keyboard unlock. In
this mode, &cdeman.dtterm; processes all keystrokes from the keyboard.4(IRM) Replace mode. In this
mode, new characters replace the character at the cursor position.12(SRM) Local echo on. In this
mode, &cdeman.dtterm; sends keyboard characters to both the host
and the display. The host does not have to echo back characters for them to
be displayed.20(LNM) New line. In this mode,
the cursor moves to the same column on the next line when &cdeman.dtterm; receives anLF,FForVTcharacter. When the Return key is pressed, &cdeman.dtterm; sends
a carriage-return (CR).Esc[pi; ... m(SG) Graphics rendition.
The default value is 0. This escape sequence selects one or more character
attributes. Valid supported values forpiare:0All attributes off1Bold2Faint4Underline5Blinking. This attribute appears as bold text7Negative image8Invisible image22Bold and Faint off24Underline off25Blinking off27Negative image off28Invisible image off30Black display (text)31Red display (text)32Green display (text)33Yellow display (text)34Blue display (text)35Magenta display (text)36Cyan display (text)37White display (text)39Default display (text)40Black background41Red background42Green background43Yellow background44Blue background45Magenta background46Cyan background47White background49Default backgroundEsc[pin(DSR) Device status report.
Valid supported values forpiare:5Operating status. The &cdeman.dtterm; utility responds with
anOKmessage of ``Esc[ 0 n''.6(CPR) Cursor position report.
The &cdeman.dtterm; utility responds with the current cursor position
in the form ``Esc[p1;p2`R`''
wherep1is the current cursor line andp2is the current cursor row.Esc[?pin(DSR) DEC private device
status report. Valid supported values forpiare:15Printer port status. The &cdeman.dtterm; utility responds
with a ``no printer available'' message of ``Esc[?13 n''.25User-defined key status. The &cdeman.dtterm; utility responds
with either a message of ``Esc[?20 n'' if UDKs are unlocked, or ``Esc[?21 n'' if UDKs are locked.26Keyboard status. The &cdeman.dtterm; utility responds with
a message of ``Esc[?27
; 1 n'', which indicates a North American keyboard.Esc[p1;p2r(DECSTBM) Set top and bottom
margins. The default value forp1is 1. The default value
forp2is the current number of lines in the terminal
window. The top and bottom margins are set top1andp2respectively. Scrolling is not performed outside the margins.Esc[p1;p2;p3tWindow manipulation. Valid values forp1(and any
additional parameters) are:1Restore (de-iconify) window.2Minimize (iconify) window.3 ;x;yMove window to [x,y].4 ;height;widthResize the &cdeman.dtterm; window toheightandwidthin pixels.5Raise the &cdeman.dtterm; window to the front of the stacking
order.6Lower the &cdeman.dtterm; window to the bottom of the stacking
order.7Refresh the &cdeman.dtterm; window.8;height;widthResize the text area toheightandwidthin characters.11Report &cdeman.dtterm; window state. If the &cdeman.dtterm; window is open (non-iconified), it returns ``Esc[ 1 t''. If the &cdeman.dtterm; window is iconified,
it returns ``Esc[ 2 t''.13Report the &cdeman.dtterm; window position. The terminal
emulator returns ``Esc[ 3 ;x;yt''.14Report the &cdeman.dtterm; window in pixels. The terminal
emulator returns ``Esc[ 4 ;height;widtht''.18Report the size of the area in characters. The terminal emulator returns
``Esc[ 8 ;height;widtht''.20Report the &cdeman.dtterm; window's icon label. The terminal
emulator returns ``Esc] Llabel Esc Backslash''.21Report the &cdeman.dtterm; window's title. The terminal emulator
returns ``Esc] ltitle
Esc Backslash''.Esc[pixRequest terminal modes. The default value is 0. Valid values are 0 or
1. Ifpiis 0, &cdeman.dtterm; responds with
the message of ``Esc[ 2; 1; 1; 112; 112
; 1; 0 x''. Ifpiis 1, &cdeman.dtterm;
responds with the message of ``Esc[ 3; 1;
1; 112; 112; 1; 0x''. This escape sequence is supported for
backward compatibility forxterm(1) only.Esc[?pih(SM) DEC private set mode.
This escape sequences sets DEC private modes. Valid supported values ofpiare:1(DECCKM) Enable cursor keys
mode. When cursor keys mode is enabled, the arrow keys send application sequences
to the host.3(DECCOLM) Enable 132-column
mode. When 132-column mode is enabled, the number of columns is the terminal
window changed to 132. When entering into 132-column mode, the left, right,
top, and bottom margins are reset to their default positions and the display
is cleared.4(DECSCLM) Enable smooth scrolling.
When smooth scrolling is enabled, lines are added and the screen is scrolled
a single line at a time.5(DECSCNM) Enable reverse
video. When reverse video mode is enabled, the foreground and background colors
of the terminal window are reversed.6(DECOM) Enable origin mode.
When origin mode is enabled, the home cursor position is the upper-left corner
of the screen, within the margins. The starting point for line numbers depends
on the current top margin. The cursor cannot be moved outside the top and
bottom margins.7(DECAWM) Enable autowrap.
When autowrap mode is enabled, characters received when the cursor is at the
right-most column of the page are inserted at the beginning of the next line.
If the cursor is at the bottom line of the scrolling region, the page is scrolled
up 1 line.8(DECARM) Enable auto-repeat
keys. This option is ignored.25(DECTCEM) Enable cursor visible.
In this mode, the text cursor is visible.40EnableDECCOLMescape sequence.
When theDECCOLMescape sequence
is enabled, the terminal emulator switches into either an 80- or 132-column
window when it receives aDECCOLMescape sequence.44Enable margin bell. When the margin bell is enabled, thedttermutility's bell (either audible or visible) is invoked when the
cursor is a predefined distance from the right margin and a key is pressed.45Enable reverse-autowrap mode. When reverse-autowrap mode is enabled,
and a backspace is received when the cursor is at the left-most column of
the page, the cursor is wrapped to the right-most column of the previous line.
If the cursor is at the top line of the scrolling region, the cursor is wrapped
to the right-most column of the bottom line of the scrolling region. If the
cursor is at the top line of terminal window, the cursor is wrapped to the
right-most column of the bottom line of the terminal window.46Enable logging. When logging is enabled, all text received from the
child process is logged to a file.Esc[?pil(RM) DEC private mode reset.
This escape sequence sets DEC private modes. Valid supported values ofpiare:1(DECCKM) Disable cursor keys
mode. When cursor keys mode is disabled, the arrow keys send ANSI cursor sequences
to the host.3(DECCOLM) Disable 132-column
mode. When 132-column mode is disabled, the number of columns is the terminal
window changed to 80. When entering into 80-column mode, the left, right,
top, and bottom margins are reset to their default positions and the display
is cleared.4(DECSCLM) Disable smooth
scrolling. When smooth scrolling is disabled, lines are added and the screen
is scrolled up to a full screen at a time depending on how fast text is received
from the child process.5(DECSCNM) Disable reverse
video. When reverse video mode is disabled, the foreground and background
colors of the terminal window are not reversed.6(DECOM) Disable origin mode.
When origin mode is disabled, the home cursor position is the upper-left corner
of the screen. The starting point for line numbers is independent of the current
top margin. The cursor can be moved outside the top and bottom margins.7(DECAWM) Disable autowrap.
When autowrap mode is enabled, characters received when the cursor is at the
right-most column of the page, replace the character already on the line.8(DECARM) Disable auto-repeat
keys. This option is ignored.25(DECTCEM) Disable cursor
visible. In this mode, the text cursor is invisible.40DisableDECCOLMescape sequence.
When theDECCOLMescape sequence
is disabled, the terminal emulator ignores theDECCOLMescape sequence and does not switch into either an 80-
or 132-column window when it is received.44Disable margin bell. When the margin bell is disabled, thedttermutility's bell is not invoked when the cursor is a pre-defined
distance from the right margin and a key is pressed.45Disable reverse-autowrap mode. When reverse-autowrap mode is disabled,
and a backspace is received when the cursor is at the left-most column of
the page, the cursor remains at that position.46Disable logging. When logging is disabled, text received from the child
process is not logged to a file.Esc[ ?pirRestore DEC private mode values. The value corresponding to modepipreviously saved is restored. Valid values forpiare the same as the DEC private modes supported bySM. It is provided to maintain backward compatibility
withxterm(1). Using this escape sequence is discouraged.Esc[ ?pisSave DEC private mode values. The value corresponding to modepiis saved. Valid values forpiare the same
as the DEC private modes supported bySM.
This escape sequence is provided to maintain backward compatibility withxterm(1). Using this escape sequence is discouraged.Esc]p1;p2 <control>-GSet text parameters. This escape sequence allows various terminal emulator
text values to be set. Valid supported values ofp1are:0Change the icon name and window title to the stringp2.1Change the icon name to the stringp2.2Change the window title to the stringp2.3Set the current working directory to the stringp2.
The terminal emulator tries to restart in this directory when it is restarted
in a new session.Esc&caret;message Esc Backslash(PM) Privacy message. The
data received in a privacy message is ignored and is not displayed.Esc_pi Esc Backslash(APC) Application program
command. The terminal emulator implements noAPCfunctions. The data is ignored and is not displayed.Esc[ ?pi`K`(DECSEL) Selective erase
in line. The default value is 0. This escape sequence only erases erasable
characters in a single line of text. Only those characters defined as erasable
by theDECSCAescape sequence are
erased. A parameter value of 0 erases from the cursor to the end of the line.
A parameter value of 1 erases from the beginning of the line to the cursor
position, inclusive. A parameter value of 2 erases the complete line.Esc[ ?pi`J`(DECSED) Selective erase
in display. The default value is 0. This escape sequence only erases erasable
characters in the display. Only those characters defined as erasable by theDECSCAescape sequence are erased. A parameter
value of 0 erases from the cursor to the end of the display. A parameter value
of 1 erases from the beginning of the display to the cursor position, inclusive.
A parameter value of 2 erases the complete display.Esc] ltext Esc BackslashSet the window title totext.Esc] Ifile Esc BackslashSet the icon to the icon found in file.Esc] Llabel Esc BackslashSet the icon name tolabel.Esc[ ! p(DECSTR) Soft terminal reset.
This function performs a soft reset. For additional information, see the ``Reset''
section in this man page.ResetThe &cdeman.dtterm; utility supports two levels of reset:
full reset and soft reset. Reset can be invoked by menu buttons, the keyboard
or by escape sequences. Soft reset performs the following actions:Turns on the text cursor (DECTCEM)Enables replace mode (IRM)Turns off origin mode (DECOM)Turns on autowrap (DECAWM)Turns off reverse wrapUnlocks the keyboard (KAM)Sets the cursor keypad mode to normal (DECCKM)Sets the numeric keypad mode to numeric (DECNKM)Sets the top and bottom margins to the first and
last lines of the window (DECSTBM)Sets all character sets (GL,G0,G1,G2andG3) to ASCIITurns off all character attributes (SGR)Sets selective erase mode off (DECSCA)Clears any cursor state information saved with
save cursor (DECSC)Full reset performs the same functions as soft reset along with the
following actions:Cursor is moved to the home positionClears the screenClears user defined keys (DECUDK)Turns off reverse video (DECSCNM)Turns off auto linefeed mode (LNM)Turns on jump scroll (DECSCLM)Transmitted Escape SequencesCursor Key ModeThe cursor keys transmit the following escape sequences depending on
the setting of the mode specified, either via theappCursorDefaultresource, or the mode specified via theDECCKMescape sequence.KeyNormalApplicationCursor UpEsc[ AEscO ACursor DownEsc[ BEscO BCursor RightEsc[ CEscO CCursor LeftEsc[ DEscO DApplication Keypad ModeThe application keypad transmits the following escape sequences depending
on the setting of the mode specified, either via theappKeypadDefaultresource, or the mode specified via theDECPNMescape sequence.KeyNumericApplicationSpaceSpaceEscO ATabTabEscO IEnterCREscO MPF1EscO PEscO PPF2EscO QEscO QPF3EscO REscO RPF4EscO SEscO S*(multiply)*EscO j+(add)+EscO k,(comma),EscO l-(minus)-EscO m/(divide)/EscO o00EscO p11EscO q22EscO r33EscO s44EscO t55EscO u66EscO v77EscO w88EscO x99EscO y= (equal)=EscO XANSI Function KeysThe function keys transmit the following escape sequences unless Sun
function keys mode has been selected, either via thedtterm-skoption, or thesunFunctionKeysresource
in &cdeman.dtterm; or the DtTerm widget.KeyEscape SequenceF1Esc[ 1 1
~F2Esc[ 1 2
~F3Esc[ 1 3
~F4Esc[ 1 4
~F5Esc[ 1 5
~F6Esc[ 1 7
~F7Esc[ 1 8
~F8Esc[ 1 9
~F9Esc[ 2 0
~F10Esc[ 2 1
~F11Esc[ 2 3
~F12Esc[ 2 4
~F13Esc[ 2 5
~F14Esc[ 2 6
~F15Esc[ 2 8
~F16Esc[ 2 9
~F17Esc[ 3 1
~F18Esc[ 3 2
~F19Esc[ 3 3
~F20Esc[ 3 4
~HelpEsc[ 2 8
~MenuEsc[ 2 9
~FindEsc[ 1 ~InsertEsc[ 2 ~DeleteEsc[ 3 ~RemoveEsc[ 3 ~SelectEsc[ 4 ~PriorEsc[ 5 ~NextEsc[ 6 ~Sun Function KeysKeyEscape SequenceF1Esc[ 2 2
4 zF2Esc[ 2 2
5 zF3Esc[ 2 2
6 zF4Esc[ 2 2
7 zF5Esc[ 2 2
8 zF6Esc[ 2 2
9 zF7Esc[ 2 3
0 zF8Esc[ 2 3
1 zF9Esc[ 2 3
2 zF10Esc[ 2 3
3 zF11Esc[ 1 9
2 zF12Esc[ 1 9
3 zF13Esc[ 1 9
4 zF14Esc[ 1 9
5 zF15Esc[ 1 9
6 zF16Esc[ 1 9
7 zF17Esc[ 1 9
8 zF18Esc[ 1 9
9 zF19Esc[ 2 0
0 zF20Esc[ 2 0
1 zF21 (R1)Esc[ 2 0
8 zF22 (R2)Esc[ 2 0
9 zF23 (R3)Esc[ 2 1
0 zF24 (R4)Esc[ 2 1
1 zF25 (R5)Esc[ 2 1
2 zF26 (R6)Esc[ 2 1
3 zF27 (R7)Esc[ 2 1
4 zF28 (R8)Esc[ 2 1
5 zF29 (R9)Esc[ 2 1
6 zF30 (R10)Esc[ 2 1
7 zF31 (R11)Esc[ 2 1
8 zF32 (R12)Esc[ 2 1
9 zF33 (R13)Esc[ 2 2
0 zF34 (R14)Esc[ 1 2
1 zF35 (R15)Esc[ 1 2
2 zHelpEsc[ 1 9
6 zMenuEsc[ 1 9
7 zFindEsc[ 1 zInsertEsc[ 2 zDeleteEsc[ 3 zRemoveEsc[ 3 zSelectEsc[ 4 zPriorEsc[ 5 zNextEsc[ 6 z