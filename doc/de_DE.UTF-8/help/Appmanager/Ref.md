
# Anwendungsmanager - Referenzinformationen
Referenzinformationen

Die folgende Liste enthält Referenzthemen für den Anwendungsmanager, die in
verschiedene Kategorien unterteilt sind.
# Allgemeine Referenzinformationen







# Anwendungsmanager - Menüs











# Anwendungsmanagerfenster und Dialogfenster





















# Anwendungsmanager - Maus
MaustastenObjekte auswählen
# Maustaste 1&emdash;Auswählen und Ziehen


Die Maustaste 1 (standardmäßig die linke Maustaste) wird verwendet, um
Objekte auszuwählen und zu ziehen.

Um ein Objekt auszuwählen, auf das entsprechende Symbol klicken.

Um die Auswahl zu erweitern, die Steuertaste gedrückt halten und auf
ein weiteres Symbol klicken.
# Maustaste 2&emdash;Ziehen


Ziehen von Objekten:

Verschieben von Objekten: Die Maustaste 1 gedrückt halten und das Symbol ziehen.

Kopieren von Objekten: Die Steuertaste drücken und das Objekt ziehen.

Symbolische Verbindung: Die Umschalttaste drücken und das Symbol ziehen.

Bei einer Maus mit zwei Tasten beide Tasten gleichzeitig drücken.
# Maustaste 3&emdash;Menüs
Kontextmenüs

Die Maustaste 3 (standardmäßig die rechte Maustaste) wird verwendet, um
temporär eingeblendete Menüs aufzurufen.

Den Mauszeiger auf das Objekt oder den Bereich stellen, für den das Menü
zur Verfügung steht, und anschließend die Maustaste 3 drücken undgedrückt halten.
# Anwendungsmanager - Dateinamen
Anwendungsmanager:DateinamenDateinamen:im AnwendungsmanagerAktionsbezeichnungenBezeichnungen für Aktionssymbole

Im Dateimanager und Anwendungsmanager werden Dateien und Ordner als
Symbole dargestellt. Die Symbolbezeichnung stimmt in der Regel mit
dem entsprechenden Dateinamen überein.

Aktionssymbole stellen manchmal eine Ausnahme zu dieser Regel dar. Ein
Beispiel: Wird das temporär eingeblendete Menü für das Aktionssymbol mit der
Bezeichnung 'Digitaluhr' in der Anwendungsgruppe 'Desktop-Werkzeuge'
angezeigt, stimmt der Dateiname, der am oberen Rand des Menüs
angezeigt wird, nicht mit der Bezeichnung überein.

Der tatsächliche Dateiname wird in den Dialogfenstern 'Datei kopieren' und
'Verweis auf Datei erstellen' angezeigt.
# Ordnerposition des Anwendungsmanagers
Anwendungsmanager:Ordnerposition

Das Verhalten des Anwendungsmanagers ähnelt dem des Dateimanagers. Der
Grund dafür ist, daß es sich beim Anwendungsmanager um ein
Dateimanagerfenster eines bestimmten Ordners auf dem System handelt, in dem
registrierte Anwendungen zusammengefaßt werden. Der Anmeldemanager
erstellt den Ordner für den Anwendungsmanager bei jedem Anmeldevorgang.

Normalerweise muß die Position des Ordners für den
Anwendungsmanager dem Benutzer nicht bekannt sein. Bei der Fehlerbehebung
kann es jedoch von Vorteil sein, die Position des Ordners zu kennen.

Die Position des Ordners lautet:/var/dt/appconfig/appmanager/`spezieller_Ordnername`

Dabei istspezieller_Ordnernameein vom System zugeordneter Name, der
für das jeweilige System und den jeweiligen Anmeldenamen eindeutig ist.

Der Ordnerspezieller_Ordnernamedarfunter keinen Umständenvom Benutzer direkt geändert werden.dtappgatherAnwendungen erneut laden, Aktion

Nachdem der Anmeldemanager den Ordner erstellt hat, führt er das
Desktop-Programmdtappgatheraus, mit dem alle Anwendungsgruppen
zusammengefaßt werden. Während einer Session kann das Programmdtappgathererneut ausgeführt werden, indem doppelt auf
'Anwendungen erneut laden' in der Anwendungsgruppe 'Desktop-Werkzeuge'
geklickt wird.
# Anwendungsmanager - Menüleiste


Die Menüleiste des Anwendungsmanagers enthält folgende Menüpunkte:








# Anwendungsmanager - Menü 'Datei'


Anwendungsmanager:Menü 'Datei'

* **Ordner erstellen** 

Fordert den Benutzer zur Eingabe eines Ordnernamens für
die Erstellung eines neuen Ordners auf.
* **Datei erstellen** 

Fordert den Benutzer zur Eingabe eines Dateinamens für
die Erstellung einer neuen Datei auf.
* **Nächsthöheres Verzeichnis** 

Wechselt in der Ordnerhierarchie in die nächsthöhere Ebene.
* **Gehe zu...** 

Ruft das Dialogfenster 'Gehe zu' auf, in dem der Benutzer
einen neuen Ordnernamen eingeben oder einen Ordner in der
Liste mit zuvor aufgerufenen Ordnern auswählen kann.
* **Suchen...** 

Ruft das Dialogfenster 'Suchen' auf, in dem nach Dateien
und Ordnern anhand von Mustern für Dateinamen oder
anhand des Dateiinhalts gesucht werden kann.
* **Schließen** 

Schließt das aktuelle Anwendungsmanagerfenster.

# Anwendungsmanager - Menü 'Ausgewählt'


Anwendungsmanager:Menü 'Ausgewählt'

Eine Reihe der Anwendungsgruppen kann nur vom Systemadministrator
geändert werden. Aus diesem Grund stehen möglicherweise nicht alle dieser
Menüpunkte zur Verfügung.

* **Verschieben nach...** 

Fordert den Benutzer zur Eingabe des vollständigen neuen
Pfadnamens für die zu verschiebende Datei auf.
* **Kopieren...** 

Fordert den Benutzer zur Eingabe eines neuen Dateinamens
für die Erstellung einer Kopie der ausgewählten Datei auf.
Dieser Befehl ist nur verfügbar, wenn genau eine Datei
ausgewählt ist.
* **Als Verweis kopieren** 

Fordert den Benutzer zur Eingabe des vollständigen neuen
Pfadnamens für den Verweis auf, der für das ausgewählte
Objekt erstellt wird.
* **In Arbeitsbereich stellen** 

Stellt das ausgewählte Objekt in die rechte Ecke des
Hintergrundmusters in den Arbeitsbereich.
* **In Papierkorb** 

Stellt die ausgewählten Objekte in den Papierkorb.
* **Umbenennen...** 

Öffnet das Feld zum Editieren des Names für das ausgewählte
Objekt.
* **Zugriffsrechte ändern...** 

Zeigt das Dialogfenster 'Zugriffsrechte' für das
ausgewählte Objekt an. Dieses Dialogfenster enthält die
Zugriffsberechtigungen für das Objekt.
* **Alles auswählen** 

Wählt alle Objekte im aktuellen Anwendungsmanagerfenster aus.
* **Alles abwählen** 

Nimmt die Auswahl für alle Objekte im aktuellen
Anwendungsmanagerfenster zurück.
* **Aktionen** 

Sind dem ausgewählten Objekt Aktionen zugeordnet, werden
diese im unteren Bereich des Menüs hinzugefügt. Dabei handelt
es sich um dieselben Aktionen, die auch im temporär eingeblendeten
Menü des Objekts angezeigt werden.

# Anwendungsmanager - Menü 'Ansicht'


Anwendungsmanager:Menü 'Ansicht'Anzeigen von Vorgaben im Anwendungsmanager

* **Neue Ansicht öffnen** 

Öffnet ein neues Anwendungsmanagerfenster des aktuellen
Ordners.
* **Optionen für Ansicht festlegen** 

Zeigt das Dialogfenster 'Optionen für Ansicht festlegen' an,
in dem die Darstellung und das Verhalten des aktuellen
Anwendungsmanagerfensters geändert werden kann.
* **Als Standardoptionen speichern** 

Sichert die aktuellen Optionen, Fenstergröße und Filterliste
als Standardwerte.
* **Verdeckte Objekte anzeigen** 

Blendet verdeckte Dateien ein/aus. Durch Auswahl von
'Filteroptionen festlegen' können die zu verdeckenden
Daten angegeben werden.
* **Filteroptionen festlegen** 

Zeigt das Dialogfenster 'Filteroptionen festlegen' an, in dem
angegeben werden kann, welche Dateien verdeckt werden sollen
(anhand von Datentyp oder Dateiname).
* **Aktualisieren** 

Aktualisiert den Inhalt des momentan verwendeten Ordners
und zeigt ihn mit den vorgenommenen Änderungen erneut an.

Der Aktualisierungsvorgang bezieht sich nicht auf Anwendungen,
die seit der letzten Anmeldung hinzugefügt wurden. Um den
Inhalt des Anwendungsmanagers zu aktualisieren, doppelt auf
'Anwendungen erneut laden' in der Anwendungsgruppe
'Desktop-Werkzeuge' klicken.

# Anwendungsmanager - Menü 'Hilfe'


Anwendungsmanager:Menü 'Hilfe'

* **Überblick** 

Zeigt allgemeine Informationen zur Einführung in den
Anwendungsmanager an.
* **Aufgaben** 

Zeigt spezielle Anweisungen zur Ausführung von Aufgaben
bei der Verwendung des Anwendungsmanagers an.
* **Referenzinformationen** 

Zeigt Informationen zu den Fenstern, Menüs und Dialogfenstern
im Anwendungsmanager an.
* **Kontexthilfe** 

Ändert den Cursorzeiger in ein Fragezeichen zum Aufrufen
von Kontexthilfe. Um Hilfe für ein bestimmtes Element
im Anwendungsmanagerfenster zu erhalten, doppelt auf
dieses Element klicken.
* **Hilfe für Hilfe** 

Stellt Hilfe für die Verwendung der Hilfefenster zur Verfügung.
* **Informationen zu Anwendungsmanager** 

Zeigt Informationen zur Version des Anwendungsmanagers
sowie den entsprechenden Copyrightvermerk an.

# Anwendungsmanager - temporär eingeblendetes Menü für Objekte


Anwendungsmanager:temporär eingeblendete MenüsTemporär angezeigte Menüs:AnwendungsmanagerFür die meisten Objekte im Anwendungsmanager stehen spezielle temporär
eingeblendete Menüs zur Verfügung.

Im oberen Teil der einzelnen Menüs sind zwei Standardbefehle
verfügbar: 'In Arbeitsbereich stellen' und 'In Papierkorb'.

Die Aktionen für den Datentyp des jeweiligen Objekts befinden sich im
unteren Teil des Menüs. Diese stimmen mit den Aktionen
überein, die im Menü 'Aktionen' angezeigt werden, wenn das Symbol des
entsprechenden Objekts ausgewählt wird.

* **In Arbeitsbereich stellen** 

Stellt das Objekt auf das Desktop des aktuellen Arbeitsbereichs.
Die Position des Objekts wird durch die RessourceobjectPlacementfestgelegt; standardmäßig ist dies die
obere rechte Ecke des Bildschirms.
* **In Papierkorb** 

Stellt das Objekt in den Papierkorb.
* **Hilfe** 

Zeig den Hilfetext für temporär eingeblendete Menüs an.
* **Aktionen** 

Sind dem Objekt Aktionen zugeordnet, werden sie im unteren
Bereich des Menüs hinzugefügt.

# Anwendungsmanagerfenster


Anwendungsmanager:HauptfensterAuf der obersten Ebene des Anwendungsmanagers stehen Symbole zur Verfügung,
die Anwendungsgruppen darstellen.

Die einzelnen Anwendungsgruppen sind Ordner, die ein oder
mehrereAnwendungssymboleenthalten (Anwendungssymbole werden auch
alsAktionssymbolebezeichnet). Eine Anwendungsgruppe kann außerdem
andere Arten von Anwendungsdateien enthalten, wie beispielsweise
Informationsdateien (README) oder Schablonen.
# Dialogfenster 'Anwendungsmanager - Datei kopieren'


Anwendungsmanager:Kopieren von ObjektenKopieren von Objekten

* **Ausgewähltes Objekt** 

Zeigt das zu kopierende Objekt an.
* **Zielordner** 

Den vollständigen neuen Pfadnamen des Zielordners angeben.
* **Zieldatei** 

Den Namen des neuen Objekts eingeben.
* **OK** 

Führt den Kopiervorgang aus und schließt das Dialogfenster.
* **Abbrechen** 

Bricht den Kopierbefehl ab und schließt das Dialogfenster.
* **Symbol anzeigen** 

&newline;Zeigt das Symbol an, das für die neue Datei verwendet wird.
* **Hilfe** 

Zeigt den Hilfetext für dieses Dialogfenster an.


Zum Kopieren von Dateien und Ordnern kann auch die Maus verwendet werden.
# Dialogfenster 'Anwendungsmanager - Verweis auf Datei/Ordner erstellen'




* **Ausgewähltes Objekt** 

Zeigt das Objekt an, für das ein Verweis erstellt wird.
* **Zielordner** 

Den vollständigen neuen Pfadnamen des Zielordners eingeben.
* **Zieldatei** 

Den Namen des neuen Objekts eingeben.
* **OK** 

Führt den Kopiervorgang aus und schließt das Dialogfenster.
* **Abbrechen** 

Bricht den Kopiervorgang ab und schließt das Dialogfenster.
* **Symbol anzeigen** 

&newline;Zeigt das Symbol an, das für die neue Datei verwendet wird.
* **Hilfe** 

Zeigt den Hilfetext für dieses Dialogfenster an.


Um Verweise für Dateien oder Ordner zu erstellen, kann auch die Maus
verwendet werden.
# Dialogfenster 'Anwendungsmanager - Datei verschieben'


Anwendungsmanager:Verschieben von ObjektenVerschieben von Objekten

* **Ausgewähltes Objekt** 

Zeigt die Datei bzw. den Ordner an, die/der verschoben
werden soll.
* **Zielordner** 

Den vollständigen neuen Pfadnamen des Zielordners eingeben.
* **OK** 

Führt das Verschieben aus und schließt das Dialogfenster.
* **Abbrechen** 

Bricht den Befehl zum Verschieben ab und schließt das
Dialogfenster.
* **Hilfe** 

Zeigt den Hilfetext für dieses Dialogfenster an.

# Dialogfenster 'Anwendungsmanager - Datei erstellen'




* **Name der neuen Datei:** 

&newline;Den Namen der neuen Datei bzw. des neuen Ordners eingeben.
Wird die Datei in einem anderen Ordner erstellt, muß der
vollständige Pfadname des neuen Ordners bzw. der neuen
Datei, der/die erstellt werden soll, eingegeben werden.
* **OK** 

Erstellt die Datei und schließt das Dialogfenster.
* **Anwenden** 

Erstellt die Datei; das Dialogfenster bleibt geöffnet,
so daß eine weitere neue Datei erstellt werden kann.
* **Abbrechen** 

Bricht den Befehl zum Erstellen einer neuen Datei ab und
schließt das Dialogfenster.
* **Symbol anzeigen** 

Erhält die Datei bei der Eingabe des neuen Dateinamens
einen anderen Dateityp, ändert sich möglicherweise das
zugehörige Symbol. Um das neue Symbol vorab anzuzeigen,
auf 'Symbol anzeigen' klicken; das Symbol im Dialogfenster wird
aktualisiert. Wird beispielsweise ein Name eingegeben, der auf.tifendet, und anschließend 'Symbol anzeigen' ausgewählt,
wird das Symbol für den Datentyp 'TIFF' angezeigt.
* **Hilfe** 

Zeigt den Hilfetext zu diesem Dialogfenster an.

# Dialogfenster 'Anwendungsmanager - Ordner erstellen'




* **Name des neuen Ordners:** 

&newline;Den Namen des neuen Ordners eingeben. Wird er in einem
anderen Ordner erstellt, muß der vollständige Pfadname
des neuen Ordners eingegeben werden, der erstellt werden soll.
* **OK** 

Erstellt den Ordner und schließt das Dialogfenster.
* **Anwenden** 

Erstellt den Ordner; das Dialogfenster bleibt geöffnet,
so daß ein weiterer Ordner erstellt werden kann.
* **Abbrechen** 

Bricht den Befehl zum Erstellen eines neuen Ordners ab
und schließt das Dialogfenster.
* **Symbol anzeigen** 

Erhält der Ordner beim Eingeben des neuen Ordnernamens
einen neuen Datentyp, ändert sich möglicherweise das
zugehörige Symbol. Um das neue Symbol vorab anzuzeigen,
auf 'Symbol anzeigen' klicken; das Symbol im Dialogfenster
wird aktualisiert.
* **Hilfe** 

Zeigt den Hilfetext zu diesem Dialogfenster an.

# Dialogfenster 'Anwendungsmanager - Filteroptionen festlegen'


Anwendungsmanager:Filtern von ObjektenFiltern von Objekten

* **Datentypen auswählen, die verdeckt/angezeigt werden sollen** 

Mit dieser Taste kann zwischen dem Anzeigen und Verdecken
von Datentypen gewechselt werden.
Eine Liste aller Datentypen, die auf dem jeweiligen System
definiert sind, wird angezeigt. Ausgewählte Datentypen werden
im Anwendungsmanager angezeigt bwz. verdeckt, je nachdem,
welche Möglichkeit mit der entsprechenden Taste ausgewählt
wurde.
* **Alles auswählen** 

Wählt alle Datentypen aus. Der Anzeigebereich des
Anwendungsmanagers ist leer, es sei denn, die Auswahl für
einzelne Datentypen wird wieder zurückgenommen.
* **Alles abwählen** 

Nimmt die Auswahl für alle Datentypen zurück.
* **Filterzeichenfolge (wahlweise)** 

Ermöglicht das Filtern nach Namen. Wird
beispielsweise*.oeingegeben, zeigt der Anwendungsmanager
Dateien, deren Name auf.oendet, nicht an. Es ist zu
beachten, daß alle Datentypen, die in diesem Feld eingegeben
werden, zur Liste der Datentypen hinzugefügt werden, die
in der Symbolliste im oberen Teil des Dialogfensters
ausgewählt wurden.
* **OK** 

Wendet die aktuellen Filtereinstellungen an und schließt das
Dialogfenster.
* **Anwenden** 

Wendet die aktuellen Filtereinstellungen an; das
Dialogfenster bleibt jedoch geöffnet.
* **Standardwerte** 

Stellt die Standardfilterliste erneut her (diese enthält
DOT_FILE, DOT_FOLDER und CURRENT_FOLDER). Die Filterliste
wird erst angewendet, wenn 'Anwenden' oder 'OK' ausgewählt
wird.
* **Abbrechen** 

Stellt die zuletzt angewendete Einstellung erneut her und
schließt das Dialogfenster.
* **Hilfe** 

Zeigt den Hilfetext zu diesem Dialogfenster an.

# Dialogfenster 'Anwendungsmanager - Suchen' (Dateien oder Ordner)


Das Dialogfenster 'Suchen' kann verwendet werden, um einen
Ordner und die darin enthaltenen Ordner nach Dateien mit bestimmten Namen
oder bestimmtem Inhalt zu durchsuchen.

* **Datei- oder Ordnername:** 

Den Namen der Datei bzw. des Ordners eingeben, nach dem
gesucht werden soll. Dabei können Platzhalterzeichen verwendet
werden.
* **Dateiinhalt** 

Mit dem Suchbefehl wird in Dateien und Ordnern nach dem Text
gesucht, der in diesem Feld eingegeben wird.
* **Zu durchsuchender Ordner:** 

Den Pfad des Ordners eingeben, bei dem mit der Suche begonnen
werden soll. Der Suchvorgang wird bei diesem Ordner gestartet
und schließt alle zugehörigen Unterordner ein.
* **Gefundende Dateien** 

Listet die Dateien bzw. die Ordner auf, die mit dem Suchbefehl
gefunden wurden. Doppelt auf eine Datei oder einen Ordner
in der Liste klicken, um ein neues Anwendungsmanagerfenster
zu öffnen, in dem die entsprechende Datei bzw. der
entsprechende Ordner angezeigt wird.
* **Ordner öffnen** 

Öffnet ein Anwendungsmanagerfenster mit dem Ordner, der
die in der Liste 'Gefundene Dateien' ausgewählte Datei
enthält. Wurde ein Ordner gefunden, wird der
Inhalt dieses Ordners im Fenster angezeigt.
* **In Arbeitsbereich stellen** 

Stellt die ausgewählte Datei bzw. den ausgewählten Ordner
auf das Hintergrundmuster des aktuellen Arbeitsbereichs.
* **Starten** 

Startet den Suchvorgang.
* **Beenden** 

Stoppt den momentan ausgeführten Suchvorgang. Es ist zu
beachten, daß diese Taste auch dann verwendet werden kann,
wenn der Cursor in Form einer Sanduhr angezeigt wird.
* **Abbrechen** 

Stoppt den momentan ausgeführten Suchvorgang und schließt
das Dialogfenster.
* **Hilfe** 

Zeigt den Hilfetext zum Suchen von Objekten an.

# Dialogfenster 'Anwendungsmanager - 'Zugriffsrechte'


Das Dialogfenster 'Zugriffsrechte' kann verwendet werden, um die
Lese-, Schreib- und Ausführungsberechtigung für die Dateien bzw. die Ordner
des jeweiligen Benutzers zu ändern. Nur der Eigentümer der Datei bzw. des Ordners
kann die Zugriffsrechte ändern. Ist der jeweilige Benutzer nicht Eigentümer
der Datei bzw. des Ordners, werden im Fenster die aktuellen Einstellungen
angezeigt, diese können jedoch nicht geändert werden. Darüber hinaus enthält
das Dialogfenster 'Zugriffsrechte' den vollständigen Pfadnamen der
Datei bzw. des Ordners, die Größe sowie Datum und Uhrzeit der letzten Änderung.

* **Eigentümer** 

Der Name des Benutzers, der Eigentümer des Objekts ist. Nur der
Systemadministrator (Benutzer 'root') kann den Eigentümer eines
Objekts ändern.
* **Gruppenname** 

Der Name der Benutzergruppe, die die Zugriffsrechte erhält, die
in der Zeile für die Gruppe innerhalb der Berechtigungsliste
angezeigt werden. Der Benutzer, der Eigentümer des Objekts ist,
kann die Gruppe ändern und eine andere Gruppe, der er angehört,
angeben. Der Benutzer 'root' kann jede beliebige Gruppe angeben.
* **Zugriffsrechte** 

Der Eigentümer kann die Lese-, Schreib- und Ausführungsberechtigung
ändern. Ein Markierungsfeld auswählen, um das entsprechende
Zugriffsrecht zur Verfügung zu stellen.
* **OK** 

Wendet die aktuellen Einstellungen an und schließt das Dialogfenster.
* **Abbrechen** 

Schließt das Dialogfenster, ohne Änderungen vorzunehmen.
* **Hilfe** 

Zeigt den Hilfetext zum Ändern von Zugriffsrechten an.

# Dialogfenster 'Anwendungsmanager - Optionen für Ansicht festlegen'


Anwendungsmanager:Optionen für AnsichtOptionen für Ansicht im AnwendungsmanagerDas Dialogfenster 'Optionen für Ansicht festlegen' kann dazu verwendet werden,
die Darstellung von Dateien und Ordnern im Anwendungsmanager zu ändern.


# Kopfzeilen


Mit der Option für den Kopfbereich kann angegeben werden, welche
Kopfzeilen im Anwendungsmanagerfenster angezeigt werden sollen.

* **Pfad als Symbole** 

Zeigt den aktuellen Ordner als Kette von Ordnersymbolen an.
* **Pfad als Text** 

Zeigt den vollständigen Pfadnamen des aktuellen Ordners in
einer Textzeile unmittelbar oberhalb des Hauptanzeigebereichs
an. Entweder mit der Maus in die Textzeile klicken und den Pfadnamen
editieren oder doppelt auf einen der Ordnernamen klicken, um
zu diesem Ordner zu wechseln.
* **Meldungszeile** 

Zeigt die Gesamtanzahl der Dateien, Ordner und der verdeckten
Dateien im aktuellen Ordner an.



# Plazierung


Mit der Option für die Plazierung kann angegeben werden, wie die Symbole
innerhalb des Anwendungsmanagerfensters angeordnet werden.

* **Wie positioniert** 

Die Objekte werden genau so angezeigt, wie sie vom
Benutzer positioniert werden.
* **Zeilen und Spalten** 

Die Objekte werden neu sortiert und in Zeilen und Spalten
angeordnet.



# Anzeige
Ordner, BaumstrukturBaumstruktursicht (Anwendungsmanager)

* **Nach einzelnem Ordner** 

Zeigt den Inhalt des aktuellen Ordners an.
* **Hierarchisch** 

Zeigt den Inhalt des aktuellen Ordners in Form eines
Baumes an.
* **Nur Ordner** 

Ist 'Hierarchisch' ausgewählt, werden nur
Ordner angezeigt. Dies ist der Standardwert.
* **Ordner, dann Dateien** 

Ist 'Hierarchisch' ausgewählt, werden zunächst
nur Ordner, nach Klicken auf + auch Dateien angezeigt.
* **Ordner und Dateien** 

Ist 'Hierarchisch' ausgewählt, werden alle
Ordner und Dateien angezeigt.



# Darstellung


* **Nur Nach Namen** 

Die einzelnen Objekte werden nur mit ihrem Namen angezeigt.
* **Mit großen Symbolen** 

Die einzelnen Objekte werden mit ihrem Namen und einem
großen Symbol angezeigt. (Dies ist der Standardwert.)
* **Mit kleinen Symbolen** 

Die einzelnen Objekte werden mit ihrem Namen und einem
kleinen Symbol angezeigt.

Ausführliche Auflistung
* **Nach Name, Datum, Größe...** 

Die einzelnen Objekte werden in einer ausführlichen Liste
angezeigt, in der Name, Änderungsdatum, Größe,
Zugriffsrechte, Eigentümer und Gruppe aufgeführt sind.



# Reihenfolge


Mit dieser Option kann die Reihenfolge ausgewählt werden, in der die
Dateien und Ordner angezeigt werden.

* **Alphabetisch** 

'Aufsteigend' (A bis Z, dann a bis z) oder 'Absteigend'
(Z bis A, dann z bis a) auswählen. (Standardwert ist
alphabetisch, A bis Z.)
* **Nach Dateityp** 

Die Dateien werden anhand des Datentyps gruppiert.
* **Nach Datum** 

'Aufsteigend' (ältestes Datum zuerst, neuestes Datum zuletzt)
oder 'Absteigend' (neuestes Datum zuerst, ältestes Datum
zuletzt) auswählen.
* **Nach Größe** 

'Aufsteigend' (kleinste Datei/kleinster Ordner zuerst,
größte Datei/größter Ordner zuletzt) oder 'Absteigend' (größte
Datei/größter Ordner zuerst, kleinste Datei/kleinster Ordner
zuletzt) auswählen.

# Richtung


Mit dieser Option kann die Richtung ausgewählt werden, in der die
Dateien und Ordner aufgelistet werden.

* **Aufsteigend** 

Ältestes Datum zuerst, neuestes Datum zuletzt;
kleinste Datei/kleinster Ordner zuerst, größte Datei/größter
Ordner zuletzt.
* **Absteigend** 

Neuestes Datum zuerst, ältestes Datum zuletzt;
größte Datei/größter Ordner zuerst, kleinste Datei/kleinster
Ordner zuletzt.
