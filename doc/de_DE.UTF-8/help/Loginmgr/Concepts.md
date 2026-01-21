
# Anmeldemanager - Konzepte





# Einführung in Desktop-Sessions
Starten: Desktop-SessionAnmelden an der Desktop-SessionDesktop: Session startenSession: Desktop startenVerwendung: StandardsitzungStandardsitzung: DefinitionAktuelle Session: Definition

Eine Session ist die Gesamtheit von Anwendungen, Einstellungen und Ressourcen,
die auf dem Desktop vorhanden sind. Eine Desktop-Session findet in dem
Zeitraum zwischen dem Anmelden und dem Abmelden statt. Das Anmeldefenster, das
vom Anmeldemanager erstellt wird, ermöglicht den Zugang zum Desktop.
In ihm werden die Benutzer-ID und das Kennwort eingegeben.

Das Menü 'Optionen' im Anmeldefenster listet die verfügbaren Optionen auf.
Neben der Ausführung einer normalen Desktop-Session kann auch die Ausführung
einer einfachen Motif-Session ausgewählt werden. Die Sprache für die Session
kann ebenfalls ausgewählt werden.

Nachdem der Anmeldemanager die Benutzer-ID und das Kennwort
überprüft hat, wird der Session-Manager gestartet. Die Session-Verwaltung
besteht aus einer Gruppe von Konventionen und Protokollen, die einem
spezifischen Session-Manager, wie beispielsweise dem Session-Manager des
Common Desktop Environment gestatten, die Session zu sichern und
wiederherzustellen. Der Benutzer kann sich so am System anmelden und dieselbe
Kombination von aktiven Anwendungen, Einstellungen und Ressourcen vorfinden,
die bei der Abmeldung vorhanden waren.
Folgendes wird vom Session-Manager gesichert und wiederhergestellt:

Einstellungen für Aussehen und Verhalten&emdash;beispielsweise
Schriftarten, Farben und Mauseinstellungen.

Die Fensteranwendungen, die aktiv waren&emdash;beispielsweise
die Fenster des Dateimanagers und des Texteditors. Bestimmte Arten von
Anwendungen können vom Session-Manager nicht gesichert und wiederhergestellt
werden. Wird beispielsweise der Editorviaus einer
Befehlszeile in einem Terminal-Fenster gestartet, kann der Session-Manager
die Editier-Session nicht wiederherstellen.

Meldet sich der Benutzer zum ersten Mal am Desktop an, wird eine
Standardstartsession geladen. Danach unterstützt der Session-Manager
das Konzept einer aktuellen und einer Standardsitzung.
# Startsession


Meldet sich der Benutzer zum ersten Mal am Desktop an, wird vom
Session-Manager eine Startsession erstellt, für die Systemstandardwerte
verwendet werden. Als Standardvorgabe werden der Dateimanager und eine
Einführung in CDE (Common Desktop Environment) automatisch gestartet.
# Aktuelle Session


Normalerweise sichert das Desktop bei der Abmeldung Session-Informationen
und verwendet diese Informationen, um die nächste Session zu starten. Werden
während der Session Anwendungen gestartet oder beendet oder wird
mit dem Umgebungsmanager das Aussehen oder das Verhalten des Systems verändert,
werden diese Änderungen in der nächsten Session berücksichtigt.

Die momentan ausgeführte Session wird immer alsaktuelle Sessionbetrachtet, unabhängig davon, ob sie bei der Anmeldung aus einer gesicherten
Standardsitzung, einer gesicherten aktuellen Session oder der Standardstartsession
des Systems wiederhergestellt wurde. Je nach den Einstellungen im Dialogfenster
'Neustart' des Umgebungs-Managers, sichert der Session-Manager die aktuelle
Session automatisch, wenn die Session beendet wird. Bei der nächsten
Anmeldung am Desktop wird die zuvor gesicherte aktuelle Session
wiederhergestellt. Dies bedeutet, daß der Status des Desktops so
wiederhergestellt wird, wie er bei der letzten Abmeldung war.
# Standardsitzung


Das Desktop stellt außerdem eineStandardsitzungzur Verfügung.
Eine Standardsitzung ist eine Session, die explizit gesichert wird. Sie ist
wie eine Momentaufnahme der aktuellen Session zu einem bestimmten Zeitpunkt.
Wurde eine Standardsitzung gesichert, kann der Benutzer angeben, daß das
Desktop bei der nächsten Anmeldung statt der aktuellen Session diese
Session wiederherstellen soll.

Weitere Informationen befinden sich in:






# Weitere Arten der Anmeldung
Session: andere als die normale SessionStart: einfache Motif-SessionStart: Befehlszeilenanmeldung, SessionEinfache Motif-Session startenBefehlszeilenanmeldung: Session starten

Außer der normalen Desktop-Session gibt es folgende weitere Arten von Sessions:

* **Einfache Motif-Session** 

Eine einfache Motif-Session stellt ein
Terminal-Fenster und den Fenstermanager zur Verfügung. Sie ist
hilfreich, wenn Befehle vor der Anmeldung an der nächsten
Desktop-Session ausgeführt werden sollen.
(Siehe.)
* **Befehlszeilenanmeldung** 

Die Befehlszeilenanmeldung ermöglicht das
zeitweilige Verlassen des Desktops, um auf der Systemkonsole zu
arbeiten. (Siehe.)
