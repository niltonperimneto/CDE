
# Aktion erstellen - Referenz

# Allgemeine Informationen zur Anwendung 'Aktion erstellen'





# Aktion erstellen - Fenster









# Dateien, die mit 'Aktion erstellen' erstellt wurden
Aktion erstellen:erstellte DateienAktionsname

Die Anwendung 'Aktion erstellen' erzeugt folgende Ausgabe:

Eine Aktionsdefinition, mit der ein Befehl ausgeführt wird. Eine Datei`Home-Verzeichnis`/.dt/types/`Aktionsname`.dtwird erstellt, die die
Definition enthält.

Aktionssymbol

EinAktionssymbolfür die Aktion. Das Aktionssymbol wird in das
Home-Verzeichnis des Benutzers gestellt. Das Symbol führt den Aktionsbefehl
aus, wenn es doppelt geklickt wird. Wahlweise kann durch die Angabe von
für die Übergabe geeigneter Datentypen aus dem Aktionssymbol ein
Übergabebereich erzeugt werden.

Werden weitere Funktionen von 'Aktion erstellen' verwendet, können außerdem
folgende Elemente erstellt werden:

Ein oder mehrere Datentypen für die Datendateien der Anwendung

Öffnungs- und Druckaktionen für den Datentyp

Die Datentypen und die Öffnungs- und Druckaktionen werden auch in die Datei`Home-Verzeichnis`/.dt/types/`Aktionsname`.dtgeschrieben.
# 'Aktion erstellen' - Beschränkungen

# Beschränkungen für 'Aktion erstellen'
Aktion erstellen:Aktionsbeschränkungen

Die Anwendung 'Aktion erstellen' kann nicht zur Erstellung der Aktion für
eine Anwendung verwendet werden, wenn:

In der Befehlszeile ein Parameter erforderlich ist, der keinen Dateinamen
angibt. Beispielsweise kann 'Aktion erstellen' nicht verwendet werden, um
eine Aktion für folgenden Befehl zu schreiben:lp -d`Gerät``Dateiname`

Hierbei muß der Benutzer jedesmal bei Ausführung des Befehls ein Gerät
angeben. (Eine Aktion, die diesen Schritt ausführt, kann manuell erstellt
werden.)

Dem Benutzer eine andere Bezeichnung auf dem Aktionssymbol angezeigt werden
soll als der Aktionsname. Beispielsweise kann 'Aktion erstellen' nicht
verwendet werden, um eine landessprachliche Version einer bestehenden
Aktion zur Verfügung zu stellen.

Für die Aktion einige der erweiterten Funktionen der Aktionsdatenbank
erforderlich sind, z. B. Aktionen, die

Befehle auf Systemen starten, die fern von der Aktionsdefinition sind.

andere Aktionen aufrufen.

unter einer anderen Benutzer-ID ausgeführt werden müssen (z. B.
als Superuser).

die Funktion "map" häufig verwenden.

sich sehr unterschiedlich verhalten, je nach Anzahl der
Dateiargumente, die mit der Aktion geliefert werden.
# Beschränkungen für Datentyp
Aktion erstellen:Datentypbeschränkungen

Die Anwendung 'Aktion erstellen' kann nicht zur Erstellung von Datentypen
für eine Anwendung verwendet werden, wenn

Für den Datentyp zusätzliche Aktionen zugeordnet sein müssen, nicht nur
Öffnen und Drucken.

Die Öffnungsaktion für den Datentyp nicht der Befehl der Aktion ist.
Beispielsweise kann 'Aktion erstellen' nicht verwendet werden, um den
Datentyp zu erstellen, der ein eindeutiges Symbol für das Verzeichnis
zur Verfügung stellt, das die Anwendungsgruppe der Anwendung darstellt.
# 'Aktion erstellen' - Hauptfenster
Aktion erstellen:Hauptfenster

Informationen zu Aufgaben befinden sich in.

* **Aktionsname** 

Den Namen der Aktion eingeben. Bei dem Namen muß die
Groß-/Kleinschreibung beachtet werden, und er darf keine Leerzeichen enthalten.
* **Aktionssymbole** 

Zeigt das Symbolabbild an, das vom Aktionssymbol verwendet wird.
* **Gruppe suchen** 

Zeigt das Dialogfenster 'Gruppe suchen' an. Das Dialogfenster
'Gruppe suchen' verwenden, um eine andere bestehende Bitmap- oder
Pixmap-Datei auszuwählen.
* **Symbol bearbeiten** 

Öffnet den Symboleditor. Den Symboleditor verwenden, um
ein neues Symbol zu erstellen oder ein bestehendes zu editieren.
* **Befehl beim Öffnen der Aktion** 

Den Befehl eingeben. Die Syntax$`n`für ein Dateiargument verwenden.
* **Hilfetext für Aktionssymbol** 

Den Text der Kontexthilfe für das
Aktionssymbol eingeben.
* **Fensterart** 

Die Art des Fensters auswählen, die von der Aktion erstellt wird:

Grafisch: Die Anwendung zeigt ihr eigenes Fenster an.

Terminal (automatisches Schließen): Die Aktion zeigt ein Terminal-Fenster an,
das sich schließt, wenn die Aktion abgeschlossen ist.

Terminal (manuelles Schließen): Die Aktion zeigt ein Terminal-Fenster an, das
der Benutzer manuell schließen muß.

Keine Ausgabe: Die Anwendung benötigt kein Fenster.

# Erweiterte Funktionen


Die erweiterten Funktionen nur verwenden, wenn der Befehl im Feld
'Befehl beim Öffnen der Aktion' ein Dateiargument enthält.

* **Beim Öffnen einer Aktion Benutzer nach folgendem fragen** 

Die Dateieingabeaufforderung
eingeben, die angezeigt wird, wenn doppelt auf das Aktionssymbol geklickt wird.
* **Datentypen, die diese Aktion verwenden** 

Eine Liste der Datentypen, die für
diese Aktion erstellt wurden. Für diese Liste besteht nur Lesezugriff. Einträge
werden hinzugefügt, wenn Datentypen mit Hilfe des Dialogfensters 'Datentyp
hinzufügen' erstellt werden.
* **Hinzufügen** 

Öffnet das Dialogfenster 'Datentyp hinzufügen' zur Erstellung
eines neuen Datentyps.
* **Löschen** 

Löscht den ausgewählten Datentyp aus der Liste 'Datentypen, die
diese Aktion verwenden'.
* **Bearbeiten** 

Diese Taste auswählen, um die Datentypen, die in der Liste
'Datentypen, die diese Aktion verwenden' ausgewählt wurden, zu bearbeiten.
* **Für Übergabe geeignete Datentypen** 

Auswählen, ob das Aktionssymbol Dateien aller
Datentypen oder nur Dateien der Datentypen in der Liste
'Datentypen, die diese Aktion verwenden' akzeptiert.

# Dialogfenster 'Datentyp hinzufügen'
Datentyp hinzufügen, Dialogfenster

Informationen zu Aufgaben befinden sich in.

* **Name der Datentypfamilie** 

Der Datentypname. Ein Name wird automatisch
zur Verfügung gestellt. Das Textfeld kann editiert werden.
Bei dem Namen muß die Groß-/Kleinschreibung beachtet werden, und er darf
keine Leerzeichen enthalten.
* **Identifizierende Merkmale** 

Eine Liste von Kriterien für die Eingabe der
Datei. 'Bearbeiten' verwenden, um die Merkmale zu setzen und zu ändern.
* **Bearbeiten** 

Zeigt das Dialogfenster 'Identifizierende Merkmale' an.
* **Hilfetext für Datentypsymbol** 

Den Text der Kontexthilfe für Dateien mit diesem
Datentyp eingeben.
* **Datentypsymbole** 

Zeigt das Symbolabbild an, das vom Datentyp verwendet wird.
* **Gruppe suchen** 

Zeigt das Dialogfenster 'Gruppe suchen' an. Das Dialogfenster
'Gruppe suchen' verwenden, um eine andere bestehende Bitmap- oder
Pixmap-Datei auszuwählen.
* **Symbol bearbeiten** 

Öffnet den Symboleditor. Den Symboleditor verwenden, um
ein neues Symbol zu erstellen oder ein bestehendes zu editieren.
* **Befehl zum Öffnen dieses Datentyps** 

Zeigt den Befehl an, der ausgeführt wird,
wenn der Benutzer doppelt auf den Datentyp klickt. Dies ist derselbe Befehl,
der im Feld 'Befehl beim Öffnen der Aktion' angezeigt wird.
* **Befehl zum Drucken dieses Datentyps** 

Die Befehlszeile eingeben, die von der
Anwendung zur Verfügung gestellt wird, um den Datentyp zu drucken.
* **OK** 

Erstellt die Datentypinformationen, fügt den Datentyp zur Liste 'Datentypen,
die diese Aktion verwenden' hinzu und schließt das Dialogfenster.
* **Anwenden** 

Erstellt die Datentypinformationen und fügt den Datentyp zur Liste
'Datentypen, die diese Aktion verwenden' hinzu. Das Dialogfenster bleibt
geöffnet.
* **Abbrechen** 

Schließt das Dialogfenster 'Datentyp hinzufügen', ohne einen
Datentyp zu erstellen.
* **Hilfe** 

Zeigt den Online-Hilfetext an.

# Dialogfenster 'Identifizierende Merkmale'
Identifizierende Merkmale, Dialogfenster

Informationen zu Aufgaben befinden sich in.

* **Alle einschießen** 

Dateien: Auswählen, wenn der Datentyp nur für Dateien gilt.

Ordner: Auswählen, wenn der Datentyp nur für Ordner gilt.
* **Namensmuster** 

Das Markierungsfeld auswählen und das Namensmuster eingeben.
Folgende Sonderzeichen können verwendet werden:

*: Gleicht jede Folge von Zeichen ab.

?: Gleicht jedes einzelne Zeichen ab.
* **Zugriffsmaske** 

Das Markierungsfeld auswählen und danach die geltenden
Wechselschalter auswählen. 'Egal' auswählen, wenn die Zugriffsrechte nicht
relevant sind.
* **Inhalt** 

Das Markierungsfeld auswählen und die Daten eingeben, die die
Datei enthalten muß.
* **Typ** 

Den Datentyp auswählen. 'Zeichenfolge' für Text-(ASCII)-Daten verwenden.
* **Startbyte** 

Die Position in der Datei angeben, bei der mit der Suche nach den
Daten begonnen werden soll. '1' verwenden, um am Anfang der Datei mit der
Suche zu beginnen.
* **OK** 

Wendet die Merkmale für die identifizierenden Merkmale im Dialogfenster
'Datentyp hinzufügen' an und schließt das Dialogfenster 'Identifizierende Merkmale'.
* **Abbrechen** 

Schließt das Dialogfenster, ohne die Änderungen zu speichern.
* **Inhalt löschen** 

Löscht den Inhalt des Fensters und stellt die Standardwerte
wieder her.
* **Hilfe** 

Zeigt den Online-Hilfetext an.

# Dialogfenster 'Gruppe suchen'
Gruppe suchen, Dialogfenster

Mit dem Dialogfenster 'Gruppe suchen' kann der Benutzer das Symbolabbild
angeben, das für eine Aktion oder einen Datentyp verwendet werden soll.
Informationen zu Aufgaben befinden sich in.

* **Symbolordner** 

Listet die Ordner im Suchpfad für Symbole auf. Doppelt auf
einen Ordner klicken, um die darin enthaltenen Symbole anzuzeigen.
* **Symboldateien** 

Listet die Symbole im Verzeichnis auf. Auf eine Symboldatei
klicken, um sie auszuwählen. Der jeweilige Name wird im Textfeld 'Den
Namen der Symboldatei eingeben' angezeigt.
* **Den Namen der Symboldatei eingeben** 

Textfeld für die Eingabe desBasisnamensder Symboldatei. Der Inhalt des
Feldes ändert sich, wenn der Benutzer auf einen Eintrag in der Liste
der Symboldateien klickt.
* **OK** 

Wählt das Symbol, das im Textfeld 'Den Namen der Symboldatei eingeben'
angegeben ist, aus und schließt das Dialogfenster 'Gruppe suchen'.
* **Abbrechen** 

Schließt das Dialogfenster, ohne das Symbol zu ändern.
* **Hilfe** 

Zeigt den Online-Hilfetext an.

# Dialogfenster 'Aktion erstellen - Öffnen'
Aktion erstellen:Aktionsdatei öffnenAktion:öffnenAktion:editieren

* **Pfad- oder Ordnernamen eingeben&sigspace;** 

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
