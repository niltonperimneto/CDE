
# Hilfemanagerfenster mit Gruppen zusammengehöriger Hilfethemen


Eine Gruppe zusammengehöriger Hilfethemen umfaßt eine oder mehrere
Hilfebände. Der Benutzer kann einzelne Hilfebände vom Hilfemanager
aus aufrufen und anzeigen.

Das Hilfemanagerfenster enthält die zusätzliche Navigationstaste
'Oberste Ebene'. Nachdem der Benutzer verschiedene Hilfebände aufgerufen hat,
kann er mit dieser Navigationstaste wieder zur Hauptanzeige des
Hilfemanagers zurückkehren.
# Siehe auch:



# Suche nach regulären Ausdrücken


* **Zeichen** 

Bedeutung
* **&sigspace;. (Punkt)** 

Entspricht einem beliebigen Zeichen
* **&sigspace;* (Stern)** 

Entspricht keinem oder einer beliebigen Anzahl des
vorhergehenden Zeichens
* **&sigspace;? (Fragezeichen)&sigspace;** 

Entspricht keinem oder einem
des vorhergehenden Zeichens
* **&sigspace;| (Senkrechter Strich)** 

Gibt zwei Suchmuster an und entspricht
einem der beiden Muster (logisches ODER)
* **&sigspace;() (Klammern)** 

Schließt einen Musterausdruck ein


Um mit einem regulären Ausdruck nach einem Zeichen zu suchen, das
eine Sonderbedeutung hat, dem betreffenden Zeichen einen umgekehrten
Schrägstrich &newline; voranstellen.

&sigspace;
# Beispiele


Mit dem folgenden Ausdruck werden Indexeinträge gesucht, die das Wort
'Maus', dahinter eine beliebige Anzahl Zeichen, gefolgt von dem
Wort 'klicken' enthalten.Maus.*klicken

Mit diesem Ausdruck werden Indexeinträge gesucht, die das Wort 'Maus'
oder 'Klicken' enthalten.Maus | klicken

Mit diesem Beispiel werden Indexeinträge gesucht, die die Wörter
'Session-Manager' oder 'Umgebungsmanager' entalten.(Session | Umgebungs).*anager
# Siehe auch:




Weitere Informationen zu regulären Ausdrücken sind auf der elektronischen
Handbuchseiteregexp(5)enthalten.