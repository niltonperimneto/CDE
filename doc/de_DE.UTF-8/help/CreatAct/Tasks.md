
# Aktion erstellen - Aufgaben

# Erstellen und Bearbeiten von Aktionen





# Erstellen und Bearbeiten von Datentypen





# Symbole für Aktionen und Datentypen



# Erstellen einer Aktion mit der Anwendung 'Aktion erstellen'
Aktion:erstellen

Die Anwendungsgruppe 'Desktop-Anwendungen' im Anwendungsmanager öffnen und
doppelt auf 'Aktion erstellen' klicken.

Dadurch wird das Hauptfenster 'Aktion erstellen' angezeigt.

Im Textfeld 'Aktionsname' den Namen eingeben, der das Aktionssymbol bezeichnet.

Der Aktionsname kann beliebige Zeichen enthalten, mit Ausnahme von Leerzeichen.

Die Steuerelemente für Aktionssymbole verwenden, um das Symbol für die Anwendung
anzugeben. Anfänglich wird das Standardsymbol angezeigt.

Um andere bestehende Symbole zu verwenden, auf 'Gruppe suchen' klicken, um
das Dialogfenster 'Gruppe suchen' zu öffnen. Weitere Informationen befinden
sich in.

Um neue Symbole zu erstellen, auf 'Symbole bearbeiten' klicken, um den
Symboleditor zu starten. Siehe.

Im Textfeld 'Befehl beim Öffnen der Aktion' den Befehl eingeben, mit dem die
Anwendung gestartet wird.

Die Syntax$`n`für ein Dateiargument verwenden. Beispiel:emacs
bitmap $1
diff $1 $2
lp -oraw $1

Enthält die Befehlszeile ein Dateiargument ($`n`), wird das Aktionssymbol
zum Übergabebereich für Dateien.

Die Befehlszeilen werden nicht an eine Shell übergeben, es sei denn, der
Benutzer gibt die Verwendung einer Shell explizit an. Beispielsweise
verwenden die folgenden Zeilen die Shell-Verarbeitung:/bin/sh -c `ps | lp'
/bin/sh -c `spell $1 | more'

Im Textfeld 'Hilfetext für Aktionssymbol' die Hilfeinformationen eingeben,
die angezeigt werden sollen, wenn der Benutzer die Kontexthilfe zumAktionssymbolaufruft.

Der Text im Textfeld wird automatisch umgebrochen. Diese Zeilenumbrüche
werden jedoch online nicht beibehalten. Soll ein "harter" (unveränderlicher)
Zeilenumbruch angegeben werden,&bsol;nverwenden.

Das Menü der Taste 'Fensterart' verwenden, um die erforderliche unterstützte
Fenstertechnik auszuwählen.

* **Grafisch (X-Window)** 

Die Anwendung zeigt ihr eigenes Fenster an.
* **Terminal (automatisches Schließen)** 

Die Anwendung wird in einem Terminal-Fenster
ausgeführt, das sich automatisch schließt, wenn der Benutzer die Anwendung
verläßt.
* **Terminal (manuelles Schließen)** 

Die Anwendung wird in einem Terminal-Fenster
ausgeführt, das so lange geöffnet bleibt, bis der Benutzer es schließt.
* **Keine Ausgabe** 

Die Anwendung erzeugt keine Ausgabe am Bildschirm.


Wie folgt vorgehen:

Enthält die Anwendung Datendateien und sollen ein oder mehrere Datentypen
für die Dateien erstellt werden, siehe.

Ist es nicht erforderlich, einen Datentyp zu erstellen:

Die Aktion mit der Option 'Speichern' aus dem Menü 'Datei' speichern.

Die neue Aktion testen. Dazu das entsprechende Symbol im
Standardverzeichnis des Benutzers doppelt klicken.
# Erstellen eines Datentyps mit der Anwendung 'Aktion erstellen'
Datentyp:erstellen

Die Aktion für die Anwendung definieren. Siehe Schritt 1 bis 6 des
Abschnitts.

Auf die Taste 'Erweitert' klicken, um das Fenster 'Aktion erstellen' zu
erweitern.

Soll dasAktionssymboleine Eingabeaufforderung ausgeben, wenn
doppelt auf das Symbol geklickt wird, den Text der Eingabeaufforderung im
Textfeld 'Beim Öffnen einer Aktion Benutzer nach folgendem fragen' eingeben.

Dieses Feld muß verwendet werden, wenn in der Befehlszeile der Anwendung
ein erforderliches Dateiargument vorkommt.

Dieses Feld muß leer bleiben, wenn in der Befehlszeile der Anwendung kein
Dateiargument vorkommt.

Ist das Dateiargument in der Befehlszeile der Anwendung wahlfrei, kann der
Benutzer wählen: Stellt er einen Text für die Eingabeaufforderung zur
Verfügung, fordert das Aktionssymbol beim Doppelklicken zur Eingabe des
Dateinamens auf. Stellt er keinen Text für die Eingabeaufforderung
zur Verfügung, wird die Aktion mit einer leeren Zeichenfolge als
Dateiargument ausgeführt.

Die Typen von Dateien angeben, die die Aktion als Argumente akzeptiert:

Kann die Aktion alle Dateitypen akzeptieren, auf 'Alle Datentypen'
klicken.

Kann die Aktion nur die Datentypen akzeptieren, die der Benutzer
für die Anwendung erstellt hat, auf 'Nur obige Liste' klicken.

Anfänglich ist die Liste 'Datentypen, die diese Aktion verwenden' leer. Erstellt
der Benutzer Datentypen für die Anwendung, werden diese zur Liste hinzugefügt.

Auf 'Hinzufügen' klicken, um das Dialogfenster 'Datentypen hinzufügen'
anzuzeigen.

Wahlweise: Soll der Standarddatentypname nicht verwendet werden, einen neuen
Namen für den neuen Datentyp im Textfeld 'Name der Datentypfamilie' eingeben.
Der Name darf keine Leerzeichen enthalten.

Auf die Taste 'Bearbeiten' neben dem Feld 'Identifizierende Merkmale' klicken,
um das Dialogfenster 'Identifizierende Merkmale' anzuzeigen.

Dieses Dialogfenster verwenden, um die Merkmale festzulegen, die zur
Unterscheidung des Datentyps von anderen verwendet werden. Der Benutzer
kann ein Kriterium (z. B. Namensmuster) oder eine Kombination von Kriterien
(z. B. sowohl Namensmuster als auch Berechtigungsmuster) auswählen.

Entweder auf die Umschalttaste 'Datei' oder 'Ordner' klicken, um anzugeben,
ob der Datentyp eine Datei oder einen Ordner darstellt.

Hängt der Datentyp vom Namen ab, auf das Markierungsfeld 'Namensmuster'
klicken und das Namensmuster eingeben. Dabei können die Platzhalter*und?verwendet werden.

* ***** 

Gleicht jede Folge von Zeichen ab.
* **?** 

Gleicht jedes einzelne Zeichen ab.


Hängt der Datentyp von den Zugriffsrechten ab, auf das Markierungsfeld
'Berechtigungsmuster' klicken und danach auf die für den Datentyp erforderlichen
Zugriffsrechte klicken.

* **Ein** 

Die Datei muß über die angegebenen Zugriffsrechte verfügen.
* **Aus** 

Die Datei darf nicht über die angegebenen Zugriffsrechte verfügen.
* **Egal** 

(Standardwert) Die Zugriffrechte sind nicht relevant.


Hängt der Datentyp vom Inhalt ab, auf das Markierungsfeld 'Inhalt' klicken,
das angeforderte Informationsmuster, nach dem gesucht werden soll,
eingeben und den Typ des Inhalts festlegen. Wahlweise die Byteposition
angeben, an der mit der Suche begonnen werden soll.

Auf 'OK' klicken, um das Dialogfenster 'Identifizierende Merkmale' zu schließen.

Die Merkmale werden im Feld 'Identifizierende Merkmale' im Dialogfenster
'Datentyp hinzufügen' angezeigt.

Die Informationen zu den Zugriffsrechten im Feld 'Identifizierende Merkmale'
verwenden folgende Codierung:

* **d** 

Verzeichnis
* **r** 

Lesezugriff
* **w** 

Schreibzugriff
* **x** 

Ausführbar
* **!** 

NICHT
* **&** 

UND


Den Hilfetext für den Datentyp im Textfeld 'Hilfetext für Datentypsymbol'
eingeben.

Die Steuerelemente für die Datentypsymbole verwenden, um das Symbol für
die Anwendung anzugeben.

Anfänglich werden die Standardsymbole angezeigt.

Um ein anderes bestehendes Symbol zu verwenden, auf 'Gruppe suchen' klicken, um
das Dialogfenster 'Gruppe suchen' zu öffnen. Weitere Informationen befinden
sich in.

Um neue Symbole zu erstellen, auf 'Symbole bearbeiten' klicken, um den
Symboleditor zu starten. Siehe.

Wahlweise: Stellt die Anwendung einen Druckbefehl für das Drucken von
Datendateien von der Befehlszeile aus zur Verfügung, den Befehl im Textfeld
'Befehl zum Drucken dieses Datentyps' eingeben. Dabei die Syntax$`n`für ein Dateiargument verwenden.

Auf 'OK' klicken, um das Dialogfenster 'Datentyp hinzufügen' zu schließen und
den Datentyp zur Liste der Datentypen im Fenster 'Aktion erstellen'
hinzuzufügen.
# Angeben von Symbolen für Aktionen und Datentypen


Das Hauptfenster 'Aktion erstellen' und das Dialogfenster 'Datentyp hinzufügen'
enthält Tasten zum Angeben des Symbols, das von Aktionen und Datentypen
verwendet werden soll.

Um ein bestehende Symbol zu verwenden, auf 'Gruppe suchen' klicken.
Siehe.

Um mit Hilfe des Symboleditors ein neues Symbol zu erstellen, auf
'Symbol bearbeiten' klicken. Siehe.
# Verwenden des Dialogfensters 'Gruppe suchen'


Das Dialogfenster 'Gruppe suchen' wird angezeigt, wenn der Benutzer auf
'Gruppe suchen' im Hauptfenster 'Aktion erstellen' oder im Dialogfenster
'Datentyp hinzufügen' klickt.

Im Dialogfenster 'Gruppe suchen' kann der Benutzer folgendes angeben:

Ein Symbol in einem Ordner in der Liste der Symbolordner. Die Liste der
Symbolordner enthält alle Ordner im Suchpfad für Symbole.

Ein Symbol, das Teil eines Registrierungspakets ist, das unter Verwendung
vondtappintegratein das Desktop integriert wird.
# Angeben eines Symbols in einem Symbolordner


In der Liste der Symbolordner doppelt auf den Ordnerpfad klicken, in dem
das Symbol enthalten ist.

In der Liste der Symboldateien werden alle Symboldateien in diesem Ordner
angezeigt.

In der Liste der Symboldateien auf das Symbol klicken, das verwendet
werden soll.

Auf 'OK' klicken.
# Angeben eines Symbols in einem Registrierungspaket


Ist der Benutzer ein Systemadministrator oder Programmierer, der ein
Registrierungspaket erstellt, befinden sich die Symbolabbilddateien anfänglich
in einem Verzeichnis im Registrierungspaket:`app_root`/dt/appconfig/icons/`Sprache`

Nach der Registrierung sind die Symboldateien in einemSymbolordner. Daher sollten in den Definitionen für Aktion und
Datentyp die Basisdateinamen verwendet werden.

Die folgende Prozedur verwenden, um einen Basisnamen für ein Symbol anzugeben,
das sich momentan nicht in einem Symbolordner befindet:

Im Textfeld 'Den Namen der Symboldatei eingeben' denBasisnamender Symboldatei eingeben.

Auf 'OK' klicken.

Ein Fehlerdialogfenster wird angezeigt, das besagt, daß die Symboldatei nicht
in einem Symbolordner gefunden werden konnte.

Im Fehlerdialogfenster auf 'Name OK' klicken. Dadurch wird das
Fehlerdialogfenster und das Dialogfenster 'Gruppe suchen' geschlossen.

Ignorieren, daß im Feld für Aktionssymbole oder Datentypsymbole das
Symbolabbild fehlt. Das Symbolabbild wird gefunden, sobald die
Anwendung registriert ist.
# Erstellen eines neuen Symbolabbilds


Auf 'Symbol bearbeiten' im Fenster 'Aktion erstellen' oder 'Datentyp
hinzufügen' klicken. Dadurch wird der Symboleditor gestartet.

Den Symboleditor verwenden, um ein neues Symbol zu zeichnen. Siehe

Symboleditor - Aufgaben



'Speichern' aus dem Menü 'Datei' auswählen, im die Symboldatei zu speichern.

Die Symboldatei im Ordner`Standardverzeichnis`/.dt/iconsspeichern.
Siehe.

Ist der Benutzer ein Systemadministrator oder Programmierer, der ein
Registrierungspaket erstellt, sollte die Symboldatei im Verzeichnis`approot`/dt/appconfig/icons/`Sprache`gespeichert werden.
# Symbolgrößen und -Namen


Desktopsymbole verwenden folgende Namenskonvention:

* **Größe (Pixel)** 

Name
* **32 x 32** 

`Basisname`.m.pmoder`Basisname`.m.bm
* **16 x 16** 

`Basisname`.s.pmoder`Basisname`.s.bm


Für Aktionssymbole den Aktionsnamen als Basisnamen verwenden.

Für Datentypsymbole den Datentypnamen als Basisnamen verwenden.
# Bearbeiten einer bestehenden Aktion
Aktion:bearbeiten

Die Anwendung 'Aktion erstellen' kann zum Bearbeiten einer bestehenden
Aktion verwendet werden, wenn:

Die Aktion mit Hilfe der Anwendung 'Aktion erstellen' erstellt wurde.

Unddie Datei, die die Aktionsdefinition enthält, nicht manuell (mit
einem Texteditor) bearbeitet wurde.

Die Anwendungsgruppe 'Desktop-Anwendungen' im Anwendungsmanager öffnen und
doppelt auf 'Aktion erstellen' klicken.

Dadurch wird das Hauptfenster 'Aktion erstellen' angezeigt.

'Öffnen' aus dem Menü 'Datei' auswählen. Dadurch wird das Dialogfenster
'Aktion erstellen - Öffnen' angezeigt.

In der Liste der Dateien die Datei auswählen, die die Aktionsdefinition
enthält. Sie hat den Namen`Aktionsname`.dt.

'OK' auswählen.
# Bearbeiten eines bestehenden Datentyps
Datentyp:bearbeiten

Die Anwendung 'Aktion erstellen' kann zum Bearbeiten eines bestehenden
Datentyps verwendet werden, wenn:

Der Datentyp mit Hilfe der Anwendung 'Aktion erstellen' erstellt wurde.

Unddie Datei, die die Datentypdefinition enthält, nicht manuell (mit
einem Texteditor) bearbeitet wurde.

Die Anwendungsgruppe 'Desktop-Anwendungen' im Anwendungsmanager öffnen und
doppelt auf 'Aktion erstellen' klicken.

Dadurch wird das Hauptfenster 'Aktion erstellen' angezeigt.

'Öffnen' aus dem Menü 'Datei' auswählen. Dadurch wird das Dialogfenster
'Aktion erstellen - Öffnen' angezeigt.

In der Liste der Dateien doppelt auf die Datei klicken, die die
Datentypdefinition enthält.

Der Name der Datei ist`Aktionsname`.dt, wobeiAktionsnameder
Name der Aktion ist, die zur gleichen Zeit erstellt wurde wie der
Datentyp.

Auf 'Erweitert' klicken.

In der Liste 'Datentypen, die diese Aktion verwenden' den Datentyp
auswählen, der bearbeitet werden soll.

Auf 'Bearbeiten' klicken, um das Dialogfenster 'Datentyp bearbeiten'
zu öffnen.

Die Änderungen im Dialogfenster 'Datentyp bearbeiten' vornehmen. Danach
auf 'OK' klicken.

Um die bearbeitete Definition zu speichern, 'Speichern' aus dem Menü 'Datei'
auswählen.