dtterm - AufgabenStarten von dttermMehrere Möglichkeiten stehen zur Verfügung, um einen &ProductName;-dtterm-Terminal-Emulator zu starten:Vom Bedienfeld ausVom Dateimanager ausMit Hilfe eines Befehls in einem bestehenden
Terminal-FensterVom Anwendungsmanager ausDurch die Auswahl 'Erstellen' imdtterm-Menü 'Fenster'Starten von dtterm vom Bedienfeld ausStarten: Terminal-EmulatorSchließen: Terminal-EmulatorTerminal: TasteTaste: TerminalDas Steuerelement für Terminal-Fenster befindet sich in der
Bedientafel
'Persönliche Anwendungen'.Auf das Steuerelement für Terminal-Fenster
klicken. Die Betriebsanzeige
blinkt und zeigt an, daß das Terminal-Fenster aktiviert wird.Nach einem kurzen Moment wird dasdtterm-Fenster angezeigt.Starten von dtterm vom Dateimanager aus'Terminal-Fenster öffnen' aus
dem Menü 'Datei' auswählen.Dadurch wirddttermmit demselben aktuellen
Verzeichnis geöffnet
wie das Dateimanagerfenster.Starten eines anderen Terminal-Emulators anstelle von dttermUm einen anderen Terminal-Emulator alsdttermzu verwenden, diesen von einer Befehlszeile
in einem
bestehenden Terminal-Emulator-Fenster aus starten.An der Eingabeaufforderung den Namen
des Terminal-Emulators und die
gewünschten Optionen eingeben. Um beispielsweisextermzu starten, folgendes eingeben:xterm  [Optionen] &OptionenStellt wahlfreie Elemente zum Anpassen des Terminal-Emulators dar.&Gibt an, daß der Terminal-Emulator im Hintergrund läuft,
so
daß der Benutzer in seinem ursprünglichen Fenster weiterarbeiten
kann, während der Terminal-Emulator läuft.Der Terminal-Emulator startet im aktuellen Arbeitsbereich, es sei denn,
durch
die Optionen wurde ein anderer Bereich angegeben.Starten von dtterm vom Fenstermenü ausDie Option 'Neu' aus dem Fenstermenü
in einem bestehendendtterm-Fenster auswählen. Ein dupliziertesdtterm-Fenster erscheint.BeispieleDer folgende Befehl startet eindtterm-Fenster
im ArbeitsbereichProject Notes:dtterm -xrm '*workspaceList: "Project Notes"' &Der folgende Befehl startet eindtterm-Fenster
auf einem Bildschirm auf dem System "lgmcd":dtterm -display lgmcd:0.1 &Siehe auchWeitere Einzelheiten zu den fürdttermzur Verfügung stehenden Optionen befinden sich
in der elektronischen Handbuchseitedtterm (1X).Schließen von dttermSchließen:dttermFenstermenü: TasteIn der Befehlszeileexiteingeben und die Eingabetaste drücken.Oder'Schließen'
aus dem Menü 'Fenster' auswählen.Oder'Schließen'
aus dem Fenstersteuerungsmenü auswählen (auf das über die Taste
in der oberen linken Ecke des Fensterrahmens zugegriffen werden kann).Die Auswahldttermaus dem Menü
'Fenster' ist die bevorzugte Methode,dttermzu schließen.Kopieren und Einfügen von TextAusschneiden: TextEinfügen: TextText: ausschneiden und einfügenKopieren von TextMit gedrückter Maustaste 1 den
Zeiger über den Text ziehen, der kopiert werden soll. Der Text wird hervorgehoben.Die Maustaste 1 loslassen, wenn der
gewünschte Text hervorgehoben ist.Der Text wird nicht von seiner ursprünglichen Position entfernt.Einfügen von TextDer Cursor an die Stelle setzen, an
der der Text eingefügt werden soll.Auf die Maustaste 2 klicken.Eine Kopie der aktuellen Auswahl wird an der angegebenen Position eingefügt.
Zusätzliche Kopien können eingefügt werden, indem diese Schritte
wiederholt werden.Ändern der Größe des dtterm-FenstersGröße des Fensters ändern'Fenstergröße' aus dem Menü
'Optionen' auswählen.Eine der drei Größen auswählen:80x24132x24StandardIn einigen Fällen, je nach verwendeter Anzeigen- und Schriftartgröße,
ist es nicht möglich, dasdtterm-Fenster
auf 132 Spalten zu vergrößern. Tritt dieser Fall auf, vergrößertdttermdas Fenster auf die unter den gegebenen Umständen
maximal zulässige Spaltenanzahl.Dasdtterm-Fenster kann auch mit Hilfe
der Bedienelemente der Fenstersteuerung in der
Größe verändert werden.Siehe auchÄndern der Größe des Fensterinhaltsresize: FunktionÄndert der Benutzer die Größe eines Terminal-Emulator-Fensters,
ist den
Anwendungen, die in dem Fenster laufen, die Größenänderung
möglicherweise
nicht automatisch bekannt. Die folgende Prozedur verwenden, um die
Größe der Anwendungsausgabe zu ändern.An einer Eingabeaufforderung folgendes
eingeben:eval `resize`Siehe auchStarten von Anwendungen in einem dtterm-FensterDen Befehl zum Starten der Anwendung
an einer Eingeabeaufforderung eingeben.Die allgemeine Syntax zum Starten einer Anwendung lautet:Anwendung[Optionen] &AnwendungDer Anwendungsname.optionsEine Liste wahlfreier Informationen, die an die Anwendung
übergeben werden sollen.&Gibt an, daß die Anwendung im Hintergrund läuft, so daß
der
Benutzer im Terminal-Emulator-Fenster weiterarbeiten kann,
während die Anwendung läuft.BeispielUm eine digitale Uhr von der Befehlszeile aus zu starten, folgendes
eingeben:xclock -digital &Siehe auchInformationen zum jeweiligen Befehl
und dessen Optionen befinden sich
auf der elektronischen Handbuchseite oder in anderen Dokumentationen
zur entsprechenden Anwendung.Anmelden an einem fernen SystemVerwendung von rloginDen Befehlrloginin einem bestehenden
Terminal-Emulator-Fenster verwenden, um sich an einem fernen Host anzumelden.
Dient das Fenster dann als ein Terminal zum fernen Host, kann der Benutzer
dort Anwendungen ausführen und, falls gewünscht, die Bildschirmausgabe
zu seinem System umleiten.BeispielMit dem folgenden Befehl meldet sich der Benutzer am Systemtherean, führt den Clientxloadaus und leitet die Bildschirmausgabe zu seinem ursprünglichen System
um. Dabei wird angenommen, der Benutzer arbeitet auf dem Systemhere.rlogin there
xload -display here:0Verwendung von remshDer Befehlremshstartet eine
Schale auf einem fernen Host, führt dort einige C