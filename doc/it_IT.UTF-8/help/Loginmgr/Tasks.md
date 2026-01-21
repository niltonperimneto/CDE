Attività della Gestione del loginAvvio e termine di una sessione del desktopMetodi di login alternativiPersonalizzazione dell'avvio di una sessione e del logoutDeterminare la modalità di avvio della sessione
successivaSalvare una sessione specificaAbilitare o disabilitare la conferma di logoutEffettuare il login in una sessione del desktoplogin in una sessione del desktopDigitare il proprio ID utente e
premere Return o fare clic su OK.Digitare la propria parola chiave
e premere Return o fare clic su OK.Se la Gestione del login non riconosce l'ID o la parola chiave specificata,
provare a reinserire le informazioni.Quando si esegue il login, la Gestione delle sessioni avvia una sessione
del desktop:Se si tratta del primo login dell'utente,
viene avviata una sessione nuova.Se l'utente ha già effettuato
altri login, viene ripristinata la sessione precedente.Effettuare il logout da una sessione del desktoplogout da una sessione del desktopsessioni:finePannello principale:controllo di uscitalogout, dal menu di controllo dello spazio di lavoroFare clic sul controllo di uscita
del Pannello principale.Oppure,scegliere
Logout dal menu dello spazio di lavoro (visualizzato premendo il pulsante
del mouse 3).Quando si effettua il logout da una normale sessione del desktop, la
Gestione delle sessioni salva le informazioni sulla sessione corrente per
poterle ripristinare al login successivo.Avviare una sessione in un'altra lingualingue: opzionisessioni:avvio in altre lingueScegliere Lingua dal menu Opzioni
nello schermo di login.Scegliere il gruppo di lingue che
comprende quella desiderata.Dal menu del gruppo, scegliere la
lingua desiderata.
Dopo la selezione, lo schermo di login viene ripresentato
nella lingua selezionata.Effettuare il login.La lingua predefinita viene impostata dall'amministratore del sistema.
Tuttavia, il menu Opzioni permette di accedere al desktop usando lingue diverse.
Scegliendo un'opzione dal menu Lingue si imposta la variabile d'ambiente LANG
per la sessione corrente. Al termine della sessione viene ripristinata la
lingua predefinita.Effettuare il login e il logout da una sessione protettalogin e logout da una sessione protettasessione protetta: login e logoutemulatore di terminale: uso per una sessione protettaUna sessione protetta è una sessione che avvia opzionalmente
una Gestione di finestre e un'unica finestra di terminale. Può essere
usata quando occorre eseguire una serie di comandi da una finestra di terminale
prima del login in una sessione del desktop.Per effettuare il loginNello schermo di login, scegliere
Sessione dal menu Opzioni.Scegliere Sessione a finestra singola
dal menu secondario.Effettuare il login.Per effettuare il logoutDigitare il comandoexitnella finestra Terminale.Abilitare il login dalla riga comandiserver di finestre X: arresto temporaneouso del Login dalla riga comandiLogin dalla riga comandiNello schermo di login, scegliere
Login della riga comandi dal menu Opzioni.
Lo schermo di login scompare e viene sostituito dal prompt di una console.Inserire l'ID utente e la parola
chiave.Il login della riga comandi non fa parte di una sessione del desktop.
Quando il sistema si trova in questa modalità, il desktop viene "sospeso".
Per il login viene usato il meccanismo del sistema operativo anziché
la Gestione del login. Non compaiono finestre perché il server X non
è attivo.Alcune configurazioni (ad esempio, i terminali X) non prevedono l'opzione
del Login della riga comandi.Uscire dalla modalità di login della riga comandiuso del Login dalla riga comandiLogin dalla riga comandiDigitareexitdalla riga comandi.Salvare una sessione inizialesalvataggio di una sessione inizialesessione iniziale: salvataggioFare clic sul controllo della Gestione
degli stili nel Pannello principale.Fare clic sul controllo Avvio della
Gestione degli stili.Fare clic su Impostare sessione iniziale
nel riquadro di dialogo.Fare clic su OK nel riquadro di dialogo
di conferma.Fare clic su OK.Questa operazione salva lo stato corrente della sessione.Vedere ancheDeterminare la modalità di avvio della sessione
successivaSalvare una sessione specificaAbilitare o disabilitare la conferma di logoutAvviare automaticamente la sessione iniziale al loginavvio di una sessione protetta al loginsessione iniziale: avvio automatico al loginFare clic sul controllo della Gestione
degli stili nel Pannello principale.Fare clic sul controllo Avvio della
Gestione degli stili. Viene aperto il riquadro di dialogo Avvio.Selezionare Ritornare alla sessione
iniziale.Fare clic su OK.Quando si seleziona Ritornare alla sessione iniziale, la Gestione delle
sessioninonsalva la sessione corrente al logout.Vedere anche..Per scegliere la sessione da avviarescegliere: tra la sessione corrente e quella inizialesessione corrente e sessione iniziale: scegliere trasessione iniziale e sessione corrente: scegliere traSullo schermo di login, fare clic sul pulsante Opzioni.sessionedeterminazione al loginFare clic su Sessione. Il menu Sessione elenca le sessioni
disponibili:Sessione CorrenteAvvia la sessione più recente.Sessione InizialeAvvia la sessione iniziale (qualora se ne fosse stata impostata una).nome-video- CorrenteAvvia la sessione corrente per il video in uso, qualora ve ne fosse
una. Altrimenti, verrà creata una nuova sessione corrente per il video
e verrà avviata la prima che esista delle sessioni seguenti: una sessione
iniziale per il video, la sessione iniziale generica oppure una nuova sessione
utente.nome-video- InizialeAvvia la sessione iniziale per il video in uso, qualora ve ne fosse
una. Altrimenti, verrà creata una nuova sessione specifica per il video
in uso e verrà avviata la sessione iniziale generica (qualora esistesse)
oppure una nuova sessione utente.Sessione protettaAvvia una sessione protetta.Fare clic sulla sessione che si desidera avviare.Vedere anche..Impostare variabili d'ambiente personaliimpostazione di variabili d'ambiente personalivariabili d'ambiente personali: impostazioneLe variabili d'ambiente personali possono essere impostate nello scriptDirectoryIniziale/.dtprofile.Aprire con un editor il fileDirectoryIniziale/.dtprofile.Aggiungere le righe appropriate per
impostare le variabili d'ambiente.Per i comandi di questo file sono ammesse le sintassi sh o ksh. Inserire
solo i comandi che impostano le variabili d'ambiente, non comandi che eseguono
operazioni di I/O come "tset" o "stty".I file/etc/profileeDirectoryIniziale/.profilenon vengono letti dal desktop,
poiché possono contenere comandi di I/O del terminale non appropriati
per un'interfaccia grafica.Il desktop imposta automaticamente le seguenti variabili d'ambiente
per ogni utente:DISPLAYviene impostata sul valore del primo campo del file XserversEDITORviene impostata sull'editor predefinito del desktopENVviene impostata su "DirectoryIniziale/.kshrc"HOMEviene impostata sulla directory iniziale dell'utente (derivata da /etc/passwd)KBD_LANGviene impostata sul valore di $LANG per alcune lingue (vedere Xsession)LANGviene impostata sulla lingua NLS corrente del display (se presente)LC_ALL, LC_MESSAGESvengono impostate sul valore di $LANGLOGNAMEviene impostata sul nome dell'utenteMAILviene impostata come "/var/mail/$USER"PATHviene impostata sul valore della risorsa "userPath" di DtloginUSERviene impostata sul nome dell'utenteSHELLviene impostata sulla shell predefinita dell'utente (derivata da /etc/passwd)TERMviene impostata su xtermTZviene impostata sul valore della risorsa "timeZone" di DtloginVedere anche.La pagina di spiegazioni Dtlogin(1X)
per informazioni sull'impostazione delle variabili d'ambiente.Usare un file .profile o .login predefinitouso di.dtprofile.dtprofileSe l'utente dispone già di un file di definizione dell'ambiente
(.profileo.login),
questa procedura permette di continuare a utilizzare tale file. In questo
modo si eviteranno conflitti di assegnazione delle variabili tra.dtprofilee il file di definizione dell'ambiente. Con alcune
modifiche, sarà possibile adattare il fileDirectoryIniziale/.profile
(o .login)in modo da utilizzarlo sia all'interno che all'esterno
del desktop.Aprire con un editor il file.profile o .logine creare due sezioni:In una sessione, inserire i comandi
non applicabili al desktop (ad esempio, i comandi che richiedono un I/O del
terminale o le variabili per le quali si desidera riprodurre i valori predefiniti
del desktop). Racchiudere questi comandi con un'istruzione "if" che controlli
l'impostazione della variabile d'ambiente "DT".Nell'altra sessione, inserire le
variabili applicabili indipendentemente dall'avvio o meno del desktop.Modificare.dtprofilein modo che legga il file di definizione dell'ambiente,
togliendo il simbolo di commento dalla riga seguente alla fine del file.dtprofile.DTSOURCEPROFILE=trueRipetere il login.L'esempio seguente spiega come dividere il file in una sezione da utilizzare
senza il desktop e in una per le variabili applicabili a entrambi gli ambienti.Esempio per sh/ksh#
  # comandi e variabili d'ambiente da usare per il login  
  # senza il desktop
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
  # variabili d'ambiente comuni alle sessioni con e senza desktop 
  #
  PATH=$HOME/bin:$PATH
       ...Esempio per csh#
  #  comandi e variabili d'ambiente da usare per il login  
  #  senza il desktop
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
  # variabili d'ambiente comuni alle sessioni con e senza desktop
  #
  setenv PATH $HOME/bin:$PATHUn errore in .dtprofile o in .profile (.login) può impedire l'esecuzione
corretta del login. In tal caso, effettuare il login con una sessione protetta
e correggere l'errore.Se si avvia l'emulatore di terminale con l'opzione-ls, il file.logino.profileverrà letto automaticamente.