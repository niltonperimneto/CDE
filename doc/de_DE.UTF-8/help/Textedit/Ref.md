
# Texteditor - Referenz

# Texteditor - Editier- und Positioniertasten





# Texteditor - Menüs















# Texteditor - Fenster und Dialogfenster





















# Allgemeine Referenz







# Siehe auch



# Texteditor - Menükurzwahltasten
Menüs: Kurzwahltasten

* **Schließen** 

Alt+F4
* **Kopieren** 

Strg+C
* **Ausschneiden** 

Strg+X
* **Löschen** 

Löschtaste
* **Suchen/Ersetzen** 

Strg+F
* **Überschreiben** 

Einfügetaste
* **Einfügen** 

Strg+V
* **Drucken** 

Strg+P
* **Alles auswählen** 

Strg+/
* **Rückgängig** 

Strg+Z


Verfügt die Tastatur des Benutzers nicht über eine Taste Alt, den
Systemadministrator nach der entsprechenden Taste fragen.
# Editier- und Positioniertasten
EditiertastenSteuertasteTaste Alt
# Editiertasten


* **Taste** 

Aktion
* **Rückschritt** 

Löscht das Zeichen vor dem Cursor.
* **Löschen** 

Löscht das Zeichen, das auf den Einfügecursor folgt.
* **Strg+Löschen** 

Löscht alle Zeichen vom Cursor bis zum Ende der
aktuellen Zeile.
* **Strg+Rückschritt** 

Löscht das vorherige Wort.
* **Umschalt+Rückschritt** 

Löscht die Zeichen vom Cursor bis zum Anfang
der Zeile.

# Positioniertasten
Steuertaste, mit den Pfeiltasten verwendetPfeiltastenCursortastenTastaturpositionierungTasten: CursorbewegungTasten: Pfeil

* **Taste** 

Cursorbewegung
* **Aufwärtspfeil** 

Eine Zeile nach oben
* **Abwärtspfeil** 

Eine Zeile nach unten
* **Linkspfeil** 

Ein Zeichen nach links
* **Rechtspfeil** 

Ein Zeichen nach rechts
* **Strg+Rechtspfeil** 

Ein Wort nach rechts
* **Strg+Linkspfeil** 

Ein Wort nach links
* **Strg+Abwärtspfeil** 

Zum Anfang des nächsten Absatzes
* **Strg+Aufwärtspfeil** 

Zum Anfang des vorherigen Absatzes
* **Pos1** 

Zum Anfang der aktuellen Zeile
* **Ende** 

Zum Ende der aktuellen Zeile
* **Strg+Pos1** 

Zum Anfang des Dokuments
* **Strg+Ende** 

Zum Ende des Dokuments


Einige Tastenkombinationen weichen möglicherweise auf dem System
des Benutzers ab. Ist dies der Fall, den Systemadministrator nach
den entsprechenden Tasten fragen.

&sigspace;

&sigspace;

Um die Emacs-Tastenbelegungen im Texteditor zu verwenden, siehe.

&sigspace;
# Siehe auch











# Unix-Tastaturbelegungen
Unix-TastaturbelegungenTastaturbelegungen: UnixEmacs-Tastaturbelegungen

Die Unix-Tastaturbelegungen stellen einen Satz von erweiterten
Emacs-Tasten zur Verfügung, wie z. B.
Strg+F (Vorwärtszeichen) und Strg+N (nächste Zeile) im Texteditor.
Um die Unix-Tastaturbelegungen zu aktivieren (die standardmäßig
ausgeschaltet sind), folgende Schritte ausführen:

Die Datei.Xdefaultsim Standardverzeichnis
editieren, in dem folgende Zeile zur Datei hinzugefügt wird:#include "/usr/dt/app-defaults/`Sprache`/UNIXbindings"

Die Variable`Sprache`durch den Wert der entsprechenden
Sprachumgebungsvariablen ersetzen. Existiert die Datei.Xdefaultsnicht, muß sie der Benutzer in
seinem Standardverzeichnis erstellen.

Von der aktuellen Session abmelden.

Anmelden und den Texteditor erneut starten.

&sigspace;

Bei der Verwendung von Unix-Tastaturbelegungen stellt der
Texteditor alternative Menükurzwahltasten für folgende Befehle
zur Verfügung:

* **Befehl** 

Alternative Menükurzwahltaste
* **Rückgängig (Strg+Z)** 

Strg+_
* **Einfügen (Strg+V)** 

Umschalt+Einfügen
* **Suchen/Ersetzen (Strg+F)&sigspace;** 

Strg+S
* **Drucken (Strg+P)** 

keine Alternative


&sigspace;
Sollen diese Menükurzwahltasten geändert werden, den Inhalt der
Datei/usr/dt/app-defaults/`Sprache`/UNIXbindingsin die eigene Datei.Xdefaultskopieren und danach die
Änderungen vornehmen.

Sind die Unix-Tastaturbelegungen aktiviert, löscht die
Löschtaste das vorherige Zeichen und nicht das Zeichen, das
auf den Cursor folgt.
# Siehe auch







# Texteditor - Menü 'Datei'


Menüs:Datei

* **Erstellen** 

Löscht den Inhalt des Texteditor-Fensters. Sind im Dokument
Änderungen vorhanden, die noch nicht gespeichert sind, wird ein Dialogfenster
angezeigt, mit dem die Änderungen gespeichert werden können, bevor der
Inhalt des Fensters gelöscht wird.
* **Öffnen** 

Zeigt ein Dialogfenster an, mit dem eine bestehende Datei
geöffnet werden kann.
* **Einfügen** 

Zeigt ein Dialogfenster an, mit dem ein Datei angegebenen
werden kann, die in das aktuelle Dokument eingefügt werden
soll.
* **Speichern** 

Speichert das Dokument in die aktuelle Datei. Ist die Option
'Zeilenumbruch an Fenster anpassen' aktiviert, wird das Dialogfenster
'Speichern' angezeigt. Wurde das aktuelle Dokument noch nicht gespeichert, wird
das Dialogfenster 'Speichern als' angezeigt.
* **Speichern als** 

Zeigt ein Dialogfenster zum Speichern eines Dokuments in
einer neuen Datei.
* **In Datei kopieren** 

Kopiert die angezeigten oder editierten Informationen
in einer separaten Datei, ohne die Editier-Session zu wechseln. In einigen
Fällen wird der Befehl 'Speichern als' durch den Befehl 'In Datei kopieren'
ersetzt.
* **Drucken** 

Zeigt ein Dialogfenster an, in dem Druckoptionen ausgewählt
werden können und ein Dokument gedruckt werden kann.
* **Schließen** 

Schließt das Texteditor-Fenster und verläßt den Texteditor.

# Siehe auch







# Texteditor - Menü 'Bearbeiten'


Menüs:Bearbeiten

* **Rückgängig** 

Kehrt die letzte Operation zum Ausschneiden, Einfügen, Löschen
des Inhalts, Löschen, Ersetzen, Einschließen oder Formatieren um.
* **Ausschneiden** 

Löscht den ausgewählten Text und speichert ihn in der
Zwischenablage.
* **Kopieren** 

Kopiert den ausgewählten Text und speichert ihn in der
Zwischenablage.
* **Einfügen** 

Kopiert den Text der letzten Ausschneide- oder Kopieroperation
an die aktuelle Cursorposition.
* **Leeren** 

Ersetzt den ausgewählten Text durch Leerzeichen.
* **Löschen** 

Löscht den ausgewählten Text.
* **Alles auswählen** 

Wählt den gesamten Text im Dokument aus.
* **Suchen/Ersetzen** 

Öffnet ein Dialogfenster, mit dem der Benutzer nach
Wörtern oder Ausdrücken im Dokument suchen und gegebenenfalls
Änderungen an den gefundenen Stellen vornehmen kann.
* **Rechtschreibprüfung** 

Führt das Rechtschreibprüfprogramm für dieses
Dokument aus.

# Siehe auch







# Texteditor - Menü 'Format'


Menüs:Format

* **Einstellungen** 

Zeigt ein Dialogfenster an, in dem die Ränder und
Absatzausrichtungen eingestellt und Formateinstellungen für das Dokument
angewendet werden können.
* **Absatz** 

Wendet die Einstellungen für den Absatz an, der den Cursor
enthält.
* **Alle** 

Wendet die Einstellungen für das gesamte Dokument an.

# Siehe auch







# Texteditor - Menü 'Optionen'


Menüs: Optionen

* **Überschreibmodus** 

Schaltet den Texteingabemodus so um, daß der
Benutzer bestehende Zeichen überschreiben kann.
* **Zeilenumbruch an Fenster anpassen** 

Schaltet den Texteingabemodus so
um, daß Zeilen am Rand des Fensters automatisch umgebrochen werden. Ist
'Zeilenumbruch an Fenster anpassen' nicht eingeschaltet, muß der Benutzer
am Ende jeder Zeile die Eingabetaste drücken.
* **Statuszeile** 

Schaltet die Anzeige einer Statuszeile am unteren Rand
des Fensters ein. Die Statuszeile zeigt die Nummer der Zeile, in der
sich der Cursor befindet, die Gesamtanzahl an Zeilen im Dokument sowie
Anwendungsmeldungen an und gibt an, wenn der Überschreibungsmodus
eingeschaltet ist. Sie stellt außerdem die Möglichkeit zur Verfügung, den
Cursor zu einer bestimmten Zeilennummer zu bewegen.

# Siehe auch













# Texteditor - Menü 'Hilfe'


Menüs:Hilfe

* **Überblick** 

Erläutert, wie der Texteditor gestartet wird.
* **Inhaltsverzeichnis** 

Stellt einen Umriß der Hilfethemen zum
Texteditor zur Verfügung.
* **Aufgaben** 

Stellt die Aufgabenanweisungen für die meisten Operationen des
Texteditors zur Verfügung.
* **Referenzinformationen** 

Faßt die Funktionen des Texteditors, wie z. B. Menüs, Dialogfenster,
Ressourcen und Befehlszeilenoptionen, zusammen.
* **Kontexthilfe** 

Ändert den Cursor in ein Fragezeichen, mit dem auf ein
Element in einem Texteditorfenster oder -dialogfenster geklickt
werden kann. Eine Beschreibung zu diesem Element wird angezeigt.
* **Hilfe für Hilfe** 

Zeigt Informationen zur Verwendung der Hilfefenster an.
* **Informationen zu Texteditor** 

Zeigt die Version und den Copyrightvermerk des
Texteditors an.

# Siehe auch





# Texteditorfenster


Fenster, Texteditor

&sigspace;

* **Menüleiste** 

Der Texteditor stellt fünf Menüs zur Verfügung: Datei,
Bearbeiten, Format, Optionen und Hilfe.
* **Dokumentenfenster** 

Der Bereich, in dem der Benutzer den Inhalt seines
Dokuments eingibt.
* **Statuszeile** 

Zeigt die Nummer der Zeile, in der
sich der Cursor befindet, die Gesamtanzahl an Zeilen im Dokument sowie
Anwendungsmeldungen an und gibt an, wenn der Überschreibungsmodus
eingeschaltet ist. Sie stellt außerdem die Möglichkeit zur Verfügung, den
Cursor zu einer bestimmten Zeilennummer zu bewegen

# Siehe auch







# Dialogfenster 'Texteditor - Datei öffnen'


* **Pfad- oder Ordnernamen eingeben** 

Gibt den Pfadnamen des aktuellen
Ordners an.
* **Filter** 

Ein Stern (*) zeigt alle Dateien an. Der Benutzer kann Platzhalterzeichen
angeben, um nur die Dateien anzuzeigen, die mit einer bestimmten Erweiterung
übereinstimmen. Beispielsweise werden bei der Angabe '*.doc' nur die Dateien
mit der Erweiterung .doc aufgelistet.
* **Dateien** 

Listet die Dateien im aktuellen Ordner auf.
* **Ordner** 

Listet die Ordner im aktuellen Ordner auf.
Der Benutzer kann eine Datei im aktuellen Ordner, einen Unterordner oder
einen anderen Ordner öffnen.
* **Dateinamen eingeben** 

Zeigt den Dateinamen an, der in der Liste 'Dateien'
ausgewählt wurde. Die Eingabetaste drücken oder auf 'OK' klicken, um die
Datei zu öffnen. Der Benutzer kann den Namen der zu öffnenden Datei auch
eingeben.
* **OK** 

Öffnet die Datei, die im Feld 'Dateinamen eingeben' angegeben ist.
* **Aktualisieren** 

Zeigt eine neue Dateiliste an, nachdem der Filterschlüssel
geändert wurde oder zu einem neuen Ordner gewechselt wurde.
* **Abbrechen** 

Bricht das Öffnen ab.
* **Hilfe** 

Beschreibt die Texteingabefelder, die Auswahllisten und die
Tasten im Dialogfenster.

# Siehe auch







# Dialogfenster 'Texteditor - Speichern als'


* **Pfad- oder Ordnernamen eingeben** 

Gibt den Pfadnamen des aktuellen
Ordners an.
* **Filter** 

Ein Stern (*) zeigt alle Dateien an. Der Benutzer kann Platzhalterzeichen
angeben, um nur die Dateien anzuzeigen, die mit einer bestimmten Erweiterung
übereinstimmen. Beispielsweise werden bei der Angabe '*.doc' nur die Dateien
mit der Erweiterung .doc aufgelistet.
* **Dateien** 

Listet die Dateien im aktuellen Ordner auf.
* **Ordner** 

Listet die Ordner im aktuellen Ordner auf.
* **Dateinamen eingeben** 

Feld, in dem der neue Dateiname für das
Dokument eingegeben wird.
* **OK** 

Speichert die Datei unter dem angegebenen Namen.
* **Aktualisieren** 

Zeigt eine neue Dateiliste an, nachdem der Filterschlüssel
geändert wurde oder zu einem neuen Ordner gewechselt wurde.
* **Abbrechen** 

Bricht die Operation 'Speichern als' ab.


Wird die Option 'Zeilenumbruch an Fenster anpassen' verwendet, werden
folgende zusätzliche Auswahlmöglichkeiten im Dialogfenster angezeigt:

Zeilenumbruchzeichen an das Ende von Zeilen mit Worttrennung stellen.

Dies ist die Standardauswahl. Dadurch werden Zeilenumbruchzeichen am
Ende jeder umgebrochenen Zeile hinzugefügt und somit genau die
Zeilenumbrüche übernommen, wie sie im Dokument erscheinen.

Keine Zeilenumbrüche einfügen. Es werden nur die mit der Eingabetaste
erzeugten Zeilenumbrüche übernommen.

Bei dieser Option bleiben die bedingten ("soft") Zeilenumbrüche, die
mit der Option 'Zeilenumbruch an Fenster anpassen' eingefügt wurden,
bestehen. Wird das Dokument erneut geöffnet, paßt sich der Text der
Breite des neuen Fensters an.
# Siehe auch







# Dialogfenster 'Texteditor - Speichern'


&sigspace;
Wird die Option 'Zeilenumbruch an Fenster anpassen' verwendet, wird das
Dialogfenster 'Speichern' angezeigt, wenn der Benutzer Änderungen in seinem
Dokument speichert. Die Zeilenumbrüche, die von der Option
'Zeilenumbruch an Fenster anpassen' eingefügt wurden, können auf zwei
Arten bearbeitet werden:

Zeilenumbruchzeichen an das Ende von Zeilen mit Worttrennung stellen.

Dies ist die Standardauswahl. Dadurch werden Zeilenumbruchzeichen am
Ende jeder umgebrochenen Zeile hinzugefügt und somit genau die
Zeilenumbrüche übernommen, wie sie im Dokument erscheinen.

Keine Zeilenumbrüche einfügen. Es werden nur die mit der Eingabetaste
erzeugten Zeilenumbrüche übernommen.

Bei dieser Option bleiben die bedingten ("soft") Zeilenumbrüche, die
mit der Option 'Zeilenumbruch an Fenster anpassen' eingefügt wurden,
bestehen. Wird das Dokument erneut geöffnet, paßt sich der Text der
Breite des neuen Fensters an.

* **Ja** 

Speichert die aktuelle Datei oder zeigt das Dialogfenster
'Speichern als' zum Speichern des Dokuments an.
* **Nein** 

Setzt die Operation fort, ohne die Datei zu speichern.
* **Abbrechen** 

Bricht die Operation ab.
* **Hilfe** 

Zeigt die Beschreibung zum Dialogfenster 'Speichern' an.


Das Dialogfenster 'Speichern' wird auch angezeigt, wenn der Benutzer
einen Menübefehl auswählt, der dazu führt, daß die aktuellen
Änderungen verlorengehen.
# Siehe auch







# Dialogfenster 'Texteditor - Datei einfügen'


* **Pfad- oder Ordnernamen eingeben** 

Gibt den Pfadnamen des aktuellen
Ordners an.
* **Filter** 

Ein Stern (*) zeigt alle Dateien an. Der Benutzer kann Platzhalterzeichen
angeben, um nur die Dateien anzuzeigen, die mit einer bestimmten Erweiterung
übereinstimmen. Beispielsweise werden bei der Angabe '*.doc' nur die Dateien
mit der Erweiterung .doc aufgelistet.
* **Dateien** 

Listet die Dateien im aktuellen Ordner auf.
* **Ordner** 

Listet die Ordner im aktuellen Ordner auf.
Der Benutzer kann eine Datei im aktuellen Ordner, einen Unterordner oder
einen anderen Ordner öffnen.
* **Dateinamen eingeben** 

Zeigt den Dateinamen an, der in der Liste 'Dateien'
ausgewählt wurde. Die Eingabetaste drücken oder auf 'OK' klicken, um die
Datei zu öffnen. Der Benutzer kann den Namen der zu öffnenden Datei auch
eingeben.
* **OK** 

Fügt die Datei, die im Feld 'Dateinamen eingeben' angegeben wurde,
an der Position des Cursors ein.
* **Aktualisieren** 

Zeigt eine neue Dateiliste an, nachdem der Filterschlüssel
geändert wurde oder zu einem neuen Ordner gewechselt wurde.
* **Abbrechen** 

Bricht das Einfügen ab.
* **Hilfe** 

Beschreibt die Texteingabefelder, die Auswahllisten und die
Tasten im Dialogfenster.

# Siehe auch







# Dialogfenster 'Texteditor - Rechtschreibprüfung'




* **Rechtschreibfehler** 

Listet die Rechtschreibfehler auf, die im
Dokument gefunden wurden.
* **Ersetzen durch** 

Feld, in dem der Benutzer das korrekt geschriebene
Wort eingeben kann.
* **Suchen** 

Sucht das erste Auftreten des Rechtschreibfehlers. Dabei
wird an der Position des Einfügecursors mit der Suche begonnen.
* **Ersetzen** 

Ersetzt das hervorgehobene Wort durch die korrekte Schreibweise.
* **Alle ersetzen** 

Ersetzt alle Vorkommen des Rechtschreibfehlers.
* **Schließen** 

Schließt das Dialogfenster.


&sigspace;

Der Befehl für die Rechtschreibprüfung kann nur die
Rechtschreibung englischer Texte prüfen.
# Siehe auch







# Dialogfenster 'Texteditor - Suchen/Ersetzen'




* **Suchen** 

Feld, in dem der Benutzer ein Wort oder einen Ausdruck eingibt,
nach dem er suchen will. Dabei ist die Groß-/Kleinschreibung zu beachten.
* **Ersetzen durch** 

Feld, in dem der Benutzer den Ersetzungstext eingibt.
* **Suchen** 

Sucht das erste Vorkommen der Suchzeichenfolge.
* **Ersetzen** 

Ersetzt das hervorgehobene Wort durch den Ersetzungstext.
* **Alle ersetzen** 

Ersetzt alle Vorkommen der Suchzeichenfolge.
* **Schließen** 

Schließt das Dialogfenster.

# Siehe auch







# Dialogfenster 'Texteditor - Formateinstellungen'




&sigspace;

* **Linker Rand** 

Die Anzahl an Zeichen, die der gedruckte Text vom linken
Papierrand eingerückt werden soll.
* **Rechter Rand** 

Die Anzahl an Spalten für den Text.
* **Ausrichtung links** 

Richtet die Textzeilen am linken Rand aus.
* **Ausrichtung rechts** 

Richtet die Textzeilen am rechten Rand aus.
* **Blocksatz** 

Richtet den Text als Block mit gleichen linken und
rechten Rändern aus.
* **Zentriert** 

Zentriert den Text zwischen den Rändern.
* **Absatz** 

Wendet die Einstellungen auf den Absatz an, der
den Cursor enthält.
* **Alles** 

Wendet die Einstellungen auf das gesamte Dokument an.
* **Schließen** 

Schließt das Dialogfenster.

# Siehe auch







# Dialogfenster 'Texteditor - In Datei kopieren'


Andere Anwendungen können den Texteditor als Hilfsprogramm zum
Editieren von Informationen verwenden und dabei möglicherweise
Einschränkungen in bezug auf das Speichern von Änderungen
enthalten. Beispielsweise kann in einigen Fällen der Befehl
'Speichern als' durch den Befehl 'In Datei kopieren' ersetzt werden,
wodurch der Benutzer eine Kopie der Informationen, die er momentan
anzeigt oder editiert, in eine neue Datei kopieren, ohne die Editier-Session
zu wechseln.

* **Pfad- oder Ordnernamen eingeben** 

Gibt den Pfadnamen des aktuellen
Ordners an.
* **Filter** 

Ein Stern (*) zeigt alle Dateien an. Der Benutzer kann Platzhalterzeichen
angeben, um nur die Dateien anzuzeigen, die mit einer bestimmten Erweiterung
übereinstimmen. Beispielsweise werden bei der Angabe '*.doc' nur die Dateien
mit der Erweiterung .doc aufgelistet.
* **Dateien** 

Listet die Dateien im aktuellen Ordner auf.
* **Ordner** 

Listet die Ordner im aktuellen Ordner auf.
* **Dateinamen eingeben** 

Feld, in dem der Benutzer den neuen Namen für sein
Dokument eingibt.
* **OK** 

Kopiert die Informationen in die angegebene Datei.
* **Aktualisieren** 

Zeigt eine neue Dateiliste an, nachdem der Filterschlüssel
geändert wurde oder zu einem neuen Ordner gewechselt wurde.
* **Abbrechen** 

Bricht die Operation 'In Datei kopieren' ab.


Wird die Option 'Zeilenumbruch an Fenster anpassen' verwendet, werden
folgende zusätzliche Auswahlmöglichkeiten im Dialogfenster angezeigt:

Zeilenumbruchzeichen an das Ende von Zeilen mit Worttrennung stellen.

Dies ist die Standardauswahl. Dadurch werden Zeilenumbruchzeichen am
Ende jeder umgebrochenen Zeile hinzugefügt und somit genau die
Zeilenumbrüche übernommen, wie sie im Dokument erscheinen.

Keine Zeilenumbrüche einfügen. Es werden nur die mit der Eingabetaste
erzeugten Zeilenumbrüche übernommen.

Bei dieser Option bleiben die bedingten ("soft") Zeilenumbrüche, die
mit der Option 'Zeilenumbruch an Fenster anpassen' eingefügt wurden,
bestehen. Wird das Dokument erneut geöffnet, paßt sich der Text der
Breite des neuen Fensters an.
# Siehe auch





# Dialogfenster im Texteditor bei vorhandener Datei


Wird beim Speichern eines Dokuments der Name einer bereits vorhandenen
Datei eingegeben, wird dieses Dialogfenster angezeigt

Um die ursprüngliche Datei zu überschreiben, auf 'OK' klicken.

Um einen anderen Dateinamen einzugeben, auf 'Abbrechen' klicken und
'Speichern als' aus dem Menü 'Datei' auswählen.
# Siehe auch





# Texteditor - Befehlszeilensyntax und Optionen
Texteditor: im Terminal-Fenster starten

Die Befehlszeilensyntax zum Starten des Texteditors lautet:dtpad [`Optionen`]

Dabei sind folgende`Optionen`möglich:

* **-server** 

Gibt an, daß der Texteditor im Server-Modus
gestartet werden soll, ohne daß ein Startfenster angezeigt wird. Nachfolgende
Aufrufe des Texteditors, die im Standard-Requester-Modus ausgeführt werden,
führen dazu, daß der Server ein separates Editierfenster für jeden Aufruf
erstellt.
* **-standAlone** 

Mit dieser Option wird der Texteditor
im eigenständigen Modus ausgeführt, in dem er die Editierung unabhängig
vom Texteditor-Server bearbeitet. Diese Option kann hilfreich sein,
wenn der Benutzer ein Exemplar des Texteditors mit einer anderen Umgebung
ausführen will als in anderen Texteditor-Fenstern. Beispiel: Ein
Exemplar wird mit anderen länderspezifischen Angaben oder mit anderen
Farbressourcen ausgeführt. Diese Option entspricht der Einstellung 'True'
für die RessourcestandAlone.
* **-exitOnLastClose** 

Gibt an, daß der Texteditor-Server-Prozeß
beendet werden soll, wenn das letzte Editierfenster für die Anzeige geschlossen
wird. Diese Option ist nur zusammen mit der Option-servergültig. Ist diese Option nicht angegeben, bleibt der Texteditor-Server-Prozeß
auf unbestimmte Zeit aktiv, auch wenn alle Editierfenster geschlossen wurden.
* **-noBlocking** 

Gibt an, daß der Texteditor-Requester
beendet wird, sobald der Texteditor-Server angibt, daß er auf eine
Datei im angegebenen Ordner zugreifen kann. Wird diese Option nicht angegeben,
blockiert der Texteditor-Requester, und wird erst verlassen, wenn
er einen Hinweis vom Texteditor-Server empfängt, daß das Fenster
geschlossen wurde.
* **-statusLine** 

Diese Option führt dazu, daß der
Texteditor eine Statuszeile am unteren Rand des Editierfensters anzeigt.
Die Statuszeile zeigt die Nummer der Zeile, in der
sich der Cursor befindet, die Gesamtanzahl an Zeilen im Dokument sowie
Anwendungsmeldungen an und gibt an, wenn der Überschreibungsmodus
eingeschaltet ist. Sie stellt außerdem die Möglichkeit zur Verfügung, den
Cursor zu einer bestimmten Zeilennummer zu bewegen.
* **-wrapToFit** 

Gibt an, daß beim Start die Option
'Zeilenumbruch an Fenster anpassen' eingeschaltet werden soll.

# Siehe auch






Eine vollständige Liste und eine Beschreibung der
Befehlszeilenoptionen des Texteditors befinden sich auf den
elektronischen Handbuchseitendtpad(1).
# Texteditor - Ressourcen
AnwendungsressourcenRessourcen

Mit den folgenden Ressourcen wird das Aussehen und Verhalten des
Texteditors gesteuert.

Dtpad*server: [ true | false ]

Gibt an, daß der Texteditor im Server-Modus gestartet werden soll,
um alle nachfolgenden Editieraufrufe für die Anzeige zu verarbeiten.
Die Einstellung 'True' für diese Ressource entspricht der Befehlszeilenoption-server.

Dtpad*standAlone: [ true | false ]

Gibt an, ob der aktuelle Texteditor-Prozeß im eigenständigen Modus
ausgeführt werden soll, in dem er seine eigene Editierung bearbeitet, oder
im Requester-Modus, in dem die eigentliche Editierung von einem separaten
Texteditor-Server-Prozeß bearbeitet wird.
Die Einstellung 'True' für diese Ressource entspricht der Befehlszeilenoption-standAlone.

Dtpad*blocking: [ true | false ]

Gibt an, daß, wenn der Texteditor im Standard-Requester-Modus
ausgeführt wird, in dem die eigentliche Editierung von einem separaten
Texteditor-Server-Prozeß bearbeitet wird, der Requester-Prozeß nicht
beendet werden soll, bevor das Fenster, das dem Editieraufruf zugeordnet
ist, geschlossen ist.
Die Einstellung 'False' für diese Ressource entspricht der Befehlszeilenoption-noBlocking.

Dtpad*exitOnLastClose: [ true | false ]

Gibt an, ob der Texteditor-Server-Prozeß beendet werden soll, wenn
das letzte aktive Editierfenster geschlossen wird. Ist die Ressource auf
'False' gesetzt, wird der Texteditor-Server weiter ausgeführt, bereit zum
Empfang einer Meldung, daß eine Datei editiert werden soll. Ist diese
Ressource auf 'True' gesetzt, wird der Texteditor-Server beendet, wenn das
letzte aktive Editierfenster geschlossen wird.

Dtpad*statusLine: [ true | false ]

Gibt an, ob der Texteditor die Statuszeile am unteren Rand des
Editierfensters anzeigen soll.
Die Einstellung 'True' für diese Ressource entspricht der Befehlszeilenoption-statusLine.

Dtpad*wrapToFit: [ true | false ]

Gibt an, ob die Option 'Zeilenumbruch an Fenster anpassen' beim
Starten des Texteditors aktiviert (True) oder inaktiviert (False)
wird. Die Einstellung 'True' für diese Ressource entspricht der
Befehlszeilenoption-wrapToFit.
# Siehe auch






Eine vollständige Liste und eine Beschreibung der
Ressourcen des Texteditors befinden sich auf den
elektronischen Handbuchseitendtpad(1).
# Texteditor - Dateigruppe
Texteditor, ausführbare DateiAnwendungsstandarddateienRessourcen

Die ausführbare Datei des Texteditors und die Datei mit den
Anwendungsstandardwerten befinden sich in folgenden Verzeichnissen:

* **Ausführbare Datei** 

/usr/dt/bin/dtpad
* **Datei mit Anwendungsstandardwerten** 

/usr/dt/app-defaults/`Sprache`/Dtpad

# Siehe auch




