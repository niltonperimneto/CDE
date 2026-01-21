
# Post - Meldungen


In diesem Abschnitt werden die möglichen Gründe für Fehlermeldungen
der Anwendung 'Post' und deren Lösungen beschrieben.FehlermeldungenMeldungen, Fehler
# Post hat Probleme, den Status dieses Briefkastens
festzustellen. Den Briefkasten durch Auswahl von OK schließen
und das Programm 'Post' neu starten.

Der Briefkasten befindet sich in einem Status, der dem System nicht bekannt
ist. Der Grund dafür könnte sein, daß der Benutzer in einem anderen
Exemplar der Anwendung 'Post' eine Speicheroperation durchgeführt hat,
während dieses Exemplar ausgeführt wurde.
# Mögliche Lösung:


Versuchen, die Anwendung 'Post' zu schließen und erneut zu starten.

Funktioniert diese Lösung nicht, den Systemadministrator
benachrichtigen.
# Anlagenameist eine ausführbare Anlage.
Soll sie ausgeführt werden?

Bei der ausgewählten Anlage handelt es sich um eine ausführbare Datei.
Der Benutzer kann die ausführbare Datei entweder ausführen oder die
Operation abbrechen.
# Mögliche Lösung:


Hat der Benutzer nicht erwartet, die ausführbare Datei auszuführen,
kann er die Anlage in einer Datei speichern und zu einem späteren
Zeitpunkt ausführen.
# Die Weiterleitungsfunktion wird bereits für
eine andere Funktion als 'Abwesenheit' benutzt. Die Funktion
'Abwesenheit' wird beim Ausführen an diese andere
Weiterleitungsaktivität angehängt. Soll die Funktion
'Abwesenheit' trotzdem gestartet werden?

Ist die Option 'Abwesenheit' in der Anwendung 'Post' aktiviert, verwendet
sie die Weiterleitungsfunktion, um die Nachrichten zu speichern und
den Absendern zu antworten. Verwendet der Benutzer diese
Weiterleitungsfunktion zu anderen Zwecken, z. B. um seine Post an einen
anderen Benutzereintrag weiterzuleiten, wird die Option 'Abwesenheit' an
die Datei.forwardangehängt und beide Funktionen sind wirksam.
# Mögliche Lösung:


Die Weiterleitungsfunktion in der Datei.forwardlöschen, wenn
nur die Option 'Abwesenheit' wirksam sein soll.

Die Option 'Abwesenheit' inaktivieren, wenn nur die
Weiterleitungsfunktion wirksam sein soll.
# Das Programm 'Abwesenheit' wird bereits in
der Datei.forwardausfgeführt. In der Dokumentation
kann nachgelesen werden, wie das Programm beendet und
aus der Datei.forwardentfernt werden kann.
Nach dem Beheben dieses Problems diesen Befehl versuchen.

Der Benutzer versuchte, die Funktion 'Abwesenheit' zu starten, während
sie bereits ausgeführt wurde.
# Mögliche Lösung:


Nur ein Exemplar der Funktion 'Abwesenheit' ausführen.
# Die Datei.vacation.msgkann nicht geöffnet
werden -- fehlende Schreibberechtigung.


Die Datei, die den Abwesenheitshinweis des Benutzers enthält und
die normalerweise im Standardverzeichnis gespeichert ist, ist
momentan nur im Lesezugriff.
# Mögliche Lösung:


Die Dateiberechtigungen ändern, so daß Schreibberechtigung vorliegt.
Danach versuchen, einen Abwesenheitshinweis zu erstellen und zu speichern.
# Die Datei.vacation.msgist bereits vorhanden.
Soll sie durch den neuen Text ersetzt werden?


In der Datei.vacation.msgsteht momentan ein Abwesenheitshinweis.
Soll dieser überschrieben werden?
# Mögliche Lösung:


Der Benutzer kann den bestehenden Text mit der neuen Meldung
überschreiben, oder er kann den Vorgang abbrechen und somit den
bestehenden Abwesenheitshinweis speichern.
# Es kann kein Erstellungsfenster erzeugt werden.


Der Benutzer hat 'Neue Nachricht' aus dem Menü 'Erstellen' ausgewählt,
die Anwendung 'Post' war aber nicht in der Lage, ein Fenster 'Neue
Nachricht' zu öffnen.
# Mögliche Lösung:


Den Versuch wiederholen und gegebenenfalls die Anwendung 'Post'
erneut starten.
# Die Schablone ist nicht vorhanden.


Die vom Benutzer ausgewählte Schablone befindet sich nicht an
der Position, die im Dialogfenster 'Postoptionen - Schablonen'
angegeben wurde.
# Mögliche Lösung:


Sicherstellen, daß im Dialogfenster 'Postoptionen - Schablone'
die korrekte Position (vollständiger Pfadname) der Schablone angegeben ist.
# Die Schablone ist offensichtlich beschädigt.


Die Schablonendatei befindet sich an der angegebenen Position, sie ist aber
nicht funktionsfähig.
# Mögliche Lösung:


Sicherstellen, daß die Schablonendatei nicht beschädigt ist. Die
Datei, falls möglich, durch eine Sicherungsdatei ersetzen und erneut
versuchen, die Schablone einzufügen.
# Es steht nicht genügend Speicher zum Laden
der Schablone zur Verfügung.

Die Schablone, die der Benutzer in der Anwendung 'Post' verwenden will,
ist größer als der zur Verfügung stehende Speicherplatz.
# Mögliche Lösung:


Einige Anwendungen auf dem Desktop schließen und versuchen, die
Schablone erneut zu laden.
# ToolTalk ist nicht initialisiert. Das Postprogramm
kann nicht ohne ToolTals ausgeführt werden.
/usr/dt/bin/dtsession starten oder den Systemadministrator fragen.

Während versucht wurde, eine Funktion auszuführen, konnte
ToolTalk nicht initialisiert werden.
# Mögliche Lösung:


Zusammen mit dem Systemadministrator überprüfen, ob das System
korrekt mit ToolTalk arbeitet.
# Post wurde nicht ordnungsgemäß installiert und
kann nicht ausgeführt werden, da die Ausführungsgruppe falsch
gesetzt ist.

Auf einigen Systemen muß zum Ausführen der Anwendung 'Post' die Gruppen-ID
(Ausführungsgruppe) auf "mail" gesetzt sein. Die Ausführungsgruppe
des Benutzers ist nicht so gesetzt, daß die Anwendung 'Post' ausgeführt
werden kann.
# Mögliche Lösung:


Zusammen mit dem Systemadministrator prüfen, ob die
Gruppenberechtigungen in der ausführbaren Datei der Anwendung 'Post'
korrekt gesetzt sind.
# Fehlende Berechtigung zum Anzeigen vonDateiname.


Der Benutzer verfügt nicht über die Berechtigung, den angegebenen Briefkasten
anzuzeigen. Die Dateiberechtigungen sind möglicherweise so gesetzt, daß
der Benutzer keine Leseberechtigung hat.
# Mögliche Lösung:


Ist der Benutzer auch Eigentümer der Datei, die Berechtigungen so
ändern, daß die Datei gelesen werden kann.

Ist der Benutzer nicht der Eigentümer der Datei, den Dateieigentümer
bitten, die Berechtigungen so zu ändern, daß die Datei gelesen werden kann.
# Der Briefkasten ist ein Verzeichnis und kann nicht
geöffnet werden.

Bei dem ausgewählten Briefkasten handelt es sich um ein Verzeichnis
(einen Ordner); er kann nicht geöffnet werden.
# Mögliche Lösung:


Einen Briefkasten angeben, bei dem es sich um eine Datei handelt.
# Es muß eine Anlage ausgewählt werden, bevor eine Datei
mit dem Befehl 'Speichern als' gespeichert werden kann.

Der Benutzer hat die Option 'Speichern als' ausgewählt, ohne vorher
eine Anlage ausgewählt zu haben.
# Mögliche Lösung:


Zunächst eine Anlage und danach 'Speichern als' auswählen.
# Der Briefkasten ist gesperrt. Die Sperre manuell
aufheben und erneut versuchen oder den Systemadministrator verständigen.

Möglicherweise greift ein anderer Benutzer momentan auf diesen Briefkasten
zu, oder der Benutzer hat diesen Briefkasten an anderer Stelle bereits
geöffnet.
# Mögliche Lösung:


Möchte der Benutzer noch immer auf diesen Briefkasten zugreifen,
'OK' auswählen, so daß die Anwendung 'Post' anfordert, die Sperre für
diesen Briefkasten aufzuheben. Ist die Anforderung erfolgreich, verfügt
der Benutzer über den Zugriff auf diesen Briefkasten.

'Abbrechen' auswählen, um den Versuch, diesen Briefkasten zu öffnen,
zu beenden.
# Sollen die in diesem Briefkasten zum Löschen
markierten Nachrichten vernichtet werden?

Liegen beim Versuch, einen Briefkasten zu schließen, gelöschte Nachrichten
vor, wird der Benutzer gefragt, ob er die zum Löschen markierten
Nachrichten vernichten will.
# Mögliche Lösung:


Auf 'Abbrechen' klicken, damit die Nachrichten nicht für immer
gelöscht werden.

Die Anwendung 'Post' wird geschlossen, nachdem die Änderungen gespeichert
wurden.
# Dateinamekann nicht überschrieben
werden. Dateiberechtigungen überprüfen und erneut versuchen.

In die ausgewählte Datei kann nicht geschrieben werden.
# Mögliche Lösung:


Die Dateiberechtigungen überprüfen und gegebenenfalls ändern,
bevor die Operation erneut versucht wird.
# Anlagennamekann nicht geöffnet werden.


Die angegebene Anlage kann nicht geöffnet werden. Möglicherweise
verfügt der Benutzer nicht über Leseberechtigung für die Datei.
# Mögliche Lösung:


Die Berechtigungen für die Anlagedatei überprüfen und gegebenenfalls
ändern, bevor die Operation erneut versucht wird.
# Es kann kein Speicher zugeordnet werden.


Für die ausgewählte Operation steht nicht genügend Speicherplatz zur
Verfügung.
# Mögliche Lösung:


Einige Anwendungen schließen und den Versuch wiederholen.
# Dateinameist bereits vorhanden. Ersetzen?


Der Benutzer hat versucht, eine Anlage unter einem Dateinamen zu speichern,
der in seinem Dateisystem bereits vorhanden ist.
# Mögliche Lösung:


Soll die vorhandene Datei nicht überschrieben werden, den Vorgang
abbrechen und einen anderen Dateinamen eingeben.
# Dateinamekann nicht ersetzt werden.


Die Datei, deren Namen angegeben wurde, kann nicht überschrieben werden.
Ein möglicher Grund ist, daß in das Verzeichnis nicht geschrieben werden
kann oder daß der Benutzer versucht, in einem Dateisystem mit Lesezugriff
zu schreiben.
# Mögliche Lösung:


Die Verzeichnisberechtigungen überprüfen und, falls möglich,
Schreibberechtigung hinzufügen.

Einen anderen Standort (mit Schreibzugriff) angeben, an dem
die Nachricht gespeichert werden soll.
# Dateinamekann nicht erstellt werden.


Der angegebene Briefkasten kann nicht erstellt werden.
Ein möglicher Grund ist, daß in das Verzeichnis nicht geschrieben werden
kann oder daß der Benutzer versucht, in einem Dateisystem mit Lesezugriff
zu schreiben.
# Mögliche Lösung:


Die Verzeichnisberechtigungen überprüfen und, falls möglich,
Schreibberechtigung hinzufügen.

Einen anderen Standort (mit Schreibzugriff) angeben, an dem
die Nachricht gespeichert werden soll.
# Der Name ist bereits vorhanden. Überschreiben?


Der Benutzer hat eine Anlage angegeben, die bereits in seinem
Dateisystem vorhanden ist.
# Mögliche Lösung:


Soll die vorhandene Anlage nicht überschrieben werden, den Vorgang
abbrechen und einen anderen Namen für die Anlage eingeben.
# Dateinameist bereits vorhanden. Überschreiben?


Der Benutzer hat einen Briefkasten angegeben, der bereits
in seinem Dateisystem vorhanden ist.
# Mögliche Lösung:


Soll der vorhandene Briefkasten nicht überschrieben werden, den Vorgang
abbrechen und einen anderen Namen für den Briefkasten eingeben.
# Dateinamekann nicht geschrieben werden.


Der Benutzer hat versucht, eine Operation 'Als Text speichern' auszuführen,
verfügt aber nicht über Schreibberechtigung für die Datei.
# Mögliche Lösung:


Die Dateiberechtigungen überprüfen und, falls möglich,
Schreibberechtigung hinzufügen.

Einen anderen Standort (mit Schreibzugriff) angeben, an dem
die Datei gespeichert werden soll.
# Das Erstellungsfenster enthält Text oder
Anlagen, die beim Schließen des Fensters verlorengehen.
Soll das Erstellungsfenster geschlossen werden?

Der Benutzer hat versucht, das Erstellungsfenster zu schließen, während es
Text enthielt.
# Mögliche Lösung:


Auf 'OK' klicken, um das Erstellungsfenster zu schließen. Dabei wird
der eingegebene Text nicht gespeichert.

'Abbrechen' auswählen, um zum Erstellungsfenster zurückzukehren.
# Es steht nicht genügend Speicher zum Laden der
vorhandenen Datei.vacation.msgzur Verfügung.

Die aktuelle Datei.vacation.msgist zu groß.
# Mögliche Lösung:


Einige Anwendungen schließen und erneut versuchen, die Datei zu laden.
# Die vorhandene Datei.vacation.msgist offensichtlich
beschädigt.

Die Datei befindet sich an der korrekten Position, sie ist aber
nicht funktionsfähig.
# Mögliche Lösung:


Die Datei (normalerweise im Standardverzeichnis gespeichert) überprüfen,
um sicherzustellen, daß sie nicht beschädigt ist. Ist sie beschädigt, durch
eine neue Datei ersetzen.
# You have an attachment open that may have
unsaved changes. Sending this message will break the
connection to the open attachment. Any unsaved changes
will not be part of the message. You can use Save As to
save changes after the connection is broken, but the changes
will not be part of the attachment.

Der Benutzer hat versucht, eine Nachricht zu senden, ohne vorher Änderungen
an einigen geöffneten Anlagen zu speichern. Durch diese Warnung wird der
Benutzer darauf hingewiesen und gefragt, ob dies beabsichtigt ist.
# Mögliche Lösung:


Die Änderungen an den Anlagen speichern, bevor die Nachricht gesendet
wird.
# Nur eine Anlage auswählen und 'Umbenennen' auswählen.


Der Benutzer hat 'Umbenennen' ausgewählt, während mehr als eine Anlage
ausgewählt war.
# Mögliche Lösung:


Nur eine Anlage auswählen, bevor 'Umbenennen' ausgewählt wird.
# Einige der Adressen in der Nachricht sind nicht korrekt
und beziehen sich nicht auf bekannte Benutzer des Systems.
Sicherstellen, daß alle Adressen gültig sind und den Vorgang wiederholen.

Der Benutzer hat einen falschen Benutzernamen eingegeben oder einen Namen
verwendet, den das System nicht erkennt.
# Mögliche Lösung:


Alle Benutzernamen überprüfen, gegebenenfalls korrigieren und die
Nachricht erneut senden.
# Im Postprogramm ist nicht genügend Speicher verfügbar,
um diese Nachricht zu senden. Versuchen, andere Anwendungen zu
beenden und die Nachricht danach erneut zu senden.

Das Postprogramm verfügt nicht mehr über genügend Speicherplatz.
# Mögliche Lösung:


Versuchen, einige andere Anwendungen zu schließen, um Speicherplatz
freizugeben.
# Bei dem Versuch, die Nachricht zu senden, ist ein
Fehler aufgetreten. Überprüfen, ob die Nachricht empfangen wurde.
Wenn nicht, muß die Nachricht erneut gesendet werden.

Ein Fehler bei der Postübertragung (sendmail) ist aufgetreten.
# Mögliche Lösung:


Den geplanten Empfänger (nicht mit Hilfe der Anwendung 'Post')
kontaktieren und fragen, ob die Nachricht empfangen wurden. Falls
nicht, muß die Nachricht erneut gesendet werden.
# Postprogramm beenden?


Die Anwendung 'Post' scheint inkorrekt installiert zu sein. Der Benutzer
muß die Anwendung beenden und einige Änderungen vornehmen.
# Mögliche Lösung:


Der Benutzer muß die Anwendung verlassen und die Berechtigungen
ändern. Dazu sollte der Systemadministrator unter der Benutzer-ID Root
die Berechtigungen in /usr/dt/bin/dtmail ändern.
# Schwerwiegender Fehler.


Bei der Anwendung 'Post' ist ein schwerwiegender (nicht behebbarer) Fehler
aufgetreten.
# Mögliche Lösung:


Den Systemadministrator kontaktieren.
# Behebbarer Fehler.


Bei der Anwendung 'Post' ist ein behebbarer Fehler aufgetreten.
# Mögliche Lösung:


Die Anweisungen im Fehlerdialog befolgen.

Oder

Den Systemadministrator kontaktieren.
# Insufficient Memory for Requested Operation

# Mögliche Lösungen:


Sicherstellen, daß nicht zu viele Anwendungen gleichzeitig ausgeführt
werden.

Falls möglich, einige Anwendungen schließen, um Systemspeicherplatz
freizugeben.

Den Auslagerungsspeicher vergrößern. Ist dem Benutzer nicht bekannt,
wie der Auslagerungsspeicher vergrößert wird, den Systemadministrator
kontaktieren.

SieheÖffnen und Schließen eines Anwendungsfensters.