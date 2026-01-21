
# Anmeldemanager - Aufgaben

# Starten und Beenden einer Desktop-Session







# Andere Arten der Anmeldung







# Anpassung des Session-Neustarts und der Abmeldung von einer Session






Bestimmen, wie die nächste Session gestartet wird

Speichern einer Session, während sie läuft

Festlegen der Abmeldebestätigung






# Anmelden an einer Desktop-Session
Anmelden an einer Desktop-Session

Die Benutzer-ID eingeben und die Eingabetaste drücken oder auf 'OK' klicken.

Das Kennwort eingeben und die Eingabetaste drücken oder auf 'OK' klicken.

Erkennt der Anmeldemanager die Benutzer-ID oder das Kennwort nicht, die
Eingabe wiederholen.

Nach der Anmeldung startet der Session-Manager eine Session nach folgenden
Kriterien:

Ist dies die erste Anmeldung, wird eine neue Session gestartet.

War der Benutzer bereits zuvor angemeldet, wird die vorherige Session
wiederhergestellt.
# Abmelden von einer Desktop-Session
Abmelden von einer Desktop-SessionSession: beendenBedienfeld: Bedienelement für EndeArbeitsbereichsmenü: Element 'Abmelden'

Im Bedienfeld auf das Bedienelement für Ende klicken.

Oderdas Menüelement 'Abmelden' im Arbeitsbereichsmenü (wird durch
Drücken von Maustaste 3 eingeblendet) auswählen.

Meldet sich der Benutzer aus einer normalen Desktop-Session ab,
sichert der Session-Manager Informationen über die aktuelle Session, so daß
diese Informationen bei der nächsten Anmeldung wiederhergestellt werden können.
# Verwenden einer anderen Sprache


in einer SessionSprache: andereSitzung: in anderen Sprachen

Im Menü 'Optionen' im Anmeldefenster das Menüelement 'Sprachen' auswählen.

Die Sprachengruppe auswählen, die die gewünschte Sprache enthält.

Die gewünschte Sprache aus dem Menü der Sprachen in der Gruppe auswählen.
Nach Auswahl der Sprache wird das Anmeldefenster in der ausgewählten
Sprache erneut angezeigt.

Anmelden.

Die Standardsprache des Systems wird vom Systemadministrator festgelegt.
Das Menü 'Optionen' ermöglicht den Zugriff auf andere Sprachen. Wird eine
Sprache im Menü 'Sprachen' ausgewählt, wird die Umgebungsvariable LANG für
die Session festgelegt. Am Ende der Session wird die Standardsprache
wiederhergestellt.
# Anmelden an und Abmelden von einer einfachen


Motif-SessionAnmelden an und Abmelden von einfacher Motif-SessionEinfache Motif-Session: An- und AbmeldenTerminal-Emulator: in einfacher Motif-SessionEine einfache Motif-Session ist eine Session, die wahlweise einen
Fenstermanager und ein einzelnes Terminal-Fenster startet. Sie ist
hilfreich, wenn der Zugriff auf ein einzelnes Terminal-Fenster gewünscht
wird, um vor der Anmeldung an einer Desktop-Session Befehle auszuführen.
# Anmelden


Im Menü 'Optionen' im Anmeldefenster das Menüelement 'Session' auswählen.

Aus dem Untermenü 'Session' das Element 'einfache Motif-Session' auswählen.

Anmelden.
# Abmelden


Im Terminal-Fenster den Befehlexiteingeben.
# Starten des Modus für Befehlszeilenanmeldung
X-Server: temporär aussetzenVerwenden: Modus für BefehlszeilenanmeldungModus für Befehlszeilenanmeldung

Im Menü 'Optionen' im Anmeldefenster das Menüelement 'Befehlszeilenanmeldung'
auswählen. Das Anmeldefenster wird durch eine
Eingabeaufforderung der Konsole ersetzt.

Die Benutzer-ID und das Kennwort entsprechend der Aufforderungen eingeben.

Die Befehlszeilenanmeldung ist keine Desktop-Session. Befindet
sich das System im Modus für Befehlszeilenanmeldung, ist das Desktop ausgesetzt.
Die Anmeldung erfolgt nicht über den Anmeldemanager, sondern über eine
Funktion des Betriebssystems. Es stehen keine Fenster zur Verfügung, da der
X-Server nicht ausgeführt wird.

Bestimmte Konfigurationsarten (beispielsweise Xstations) stellen keine
Möglichkeit zur Anmeldung im Befehlszeilenmodus zur Verfügung.
# Verlassen des Modus für Befehlszeilenanmeldung
Verwenden: Modus für BefehlszeilenanmeldungModus für Befehlszeilenanmeldung

An der Eingabeaufforderung der Befehlszeileexiteingeben.
# Sichern einer Standardsitzung
Sichern: StandardsitzungStandardsitzung sichern

Im Bedienfeld auf das Bedienelement für den Umgebungsmanager klicken.

Im Umgebungsmanager auf das Bedienfeld 'Neustart' klicken.

Im Dialogfenster 'Neustart' auf die Taste 'Standardsitzung festlegen' klicken.

Im eingeblendeten Dialogfenster zur Bestätigung auf 'OK' klicken.

Auf 'OK' klicken.

Hierdurch wird der aktuelle Status der Session gesichert.
# Siehe auch


Bestimmen, wie die nächste Session gestartet wird

Speichern einer Session, während sie läuft

Festlegen der Abmeldebestätigung




# Automatisches Starten der Standardsitzung bei


der AnmeldungStarten: Standardsitzung bei der AnmeldungStandardsitzung: Automatisches Starten bei Anmeldung

Im Bedienfeld auf das Bedienelement für den Umgebungsmanager klicken.

Im Umgebungsmanager auf das Bedienfeld 'Neustart' klicken. Das
Dialogfenster 'Neustart' wird eingeblendet.

Den Wechselschalter 'Zu Standardsitzung zurückkehren' auswählen.

Auf 'OK' klicken.

Wurde 'Zu Standardsitzung zurückkehren' ausgewählt, sichert der Session-Manager
die aktuelle Session bei der Abmeldungnicht.
# Siehe auch


.

.Auswahl der SessionAuswählen: zwischen aktueller und StandardsitzungAktuelle und Standardsitzung: auswählen zwischenStandard- und aktuelle Sitzung: auswählen zwischenAuf die Schaltfläche Optionen im Anmeldebildschirm klicken.SessionBei der Anmeldung bestimmenAuf Session klicken. Das Menü Session listet die verfügbaren Sessions auf:AktuellStartet Ihre letzte Session.StandardStartet Ihre Standardsitzung (falls Sie eine eingerichtet haben).display-name- AktuellStartet die aktuelle Session für Ihren Bildschirm, falls vorhanden.
Andernfalls wird eine neue aktuelle Session für Ihren Bildschirm erstellt,
und die erste der folgenden existierenden Sessions wird gestartet: eine Standardsitzung
für Ihren Bildschirm, die auswählbare Standardsitzung oder eine neue
Benutzer-Session.display-name- StandardStartet die Standardsitzung für Ihren Bildschirm, falls vorhanden.
Andernfalls wird eine neue Session speziell für Ihren Bildschirm erstellt,
und Ihre auswählbare Standardsitzung (falls vorhanden) oder eine neue Benutzer-Session
wird gestartet.Failsafe-SessionStartet eine Failsafe-Session.Auf die Session klicken, die Sie starten möchten.Siehe auch..
# Festlegen von persönlichen Umgebungsvariablen
Festlegen: persönliche UmgebungsvariablenPersönliche Umgebungsvariablen: festlegenUmgebungsvariablen: persönliche festlegenVariablen, Umgebung: persönliche festlegen

Persönliche Umgebungsvariablen können in der ProzedurdateiStandardverzeichnis/.dtprofilefestgelegt werden.

Die Datei`Standardverzeichnis`/.dtprofile editieren.

Zeilen zu der Datei hinzufügen, um die Umgebungsvariablen festzulegen.

Das Desktop akzeptiert für die Befehle in dieser Datei Syntaxkonventionen für
'sh' oder 'ksh'. Es sollten nur Befehle verwendet werden, die
Umgebungsvariablen festlegen, nicht solche, die Terminal-Ein-/Ausgabe
ausführen, wie beispielsweise "tset" oder "stty".

Die Dateien/etc/profileundStandardverzeichnis/.profilewerden vom Desktop nicht
gelesen, da sie auf Terminal-Ein-/Ausgabe basierende Befehle enthalten
könnten, die für eine Grafikschnittstelle nicht geeignet sind.

Das Desktop legt automatisch die folgenden Umgebungsvariablen für
jeden Benutzer fest:

* **DISPLAY** 

festgelegt auf den Wert des ersten Feldes in der Datei 'Xservers'
* **EDITOR** 

festgelegt auf den Standardeditor des Desktops
* **ENV** 

festgelegt auf "Standardverzeichnis/.kshrc"
* **HOME** 

festgelegt auf das Standardverzeichnis des Benutzers (aus /etc/passwd)
* **KBD_LANG** 

festgelegt auf den Wert für $LANG für bestimmte Sprachen (siehe Xsession)
* **LANG** 

festgelegt auf die aktuelle NLS-Sprache der Anzeige (falls vorhanden)
* **LC_ALL, LC_MESSAGES** 

festgelegt auf den Wert von $LANG
* **LOGNAME** 

festgelegt auf den Benutzernamen
* **MAIL** 

festgelegt auf "/var/mail/$USER"
* **PATH** 

festgelegt auf den Wert der Ressource "Benutzerpfad" für Dtlogin
* **USER** 

festgelegt auf den Benutzernamen
* **SHELL** 

festgelegt auf die Standard-Shell des Benutzers (aus /etc/passwd)
* **TERM** 

festgelegt auf xterm
* **TZ** 

festgelegt auf den Wert der Ressource "Zeitzone" für Dtlogin

# Siehe auch


.

Die elektronische Handbuchseite 'dtlogin(1X)', auf der sich genauere
Angaben zum Festlegen von Umgebungsvariablen befinden.
# Verwendung einer bestehenden Datei '.profile'


oder '.login'

Verwendung: Shell-Umgebungsdatei mit.dtprofileShell-Umgebungsdatei, Verwendung mit.dtprofileUmgebungsdatei, Shell, Verwendung mit.dtprofileExistiert bereits eine Shell-Umgebungsdatei (.profileoder.login),
erlaubt das folgende Verfahren, diese Datei weiterhin zu verwenden. Daduch
wird das Duplizieren von Variablenzuordnungen zwischen der
Datei.dtprofileund der Shell-Umgebungsdatei vermieden. Die DateiStandardverzeichnis/.profile(oder.login)
kann durch kleinere Änderungen so angepaßt werden, daß sie mit dem oder
ohne das Desktop verwendet werden kann.

Die Datei.profileoder.loginso editieren, daß die folgenden zwei Abschnitte erstellt werden:

Ein Abschnitt enthält Befehle, die nicht für das Desktop verwendet werden
(zum Beispiel Befehle die Terminal-Ein-/Ausgabe benötigen oder Variablen,
für die die Standardwerte des Desktops übernommen werden sollen). Diese
Befehle müssen von einer IF-Anweisung eingeschlossen sein, die die
Einstellung der Umgebungsvariablen 'DT' abfragt.

Der andere Abschnitt enthält Variablen, die unabhängig
davon, ob das Desktop ausgeführt wird, gültig sind.

Die Datei.dtprofiledurch Entfernen der
Kommentarzeichen der folgenden Zeile am Ende der Datei so ändern, daß die
Shell-Umgebungsdatei einbezogen wird.DTSOURCEPROFILE=true

Erneut anmelden.

Das folgende Beispiel zeigt, wie die Datei in einen Abschnitt, der nicht
für das Desktop gültig ist und einen Abschnitt für Variablen, die für
beide Umgebungen gültig sind, aufgeteilt wird.
# Beispiel für sh/ksh
#
  #  Befehle und Umgebungsvariablen, die verwendet werden, wenn die
  #  Anmeldung ohne das Desktop erfolgt
  #
  if [ ! "$DT" ]; then
    stty ...
    tset ...
    DISPLAY=mydisplay:0
    MAIL=/var/mail/$USER
    EDITOR=/bin/vi
                ...
  fi

  #
  # Umgebungsvariablen, die Anmeldungen mit und ohne Desktop gemeinsam sind
  #
  PATH=$HOME/bin:$PATH
       ...
# Beispiel für csh
#
  #  Befehle und Umgebungsvariablen, die verwendet werden, wenn die
  #  Anmeldung ohne das Desktop erfolgt
  #
  if (! ${?DT}) then
     stty ...
     tset ...
     setenv DISPLAY mydisplay:0
     setenv MAIL /var/mail/$USER
     setenv EDITOR /bin/vi
         ...
  fi
  #
  # Umgebungsvariablen, die Anmeldungen mit und ohne Desktop gemeinsam sind
  #
  setenv PATH $HOME/bin:$PATH

Fehler in den Dateien .dtprofile oder .profile (.login) verhindern
möglicherweise eine erfolgreiche Anmeldung. Ist dies der Fall, muß die
Anmeldung mit der einfachen Motif-Session erfolgen und der Fehler
behoben werden.

Wird ein Terminal-Emulator mit der Option-lsgestartet, wird die
Datei.loginoder.profileautomatisch gelesen.