Dateimanager - KonzepteDie folgenden Abschnitte enthalten Informationen zur genaueren Kenntnis
des Dateimanagers.Hierarchisches DateisystemHierarchisches DateisystemDateisystem, hierarchischDefinition: DateiFür Benutzer, die mit Computern nicht vertraut sind, ist das Konzept
eines hierarchischen Dateisystems möglicherweise neu. Die folgenden Abschnitte
enthalten eine Beschreibung der grundlegenden Komponenten eines hierarchischen
Dateisystems.Was ist eine Datei?EineDateiist ein Behälter mit Informationen.
Die meisten Dateien enthalten Informationen (Daten) eines bestimmten Formats&emdash;ein
Dokument, ein Arbeitsblatt oder eine Grafik. Das Format ist die Art und
Weise, in der die Daten innerhalb der Datei angeordnet sind. Das Format
einer Datei wird auch als der Datentyp der Datei bezeichnet.Befindet sich der Dateimanager in einem der Modi für die Symbolanzeige,
ist der Datentyp einer Datei am Symbol, das die Datei darstellt, zu erkennen.
Für jeden Datentyp gibt es ein bestimmtes Symbol.Die meisten Anwendungsprogramme können nur eine begrenzte Anzahl
von
Datentypen verarbeiten. So kann ein Dokumenteditor beispielsweise
in der Regel keine Arbeitsblattdatei lesen. Das Desktop
unterstützt den Benutzer mit Hilfe einer Datenbank für
"data types"Datentypen dabei, verschiedene Dateitypen zu
unterscheiden. Durch den Datentyp werden Dateien mit einem bestimmten
Format identifiziert und den entsprechenden Anwendungen zugeordnet.
Wird auf eine Datei doppelt geklickt, startet das Desktop in den meisten
Fällen automatisch die Anwendung, die den Datentyp dieser Datei verarbeiten
kann.Die maximal zulässige Größe eines Dateinamens ist von
System zu System
unterschiedlich. Bei manchen Betriebssystemen dürfen Dateinamen maximal
14 Zeichen lang sein. Falls erforderlich, den Systemadministrator
informieren.Was ist ein Ordner?Definition: OrdnerEinOrdnerist ein Behälter für
Dateien, ähnlich einem Ordner in einem
Aktenschrank. Aus diesem Grund verwendet der Dateimanager einen Aktenordner
als Symbol für einen Ordner. Ein Ordner kann weitere Ordner enthalten;
diese werden auch alsUnterordnerbezeichnet. Mit Ordnern
und Unterordnern
können mehrere hierarchisch angeordnete Organisationsebenen erstellt
werden.
(In anderem Zusammenhang werden Ordner häufig auch als Verzeichnisse
bezeichnet.)Würde die Ordnerhierarchie so aufgezeichnet, daß jeder Unterordner
unterhalb des Ordners steht, in dem er enthalten ist (dem
übergeordneten Ordner) und jeder Ordner mit dem jeweils übergeordneten
Ordner durch eine Linie verbunden ist, würde ein umgekehrter Baum entstehen.
Aus diesem Grund wird eine Ordnerhierarchie auch alsOrdnerbaumbezeichnet.Innerhalb eines einzelnen Ordners muß jede Datei einen eindeutigen
Namen
haben. Dateien in unterschiedlichen Ordnern können den gleichen Namen
haben.Beim Navigieren von Ordner zu Ordner wird die aktuelle Position alsaktueller Ordnerbezeichnet.Was ist ein Pfad?Definition: PfadPfad: DefinitionDie Position einer Datei wird oft dadurch angegeben, daß
die Namen der Ordner
und Unterordner aufgelistet werden, über die die Datei aufgerufen wird.
Diese Liste wird alsPfadbezeichnet (siehe).
Der Pfad einer Datei wird im Dateimanager an zwei Stellen angezeigt:
zum einen im grafischen Pfad als Abfolge von Ordnersymbolen,
zum anderen in Textform in der Pfadzeile oberhalb des Anzeigebereichs.Pfade und PfadnamenDer Pfad eines Objekts gibt an, an welcher Position im Dateisystem
sich das Objekt befindet. Er kann auf drei Arten angegeben werden:
als absoluter Pfad, als relativer Pfad und als vollständig qualifizierter
Pfad.Absolute PfadeEinabsoluter Pfadbeginnt beimStammordner. Der Stammordner ist
der oberste Ordner des hierarchischen Ordnerbaums. Beginnt ein Pfad mit
einem Schrägstrich (/), handelt es sich
um einen absoluten Pfad, der
vom Stammordner aus angegeben wird. So ist beispielsweise der folgende Pfad
der absolute Pfad für die DateiBrief:/usr/dt/config/BriefRelative PfadeEinrelativer Pfadgibt die Position einer
Datei oder eines Ordners
relativ zum aktuellen Ordner an. Befindet sich der Benutzer in einem Ordner
und möchte innerhalb des Ordnerbaums auf eine untergeordnete Ebene
wechseln, muß nicht der volle Pfadname eingegeben werden. Es genügt,
wenn
der Teil des Pfades eingegeben wird, der mit dem Namen des nächsten Ordners
im Pfad beginnt. Ein Pfad, der nicht mit einem Schrägstrich beginnt,
ist
ein relativer Pfad. Ist der aktuelle Ordner beispielsweise/usr/dt, und soll ''/usr/dt/config/Briefe''
zum aktuellen Ordner werden,
lautet der zu verwendende relative Pfad:config/Briefe.. (übergeordneter Ordner)Bei der Angabe von relativen Pfaden sind zwei spezielle Ordnernamen
von Nutzen. Der Ordner.(durch einen Punkt
dargestellt) steht für den
aktuellen Ordner. Der Ordner..(durch zwei
Punkte dargestellt) steht
für denübergeordnetenOrdner &emdash;dieser
befindet sich in
der Ordnerhierarchie auf der nächsthöheren Ebene. Ist der aktuelle
Ordner
beispielsweise/usr/dt/config/panels, lautet
der relative Pfad für
die Dateisys.dtwmrcwie folgt:../sys.dtwmrcErklärung: Diese Datei befindet sich im Ordner/usr/dt/config, d. h.
eine Ebene über dem aktuellen Ordner.Siehe auchEigentumsrechte für Objekte / SicherheitDrei Gruppen von Benutzern können auf Objekte zugreifen:Eigentümer,GruppeundAlle. Der Zugriff wird in drei Funktionen unterteilt:Leseberechtigung,SchreibberechtigungundAusführungsberechtigung.Wer hat Zugriff?Die drei Hauptbenutzerklassen sind:EigentümerGewöhnlich die Person, die die Datei erstellt hat.GruppeMehrere Benutzer, die vom Systemadministrator in einer Gruppe zusammengefaßt
wurden. So könnten beispielsweise die Mitarbeiter einer Abteilung derselben
Gruppe angehören.AlleAlle anderen Benutzer auf dem System.Art des ZugriffsDurch das Zugriffsrecht für eine Datei wird angegeben, auf
welche Weise der Eigentümer, Mitglieder der Gruppe und andere Benutzer
auf diese Datei zugreifen können.LeseberechtigungErmöglicht den Zugriff zum Kopieren oder Lesen des Inhalts eines
Objektes.SchreibberechtigungErmöglicht den Zugriff zum Ändern des Objektinhalts sowie
zum Entfernen des Objekts.AusführungsberechtigungDatei: Ermöglicht den Zugriff zumAusführender Datei (ausführbare Dateien, Prozeduren und Aktionen); Ordner: Ermöglicht
den Zugriff zum Ausführen von Befehlen, Prozeduren und Aktionen innerhalb
dieses Ordners.Mit dem Dateimanager ist das Anzeigen und Ändern der Zugriffsrechte
für Dateien und Ordner möglich. Siehe auchund.BeispieleUm aus einem Ordner einen privaten Ordner zu machen, wie folgt
vorgehen:Die Eigenschaften eines Ordners folgendermaßen
ändern: Der Eigentümer
erteilt lediglich sich selbst Lese-, Schreib- und Ausführungsberechtigung,
jedoch der Gruppe und den anderen Benutzern keine Berechtigungen.
Das bedeutet, daß nur der Eigentümer selbst und der Benutzer 'root'
den
Inhalt des Ordners anzeigen können.Um ein erstelltes Objekt für die Verwendung durch alle Benutzer
zur Verfügung
zu stellen, es jedoch vor versehentlichem Überschreiben zu schützen,
wie
folgt vorgehen:Die Eigenschaften der Datei so ändern,
daß Eigentümer, Gruppe und
andere Benutzer Lese- und Ausführungsberechtigung erhalten. Keine
Schreibberechtigung erteilen.StandardzugriffsrechteDie Standardzugriffsrechte, die beim Erstellen einer neuen Datei
oder eines
neuen Ordners verwendet werden, können vom Systemadministrator geändert
werden. Um festzustellen, wie die aktuellen Standardwerte lauten, eine neue
Datei bzw. einen neuen Ordner erstellen und anschließend 'Zugriffsrechte
ändern' im Menü 'Ausgewählt' auswählen, um die Standardzugriffsrechte
anzuzeigen.Vereinfachen des Zugriffs auf Objekte - Einführung von ArbeitsbereichsobjektenArbeitsbereichsobjekteArbeitsbereich: ObjekteDer Dateimanager bietet die Möglichkeit, alle Objekte des Dateisystems
anzuzeigen. Das Objekt wird jedoch nur dann angezeigt, wenn auch der
Ordner, in dem es enthalten ist, angezeigt wird.Um den Zugriff auf das Objekt zu erleichtern, kann es direkt auf das
Hintergrundmuster des Arbeitsbereichs gestellt werden.
Der Arbeitsbereich ist der Teil der Anzeigenoberfläche, in den das Fenster
gestellt wird. (Siehe auch.)
Wird ein
Objekt in diesen Bereich gestellt, wird es alsArbeitsbereichsobjektbezeichnet.Wird ein Objekt in den Arbeitsbereich gestellt, ändert sich die
ursprüngliche
Datei bzw. der ursprüngliche Ordner nicht. Tatsächlich ist das Symbol,
das
auf dem Desktop angezeigt wird, nur eine verkürzte Zugriffsmöglichkeit
(ein Verweis) auf die eigentliche Datei bzw. den eigentlichen Ordner. Alle
Operationen, die für das Arbeitsbereichsobjekt ausgeführt werden,
werden
eigentlich für die Datei oder den Ordner ausgeführt, für die
bzw. den es
steht.Arbeitsbereichsobjekte für einen einzigen ArbeitsbereichWird ein Objekt in den Arbeitsbereich gestellt, wird es nur im
aktuellen
Arbeitsbereich angezeigt. Soll das Objekt auch in anderen Arbeitsbereichen
angezeigt werden, muß zu diesen Arbeitsbereichen umgeschaltet und das
Objekt
hineingestellt werden.Verwendung von ArbeitsbereichsobjektenArbeitsbereichsobjekte werden auf die gleiche Weise verwendet
wie die Objekte
innerhalb des Dateimanager- oder Anwendungsmanagerfensters. Um die
Standardaktion für ein Objekt auszuführen, doppelt auf das zugehörige
Symbol auf dem Desktop klicken.Jedes Arbeitsbereichsobjekt verfügt darüber hinaus über
ein temporär
eingeblendetes Menü, das Befehle und Aktionen für das Objekt enthält.
Um das
Menü für ein Arbeitsbereichsobjekt mit Hilfe der Maus anzuzeigen,
den
Mauszeiger auf das Symbol stellen und anschließend die Maustaste 3 drücken
und gedrückt halten. Um das Menü mit Hilfe der Tastatur anzuzeigen,
die Tastenkombination Alt+Tabulatortaste drücken, bis das Symbol hervorgehoben
ist, und anschließend die Tastenkombination Umschalttaste+F10 drücken.Platzhalterzeichen für die Suche nach DateienPlatzhalterzeichen, ÜbereinstimmungÜbereinstimmung, PlatzhalterzeichenWird ein Datei- oder Ordnername angegeben, können Platzhalterzeichen,
wie beispielsweise ein Stern (*) oder ein
Fragezeichen (?) verwendet
werden.*steht für eine beliebige Zeichenfolge
aus null oder mehreren
Zeichen;?steht für ein beliebiges
einzelnes Zeichen.Beispieleba*Entspricht allen Namen, die mit der Zeichenfolgebabeginnen.ba?Entspricht allen dreistelligen Namen, die mit der Zeichenfolgebabeginnen.*.vfEntspricht allen Namen, die mit der Erweiterung.vfenden.*.???Entspricht allen Namen, die über eine dreistellige, durch einen
Punkt
abgetrennte Erweiterung verfügen.Für die Angabe von Dateiname und Inhalt kann dieselbe Syntax für
reguläre
Ausdrücke verwendet werden, die auch für den Befehlfindzulässig ist.
(Die elektronische Handbuchseitefind (1)enthält weitere Informationen
hierzu.)Verwendung des Dateimanagers für die Anzeige in SymbolenDateien, deren Name auf.pmoder.bmendet, enthalten
Symbolzeichnungen. Diese verwendet der Dateimanager für die Symbolerstellung.
Standardmäßig müssen diese Dateien geöffnet werden, um
die darin
enthaltenen Zeichnungen anzeigen zu können. Wird die Anzeige in Symbolen
aktiviert, zeigt der Dateimanager für jede dieser Dateien das Symbol
an, dessen
Symbolzeichnung in der entsprechenden Datei gespeichert ist.Informationen zur Rekonfiguration des Dateimanagers
für die Anzeige in Symbolen sind in den folgenden Abschnitten enthalten: