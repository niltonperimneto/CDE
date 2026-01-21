Sprachenabh&auml;ngige SessionsDie Benutzerschnittstelle des Desktops l&auml;&szlig;t sich in vielen
verschiedenen Sprachen anpassen. Verschiedene Elemente wie beispielsweise
Bildschirme, Standardsprachen, Schriftarten, Eingabemethoden (Tastatur) und
Symbole k&ouml;nnen ver&auml;ndert werden. Zus&auml;tzlich k&ouml;nnen die
Men&uuml;s, Online-Hilfefunktionen und Fehlermeldungen sprachenabh&auml;ng
eingestellt werden und sind in mehreren Sprachen verf&uuml;gbar.sprachspezifischSessionsSessionsprachspezifischeStandardsprache bei der AnmeldungDie Meldungen und Men&uuml;s im Eingangsanmeldefenster werden in der
Standardsprache angezeigt. Wurde keine Standardsprache definiert, werden die
Meldungen und Men&uuml;s unter Verwendung der generischen 'C'-Umgebung als
l&auml;nderspezifische Angaben angezeigt. Die Sprache kann im Men&uuml; Optionen
im Anmeldebildschirm ge&auml;ndert werden.Anmelden zu einer sprachspezifischen SessionsprachspezifischAnmeldungAnmeldenzu einer sprachspezifischen SessionAnmeldung, sprachspezifischeEs ist einfach, sich durch das Desktop zu einer sprachspezifischen Session
anzumelden. Es k&ouml;nnen jedoch spezifische Hardwarevoraussetzungen wie
zum Beispiel Tastatur und Drucker n&ouml;tig sein, um die eingegrenzte Session
verwendbarer zu machen. Diese Voraussetzungen unterscheiden sich je nach Sprache,
Zeichensatz und Land. Software und Zeichens&auml;tze k&ouml;nnen die effektive
Sprachenabh&auml;ngigkeit des Systems weiter erh&ouml;hen. Anmelden einer
sprachspezifischen Session:Men&uuml; 'Option' im Anmeldebildschirm verwenden, um eine
Sprache zu w&auml;hlen.Die Sprachenliste enth&auml;lt alle Sprachen, die unterst&uuml;tzt werden.Normal mit Name und Kennwort anmelden.Erstellen oder Bearbeiten einer sprachspezifischen
DateisprachspezifischDatenSprachspezifische Dateien k&ouml;nnen erstellt, bearbeitet und gedruckt
werden. Zudem k&ouml;nnen Dateien l&auml;nderspezifische Namen gegeben werden.
Allerdings sollten die Namen von Systemverwaltungsdateien, die in einem Netzwerk
gemeinsam genutzt werden, nur ASCII-Zeichen enthalten. Verschiedene Systeme
im Netzwerk d&uuml;rfen verschiedene l&auml;nderspezifische Angaben verwenden.Erfolgte die Anmeldung auf einem Desktop in einer spezifischen Sprache,
werden alle Anwendungen in dieser Sprache aufgerufen. Trotzdem kann eine Anwendung
auch in einer anderen Sprache aufgerufen werden.Soll eine Datei in einer anderen Sprache erstellt werden, mu&szlig;
ein neues Exemplar des Texteditors aufgerufen und die gew&uuml;nschte Sprache
festgelegt werden.Erstellen oder Bearbeiten einer sprachspezifischen DateiErstelleneiner Datei mit sprachspezifischen
ZeichenBearbeiteneiner Datei mit sprachspezifischen ZeichenDateienmit sprachspezifischen ZeichensprachspezifischZeichen
in einer DateiZur Erstellung oder Bearbeitung einer sprachspezifischen Datei kann
entweder der Text-Editor in der jeweiligen Sprache direkt gestartet werden,
oder Sie k&ouml;nnen dieLANG-Umgebungsvariable
vor dem Starten des Editors einstellen.Um den Text-Editor mit der Sprache direkt zu starten, den Befehldtpadaufrufen, wobei die Sprache f&uuml;r die Option-xnllanguageangegeben wird. Zum Beispiel:/usr/dt/bin/dtpad -xnllanguageJapanese_localenameUmLANGvor dem Aufrufen
des Editors einzustellen, die folgenden Schritte durchf&uuml;hren:DieLANG-Umgebungsvariable
in einem Terminal-Fenster auf die gew&uuml;nschte Sprache einstellen. Um zum
Beispiel die l&auml;nderspezifischen Angaben auf Japanisch einzustellen, k&ouml;nnen
Sie eingeben:TerminalSprache einstellen
durchLANG-UmgebungsvariableLANG=Japanese_localenamewobeiJapanese_localenameden japanischen
Zeichensatz angibt. Ihre spezifische Plattform bestimmt den Wert f&uuml;rJapanese_localename.In demselben Fenster den Text-Editor (dtpad) unter der gew&uuml;nschten Sprache aufrufen durch die Eingabe
von:Text-Editormit einer bestimmten
Sprache startenStartendes Text-Editors mit einer bestimmten SprachesprachspezifischText-Editor/usr/dt/bin/dtpad &Nachdem die l&auml;nderspezifischen Dateien installiert sind, k&ouml;nnen
Sie nun japanische Zeichen eingeben. Sie k&ouml;nnen au&szlig;erdem die Text-Editor-Session
zur Bearbeitung einer zuvor erstellten japanischen Datei benutzen.Ein Beispiel f&uuml;r die Angabe eines Schriftsatzes finden Sie unter.Verwendung eines sprachspezifischen Terminal-EmulatorsDas folgende Beispiel verwendetdttermund startet
einen japanischen Terminal-Emulator. Es setzt voraus, da&szlig; die Standardsprache
nicht Japanisch ist, da&szlig; Sie die Korn-Shell benutzen und da&szlig; die
l&auml;nderspezifischen Dateien installiert wurden.Terminal-EmulatorsprachspezifischsprachspezifischTerminal-EmulatorStartenden Terminal-Emulator mit einer bestimmten SpracheVon einer Befehlszeile im Terminal-Fenster einer Korn-Shell
geben Sie ein:LANG=Japanese_localenamedttermwobeiJapanese_localenameden japanischen
Zeichensatz aufruft. Ihre spezifische Plattform bestimmt den Wert f&uuml;rJapanese_localename.Angeben der SchriftartenSchriftartinternationalisierenInternationalisierung und SchriftartenDer Benutzer &auml;ndert normalerweise Schriftarten mit Hilfe des Umgebungsmanagers.
Dieser startet wiederum den Arbeitsbereichsmanager erneut und setzt die Schriftarten
des Desktops zur&uuml;ck. Schriftarten lassen sich ebenfalls von der Befehlszeile
oder in Ressourcendateien anpassen. In einer internationalisierten Umgebung
mu&szlig; der Benutzer die Schriftarten angeben, die vom codierten Zeichensatz
unabh&auml;ngig sind. Das ist notwendig, da die Spezifikation unter verschiedenen
l&auml;nderspezifischen Angaben verwendet werden kann, die andere codierte
Zeichens&auml;tze als der Zeichensatz der Schriftart haben. Daher sollten
alle Schriftartlisten mit einem Schriftartensatz angegeben werden.SchriftartspezifikationSchriftartspezifikationEineSchriftartspezifikationinnerhalb einer Schriftartliste kann entweder
ein X Logical Function Description (XLFD)-Name oder ein Alias f&uuml;r denXLFDXLFD-Namen sein. Im folgenden
werden beispielsweise g&uuml;ltige Schriftartspezifikationen f&uuml;r eine
14-Punkt-Schriftart gezeigt:-dt-interface system-medium-r-normal-serif-*-*-*-*-p-*-iso8859-1Oder:-*-r-*-14-*iso8859-1Spezifikation des SchriftartsatzesSchriftsatz, SpezifikationDieSpezifikation des Schriftartsatzesinnerhalb einer Schriftartliste
ist eine Liste mit XLFD-Namen oder deren Alias-Namen (manchmal auchListe der Dateinamen ohne Pfadgenannt). Die Namen sind durch Strichpunkte
getrennt und jedes Leerzeichen vor oder nach dem Strichpunkt wird nicht beachtet.
Platzhalterzeichen k&ouml;nnen angegeben werden, um XLFD-Namen verk&uuml;rzen
zu helfen.Dateiname ohne Pfad, SchriftartlisteDie Spezifikation eines Schriftartsatzes wird durch die l&auml;nderspezifischen
Angaben festgelegt, die gerade ausgef&uuml;hrt werden. Zum Beispiel definieren
die l&auml;nderspezifischen Angaben f&uuml;r Japanisch drei Schriftarten (Zeichens&auml;tze),
die n&ouml;tig sind, um alle Zeichen der l&auml;nderspezifischen Angaben anzuzeigen.
Das folgende Beispiel kennzeichnet den Satz ben&ouml;tigter Mincho-Schriftarten.Beispiel einer Namenliste mit Zeichensatz:-dt-interface system-medium-r-normal-serif-*-*-*-*-p-*-14;
-dt-mincho-medium-r-normal--14-*-*-m-*-jisx0201.1976-0;
-dt-mincho-medium-r-normal--28-*-*-*-m-*-jisx0208.1983-0:Beispiel eines einzigen Musternamens ohne Zeichensatz:-dt-*-medium-*-24-*-m-*:Die beiden vorangegangenen Vorg&auml;nge k&ouml;nnen mit l&auml;nderspezifischen
Angaben f&uuml;r Japanisch verwendet werden, solange es Schriftarten gibt,
die die Liste der Dateinamen ohne Pfad abgleichen.Wechseln von SchriftartenDie Schriftarten desDttermk&ouml;nnen durch eine
der beiden folgenden Methoden ge&auml;ndert werden:Schriftarten von der Befehlszeile aus angebenSchriftarten innerhalb einer Ressourcendatei angeben.Angeben der Schriftarten von der Befehlszeile ausSchriftartvon der Befehlszeile
aus angebenUm die Schriftarten des Men&uuml;s von
der Befehlszeile aus zu &auml;ndern, mu&szlig; folgendes eingeben werden:dtterm -xrm '*fontList: fontset'Fontsetist dabei eine Spezifikation eines Schriftartsatzes.
Die Spezifikation eines Schriftartsatzes kann angegeben werden durch eine
volle XLFD-Namenliste (X Logical Font Description), ein einfaches XLFD-Muster
oder einen Alias-Namen. Dabei ist zu beachten, da&szlig; die Spezifikation
eines Schriftartsatzes durch die l&auml;nderspezifischen Angaben festgelegt
sind, die gerade ausgef&uuml;hrt werden.BeispieleSoll mit Ausnahme des Men&uuml;s &uuml;berall eine gr&ouml;&szlig;ere
Schriftart verwendet werden, mu&szlig; folgendes eingegeben werden:dtterm -xrm '*fontList:-dt-interface user-medium-r-normal-l*-*-*-*:'Soll mit Ausnahme des Men&uuml;s &uuml;berall eine kleinere Schriftart
verwendet werden, mu&szlig; folgendes eingegeben werden:dtterm -xrm '*fontList:-dt-interface user-medium-r-normal-s*-*-*-*:'Diese Spezifikationen funktionieren bei allen l&auml;nderspezifischen
Angaben.Schriftartvon der Befehlszeile
aus angebenZur Angabe von Schriftarten innerhalb einer RessourcendateiObwohl es m&ouml;glich ist, Schriftarten durch die Bearbeitung von Ressourcendateien
einer Anwendung im Verzeichnis/usr/dt/app-defaults/languageeinzustellen, wird dieses Verfahren nicht
empfohlen. Solche Dateien werden bei jeder Neuinstallation automatisch &uuml;berschrieben.
Schriftarten sollten stattdessen eingestellt werden, indem die Ressourcen
Ihrer pers&ouml;nlichen DateiStandardverzeichnis/.Xdefaultshinzugef&uuml;gt werden.Ausw&auml;hlen der Eingabemethode und der TastaturAlle l&auml;nderspezifischen Angaben haben eine einzelne, mit ihnen
verbundene, Standardeingabemethode. Falls der Benutzer nichts &auml;ndert,
wird diese Standardeinstellung ausgew&auml;hlt. Da m&ouml;glicherweise viele
Eingabemethoden zu jeder beliebigen Zeit installiert sein k&ouml;nnen, erkl&auml;ren
die n&auml;chsten Abschnitte, wie verschiedene Eingabemethoden durch den Benutzer
ausgew&auml;hlt werden k&ouml;nnen.&Uuml;ber die Verwendung von Ressourcen zur Einstellung der Eingabemethode
und des Eingabemethodenstils f&uuml;r die Vorbearbeitung hinaus kann das BedienelementIntl'(Internationalisierung) der Umgebungsparameter benutzt werden,
um diese Werte interaktiv einzustellen. Details finden Sie in den B&uuml;chernCDE Benutzerhandbuch f&uuml;r Fortgeschrittene und Systemverwalter