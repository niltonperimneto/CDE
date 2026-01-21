
# Symboleditor - Referenz











# Suchpfade für Symbole
Suchpfade: für SymboleSymbole: Suchpfade

Der Suchpfad für Abbilddateien ist durch zwei Umgebungsvariablen definiert:

Das System verwendet DTUSERAPPSEARCHPATH und DTAPPSEARCHPATH, es sei denn,
die folgenden Variablen existieren:

* **DTUSERICONSEARCHPATH** 

Eine persönliche Variable. Falls sie verwendet
wird, sollte sie in der Datei/`Standardverzeichnis`/.dtprofiledefiniert sein.
* **DTICONSEARCHPATH** 

Dies ist eine systemweite Variable.

# Standardsuchpfad


Der Standardwert für DTICONSEARCHPATH ist:/`Standardverzeichnis`/.dt/icons/etc/dt/appconfig/icons/C/usr/dt/appconfig/icons/C
# Ändern des persönlichen Suchpfads für Symbole
Hinzufügen: Verzeichis zum AktionssuchpfadVerzeichnis: zum Aktionssuchpfad hinzufügenAktion: Suchpfad, Verzeichnis hinzufügenSuchpfad: AktionenPfad: AktionssuchpfadDTACTIONSEARCHPATH: UmgebungsvariableUmgebungsvariable: DTACTIONSEARCHPATH

Um ein Verzeichnis zum Suchpfad hinzuzufügen,/`Standardverzeichnis`/.dtprofileeditieren. Eine Zeile hinzufügen, die
einen Wert für DTUSERICONSEARCHPATH definiert:DTUSERICONSEARCHPATH=`Pfad`[,`Pfad`...]
# Ändern des systemweiten Suchpfads für Symbole


Systemweite Variablen für Suchpfade sind in Dateien definiert, die sich im
Verzeichnis/etc/dt/Xsession.dbefinden.

Falls DTICONSEARCHPATH bereits in einer Datei in diesem Verzeichnis
definiert ist, den Wert editieren, um das neue Verzeichnis im Suchpfad
hinzuzufügen.

Falls DTICONSEARCHPATH noch nicht in diesem Verzeichnis definiert ist, sollte
es definiert werden, um den Standardsuchpfad sowie andere Pfade, die
hinzugefügt werden sollen, einzuschließen.
(Der Standardsuchpfad wird als Kommentar in der Datei/usr/dt/bin/dtsearchpathangeführt.)
# Werkzeuge des Symboleditors
Werkzeuge: SymboleditorSymboleditor: WerkzeugeWerkzeuge: zeichnen

Wählt der Benutzer eines der Werkzeuge des Symboleditors aus, bleibt es
so lange ausgewählt, bis ein anderes ausgewählt wird. Die Werkzeuge werden
im folgenden aufgelistet und beschrieben:

Bleistift&newline;Zum freihändigen Zeichnen von Linien und einzelnen Pixeln.

Linie&newline;Zum Zeichnen von Geraden.

Rechteck&newline;Zum Zeichnen von ausgefüllten oder umrissenen Rechtecken.

Kreis&newline;Zum Zeichnen von ausgefüllten oder umrissenen Kreisen.

Löschen&newline;Zum Löschen großer Bereiche des Abbilds.

Einfärben&newline;Zum Einfärben eines farbigen Bereichs mit einer anderen
ausgewählten Farbe.

Linienzug&newline;Zum Zeichnen von verbundenen geraden Linien.

Vieleck&newline;Zum Zeichnen von verbundenen geraden Linien, wobei die erste Linie mit der
letzten Linie verbunden ist, um ein Vieleck zu bilden.

Ellipse&newline;Zum Zeichnen von ausgefüllten oder umrissenen Ellipsen.

Auswählen&newline;Zum Treffen einer primären Auswahl. Für einige Optionen im Menü 'Bearbeiten'
ist es erforderlich, zunächst eine primäre Auswahl zu treffen.


# Flächen ausfüllen


'Flächen ausfüllen' unterhalb der Werkzeugpalette auswählen, um die
Werkzeuge für Rechteck, Vieleck, Kreis und Ellipse in der Werkzeugpalette
von Umrissen zu ausgefüllten Formen zu ändern.
(Siehe auch.)
# Menüs im Symboleditor











# Menü 'Datei' im Symboleditor




* **Erstellen** 

Fordert zur Eingabe der Größe (Breite und Höhe, in Pixeln) auf
und erstellt dann einen leeren Zeichenbereich in
dieser Größe.
(Siehe.)
* **Öffnen** 

Öffnet eine bestehende Bitmap- oder Pixmap-Datei.
(Siehe.)
* **Speichern** 

Speichert das aktuelle Symbol unter seinem bestehenden Namen.
Ist das Symbol nicht benannt, fordert der Symboleditor zur
Eingabe eines Namens auf.
(Siehe.)
* **Speichern als** 

Fordert zur Eingabe eines neuen Namens auf und speichert dann
das aktuelle Symbol.
(Siehe.)
* **Beenden** 

Schließt den Symboleditor. Wurden Änderungen noch nicht
gesichert, gibt der Symboleditor eine Warnung aus, daß die
Änderungen verlorengehen, wenn mit dem Beenden fortgefahren
wird. Um die Änderungen zu sichern, 'Abbrechen' auswählen und
dann die Änderungen sichern.

# Menü 'Bearbeiten' im Symboleditor




* **Rückgängig** 

Hebt die allerletzte Zeichenoperation wieder auf und gibt das
Symbol im vorherigen Status zurück. Die meisten Operationen
innerhalb des Symboleditors können rückgängig gemacht werden.
(Siehe.)
* **Bereich ausschneiden** 

Schneidet den ausgewählten Bereich aus dem Symbol aus. Der
entfernte Bereich wird mit der Farbe Transparent gefüllt.
Der ausgeschnittene Bereich wird in der Zwischenablage des
Symboleditors gesichert.
(Siehe.)
* **Bereich kopieren** 

Kopiert den ausgewählten Bereich in die Zwischenablage des
Symboleditors.
(Siehe.)
* **Bereich einfügen** 

Fügt den Inhalt der Zwischenablage in das Symbol ein.
(Sieheoder.)
* **Bereich drehen** 

Dreht den ausgewählten Bereich um 90 Grad nach links oder
rechts. Die Richtung aus dem Untermenü auswählen.
(Siehe.)
* **Bereich wenden** 

Erstellt ein Spiegelabbild des ausgewählten Bereichs, indem er
vertikal oder horizontal gespiegelt wird. Die Richtung aus
dem Untermenü auswählen.
(Siehe.)
* **Bereich skalieren** 

Ermöglicht es, daß der ausgewählte Bereich in der Größe
verändert (oder "skaliert") wird.
(Siehe.)
* **Symbolgröße ändern** 

Fordert zur Eingabe einer neuen Größe für das aktuelle
Symbol auf.
(Siehe.)
* **Brennpunkt hinzufügen** 

Läßt den Benutzer ein einzelnes Pixel im aktuellen Symbol als
"Brennpunkt" angeben. Nach Auswahl der Option 'Brennpunkt
hinzufügen' auf das Pixel klicken, das zum Brennpunkt werden
soll.

Brennpunkte werden bei der Erstellung von Abbildern für
Mauszeiger verwendet -- der Brennpunkt stellt dietatsächlichePosition des Zeigers dar.
(Siehe.)
* **Brennpunkt löschen** 

Löscht den Brennpunkt aus dem aktuellen Symbol.
* **Bildausschnitt übernehmen** 

Übernimmt einen Bereich der Anzeige und lädt ihn in den
Zeichenbereich.
(Siehe.)
Die Anzeige für die X-Y-Größe oberhalb des Arbeitsbereichs zeigt
die Größe (in Pixel) des Bereichs an, der erfasst wurde.
* **Symbol leeren** 

Löscht den Inhalt des aktuellen Bereichs für Zeichnungen.
(Siehe.)

# Menü 'Optionen' im Symboleditor




* **Raster ein** 

Schaltet das Raster um (ein und aus). Der Standardwert
ist ein.
* **Ausgabeformat** 

Legt das Ausgabeformat fest, in dem das Symbol gesichert
wird.

XBM für das zweifarbige X-Bitmap-Format. Bitmap-Dateien
werden normalerweise durch die Dateinamenerweiterung.bmidentifiziert.

XPM für das mehrfarbige X-Pixmap-Format (der
Standardwert). Pixmap-Datei werden normalerweise durch
die Dateinamenerweiterung.pmidentifiziert.

Siehe.
* **Vergrößerung** 

Ändert die Anzeigegröße des Abbilds im Bereich für
Zeichnungen. Es kann ein Vergrößerungsfaktor zwischen
2x (das Doppelte der normalen Größe) und 12x (das
Zwölffache der normalen Größe) ausgewählt werden.

# Menü 'Hilfe' im Symboleditor




* **Überblick** 

Zeigt das einführende Hilfethema für den Symboleditor an.
* **Aufgaben** 

Zeigt Aufgabenanweisungen für fast alle Operationen im
Symboleditor.
* **Referenzinformationen** 

Zeigt zusammenfassende Referenzen zu verschiedenen Funktionen
des Symboleditors an, wie z. B. Fenster und Dialoge, Menüs
und Ressourcen.
* **Kontexthilfe** 

Zeigt eine Beschreibung des ausgewählten Elements in einem
Symboleditor-Fenster.
* **Hilfe für Hilfe** 

Stellt Hilfetext zur Verwendung der Hilfefenster zur
Verfügung.
* **Informationen zu Symboleditor** 

Zeigt die Version und den Copyrightvermerk für den
Symboleditor an.

# Fenster und Dialogfenster im Symboleditor

# Subtopics







# Hauptfenster des Symboleditors


Das Hauptfenster des Symboleditors hat fünf wichtige Bereiche:

DieStatuszeilegenau unterhalb der Menüleiste beschreibt
das momentan ausgewählte Werkzeug und die Koordinaten des Pixels, auf das
der Zeiger momentan gerichtet ist.

DerZeichenbereichist der Bereich, in dem
das Symbolabbild gezeichnet wird.

DieWerkzeugpalettestellt einige Zeichenwerkzeuge zur
Verfügung, einschließlich eines Werkzeugs zum Löschen und eines zum
Auswählen.

DieFarbpalettestellt Zeichenfarben zur Verfügung:
acht statische Farben, acht statische Graustufen und sechs dynamische
Farben.

DasSymbol in Normalgrößezeigt an, wie das Symbol in der
tatsächlichen Größe aussieht. Es zeigt sowohl die Version mit allen Farben
als auch die zweifarbige Version an.
# Farbpaletten des Symboleditors




* **Statische Farben** 

Acht Standardfarben; schwarz,
weiß, die drei Primärfarben und
die drei Sekundärfarben.
* **Statische Graustufen** 

Acht Graustufen (von 10prozentigem
bis 90prozentigem Grau).
* **Dynamische Farben** 

Sechs dynamische Farbe, die sich
ändern, wenn Farben mit Hilfe der
Umgebungsverwaltung geändert
werden.


Weitere Informationen zu diesen Themen befinden sich im Hilfetext der
Umgebungsverwaltung:

Auswählen
einer Palette

Ändern der Anzahl
an Farben, die vom Desktop verwendet werden
# Dialogfenster 'Öffnen' und 'Speichern als' im Symboleditor


* **Pfad oder Ordnername eingeben:** 

Den vollständigen Pfadnamen des Ordners eingeben,
der das zu öffnende Symbol enthält, oder des Ordners,
in dem das Symbol gespeichert werden soll.
* **Ordner** 

Zeigt eine Liste mit Ordnern an, die sich innerhalb des Ordners
befinden, der im Textfeld "Pfad oder Ordnername eingeben:" angezeigt
wird. Klickt der Benutzer doppelt auf einen Ordner in dieser Liste,
wechseln die Ordner- und Dateilisten und zeigen den Inhalt dieses
Ordners an. Der Benutzer kann aber auch einen Ordner in der Ordnerliste
auswählen und auf die Taste 'Aktualisieren' klicken.
* **Dateien** 

Zeigt eine Liste mit Dateien an, die im Ordner enthalten sind,
der im Textfeld "Pfad oder Ordnername eingeben:" angezeigt
wird. Ändert der Benutzer den Namen im Textfeld, muß er auf die
Taste 'Aktualisieren' klicken, um die neue, zugehörige Dateiliste
zu erhalten.
* **Dateinamen eingeben:** 

Zeigt den Namen des Symbols an, das geladen oder
gespeichert wird. Die einfachste Art, ein gewünschtes Symbol anzugeben, ist,
doppelt auf den Symbolnamen in der Dateiliste zu klicken. Der Benutzer kann
aber auch den Namen des gewünschten Symbols eingeben und danach auf die
Taste 'Öffnen' klicken.

Beachten,
daß das korrekte Format für Symbolnamen 'Name.Größe.Format' ist. Die Informationen
zu Größe und Format müssen im Symbolnamen enthalten sein, damit Symbole
korrekt ausgeführt werden können. Normalerweise füllt der Symboleditor die
Werte für die korrekte Größe und das korrekte Format automatisch aus. Als
Grundlage gelten die aus der Menüleiste des Editors ausgewählten Angaben.
* **Öffnen oder Speichern** 

Öffnet oder speichert die Datei und schließt das
Dialogfenster.
* **Aktualisieren** 

Ändert die Ordner- und Dateilisten und zeigt den Inhalt des
Ordners an, der im Textfeld "Pfad oder Ordnername eingeben:" angezeigt wird.
Der Benutzer kann einen Ordnernamen im Feld eingeben und danach auf die Taste
'Aktualisieren' klicken. Er kann aber auch, wenn der gewünschte Ordner in der
Ordnerliste angezeigt wird, doppelt auf den Ordnernamen klicken.
* **Abbrechen** 

Bricht das Öffnen oder Speichern ab und schließt das
Dialogfenster.
* **Hilfe** 

Zeigt den Hilfetext für dieses Dialogfenster an.

# Dialogfenster 'Speichern als' im Symboleditor




* **Pfad oder Ordnername eingeben:** 

Den vollständigen Pfadnamen des Ordners eingeben,
in dem das Symbol gespeichert werden soll.
* **Ordner** 

Zeigt eine Liste mit Ordnern an, die sich innerhalb des Ordners
befinden, der im Textfeld "Pfad oder Ordnername eingeben:" angezeigt
wird. Klickt der Benutzer doppelt auf einen Ordner in dieser Liste,
wechseln die Ordner- und Dateilisten und zeigen den Inhalt dieses
Ordners an. Der Benutzer kann aber auch einen Ordner in der Ordnerliste
auswählen und auf die Taste 'Aktualisieren' klicken.
* **Dateien** 

Zeigt eine Liste mit Dateien an, die im Ordner enthalten sind,
der im Textfeld "Pfad oder Ordnername eingeben:" angezeigt
wird. Ändert der Benutzer den Namen im Textfeld, muß er auf die
Taste 'Aktualisieren' klicken, um die neue, zugehörige Dateiliste
zu erhalten.
* **Dateinamen eingeben:** 

Den Namen des Symbols eingeben, das gesichert werden soll.
Beachten, daß das korrekte Format für Symbolnamen 'Name.Größe.Format' ist. Die
Informationen zu Größe und Format müssen im Symbolnamen enthalten sein, damit
Symbole korrekt ausgeführt werden können. Normalerweise füllt der Symboleditor
die Werte für die korrekte Größe und das korrekte Format automatisch aus. Als
Grundlage gelten die aus der Menüleiste des Editors ausgewählten Angaben.
* **Speichern** 

Speichert die Datei und schließt das Dialogfenster.
* **Aktualisieren** 

Ändert die Ordner- und Dateilisten und zeigt den Inhalt des
Ordners an, der im Textfeld "Pfad oder Ordnername eingeben:" angezeigt wird.
Der Benutzer kann einen Ordnernamen im Feld eingeben und danach auf die Taste
'Aktualisieren' klicken. Er kann aber auch, wenn der gewünschte Ordner in der
Ordnerliste angezeigt wird, doppelt auf den Ordnernamen klicken.
* **Abbrechen** 

Bricht das Speichern ab und schließt das Dialogfenster.
* **Hilfe** 

Zeigt den Hilfetext für dieses Dialogfenster an.

# Bestätigungsfenster des Symboleditors


Das Bestätigungsfenster schützt den Benutzer davor, unabsichtlich Daten zu
verlieren, indem er bestätigen muß, daß der Befehl tatsächlich ausgeführt
werden soll (z. B. den Symboleditor verlassen, ohne das Symbol zu sichern).

'OK' auswählen, um fortzufahren, oder 'Abbrechen', um den Befehl abzubrechen.