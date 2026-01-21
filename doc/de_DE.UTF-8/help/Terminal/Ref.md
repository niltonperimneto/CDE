ReferenzinformationenAuf eines der unten aufgeführten Themen klicken, um Hilfe zu diesem
bestimmten Thema aufzurufen.Elektronische Handbuchseite dtterm (1X)Siehe auchX(1)resize(1)xset(1)xterm(1)hpterm(1X)pty(1)Core(3X)XmBulletinBoard(3X)XmForm(3X)XmFrame(3X)XmPrimitive(3X)XmScrollBar(3X)SYNOPSISdtterm[-Optionen]BESCHREIBUNGDerdtterm-Client stellt die Laufzeitunterstützung
für
Legacy-Anwendungen zur Verfügung, die für ANSI X3.64-1979- und
ISO 6429:1992(E)-konforme Zeichen-Terminals wie
z. B. DEC VT220 geschrieben wurden.OPTIONENDerdtterm-Terminal-Emulator akzeptiert
alle Befehlszeilenoptionen
des Standard-X-Toolkit zusammen mit zusätzlichen Optionen, die im folgenden
aufgelistet sind. (Beginnt eine Option mit `+' statt mit `-', ist
der Standardwert der Option wiederhergestellt.)-132Normalerweise wird die DECCOLM-Escape-Zeichenfolge, die
zwischen
dem 80-Spalten- und dem 132-Spalten-Modus wechselt, ignoriert.
Diese Option führt dazu, daß die DECCOLM-Escape-Zeichenfolge erkannt
wird
und dasdtterm-Fenster entsprechend in der
Größe geändert wird.c132Zugehörige Ressource:c132.+132Diese Option führt dazu, daß die DECCOLM-Escape-Zeichenfolge
ignoriert wird. Dies ist der Standard.c132Zugehörige Ressource:c132.-awDiese Option gibt an, daß der automatische Bildumlauf
zulässig
ist. Dadurch kann der Cursor automatisch zum Anfang der nächsten Zeile
springen, wenn er an der äußerst rechten Position einer Zeile angelangt
ist
und es sich bei der Ausgabe um Text handelt. Dies ist der Standard.Zugehörige Ressource:autoWrap.autoWrap+awDiese Option gibt an, daß der automatische Bildumlauf
nicht zulässig ist.Zugehörige Ressource:autoWrap.autoWrap-backgroundHintergrundfarbeDiese Option gibt den Hintergrund des Terminal-Fensters
sowie die
Standardfarben für die Bildlaufleiste und den X11-Zeigercursor an.
Unter CDE nimmt diese Option standardmäßig den Wert des
Auswahlpixels für den primären Farbsatz oder des Hintergrundpixels
an (siehe-bs.
Ohne CDE nimmt diese Option standardmäßig den Wert*background/*Backgroundmit der Hintergrundfarbe
Schwarz an.Hintergrundfarbebeschreibt die zu
verwendende Hintergrundfarbe.backgroundZugehörige
Ressource:background.-bdRahmenfarbeDiese Option gibt die Rahmenfarbe für alle Fenster
an. Der Rahmen des
Shell-Widgets ist möglicherweise nicht sichtbar, wenn
Fenstersteuerungen, die Prozesse neu zuordnen, wie z. B.dtwm(1X)undmwm(1X), verwendet werden. Die Standardfarbe
ist schwarz.Rahmenfarbebeschreibt
die zu verwendende Rahmenfarbe.borderColorZugehörige
Ressource:borderColor.-bgHintergrundfarbeDiese Option ist identisch mit-background.Hintergrundfarbebeschreibt
die zu verwendende Hintergrundfarbe.backgroundZugehörige
Ressource:background.-bordercolorRahmenfarbeDiese Option ist identisch mit -bd.Rahmenfarbebeschreibt die zu verwendende
Rahmenfarbe.borderColorZugehörige
Ressource:borderColor.-borderwidthRahmenbreiteDiese Option gibt die Rahmenbreite des Fensters des Shell-Widgets
an.
Dieser Wert kann von Fenstersteuerungen, die Prozesse nur zuordnen,
wie z. B.dtwm(1X)undmwm(1X), überschrieben werden.
Der Standardwert ist 0.RahmenbreiteGibt die Breite des Rahmens in Pixeln an.borderwidthZugehörige
Ressource:borderWidth.-bsDiese Option gibt an, daß das Terminal-Fenster als
Hintergrundfarbe
die Motif-Auswahlfarbe anstelle der Hintergrundfarbe verwenden soll. Dies
ist das Standardverhalten.backgroundIsSelectZugehörige
Ressource:backgroundIsSelect.+bsDiese Option gibt an, daß das Terminal-Fenster als
Hintergrundfarbe
nicht die Motif-Auswahlfarbe anstelle der Hintergrundfarbe verwenden soll.Zugehörige Ressource:backgroundIsSelect.-bwRahmenbreiteDiese Option ist identisch mit-borderwidth.Zugehörige Ressource:borderWidth.-CDiese Option gibt an, daß die Ausgabe, die an/dev/consolegeleitet wird,
statt dessen an das Terminal-Fenster geleitet werden soll. Sie wird als Maßnahme
zur Verfügung gestellt, zu verhindern, daß die Ausgabe, die normalerweise
auf
der ITE angezeigt wird, die X-Server-Anzeige überschreibt.
Sie wird nicht als allgemeiner Mechanismus zur Verfügung gestellt, um
die
Ausgabe von/dev/consoleeines beliebigen
Systems an einen beliebigen
X-Server zu leiten. Zu beachten ist, daß der Benutzer der Eigner von/dev/consolesein muß und über
Schreib-/Leseberechtigung verfügen muß, damit
diese Option funktioniert.-displayBildschirmnameDiese Option gibt den X11-Display-Server an, der vondttermverwendet werden
soll. Sie nimmt standardmäßig den Wert in der Umgebungsvariablen
$DISPLAY an.Bildschirmnamegibt den
X11-Server an, zu dem die Verbindung hergestellt wird.display-eProgrammargument ...Diese Option gibt ein ausführbares Programm an, das
als Unterprozeß beim
Start vondttermaufgerufen werden soll.
Diese Option muß die letzte
Option in der Befehlszeile sein.Programmargument ...gibt die Programm- und Befehlszeilenargumente an,
die ausgeführt werden sollen.-fbSchriftartsatzDiese Option gibt einen XFontSet an, der bei der Anzeige
von Terminal-Text
in Fettdruck verwendet werden soll. Sie sollte als eine Motif XmFontList
angegeben werden. Nur Zeichen oder Monospace-Schriftarten werden unterstützt.
Das Verhalten bei der Verwendung von Proportionalschriftarten ist undefiniert.
Die Standardschriftart für Fettdruck wird auf der Basis des XLFD-Namens
des
userFont generiert. Ist diese Schriftart nicht verfügbar, wird der
Text in Fettdruck durch Überschreiben des userFont (mit einer
Einrückung von einem Pixel) generiert.Schriftartsatzgibt den zu verwendenden
Terminal-XFontSet für Fettdruck an.Zugehörige Ressource:userFont.-fgVordergrundfarbeDiese Option gibt die Vordergrundfarbe des Terminal-Fensters
sowie die
Standardvordergrundfarben für die Bildlaufleiste und den X11-Zeigercursor
an.
Unter CDE nimmt diese Ressource standardmäßig den Wert des
Vordergrundpixels für den primären Farbsatz an.
Ohne CDE nimmt diese Option standardmäßig den Wert*foregroundoder*Foregroundmit der Hintergrundfarbe Weiß an.Vordergrundfarbegibt die zu verwendende
Vordergrundfarbe an.foregroundZugehörige
Ressource:foreground.-fnSchriftartsatzDiese Option gibt einen XFontSet an, der bei der Anzeige
von Terminal-Text
verwendet werden soll. Sie sollte als eine Motif XmFontList angegeben werden.
Nur Zeichen oder Monospace-Schriftarten werden unterstützt. Das Verhalten
bei
der Verwendung von Proportionalschriftarten ist undefiniert.
Diese Schriftart wird nicht zur Anzeige von Nicht-Terminal-Text verwendet
(Menüleiste, Dialogfenstermenüs, Dialoge, etc.).
Der Standard ist, den Wert XmNtextFontList des übergeordneten Bulletin
Board
(siehe XmBulletinBoard(3X)) in der gleichen Weise zu verwenden wie den
Widget XmText.Schriftartsatzgibt den zu verwendenden
Terminal-XFontSet an.
Zugehörige Ressource:userFont.-fontSchriftartsatzDiese Option ist identisch mit-fn.Schriftartsatzgibt den zu verwendenden
Terminal-XFontSet an.Zugehörige Ressource:userFont.-foregroundVordergrundDiese Option ist identisch mit-fg.Vordergrundgibt die zu verwendende
Vordergrundfarbe an.foregroundZugehörige
Ressource:foreground.-geometryGeometriezeichenfolgeDiese Option gibt die bevorzugte Größe und
Position des Terminal-Fensters
an. Die Standardgröße ist 24 Zeilen mit je 80 Zeichen. Es gibt
keine Standardposition.Geometriezeichenfolgegibt die zu
verwendende Terminal-Geometrie an.geometryZugehörige
Ressource:geometry.-helpZeigt eine Nachricht an, in der die Verwendung vondttermzusammengefaßt.
wird.help-iconicDiese Option gibt an, daß der Terminal-Emulator
anfänglich als Symbol in
die Anzeige gestellt werden soll.iconicZugehörige
Ressource:iconic.+iconicDiese Option gibt an, daß der Terminal-Emulator
anfänglich als normales
Fenster in die Anzeige gestellt werden soll. Dies ist das Standardverhalten.Zugehörige Ressource:iconic.-jDiese Option gibt an, daß der sprungweise Bilddurchlauf
verwendet werden soll.
Mit diesem Bilddurchlauf kann eine Anzeige mehr als zeilenweise geblättert
werden. Dadurch ist es möglich, die Anzeige schneller zu aktualisieren,
wenn
mehrere Textzeilen an das Terminal gesendet werden. Die maximale Anzahl von
Zeilen, die mit dem sprungweisen Bilddurchlauf geblättert werden können,
ist auf
die Anzahl von Zeilen im Terminal-Fenster begrenzt. Es wird garantiert, daß
alle Zeilen angezeigt werden. Dies ist das Standardverhalten.jumpScrollZugehörige
Ressource:jumpScroll.+jDiese Option gibt an, daß der sprungweise Bilddurchlauf
nicht verwendet
werden soll. Eine Beschreibung des sprungweisen Bilddurchlaufs befindet sich
oben unter-j.Zugehörige Ressource:jumpScroll.-kshModeDiese Option gibt an, daß der ksh-Modus aktiviert
wird.
In der Kornshell generiert eine Taste, die mit dem Extend Modifier Bit
gedrückt wird, ein Escape-Zeichen, gefolgt von dem Zeichen, das durch
den
Tastenanschlag generiert wird. Diese Option wird zur Verwendung mit emacs
und dem Modusksh(1)oderied(1)zur Verfügung gestellt, wenn der
Befehlszeileneditor emacs ist.
Bei Verwendung dieser Option ist die normale Verwendung der Metataste zur
Generierung von erweiterten Einzelbytezeichen oder von asiatischen
Mehrbytezeichen nicht möglich.kshModeZugehörige Ressource:kshMode.+kshModeDiese Option gibt an, daß der ksh-Modus nicht aktiviert
wird. Dies ist das
Standardverhalten.Zugehörige Ressource:kshMode.-lDiese Option ermöglicht die Protokollierung der Ausgabe.
Ist die
Protokollierung aktiviert, wird die gesamte Ausgabe, die von den
Unterprozessen empfangen wird, entweder in einer Datei oder in einer
Befehls-Pipeline (wie durch die Option-lfunten angegeben)
protokolliert. Da die Daten direkt von den Unterprozessen protokolliert
werden, sind alle Escape-Zeichen und Zeilenschaltung/Zeilenvorschub-Paare,
die durch die Terminal-Zeilendisziplin gesendet werden, enthalten.
Die Ausgabe kann möglicherweise durch Escape-Zeichenfolgen aktiviert
und
inaktiviert werden.loggingZugehörige
Ressource:logging.+lDiese Option inaktiviert die Protokollierung der Ausgabe.
Eine Beschreibung
der Protokollierung der Ausgabe befindet sich oben unter-l.
Diese Option ist der Standardwert.loggingZugehörige
Ressource:logging.-lfDateinameDiese Option gibt den Namen der Datei an, in die das oben
beschriebene
Ausgabeprotokoll geschrieben wird. BeginntDateinamemit einem
Pipe-Symbol (|), wird der Rest der Zeichenfolge
als Befehl interpretiert,
der als Endpunkt einer Pipe verwendet werden soll.
Der Standarddateiname istDttermLogXXXXX(wobeiXXXXXdie Prozeß-ID vondttermist). Die
Datei wird in dem Verzeichnis erstellt,
von dem ausdttermgestartet wurde. Sind
die letzten fünf Zeichen
"XXXXX", werden sie durch die Prozeß-ID ersetzt.Dateinamegibt den zu verwendenden
Dateinamen an.logFileZugehörige
Ressource:logFile.-lsDiese Option gibt an, daß die Shell, die gestartet
wird, eine Anmelde-Shell
sein soll (d. h., das erste Zeichen vonargv[0]ist ein Gedankenstrich,
der der Shell angibt, daß sie die Dateiprofiledes Systems und die Datei$HOME/.profiledes Benutzers (fürkshundsh) bzw. die Dateicsh.logindes Systems und die Datei$HOME.logindes Benutzers
(fürcsh) lesen soll).loginShellZugehörige
Ressource:loginShell.+lsDiese Option gibt an, daß eine normale (Nicht-Anmelde-)
Shell gestartet werden
soll. Dies ist das Standardverhalten.loginShellZugehörige
Ressource:loginShell.-mapDiese Option gibt an, daßdttermbei der Ausgabe von Unterprozessen
wieder auf die vorherige Größe gebracht werden soll, wenn es auf
Symbolgröße
verkleinert ist. Eine anfängliche Zeitspanne, in derdttermbei der Ausgabe von Unterprozessen
nicht auf die vorherige
Größe gebracht werden soll, kann über die RessourcemapOnOutputDelayangegeben werden.mapOnOutputZugehörige
Ressource:mapOnOutput.+mapDiese Option gibt an, daß kein besonderes Zuordnungsverhalten
gefordert wird.
Dies ist das Standardverhalten.mapOnOutputZugehörige
Ressource:mapOnOutput.-mbDiese Option gibt an, daßdttermein Randsignal ertönen lassen soll, wenn
der Benutzer nahe am rechten Rand Zeichen eingibt. Der genaue Abstand wird
durch die Option-nbangegeben.marginBellZugehörige
Ressource:marginBell.+mbDiese Option gibt an, daß das Randsignal nicht ertönen
soll, wenn der
Benutzer nahe am rechten Rand Zeichen eingibt. Dies ist der Standardwert.marginBellZugehörige
Ressource:marginBell.-msZeigerfarbeDiese Option gibt die Vordergrundfarbe an, die für
den Zeigercursor des
Terminal-Fensters (X11) verwendet werden soll. Der Standard ist, die
Vordergrundfarbe des Terminal-Fensters zu verwenden. Sieheforeground.Zeigerfarbegibt die zu verwendende
Zeigervordergrundfarbe an.Zugehörige Ressource:pointerColor.-nameprog_nameDiese Option gibt den X11-Namen desdtterm-Fensters an.prog_nameist der zu verwendende
Name.name-nbAnzahlDiese Option gibt die Anzahl an Zeichen vom rechten Rand
aus an, bei denen
das Randsignal ertönt, falls es aktiviert ist. Der Standardwert ist 10.nMarginBellZugehörige
Ressource:nMarginBell.-rDiese Option führt dazu, daß dasdtterm-Fenster mit umgekehrten Vorder- und
Hintergrundfarben angezeigt wird. Sie ist identisch mit den Optionen-rund-reverse.+rDiese Option führt dazu, daß dasdtterm-Fenster mit den normalen Vorder- und
Hintergrundfarben angezeigt wird. Dies ist der Standardwert. Diese Option
ist
identisch mit der Option+rv.-reverseDiese Option führt dazu, daß dasdtterm-Fenster mit umgekehrten Vorder- und
Hintergrundfarben angezeigt wird. Sie ist identisch mit den Optionen-rund-rv'.-rvDiese Option führt dazu, daß dasdtterm-Fenster mit umgekehrten Vorder- und
Hintergrundfarben angezeigt wird. Sie ist identisch mit der Auswahl von
Optionen | Global, wenn danach das Optionsmenü für
"windowBackground" (Fensterhintergrund) auf "Umgekehrt" gesetzt wird. Bei
einemdtterm-Fenster, das mit dieser Option gestartet
wird, ist das Optionsmenü
"Fensterhintergrund" auf "Umgekehrt" gesetzt. Siehe"Globale Optionen".+rvDiese Option führt dazu, daß dasdtterm-Fenster mit den normalen Vorder- und
Hintergrundfarben angezeigt wird. Dies ist der Standardwert.-rwDiese Option gibt an, daß der umgekehrte Bildumlauf
aktiviert werden soll.reverseWrapZugehörige
Ressource:reverseWrap.+rwDiese Option gibt an, daß der umgekehrte Bildumlauf
nicht aktiviert werden soll.
Dies ist der Standardwert.reverseWrapZugehörige
Ressource:reverseWrap.-SccnDiese Option gibt an, daß der Terminal-Emulator
nicht mit einem vorab
geöffneten PTY- oder STREAMS-Gerät ausgeführt werden soll.
Diese Option
ist zur Verwendung vorgesehen, wenn der Slave-Name des PTY- oder STREAMS-Geräts
das Formatttyhat (d. h., wenn aufttygenau zwei Zeichen folgen).
Sie ist für den Fall vorgesehen, daß eindtterm-Fenster automatisch von
einer anderen Anwendung aus aufgerufen wird.ccgibt die letzten zwei Zeichen
des Slave-Namens des PTY- oder
STREAMS-Geräts an, wobei der Slave-Name der Formatttyhat. Dieser
Wert wird ignoriert, muß aber exakt zwei Zeichen lang sein.ngibt die Nummer des
Dateideskriptors an, der der bereits geöffneten
Master-Seite des PTY- oder STREAMS-Geräts entspricht.-Sc.nDiese Option ist identisch mit-Sccn,
wird aber für Systeme
mit einem größeren Bereich für den PTY-Namen zur Verfügung
gestellt.cgibt die letzte Komponente des
Slave-Namens des PTYs an. Dieser
Wert wird ignoriert und kann auch leer sein.ngibt die Nummer des
Dateideskriptors an, der der bereits geöffneten
Master-Seite des PTYs entspricht.-sbDiese Option gibt an, daß eine Bildlaufleiste angezeigt
werden soll. Dies
ist der Standardwert.scrollBarZugehörige
Ressource:scrollBar.+sbDiese Option gibt an, daßkeineBildlaufleiste angezeigt werden soll.scrollBarZugehörige
Ressource:scrollBar.-sfDiese Option gibt an, daß die Escape-Codes für
Sun-Funktionstasten
für die Funktionstasten generiert werden sollen anstelle der
Standard-VT220-Escape-Zeichenfolgen.sunFunctionKeysZugehörige
Ressource:sunFunctionKeys.+sfDiese Option gibt an, daß die Standard-Escape-Zeichenfolgen
für die
Funktionstasten generiert werden sollen anstelle der
Escape-Codes für Sun-Funktionstasten. Dies ist das Standardverhalten.sunFunctionKeysZugehörige
Ressource:sunFunctionKeys.-slAnzeigen[s|l]Diese Option gibt die Anzahl an Zeilen im Terminal-Puffer
an, die über die
Länge des Fensters hinausgehen. Der Optionswert besteht aus einer Zahl,
gefolgt von einem wahlfreien Suffix. Ist kein Suffix enthalten oder ist das
Suffix "l" (Buchstabe L), ist die Gesamtlänge des Terminal-PuffersAnzeigenmal die Länge des Terminal-Fensters. Ist
das
Suffix "s" (Buchstabe S), ist die Gesamtlänge des Terminal-Puffers
(Anzeigenplus 1) mal die Länge des Terminal-Fensters.dttermversucht, dasselbe Puffer-zu-Fenster-Verhältnis
beizubehalten,
wenn das Fenster vergrößert wird. Der Standardwert ist "4s."Anzeigengibt die Anzahl von Anzeigen
oder Zeilen an, die gespeichert
werden sollen.saveLinesZugehörige
Ressource:saveLines.-tiTerm-IDDiese Option stellt den Namen zur Verfügung, der
zur Auswahl der korrekten
Antwort auf Terminal-ID-Anfragen verwendet wird. Gültige Werte sindvt100,vt101,vt102undvt220. Der Standardwert istvt220.Term-IDgibt die zu verwendende Terminal-ID
an.-titleTitelzeichenfolgeDiese Option gibt den Fenstertitel an. Wird die Option-everwendet,
ist der Standardwert die letzte Komponente des Programmpfades. Wird die
Option-enicht verwendet, ist der Standardwert
die letzte Komponente
des Namens, der zur Ausführung vondttermverwendet wird (d. h.argv[0]).Titelzeichenfolgegibt den zu verwendenden
Titel an.titleZugehörige
Ressource:title.-tmTerm-ModiDiese Option gibt eine Zeichenfolge an, die Schlüsselwörter
zur
Einstellung des Terminals und die Zeichen, an die sie gebunden werden können,
enthält. Zu den zulässigen Schlüsselwörtern gehörenintr,quit,erase,kill,eof,eol,swtch,start,stop,brk,susp,dsusp,rprnt,flush,werasundlnext. Schlüsselwörter, die
für eine bestimmte Architektur nicht
anwendbar sind, werden korrekt analysiert und ignoriert. Steuerzeichen
können als^gefolgt von einem Buchstaben
(z. B.^coder^u)
angegeben werden, und^?kann als Löschzeichen
verwendet werden.
Diese Option ist sinnvoll beim Überschreiben der Standard-Terminal-Einstellungen,
da nicht bei jedem Start eines Terminal-Prozesses einstty(1)durchgeführt
werden muß. Der Standardwert ist NULL.Term-Modigibt die Zeichenfolge für
den Terminal-Modus an.ttyModesZugehörige
Ressource:ttyModes.-tnTerm-NameDiese Option gibt einen Namen an, auf den die Umgebungsvariable$TERMgesetzt werden soll. Der Standardwert
ist"vt220".Term-Namegibt den zu verwendenden
Terminal-Namen an.termNameZugehörige
Ressource:termName.-usageGibt eine Verwendungsnachricht in der Anzeige aus.-vbDiese Option gibt an, daß ein visuelles Signal anstelle
eines hörbaren
Signals verwendet werden soll. Statt eines Signaltons, der beim Empfang der
Zeichenkombination Strg-G ertönt, blinkt das Fenster.visualBellZugehörige
Ressource:visualBell.+vbDiese Option gibt an, ein hörbares Sigal anstelle
eines visuellen
verwendet werden soll. Dies ist das Standardverhalten.visualBellZugehörige
Ressource:visualBell.-wRahmenbreiteDiese Option ist identisch mit-borderwidth.Rahmenbreitegibt die Breite des
Rahmens in Pixeln an.-xrmRessourcenzeichenfolgeDiese Option ermöglicht, daß Ressourcen des
X11-Ressourcenmanagers in
einer Befehlszeile angegeben werden können.Ressourcenzeichenfolgegibt eine X11-Ressourcenzeichenfolge
an.RESSOURCENallowSendEventsDiese Ressource gibt an, daß der Terminal-Emulator
synthetische Ereignisse
(die von einer anderen Anwendung generiert und gesendet wurden) zulassen soll.
Durch das Aktivieren dieser Ressource wird eine mögliche Lücke im
Sicherheitssystem geöffnet. Der Standardwert ist 'False''.appCursorDefaultBei der Angabe 'True'' sind die Cursortasten anfänglich
im Anwendungsmodus.
Bei der Angabe 'False'' sind sie anfänglich im Cursormodus. Der Standardwert
ist 'False''.appKeypadDefaultBei der Angabe 'True'' sind die Tasten im numerischen
Tastenblock anfänglich
im Anwendungsmodus. Bei der Angabe 'False' sind sie anfänglich im numerischen
Modus. Der Standardwert ist 'False'.autoWrapDiese Ressource gibt an, ob der automatische Bildumlauf
anfänglich aktiviert ist.
Der Standardwert ist 'True'.backgroundDiese Ressource gibt die Hintergrundfarbe des Terminal-Fensters
sowie die
Standardhintergrundfarbe für die Bildlaufleiste an.
Unter CDE nimmt diese Ressource standardmäßig den Wert des
Auswahlpixels für den primären Farbsatz oder den Wert des
Hintergrundpixels für den primären Farbsatz an (siehebackgroundIsSelect).
Der Standardwert ist das Hintergrundpixel für den primären Farbsatz.
Ohne CDE nimmt diese Ressource standardmäßig die Farbe Schwarz
an.backgroundIsSelectBei der Angabe 'True' gibt diese Ressource an, daß
das Terminal-Fenster als Hintergrundfarbe die Motif-Auswahlfarbe
anstelle der Hintergrundfarbe verwenden soll.
Der Standardwert ist 'False''.blinkRateDiese Ressource gibt die Anzahl der Millisekunden an,
die der Cursor
jeweils während des Blinkens im Status"ein"und"aus"bleibt. Bei
einem Wert von 250 blinkt der Cursor zweimal pro Sekunde. Bei einem Wert
von 0 ist das Blinken ausgeschaltet. Der Standardwert ist 250.borderColorDiese Ressource gibt die Rahmenfarbe für die Fenster
an.
Der Fensterrahmen ist möglicherweise nicht sichtbar, wenn
Fenstersteuerungen, die Prozesse nur zuordnen, wie z. B.dtwmundmwm, verwendet werden.
Der Standardwert ist 'black'.borderWidthDiese Ressource gibt die Rahmenbreite des Fensters des
Shell-Widgets an.
Dieser Wert kann von Fenstersteuerungen, die Prozesse neu zuordnen,
wie z. B.dtwmundmwm,
überschrieben werden.
Der Standardwert ist0.c132Diese Ressource gibt an, ob die DECCOLM-Escape-Zeichenfolge,
die zwischen
dem 80-Spalten- und dem 132-Spalten-Modus wechselt, respektiert wird.
Der Standardwert ist 'False'.charCursorStyleDiese Ressource gibt die Form des Textcursors an. Der
Wertchar_cursor_boxgibt einen Cursor mit der
Breite und Höhe des
Zeichenrahmens der Basisschriftart an. Der Wertchar_cursor_bargibt einen Cursor mit der Breite des Zeichenrahmens der Basisschriftart und
mit einer Höhe von zwei Pixeln an, dessen oberer Rand auf der Basislinie
liegt.
Der Standardwert istchar_cursor_box.consoleModeDiese Ressource gibt an, daß die Ausgabe, die an/dev/consolegeleitet wird,
statt dessen an das Terminal-Fenster geleitet werden soll. Sie wird als Maßnahme
zur Verfügung gestellt, zu verhindern, daß die Ausgabe, die normalerweise
auf
der ITE angezeigt wird, die X-Server-Anzeige überschreibt.
Sie wird nicht als allgemeiner Mechanismus zur Verfügung gestellt, um
die
Ausgabe von/dev/consoleeines beliebigen
Systems an einen beliebigen
X-Server zu leiten. Zu beachten ist, daß der Benutzer der Eigner von/dev/consolesein muß und über
Schreib-/Leseberechtigung verfügen muß, damit
diese Option funktioniert.
Der Standardwert ist 'False''.foregroundDiese Ressource gibt die Vordergrundfarbe des Terminal-Fensters
sowie die
Standardvordergrundfarbe für die Bildlaufleiste und die Farbe für
den Zeiger an.
Unter CDE nimmt diese Ressource standardmäßig den Wert der
Vordergrundfarbe für den primären Farbsatz an.
Andernfalls nimmt sie den Standardwert"white"an.geometryDiese Ressource gibt die bevorzugte Größe und
Position des
Terminal-Fensters an. Die Standardgröße ist 24 Zeilen mit je 80
Zeichen.
Es gibt keine Standardposition.iconGeometryDiese Ressource gibt die bevorzugte Position für
das Symbol des
Terminal-Emulators an. Dieser Wert wird möglicherweise von
Fenstersteuerungen ignoriert. Es gibt keinen Standardwert.iconicDiese Option gibt an, daß der Terminal-Emulator
anfänglich als Symbol in
die Anzeige gestellt werden soll. Dieser Wert wird möglicherweise von
Fenstersteuerungen (einschließlichdtwmundmwm) ignoriert.
Der Standardwert ist 'False''.iconicNameDiese Ressource gibt den Namen für das Symbol an.
Wird die Option-everwendet, ist der Standardwert
die letzte
Komponente des Programmpfades. Wird die Option-enicht verwendet,
ist der Standardwert der Basisname des Namens, der zur Ausführung
vondttermverwendet wird (d. h.argv[0]).jumpScrollDiese Ressource gibt an, das der sprungweise Bilddurchlauf
verwendet werden soll.
Mit diesem Bilddurchlauf kann eine Anzeige mehr als zeilenweise geblättert
werden. Dadurch ist es möglich, die Anzeige schneller zu aktualisieren,
wenn
mehrere Textzeilen an das Terminal gesendet werden. Die maximale Anzahl von
Zeilen, die mit dem sprungweisen Bilddurchlauf geblättert werden können,
ist auf
die Anzahl von Zeilen im Terminal-Fenster begrenzt. Es wird garantiert, daß
alle Zeilen angezeigt werden. Der Standardwert ist 'True'.kshModeDiese Ressource gibt an, daß der ksh-Modus aktiviert
wird.
In der Kornshell generiert eine Taste, die mit dem Extend Modifier Bit
gedrückt wird, ein Escape-Zeichen, gefolgt von dem Zeichen, das durch
den
Tastenanschlag generiert wird. Diese Option wird zur Verwendung mit emacs
und dem Modusksh(1)oderied(1)zur Verfügung gestellt, wenn der
Befehlszeileneditor emacs ist.
Bei Verwendung dieser Option ist die normale Verwendung der Metataste zur
Generierung von erweiterten Einzelbytezeichen oder von asiatischen
Mehrbytezeichen nicht möglich.
Der Standardwert ist 'False'.logFileDiese Ressource gibt den Namen der Datei an, in die das
unten beschriebene
Ausgabeprotokoll geschrieben wird. BeginntDateinamemit einem
Pipe-Symbol (|), wird der Rest der Zeichenfolge
als Befehl interpretiert,
der als Endpunkt einer Pipe verwendet werden soll.
Der Standarddateiname istDttermLogXXXXX(wobeiXXXXXdie Prozeß-ID vondttermist). Die
Datei wird in dem Verzeichnis erstellt,
von dem aus der Unterprozeß gestartet wurde. Sind die letzten fünf
Zeichen
"XXXXX", werden sie durch die Prozeß-ID ersetzt.loggingDiese Ressource ermöglicht die Protokollierung der
Ausgabe. Ist die
Protokollierung aktiviert, wird die gesamte Ausgabe, die von den
Unterprozessen empfangen wird, entweder in einer Datei oder in einer
Befehls-Pipeline (wie durch die OptionlogFileoben angegeben)
protokolliert. Da die Daten direkt von den Unterprozessen protokolliert
werden, sind alle Escape-Zeichen und Zeilenschaltung/Zeilenvorschub-Paare,
die durch die Terminal-Zeilendisziplin gesendet werden, enthalten.
Die Ausgabe kann möglicherweise durch Escape-Zeichenfolgen aktiviert
und
inaktiviert werden. Der Standardwert ist 'False'.logInhibitDiese Ressource gibt an, daß die Geräte- und
Dateiprotokollierung
verhindert werden soll.
Der Standardwert ist 'False''.loginShellDiese Ressource gibt an, daß die Shell, die gestartet
wird, eine Anmelde-Shell
sein soll (d. h., das erste Zeichen vonargv[0]ist ein Gedankenstrich,
der der Shell angibt, daß sie die Dateiprofiledes Systems und die Datei$HOME/.profiledes Benutzers (fürkshundsh) bzw. die Dateicsh.logindes Systems und die Datei$HOME.logindes Benutzers
(fürcsh) lesen soll). Der Standardwert
ist 'False''.mapOnOutputDiese Ressource gibt an, daß der Terminal-Emulator
bei der Ausgabe von
Unterprozessen wieder auf die vorherige Größe gebracht werden soll,
wenn er
auf Symbolgröße verkleinert ist. Eine anfängliche Zeitspanne,
in der
er bei der Ausgabe von Unterprozessen nicht auf die vorherige
Größe gebracht werden soll, kann über die RessourcemapOnOutputDelayangegeben werden (siehe
unten).
Der Standardwert ist 'False''.mapOnOutputDelayDiese Ressource gibt die Dauer in Sekunden nach dem Systemstart
an, diedttermdie RessourcemapOnOutput(siehe oben) nicht respektiert. In
dieser Zeit kann die Anfangsausgabe (z. B. Shell-Eingabeaufforderungen) an
das Terminal gesendet werden, ohne daß das Fenster automatisch wiederhergestellt
wird.
Der Standardwert ist0(keine Verzögerung).marginBellDiese Ressource gibt an, ob das Randsignal ertönen
soll, wenn
der Benutzer nahe am rechten Rand Zeichen eingibt.
Der Standardwert ist 'False''.menuBarDiese Ressource gibt an, daß ein Aktionsleistenmenü
angezeigt werden soll.
Der Standardwert ist 'True'.menuPopupDiese Ressource gibt an, daß ein Dialogfenstermenü
aktiviert werden soll.
Der Standardwert ist 'True'.nMarginBellDiese Ressource gibt die Anzahl an Zeichen vom rechten
Rand aus an, bei denen
das Randsignal ertönt, falls es aktiviert ist. Der Standardwert ist 10.pointerBlankDiese Ressource gibt an, daß der Zeigercursor in
den Ausblendemodus gesetzt
werden soll. In diesem Modus wird der Cursor eingeschaltet, wenn der
Zeiger bewegt wird, und ausgeblendet, wenn eine auswählbare Anzahl von
Sekunden vergangen ist, oder wenn eine Tastatureingabe erfolgte. Die
Verzögerung wird über die RessourcepointerBlankDelay(siehe unten)
eingestellt. Der Standardwert ist 'False''.pointerBlankDelayDiese Ressource gibt die Anzahl an Sekunden an, die nach
dem Bewegen
des Zeigers gewartet wird, bevor der Zeigercursor ausgeblendet wird.
Der Wert 0 ruft die Zeigerausblendung nur bei Tastatureingabe auf.
Der Standardwert ist 2 Sekunden.pointerColorDiese Ressource gibt die Vordergrundfarbe an, die für
den Zeigercursor des
Terminal-Fensters (X11) verwendet werden soll. Der Standard ist, die
Vordergrundfarbe des Terminal-Fensters zu verwenden.
Sieheforeground.pointerColorBackgroundDiese Ressource gibt die Hintergrundfarbe an, die für
den Zeigercursor des
Terminal-Fensters (X11) verwendet werden soll. Der Standard ist, die
Hintergrundfarbe des Terminal-Fensters zu verwenden.
Siehebackground.pointerShapeDiese Ressource gibt das Schriftartzeichen für X-Cursor
an, das
als Zeigercursor verwendet werden soll. Sie sollte als eine Zeichenfolge
von der Kopfdatei<X11/cursorfont.h>angegeben
werden, wobei das
führendeXC_entfernt wird. Der Standardwert
istxterm.reverseVideoDiese Ressource gibt an, ob die Umkehranzeige verwendet
werden soll.
Der Standardwert ist 'False''.reverseWrapDiese Ressouce gibt an, ob der umgekehrte Bildumlauf aktiviert
werden soll.
Der Standardwert ist 'False''.saveLinesDiese Ressource gibt die Anzahl an Zeilen im Terminal-Puffer
an, die über die
Länge des Fensters hinausgehen. Der Optionswert besteht aus einer Zahl,
gefolgt von einem wahlfreien Suffix. Ist kein Suffix enthalten oder ist das
Suffix "l" (Buchstabe L), ist die Gesamtlänge des Terminal-PuffersAnzeigenmal die Länge des Terminal-Fensters. Ist
das
Suffix "s" (Buchstabe S), ist die Gesamtlänge des Terminal-Puffers
(Anzeigenplus 1) mal die Länge des Terminal-Fensters.dttermversucht, dasselbe Puffer-zu-Fenster-Verhältnis
beizubehalten,
wenn das Fenster vergrößert wird. Der Standardwert ist "4s."scrollBarDiese Ressource gibt an, ob eine Bildlaufleiste angezeigt
werden soll.
Der Standardwert ist 'True'.sunFunctionKeysDiese Ressource gibt an, ob die Escape-Codes für
Sun-Funktionstasten
für die Funktionstasten generiert werden sollen anstelle der
Standard-VT220-Escape-Zeichenfolgen. Der Standardwert ist 'False''.termIdDiese Ressource stellt den Namen zur Verfügung, der
zur Auswahl der korrekten
Antwort auf Terminal-ID-Anfragen verwendet wird. Gültige Werte sindvt100,vt101,vt102undvt220. Der Standardwert
istvt220.termNameDiese Ressource definiert den Namen an, auf den die Umgebungsvariable$TERMgesetzt werden soll.
Der Standardwert istvt220.titleDiese Ressource gibt den Fenstertitel an. Wird die Option-everwendet,
ist der Standardwert die letzte Komponente des Programmpfades. Wird die
Option-enicht verwendet, ist der Standardwert
die letzte Komponente
des Namens, der zur Ausführung vondttermverwendet wird (d. h.argv[0]).ttyModesDiese Ressource gibt eine Zeichenfolge an, die Schlüsselwörter
zur
Einstellung des Terminals und die Zeichen, an die sie gebunden werden können,
enthält. Zu den zulässigen Schlüsselwörtern gehörenintr,quit,erase,kill,eof,eol,swtch,start,stop,brk,susp,dsusp,rprnt,flush,werasundlnext. Schlüsselwörter, die
für eine bestimmte Architektur nicht
anwendbar sind, werden korrekt analysiert und ignoriert. Steuerzeichen
können als^gefolgt von einem Buchstaben
(z. B.^coder^u)
angegeben werden, und^?kann als Löschzeichen
verwendet werden.
Diese Option ist sinnvoll beim Überschreiben der Standard-Terminal-Einstellungen,
da nicht bei jedem Start eines Terminal-Prozesses einstty(1)durchgeführt
werden muß. Der Standardwert ist NULL.userBoldFontDiese Ressource gibt einen XFontSet an, der bei der Anzeige
von Terminal-Text
in Fettdruck verwendet werden soll. Sie sollte als eine MotifXmFontListangegeben werden. Nur Zeichen oder Monospace-Schriftarten werden unterstützt.
Das Verhalten bei der Verwendung von Proportionalschriftarten ist undefiniert.
Die Standardschriftart für Fettdruck wird auf der Basis des XLFD-Namens
desuserFontgeneriert. Ist diese Schriftart
nicht verfügbar, wird der
Text in Fettdruck durch Überschreiben desuserFont(mit einer
Einrückung von einem Pixel) generiert.userFontDiese Option gibt einen XFontSet an, der bei der Anzeige
von Terminal-Text
verwendet werden soll. Sie sollte als eine MotifXmFontListangegeben werden.
Nur Zeichen oder Monospace-Schriftarten werden unterstützt. Das Verhalten
bei
der Verwendung von Proportionalschriftarten ist undefiniert.
Diese Schriftart wird nicht zur Anzeige von Nicht-Terminal-Text verwendet
(Menüleiste, Dialogfenstermenüs, Dialoge, etc.).
Der Standard ist, den WertXmNtextFontListdes übergeordneten Bulletin Board
(siehe XmBulletinBoard(3X)) in der gleichen Weise zu verwenden wie den
WidgetXmText.visualBellDiese Ressource gibt an, daß ein visuelles Signal
anstelle eines hörbaren
Signals verwendet werden soll. Statt eines Signaltons, der beim Empfang der
Zeichenkombination Strg-G ertönt, blinkt das Fenster.
Der Standardwert ist 'False''.ZEIGERVERWENDUNGdttermermöglicht dem Benutzer,
Textregionen auszuwählen. Die Auswahl
basiert auf dem Modell, das im HandbuchInter-Client Communication Conventions Manual(ICCCM)
angegeben ist.dttermunterstützt nur die Hauptauswahl.
Der Benutzer kann ausgewählten
Text unter Verwendung der direkten Übertragung kopieren oder einfügen.
Die
Eingabe wird wie Tastatureingabe behandelt und an der Cursorposition
eingefügt. Die Operation zum Auswählen/Einfügen und ihre Standardzuordnungen
werden im folgenden beschrieben.auswählenDie linke Maustaste wird verwendet, um den
zu kopierenden Text auszuwählen. Den Zeiger zum Anfang des zu kopierenden
Textes bewegen, die linke Taste drücken und gedrückt halten, den
Cursor zum
Ende des zu kopierenden Textes bewegen und die Taste loslassen. Die Auswahl
de momentan ausgewählten Textes kann zurückgenommen werden, indem
die linke
Taste einmal gedrückt wird, ohne die Maus zu bewegen.einfügenDie mittlere Maustaste fügt den Text aus der
Hauptauswahl ein, wobei er wie Tastatureingabe behandelt wird.AKTIONENIn diesem Abschnitt werden diedtterm-Aktionsroutinen
beschrieben.bell([Prozentsatz])Diese Aktion läßt das Tastatursignal mit einem
bestimmtenProzentsatzoberhalb oder unterhalb der Grundlautstärke ertönen.break ()Diese Aktion sendet ein Unterbrechungssignal zum untergeordneten
Prozeß.cancel ()Diese Aktion sendet einCAN(cancel = abbrechen)-Zeichen um untergeordneten
Prozeß.do ()Diese Aktion sendet die Escape-Zeichenfolge, die derDo-Taste zugeordnet
ist, zum untergeordneten Prozeß.edit-key(Zeichenfolge)Diese Aktion sendet die Escape-Zeichenfolge, die der entsprechenden
Editiertaste zugeordnet ist, zum untergeordneten Prozeß. Die Interpretation
dieser Tasten ist anwendungsspezifisch.
Gültige Werte fürZeichenfolgesindfind,insert,next,prior,removeundselect.extend-start()Startet die Erweiterung des momentan ausgewählten
Textes.extend-end ()Erweitert die aktuelle Auswahl. Die Menge des ausgewählten
Textes hängt von
der Anzahl des Mausklickens ab.function-key-execute(Num[,Typ])Diese Aktion sendet die Escape-Zeichenfolge, die der entsprechenden
FunktionstasteNumzugeordnet ist, zum untergeordneten
Prozeß.
Gültige Werte fürNumsind1bis35. IstTypauffunction(oder gar nicht( gesetzt), wird
die Escape-Zeichenfolge, die
der FuntionstasteNumzugeordnet ist, zum untergeordneten
Prozeß gesendet.
IstTypaufUDKgesetzt,
wird die Zeichenfolge, die der
benutzerdefinierten FunktionstasteNumzugeordnet ist,
zum
untergeordneten Prozeß gesendet.grab-focus ()Diese Aktion führt eine der folgenden Funktionen
aus, je nach Anzahl der
Mausklickes. Einmal Klicken nimmt die Auswahl von ausgewähltem Text zurück
und
setzt den Auswahlanker auf die Zeigerposition, zweimal Klicken wählt
ein
Wort aus, dreimal Klicken wählt eine Textzeile aus und viermal Klicken
wählt
den gesamten Text aus.hard-reset ()Diese Aktion führt einen Kaltstart auf dem Terminal-Emulator
aus.help ()Diese Aktion sendet die Escape-Zeichenfolge, die derDEC VT220-Hilfetaste
zugeordnet ist, zum untergeordneten Prozeß. Die Interpretation dieser
Taste ist anwendungsspezifisch.keymap(Name)Diese Aktion definiert dynamisch eine neue Umsetzungstabelle,
deren
Ressourcenname ausNamemit dem Suffix Keymap (Groß-/Kleinschreibung
beachten) gebildet wird. Der NameNonestellt
die ursprüngliche
Umsetzungstabelle wieder her.keypad-key-execute(Zeichenfolge)Diese Aktion sendet die Escape-Zeichenfolge, die der entsprechenden
Taste
im numerischen Tastenblock zugeordnet ist, zum untergeordneten Prozeß.
Die Interpretation dieser Tasten ist anwendungsspezifisch. Gültige
Werte fürZeichenfolgeumfassen:f1-f4,space,tab,enter,equal,multiply,add,separator,subtract,decimal,divideund0bis9.move-cursor(Richtung)Diese Aktion sendet die Escape-Zeichenfolge, die der entsprechenden
Cursorbewegung zugeordnet ist, zum untergeordneten Prozeß. Die Interpretation
dieser Tasten ist anwendungsspezifisch. Gültige Werte fürRichtungumfassen:up,down,backwardundforward.redraw-display ()Diese Aktion zeichnet den Inhalt des Textfensters erneut.scroll(Anzahl[,Einheiten])Diese Aktion blättert den Anzeigenspeicher vorwärts,
wennAnzahlhöher als
Null ist, oder rückwärts, wennAnzahlniedriger
als Null ist. Die Anzahl der
geblätterten Zeilen basiert auf "Anzahl" und "Einheiten". Gültige
Werte fürEinheitensindpage,halfpageundline.
Der Standardwert für Einheiten istline.select-adjust ()Diese Aktion erweitert die Auswahl. Die Menge des ausgewählten
Textes hängt von
der Anzahl des Mausklickens ab:1 x Klicken = Zeichen2 x Klicken = Wort3 x Klicken = Zeile4 x Klicken = Pufferselect-all ()Diese Aktion wählt den gesamten Text aus.select-page ()Diese Aktion wählt den Text in der Anzeige aus.self-insert ()Diese Aktion sendet das Zeichen, das der gedrückten
Taste zugeordnet ist,
zum untergeordneten Prozeß.soft-reset ()Diese Aktion führt einen Warmstart auf dem Terminal-Emulator
aus.stop(Status)Diese Aktion schaltet hin und her, startet oder stoppt
den Prozeß zum
Lesen von Daten vom untergeordneten Prozeß. Gültige Werte fürStatussindtoggle,onundoff.string(Zeichenfolge)Diese Aktion fügt die angegebene Textzeichenfolge
ein, als würde es sich
um eingegebenen Text handeln. Die Zeichenfolge muß in Anführungszeichen
gesetzt werden, wenn die Leerzeichen oder nichtalphanumerische Zeichen
enthält. Die Zeichenfolge wird als Hexadezimalzeichen interpretiert,
wenn sie mit den Zeichen0xbeginnt.tab ()Diese Aktion sendet ein Tabulatorzeichen zum untergeordneten
Prozeß.visual-bell ()Diese Aktion läßt das Fenster kurz aufblinken.Virtuelle BindungDie Bindungen für virtuelle Tasten sind lieferantenspezifisch.
Virtuelle Bindungen gelten nicht, wenn derdtterm-Widget
den
Eingabefokus hat. Weitere Informationen zum Binden von virtuellen
Tasten befinden sich inVirtualBindings(3X).UMSETZUNGENDtTermbeinhaltet Umsetzungen von 'Primitive'.
Zu beachten ist, daß eine Änderung der Umsetzungen
in dem Modus#overrideoder#augmentnicht
definiert ist.Shift~Ctrl<Key>KP_Multiply:XtDisplayInstalledAccelerators()~ShiftCtrl<Key>KP_Multiply:XtDisplayAccelerators()ShiftCtrl<Key>KP_Multiply:XtDisplayTranslations()<Key>osfCancel:process-cancel()<Key>osfCopy:copy-clipboard()<Key>osfCut:copy-clipboard()<Key>osfPaste:paste-clipboard()<Key>osfBeginLine:beginning-of-buffer()<Key>osfEndLine:end-of-buffer()Shift<Key>osfUp:scroll(1,line)Shift<Key>osfDown:scroll(-1,line)<Key>osfUp:move-cursor(up)<Key>osfDown:move-cursor(down)<Key>osfLeft:move-cursor(backward)<Key>osfRight:move-cursor(forward)<Key>Find:vt-edit-key(find)<Key>Insert:vt-edit-key(insert)<Key>Select:vt-edit-key(select)<Key>Do:vt-edit-key(do)<Key>Help:vt-edit-key(help)<Key>Menu:vt-edit-key(menu)~Shift<Key>osfPageUp:vt-edit-key(prior)~Shift<Key>osfPageDown:vt-edit-key(next)<Key>osfPageUp:scroll(-1,page)<Key>osfPageDown:scroll(1,page)Mod1<Key>Break:soft-reset()Shift<Key>Break:hard-reset()~Shift ~Mod1<Key>Break:vt-break()Ctrl<Key>Cancel:stop(long)~Ctrl<Key>Cancel:stop()~Shift<Key>Tab:tab()~Mod1<Key>KP_Space:keypad-key-execute(space)~Mod1<Key>KP_Tab:keypad-key-execute(tab)~Mod1<Key>KP_Enter:keypad-key-execute(enter)~Mod1<Key>KP_F1:keypad-key-execute(f1)~Mod1<Key>KP_F2:keypad-key-execute(f2)~Mod1<Key>KP_F3:keypad-key-execute(f3)~Mod1<Key>KP_F4:keypad-key-execute(f4)~Mod1<Key>KP_Equal:keypad-key-execute(equal)~Mod1<Key>KP_Multiply:keypad-key-execute(multiply)~Mod1<Key>KP_Add:keypad-key-execute(add)~Mod1<Key>KP_Separator:keypad-key-execute(separator)~Mod1<Key>KP_Subtract:keypad-key-execute(subtract)~Mod1<Key>KP_Decimal:keypad-key-execute(decimal)~Mod1<Key>KP_Divide:keypad-key-execute(divide)~Mod1<Key>KP_0:keypad-key-execute(0)~Mod1<Key>KP_1:keypad-key-execute(1)~Mod1<Key>KP_2:keypad-key-execute(2)~Mod1<Key>KP_3:keypad-key-execute(3)~Mod1<Key>KP_4:keypad-key-execute(4)~Mod1<Key>KP_5:keypad-key-execute(5)~Mod1<Key>KP_6:keypad-key-execute(6)~Mod1<Key>KP_7:keypad-key-execute(7)~Mod1<Key>KP_8:keypad-key-execute(8)~Mod1<Key>KP_9:keypad-key-execute(9)Shift<Key>F1:vt-function-key-execute(1, UDK)Shift<Key>F2:vt-function-key-execute(2, UDK)Shift<Key>F3:vt-function-key-execute(3, UDK)Shift<Key>F4:vt-function-key-execute(4, UDK)Shift<Key>F5:vt-function-key-execute(5, UDK)Shift<Key>F6:vt-function-key-execute(6, UDK)Shift<Key>F7:vt-function-key-execute(7, UDK)Shift<Key>F8:vt-function-key-execute(8, UDK)Shift<Key>F9:vt-function-key-execute(9, UDK)Shift<Key>F10:vt-function-key-execute(10, UDK)Shift<Key>F11:vt-function-key-execute(11, UDK)Shift<Key>F12:vt-function-key-execute(12, UDK)Shift<Key>F13:vt-function-key-execute(13, UDK)Shift<Key>F14:vt-function-key-execute(14, UDK)Shift<Key>F15:vt-function-key-execute(15, UDK)Shift<Key>F16:vt-function-key-execute(16, UDK)Shift<Key>F17:vt-function-key-execute(17, UDK)Shift<Key>F18:vt-function-key-execute(18, UDK)Shift<Key>F19:vt-function-key-execute(19, UDK)Shift<Key>F20:vt-function-key-execute(20, UDK)Shift<Key>F21:vt-function-key-execute(21, UDK)Shift<Key>F22:vt-function-key-execute(22, UDK)Shift<Key>F23:vt-function-key-execute(23, UDK)Shift<Key>F24:vt-function-key-execute(24, UDK)Shift<Key>F25:vt-function-key-execute(25, UDK)Shift<Key>F26:vt-function-key-execute(26, UDK)Shift<Key>F27:vt-function-key-execute(27, UDK)Shift<Key>F28:vt-function-key-execute(28, UDK)Shift<Key>F29:vt-function-key-execute(29, UDK)Shift<Key>F30:vt-function-key-execute(30, UDK)Shift<Key>F31:vt-function-key-execute(31, UDK)Shift<Key>F32:vt-function-key-execute(32, UDK)Shift<Key>F33:vt-function-key-execute(33, UDK)Shift<Key>F34:vt-function-key-execute(34, UDK)Shift<Key>F35:vt-function-key-execute(35, UDK)~Shift<Key>F1:vt-function-key-execute(1, function)~Shift<Key>F2:vt-function-key-execute(2, function)~Shift<Key>F3:vt-function-key-execute(3, function)~Shift<Key>F4:vt-function-key-execute(4, function)~Shift<Key>F5:vt-function-key-execute(5, function)~Shift<Key>F6:vt-function-key-execute(6, function)~Shift<Key>F7:vt-function-key-execute(7, function)~Shift<Key>F8:vt-function-key-execute(8, function)~Shift<Key>F9:vt-function-key-execute(9, function)~Shift<Key>F10:vt-function-key-execute(10, function)~Shift<Key>F11:vt-function-key-execute(11, function)~Shift<Key>F12:vt-function-key-execute(12, function)~Shift<Key>F13:vt-function-key-execute(13, function)~Shift<Key>F14:vt-function-key-execute(14, function)~Shift<Key>F15:vt-function-key-execute(15, function)~Shift<Key>F16:vt-function-key-execute(16, function)~Shift<Key>F17:vt-function-key-execute(17, function)~Shift<Key>F18:vt-function-key-execute(18, function)~Shift<Key>F19:vt-function-key-execute(19, function)~Shift<Key>F20:vt-function-key-execute(20, function)~Shift<Key>F21:vt-function-key-execute(21, function)~Shift<Key>F22:vt-function-key-execute(22, function)~Shift<Key>F23:vt-function-key-execute(23, function)~Shift<Key>F24:vt-function-key-execute(24, function)~Shift<Key>F25:vt-function-key-execute(25, function)~Shift<Key>F26:vt-function-key-execute(26, function)~Shift<Key>F27:vt-function-key-execute(27, function)~Shift<Key>F28:vt-function-key-execute(28, function)~Shift<Key>F29:vt-function-key-execute(29, function)~Shift<Key>F30:vt-function-key-execute(30, function)~Shift<Key>F31:vt-function-key-execute(31, function)~Shift<Key>F32:vt-function-key-execute(32, function)~Shift<Key>F33:vt-function-key-execute(33, function)~Shift<Key>F34:vt-function-key-execute(34, function)~Shift<Key>F35:vt-function-key-execute(35, function)<KeyRelease>:key-release()<KeyPress>:insert()~Shift~Ctrl<Btn1Down>:grab-focus()Shift~Ctrl<Btn1Down>:extend-start()~Ctrl<Btn1Motion>:select-adjust()~Ctrl<Btn1Up>:extend-end()~Shift<Btn2Down>:process-bdrag()~Shift<Btn2Up>:copy-to()<EnterWindow>:enter()<LeaveWindow>:leave()<FocusIn>:focus-in()<FocusOut>:focus-out()dtterm-Escape-ZeichenfolgenJeder der im folgenden aufgeführten Abschnitte enthält eine
Liste der
anwendbaren Escape-Zeichenfolgen.
Nähere Einzelheiten enthält die elektronische Handbuchseitedtterm(5x).Positioniertasten, VT220-ModusDie folgende Tabelle zeigt die Tasten und die entsprechenden gesendeten
Escape-Zeichenfolgen für den normalen Modus und den Anwendungsmodus.Modus der Positioniertasten
 TASTE               Normal   Anwendung
=================    ======   ===========
Cursor aufwärts      Esc[A      EscOA
Cursor abwärts       Esc[B      EscOB
Cursor nach rechts   Esc[C      EscOC
Cursor nach links    Esc[D      EscODZusatztastenblock, ANSI-ModusDie folgende Tabelle zeigt die Tasten und die entsprechenden gesendeten
Escape-Zeichenfolgen für den normalen Modus und den Anwendungsmodus.Application Keypad Mode
  TASTE       Normal      Anwendung
=========     =========== ===========
Leertaste     Leerzeichen EscOA
Tabulator     Tabulaotor  EscOI
Eingabe       CR/CR-LF    EscOM
PF1           EscOP       EscOP
PF2           EscOQ       EscOQ
PF3           EscOR       EscOR
PF4           EscOS       EscOS
* (Multipl.)     *        EscOj
+ (Addition)     +        EscOk
, (Komma)        ,        EscOl
- (Subtraktion)  -        EscOm
. (Punkt)        .        EscOn
/ (Division)     /        EscOo
0                0        EscOp
1                1        EscOq
2                2        EscOr
3                3        EscOs
4                4        EscOt
5                5        EscOu
6                6        EscOv
7                7        EscOw
8                8        EscOx
9                9        EscOy
=(gleich)        =        EscOXFunktionstasten, VT220-ModusDie folgende Tabelle zeigt die Tasten und die entsprechenden gesendeten
Escape-Zeichenfolgen.TASTE        gesendete Escape-Zeichenfolge
=========     =============================
F1                   Esc[11~
F2                   Esc[12~
F3                   Esc[13~
F4                   Esc[14~
F5                   Esc[15~
F6                   Esc[17~
F7                   Esc[18~
F8                   Esc[19~
F9                   Esc[20~
F10                  Esc[21~
F11                  Esc[23~
F12                  Esc[24~
F13                  Esc[25~
F14                  Esc[26~
F15                  Esc[28~
F16                  Esc[29~
F17                  Esc[31~
F18                  Esc[32~
F19                  Esc[33~
F20                  Esc[34~
Help                 Esc[28~
Menu                 Esc[29~
Find                 Esc[1~
Insert               Esc[2~
Remove               Esc[3~
Select               Esc[4~
Prior                Esc[5~
Next                 Esc[6~Funktionstasten, sunFunctionKeys-ModusDie folgende Tabelle zeigt die Tasten und die entsprechenden gesendeten
Escape-Zeichenfolgen.TASTE        gesendete Escape-Zeichenfolge
=========     =============================
F1                   Esc[224z
F2                   Esc[225z
F3                   Esc[226z
F4                   Esc[227z
F5                   Esc[228z
F6                   Esc[229z
F7                   Esc[230z
F8                   Esc[231z
F9                   Esc[232z
F10                  Esc[233z
F11                  Esc[192z
F12                  Esc[193z
F13                  Esc[194z
F14                  Esc[195z
F15                  Esc[196z
F16                  Esc[197z
F17                  Esc[198z
F18                  Esc[199z
F19                  Esc[200z
F20                  Esc[201z
F21 (R1)             Esc[208z
F22 (R2)             Esc[209z
F23 (R3)             Esc[210z
F24 (R4)             Esc[211z
F25 (R5)             Esc[212z
F26 (R6)             Esc[213z
F27 (R7)             Esc[214z
F28 (R8)             Esc[215z
F29 (R9)             Esc[216z
F30 (R10)            Esc[217z
F31 (R11)            Esc[218z
F32 (R12)            Esc[219z
F33 (R13)            Esc[220z
F34 (R14)            Esc[221z
F35 (R15)            Esc[222z
Help                 Esc[196z
Menu                 Esc[197z
Find                 Esc[1z
Insert               Esc[2z
Remove               Esc[3z
Select               Esc[4z
Prior                Esc[5z
Next                 Esc[6zEmpfangene Escape-ZeichenfolgenDie folgende Tabelle beschreibt die empfangenen Escape-Zeichenfolgen,
die von
dtterm unterstützt werden.Escape-
Zeichenfolge   Beschreibung
============   ============
Strg-G         Signal (Strg-G)
Strg-H         Rückschritt (Strg-H)
Strg-I         Horizontaler Tabulator (HT) (Strg-I)
Strg-J         Zeilenschaltung oder Neue Zeile (NL) (Strg-J)
Strg-K         Vertikaler Tabulator, wie Zeilenschaltung
Strg-L         Formularvorschub oder Neue Seite, wie Zeilenschaltung
Strg-M         Rücklauf (Strg-M)
Esc ( B        ASCII (Basisschriftart) als G0 festlegen
Esc ( 0        DEC Special Graphic (line draw) als G0 festlegen
Esc ) B        ASCII (Basisschriftart) als G1 festlegen
Esc ) 0        DEC Special Graphic (line draw) als G1 festlegen
Esc * B        ASCII (Basisschriftart) als G2 festlegen
Esc * 0        DEC Special Graphic (line draw) als G2 festlegen
Esc + B        ASCII (Basisschriftart) als G3 festlegen
Esc + 0        DEC Special Graphic (line draw) als G3 festlegen
Strg-N         G1 in GL abbilden
Strg-O         G0 in GL abbilden
Esc n          G2 in GL abbilden
Esc o          G3 in GL abbilden
Esc N          G2 in GL für das nächste Zeichen abbilden
Esc O          G3 in GL für das nächste Zeichen abbilden
Esc Space F    7-Bit-C1-Steuerzeichen auswählen. In diesem Modus sendet
               das dtterm-Dienstprogramm alle C1-Steuerzeichen als
               7-Bit-Escape-Zeichenfolgen zum Host. D. h., CSI wird
               als&newline;f2Esc&newline;fP [zum Host gesendet.
Esc Space G    8-Bit-C1-Steuerzeichen auswählen. In diesem Modus sendet
               das dtterm-Dienstprogramm alle C1-Steuerzeichen als
               8-Bit-Steuercodes. D. h., CSI wird als der Hexadezimalwert
               0x9B zurückgesendet.
Esc#8          DEC Screen Align Test (DECALN)
Esc7           Cursor sichern (DECSC)
Esc8           Cursor wiederherstellen (DECRC)
Esc=           Anwendungstastenblock (DECPAM)
Esc>           Normaler Tastenblock (DECPNM)
EscD           Index (IND)
EscE           Nächste Zeile (NEL)
EscH           Tabulator (HTS)
EscM           Umkehrindex (RI)
EscPpi;pi|pi/hex digits;pi/hex digits;...Esc&newline;
               Gerätesteuerzeichenfolge (DCS)
EscZ           Terminal-ID zurückgeben (DECID)
Escc           Vollständige Grundstellung (RIS)
Escn           G2-Zeichensatz (LS2) auswählen
Esco           G3-Zeichensatz (LS3) auswählen
Esc[pi"p       Kompatibilitätsstufe auswählen (DECSCL)
Esc[pi@        Leerzeichen einfügen (ICH)
Esc[piA        Cursor aufwärts (CUU)
Esc[piB        Cursor abwärts (CUD)
Esc[piC        Cursor nach rechts (CUF)
Esc[piD        Cursor nach links (CUB)
Esc[piF        Cursor zurpiten vorherigen Zeile (CPL)
Esc[piG        Cursor zur Spalte p (CHA)
Esc[pi;piH     Cursorposition (CUP)
Esc[piJ        In Anzeige löschen (ED)
Esc[piK        In Zeile löschen (EL)
Esc[piL        Zeilen einfügen (IL)
Esc[piM        Zeilen löschen (DL)
Esc[piP        Zeichen löschen (DCH)
Esc[piS        p Zeilen rückwärts blättern (SU)
Esc[piT        Vorwärts blättern (SD)
Esc[piXpiZeichen löschen (ECH)
Esc[pic        Geräteattribute senden
Esc[pi;pif     Horizontale und vertikale Position (HVP)
Esc[pig        Tabulatorstopp löschen (TBC)
Esc[pih        Definitionsmodus (SM)
Esc[pil        Rücksetzmodus (RM)
Esc[pim        Zeichenattribute (SGR)
Esc[pin        Gerätestatusbericht (DSR)
Esc[pi;pir     Bildlaufbereich setzen (DECSTBM)
Esc[pix        Terminal-Parameter anfordern
Esc[?pih       DEC Private Mode Set (DECSET)
Esc[?pil       DEC Private Mode Reset (DECRSET)
Esc[?pin       DEC Private Mode Status (DSR)
Esc[?pir       Werte des DEC Private Mode wiederherstellen
Esc[?pis       Werte des DEC Private Mode sichern
Esc]?pi;piStrg-G
               Textparameter setzen
Esc]p1;p2;p3tSun-Escape-Zeichenfolgen
Esc_piEsc\     Anwendungsprogramm
Esc[?piK       Selektives Löschen in Zeile (DECSEL)
Esc[?piJ       Selektives Löschen in Anzeige (DECSED)
Esc!p          Soft Terminal Reset (DECSTR)Informationen zur dtterm-TastaturInformationen zur TastaturDieser Abschnitt enthält eine Liste der Tastaturfunktionen, die
dtterm zugeordnet sind. Es beinhaltet nur die Tasten, die eine besondere Bedeutung
für dtterm haben. Sie gelten nur, wenn der Terminal-Textbereich den Tastaturfokus
hat. Sie geltennicht, wenn der Tastaturfokus bei einem
Dialogfenster, Aktionsleistenmenü oder Dialogfenstermenü liegt.
In einigen Fällen werden zusätzliche Kombinationstaste ignoriert.
Beispielsweise gilt die Beschreibung für F1 nicht für Umschalttaste
F1, aber die Beschreibung der Tabulatortaste gilt auch für Umschalttaste
Tabulatortaste.<Key>Home          Blättert zum Anfang des Puffers.
Shift<Key>Home     Blättert zum Ende des Puffers.
Shift<Key>Prior    Blättert eine Seite zurück.
Shift<Key>Next     Blättert eine Seite vor.
<Key>Up            Sendet die Escape-Zeichenfolge, wie in dtterm(5x)
                    beschrieben.
<Key>Down
<Key>Left
<Key>Right
<Key>Prior
<Key>Next
<Key>Find
<Key>Insert
<Key>Select

<Key>Cancel        Schaltet die Unterprozessausgabe ein/aus.

<Key>Tab           Sendet ein Tabulatorzeichen.

<Key>Break         Sendet eine RS232-Unterbrechung an den Unterprozeß.
Meta<Key>Break     Warmstart, wie in dtterm(5x) beschrieben.
Shift<Key>Break    Kaltstart, wie in dtterm(5x) beschrieben.

<Key>F1            Sendet eine Escape-Zeichenfolge für Funktionstasten, wie
 bis                in dtterm(5x) beschrieben.
<Key>F35

Shift<Key>F1       Sendet eine benutzerdefinierte Zeichenfolge
 bis                (falls vorhanden) für diese Taste.
Shift<Key>F35

Ctrl<Key>F10       Activiert die Menüleiste.

ShiftCtrl<Key>F10  Activiert das Dialogfenstermenü.

Esc                Sendet die Escape-Zeichenfolge.

<Key>KP_F1        Sendet die Escape-Zeichenfolge, wie in
 bis               dtterm(5x) beschrieben.
<Key>KP_F4

<Key>KP_0         Sendet die Zeichenfolge oder Escape-Zeichenfolge,
 bis               wie in dtterm(5x) beschrieben.
<Key>KP_9

<Key>KP_Equal
<Key>KP_Multiply
<Key>KP_Add
<Key>KP_Separator
<Key>KP_Subtract
<Key>KP_Decimal
<Key>KP_Divide
<Key>KP_Space
<Key>KP_Tab
<Key>KP_EnterBeachten Sie, daß diese Tasten nicht auf den Tastaturen aller
Händler vorhanden sind. Bitte lesen Sie in der Dokumentation Ihres lokalen
Händlers über alternative Tastenkombinationen nach.