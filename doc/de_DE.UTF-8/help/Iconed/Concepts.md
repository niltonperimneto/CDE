
# Symboleditor - Konzepte







# Einführung in den Symboleditor
Verwendung: SymboleditorSymboleditor: VerwendungEditor, SymbolEditieren: SymbolSymbol: editieren

Mit dem Symboleditor können Abbilder in zwei Formaten erstellt und editiert
werden:

X-Pixmap (XPM-Format)-- Mehrfarbige Abbilder, die statische und
dynamische Farben enthalten. Pixmap-Datei werden normalerweise durch
die Dateinamenerweiterung.pmidentifiziert.

X-Bitmap (XBM-Format)-- Schwarzweiße Abbilder. Bitmap-Datei werden
normalerweise durch die Dateinamenerweiterung.pmidentifiziert.

Das Abbild wird gezeichnet, indem der Benutzer ein Werkzeug und eine Farbe
auswählt und dann im Arbeitsbereich zeichnet. Während des Zeichnens zeigt der
Symboleditor eine Kopie des Symbols in Normalgröße in beiden Formaten an.
Auch wenn das Symbol zur Verwendung auf Farbsystemen gezeichnet wird, sollte
sichergestellt werden, daß die Schwarzweißversion lesbar ist, da Symbole
möglicherweise in das Bitmap-Format umgesetzt werden, wenn nicht genügend
Farben zur Verfügung stehen, um die Version mit allen Farben anzeigen zu können.
# Empfehlungen für das Symboldesign


Versuchen, für zusammengehörige Symbole ein gemeinsames Thema zu finden.
Werden beispielsweise Symbole für eine Anwendung entwickelt, sollten
sinnvolle Ähnlichkeiten zwischen den Symbolen der Anwendung und denen
zugehöriger Datendateien bestehen.

Beim Zeichnen sicherstellen, daß die Schwarzweißversion von farbigen Symbolen
lesbar ist. Wird das Symbol auf einem Monochrom- oder Graustufenbildschirm
angezeigt (oder wenn nicht genügend Farben zur Verfügung stehen), wird das
Symbol automatisch in seinem Schwarzweißformat angezeigt.




# Farbverwendung
Farbe: Verwendung in SymbolenSymbol: Farbverwendung

Symbole, die im Desktop verwendet werden, benutzen eine Palette mit
22 Farben:

Acht statische Graustufen.

Acht statische Farben: rot, blau, grün, zyanblau, magenta, gelb,
schwarz und weiß.

Sechs dynamische Farben: Vordergrund, Hintergrund, Schatten oben,
Schatten unten, Wählen und transparent.

Diese Palette ist so umfangreich, daß damit sinnvoll gestaltete, leicht
zu lesende Symbole erstellen zu können, ohne zu viele Farbressourcen zu
beanspruchen, die möglicherweise von anderen Anwendungen benötigt werden.
Symbole, die mit dem Desktop zur Verfügung gestellt werden, verwenden
hauptsächlich Grautöne mit farbigen Akzenten.

Die dynamischen Farben sind für Symbole sinnvoll, die ihre Farbe ändern,
wenn unterschiedliche Farbpaletten in der Umgebungsverwaltung ausgewählt
werden.

Die transparente Farbe ist bei der Erstellung von Symbolen sinnvoll,
die nicht das gesamte für das Symbol vorgesehene Rechteck ausfüllen
sollen. Die Hintergrundfarbe kann an den entsprechenden Stellen
(meistens am Rand) durchscheinen.
# Empfehlungen für die Symbolgröße
Symbol: GrößenempfehlungenGröße: Symbole

Im folgenden werden die empfohlenen Größen -- in Pixeln, Breite &times; Höhe --
für die Erstellung neuer Symbole aufgeführt.

Dateimanager (Groß):

Hohe Auflösung: 32 &times; 32&newline;Mittlere Auflösung: 32 &times; 32
Geringe Auflösung: 32 &times; 32

Dateimanager (Klein):

Hohe Auflösung: 16 &times; 16&newline;Mittlere Auflösung: 16 &times; 16
Geringe Auflösung: 16 &times; 16

Anwendungsmanager (Groß):

Hohe Auflösung: 32 &times; 32&newline;Mittlere Auflösung: 32 &times; 32
Geringe Auflösung: 32 &times; 32

Anwendungsmanager (Klein):

Hohe Auflösung: 16 &times; 16&newline;Mittlere Auflösung: 16 &times; 16
Geringe Auflösung: 16 &times; 16

Bedienfeld:

Hohe Auflösung: 48 &times; 48&newline;Mittlere Auflösung: 48 &times; 48
Geringe Auflösung: 32 &times; 32

Bedientafeln des Bedienfeldes:

Hohe Auflösung: 32 &times; 32&newline;Mittlere Auflösung: 32 &times; 32
Geringe Auflösung: 16 &times; 16

Fenster in Symbolgröße:

Hohe Auflösung: 48 &times; 48&newline;Mittlere Auflösung: 48 &times; 48
Geringe Auflösung: 32 &times; 32

Desktop:

Hohe Auflösung: 32 &times; 32&newline;Mittlere Auflösung: 32 &times; 32
Geringe Auflösung: 32 &times; 32

Hintergrundabbilder können beliebig groß sein. Das Muster wird so oft
wiederholt, bis es den gesamten Arbeitsbereich ausfüllt.
# Siehe auch



# Namenskonventionen für Symboldateien
Symbol: Suche nach DateienAbbilddateien: suchenSymbol: Namenskonventionen.pmDateinamenerweiterung.bmDateinamenerweiterungBitmaps: Suche nach DateienPixmaps: Suche nach DateienKonventionen: Benennung der AbbilddateiAbbilddatei: Namenskonventionen

Jedes Symbol und jedes Hintergrundabbild wird als separate Datei
gespeichert.
Normalerweise wird ein Symbol nur mit dem Basisteil des Dateinamens
angegeben. Beispielsweise ist es möglich, auf ein Symbol nur mit dem Namenmailzu verweisen, wenn die Datei tatsächlich unter folgendem Namen
gespeichert ist:/usr/dt/appconfig/icons/`language`/mail.l.pm

Die Dateinamenskonvention zum Hinzufügen von Suffixen ist hilfreich bei
der Gruppierung von Symbolen nach Größe und Typ. Bei Desktop-Komponenten
sind viele Symbolnamen in den folgenden allgemeinen Formaten:`Basisname`.`Größe`.`Format``Basisname`.`Format`

Dabei ist`Basisname`der Abbildname, mit dem auf das Abbild verwiesen wird,
ist`Größe`ein einzelner Buchstabe, mit dem die Symbolgröße angegeben wird,
und steht`Format`pmfür X-Pixmaps undbmfür X-Bitmaps.

Folgende Symbolgrößen sind gültig:

* **Name** 

Größe Suffix
* **sehr klein (tiny)** 

16 &times; 16,`t`
* **klein (small)** 

24 &times; 24,`s`
* **mittel (medium)** 

32 &times; 32,`m`
* **groß (large)** 

48 &times; 48,`l`


Angenommen, der Benutzer gibt ein Symbol mit dem Namen "mail" für
den Typ einer Datei an, die er geschrieben hat. Steht ein Farbbildschirm
zur Verfügung und wurden die Profilwerte des Dateimanagers auf die Verwendung
"kleiner" Symbole gesetzt, ist der angenommene Dateiname "mail.s.pm"
("s" steht für klein (small) und "pm" fürPixmap, das Format für
farbige Symbole).
# Suchpfade


Das Verzeichnis, in dem ein Abbild gespeichert ist, wird bestimmt, indem in
einer Liste von Verzeichnissen nach der Datei gesucht wird. Die Liste von
Verzeichissen ist durch einen "Suchpfad" für Symbole definiert.

Wird ein Farbbildschirm verwendet, sucht das Desktop zunächst nach Dateien,
die mit.pmenden. Andernfalls sucht es nach Dateien, die mit.bmenden.

Ist ein Abbild nicht mit einem vollständigen Pfadnamen angegeben, wird der
Suchpfad nicht verwendet.
# Siehe auch


listet die Spezifikationen für Suchpfade auf.
# Speichern von Symboldateien
Speichern von SymboldateienSymboldatei: speichern

Standardmäßig suchen die Desktop-Komponenten in den folgenden Verzeichnissen
nach Symboldateien:

Persönliche Symbole:/`Standardverzeichnis`/.dt/icons

Systemweite Symbole:/etc/dt/appconfig/icons/`Sprache`

Integrierte Symbole:/usr/dt/appconfig/icons/`Sprache`

Für englische SymboleCals`Sprache`verwenden.
# Speichern von Hintergrundabbilddateien
Hintergrund: Speichern von HintergrundabbilddateienAbbilddatei: Hintergrund speichern

Jedes Hintergrundmuster, das im Dialogfenster 'Umgebungsverwaltung - Hintergrund'
aufgelistet ist, stellt eine Abbilddatei dar. Standardmäßig sucht die
Umgebungsverwaltung in den folgenden Verzeichnissen nach Hintergrundmustern:

Systemweite Abbilder:/etc/dt/appconfig/backdrops/`Sprache`

Integrierte Abbilder:/usr/dt/backdrops/`Sprache`

Wird ein neues Hintergrundabbild erstellt, auf das alle Systembenutzer Zugriff
haben sollen, das Hintergrundabbild in das Verzeichnis/etc/dt/appconfig/backdrops/`Sprache`stellen.
Um den Zugriff auf ein Hintergrundabbild auf bestimmte Benutzer zu beschränken,
das Hintergrundabbild in ein anderes Verzeichnis stellen und dieses Verzeichnis
zur Ressource*backdropDirectoriesfür die entsprechenden Benutzer
hinzufügen.