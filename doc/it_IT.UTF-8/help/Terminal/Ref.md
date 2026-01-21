Informazioni di riferimentoFare clic su uno degli argomenti seguenti.Pagina di spiegazioni dtterm (1X)Vedere ancheX(1)resize(1)xset(1)xterm(1)SINOPSIdtterm[-opzioni]DESCRIZIONEIl clientdttermfornisce un supporto
per l'esecuzione di applicazioni tradizionali scritte per i terminali a caratteri
conformi agli standard ANSI X3.64-1979 e ISO 6429:1992(E), ad esempio il DEC
VT220.OPZIONIL'emulatore di terminaledttermaccetta
tutte le opzioni standard delle righe comandi X Toolkit e una serie di altre
opzioni, tutte indicate nell'elenco seguente (se l'opzione inizia con un `+'
anziché con un `-', significa che essa viene ripristinata al valore
predefinito):-132Normalmente, la sequenza di escape che commuta tra il
modo a 80 colonne e quello a 132 colonne viene ignorata. Questa opzione abilita
il riconoscimento della sequenza di escape DECCOLM, e il conseguente ridimensionamento
della finestra didtterm.c132Risorsa associata:c132.+132Questa opzione fa sì che la sequenza di escape
DECCOLM
venga ignorata. È l'impostazione predefinita.c132Risorsa associata:c132.-awQuesta opzione indica l'abilitazione del ritorno a capo
automatico. Questo significa che il cursore, una volta raggiunta la fine della
riga corrente, passa automaticamente all'inizio della riga successiva.
È l'impostazione predefinita.Risorsa associata:autoWrap.autoWrap+awQuesta opzione indica che il ritorno a capo automatico
è disabilitato.Risorsa associata:autoWrap.autoWrap-backgroundcolore_sfondoQuesta opzione specifica il colore di sfondo della finestra
di terminale e lo sfondo predefinito usato per la barra di scorrimento e il
puntatore X11. Nel CDE, l'impostazione predefinita di questa opzione è
quella del pixel di sfondo o del pixel di selezione dell'insieme di colori
primario (vedere-bs.
Al di fuori del CDE, l'impostazione predefinita è*background/*Backgroundcon il nero come colore base.colore_sfondospecifica il colore
da utilizzare per lo sfondo.backgroundRisorsa
associata:background-bdcolore_bordoQuesta opzione specifica il colore applicato al bordo
delle finestre. Il bordo del widget della shell potrebbe non essere visibile
quando si utilizzano gestori di finestre con funzioni di "reparenting" comedtwm(1X)emwm(1X). Il colore
predefinito è il nero.colore_bordospecifica il colore
da utilizzare per i bordi.borderColorRisorsa
associata:borderColor.-bgcolore_sfondoQuesta opzione è identica a-background.colore_sfondospecifica il colore
da utilizzare per lo sfondo.backgroundRisorsa
associata:background.-bordercolorcolore_bordoQuesta opzione è identica a -bd.colore_bordospecifica il colore da utilizzare per i bordi.borderColorRisorsa
associata:borderColor.-borderwidthlarghezza_bordoQuesta opzione specifica la larghezza del bordo della
finestra del widget della shell.
Su questo valore possono prevalere le impostazioni di gestori di finestre
con funzioni di "reparenting" comedtwm(1X)emwm(1X).
Il valore predefinito è 0.larghezza_bordospecifica la larghezza del bordo delle finestre in pixel.borderwidthRisorsa
associata:borderWidth.-bsQuesta opzione specifica che la finestra di terminale
deve usare per lo sfondo il colore di selezione di Motif anziché il
colore di sfondo. È l'impostazione predefinita.backgroundIsSelectRisorsa
associata:backgroundIsSelect.+bsQuesta opzione specifica che la finestra di terminale
non deve usare per lo sfondo il colore di selezione di Motif, bensì
il colore di sfondo.Risorsa associata:backgroundIsSelect.-bwlarghezza_bordoQuesta opzione è identica a-borderwidth.Risorsa associata:borderWidth.-CQuesta opzione specifica che l'output diretto a/dev/consoledeve essere invece diretto alla finestra di
terminale. Viene usata come metodo per impedire che l'output normalmente visualizzato
sull'ITE sovrascriva il display del server X. Non è appropriata come
meccanismo generale per dirigere l'output proveniente dalla/dev/consoledi un sistema arbitrario su un server X arbitrario.
Si noti che, per poter utilizzare questa opzione, l'utente deve essere proprietario
e avere accesso in lettura e scrittura a/dev/console.-displaynome_displayQuesta opzione specifica il server di visualizzazione
X11 che dovrà essere utilizzato dadtterm.
L'impostazione predefinita è il valore contenuto nella variabile d'ambiente
$DISPLAY.nome_displayspecifica il server
X11 con cui dovrà essere stabilito il collegamento.display-eargomento_programma ...Questa opzione specifica un programma eseguibile da richiamare
come processo secondario all'avvio didtterm.
Questa opzione deve essere l'ultima della riga comandi.argomento_programma ...specifica il programma e gli argomenti della riga comandi da eseguire.-fbset_di_fontQuesta opzione specifica un set di font X (XFontSet) da
utilizzare per visualizzare sul terminale un testo in grassetto. Deve essere
specificata come XmFontList Motif. Sono supportati solo i font non proporzionali.
L'uso di un font proporzionale può produrre effetti indeterminati.
Verrà generato un font grassetto predefinito basato sul nome XLFD del
font utente (userFont).
Se quel font non è disponibile, il testo in grassetto verrà
generato sovrascrivendo (con uno sfasamento di un pixel) il font utente.set_di_fontspecifica l'XFontSet
grassetto da usare sul terminale.Risorsa associata:userFont.-fgcolore_primo_pianoQuesta opzione specifica il colore di primo piano della
finestra di terminale e il colore di primo piano predefinito usato per la
barra di scorrimento e per il puntatore X11. Nel CDE, l'impostazione predefinita
di questa risorsa corrisponde al pixel di primo piano del set di colori primario.
Al di fuori del CDE,
l'impostazione predefinita è*foregroundo*Foregroundcon il bianco come colore di
base.colore_primo_pianospecifica il colore
da utilizzare per il primo piano.foregroundRisorsa
associata:foreground.-fnset_di_fontQuesta opzione specifica un set di font X (XFontSet) da
utilizzare per la visualizzazione del testo sul terminale. Deve essere specificata
come XmFontList Motif. Sono supportati solo i font non proporzionali. L'uso
di un font proporzionale può produrre effetti indeterminati. Questo
font non verrà utilizzato per visualizzare altri tipi di testo (barre
dei menu, menu a scomparsa, riquadri di dialogo, ecc.). L'impostazione predefinita
prevede l'uso del valore XmNtextFontList del bulletin board principale (vedere
XmBulletinBoard(3X)) allo stesso modo del widget XmText.set_di_fontspecifica l'XFontSet
da utilizzare per il terminale.
Risorsa associata:userFont.-fontset_di_fontQuesta opzione è identica a-fn.set_di_fontspecifica l'XFontSet
da usare per il terminale.Risorsa associata:userFont.-foregroundprimo_pianoQuesta opzione è identica a-fg.primo_pianospecifica il colore da
utilizzare per il primo piano.foregroundRisorsa
associata:foreground.-geometrygeometriaQuesta opzione specifica la dimensione e la posizione
preferenziali per la finestra di terminale. La dimensione predefinita è
di 24 righe di 80 caratteri. Non vi è alcuna posizione predefinita.geometriaspecifica la geometria
da utilizzare per la finestra di terminale.geometryRisorsa
associata:geometry.-helpVisualizza una spiegazione riassuntiva sull'uso didtterm.help-iconicQuesta opzione specifica che l'emulatore di terminale
deve comparire inizialmente sullo schermo in forma di icona.iconicRisorsa
associata:iconic.+iconicQuesta opzione specifica che l'emulatore di terminale
deve comparire inizialmente sullo schermo come finestra normale. È
l'impostazione predefinita.Risorsa associata:iconic.-jQuesta opzione abilita lo scorrimento di più righe.
In questa modalità è possibile far scorrere il contenuto dello
schermo di più righe alla volta, ottenendo un aggiornamento più
rapido quando si inviano al terminale molte righe di testo. Il numero massimo
di righe per questo tipo di scorrimento corrisponde alle righe della finestra
di terminale. In questo modo è sempre garantita la visualizzazione
di tutte le righe. È l'impostazione predefinita.jumpScrollRisorsa
associata:jumpScroll.+jQuesta opzione specifica la disabilitazione dello scorrimento
di più righe. Per una descrizione di questa modalità di scorrimento,
vedere l'opzione-jprecedente.Risorsa associata:jumpScroll.-kshModeQuesta opzione specifica l'abilitazione del modo ksh.
In questa modalità, premendo un tasto con il bit di modifica per caratteri
estesi impostato verrà generato un carattere di escape seguito dal
carattere generato dal tasto non-esteso. Questa opzione è destinata
all'uso con emacs e con il modo di editazione emacs delle righe comandi diksh(1)oied(1). Essa è
in conflitto con l'uso normale del tasto Meta per la generazione di caratteri
estesi a un byte e per la generazione di caratteri asiatici multibyte.kshModeRisorsa associata:kshMode.+kshModeQuesta opzione disabilita il modo ksh. È l'impostazione
predefinita.Risorsa associata:kshMode.-lQuesta opzione abilita la registrazione dell'output in
un log. Se il logging è abilitato, tutto l'output ricevuto dal processo
secondario viene registrato in un file o in una pipeline di comandi (a seconda
dell'impostazione dell'opzione-lfdescritta
più avanti). Poiché i dati vengono registrati direttamente dal
processo secondario, essi includono anche tutti i caratteri di escape e le
coppie return/nuova riga caratteristiche del terminale. L'output può
essere abilitato e disabilitato mediante sequenze di escape.loggingRisorsa
associata:logging.+lQuesta opzione disabilita la registrazione dell'output
in un log. Per una descrizione di questa funzionalità, vedere l'opzione-ldescritta in precedenza. È l'impostazione predefinita.loggingRisorsa
associata:logging.-lfnome_fileQuesta opzione specifica il nome del file di log in cui
deve essere scritto l'output descritto in precedenza. Senome_fileinizia con un simbolo di pipe (|),
il resto della stringa viene interpretato come un comando da utilizzare come
elemento finale di una pipe. Il nome predefinito per il file èDttermLogXXXXX(doveXXXXXè l'id del processo didtterm), e viene creato nella directory da cui è stato
avviatodtterm. Se gli ultimi cinque caratteri
sono "XXXXX", essi vengono sostituiti dall'ID del processo.nome_filespecifica il nome da utilizzare
per il file di log.logFileRisorsa
associata:logFile.-lsQuesta opzione indica che la shell avviata dovrà
essere una shell di login (il primo carattere diargv[0]sarà un trattino, per indicare alla shell di leggere il fileprofiledel sistema e il file$HOME/.profiledell'utente (perkshesh), oppure il filecsh.logindel sistema e il file$HOME/.logindell'utente
(percsh).loginShellRisorsa
associata:loginShell.+lsQuesta opzione specifica l'avvio di una shell normale
(non di login). È l'impostazione predefinita.loginShellRisorsa
associata:loginShell.-mapQuesta opzione indica che la finestra didtterm, se chiusa ad icona, dovrà aprirsi (mapparsi)
automaticamente in seguito all'output di un processo secondario. È
possibile specificare un ritardo iniziale prima del qualedttermnon si apra utilizzando la risorsamapOnOutputDelay.mapOnOutputRisorsa
associata:mapOnOutput.+mapQuesta opzione disabilita la modalità di apertura
(mappatura) automatica. È l'impostazione predefinita.mapOnOutputRisorsa
associata:mapOnOutput.-mbQuesta opzione indica chedttermdovrà emettere un segnale acustico quando i caratteri digitati dall'utente
si avvicineranno al margine destro dello schermo. La distanza effettiva per
l'emissione del segnale è specificata dall'opzione-nb.marginBellRisorsa
associata:marginBell.+mbQuesta opzione indica che non dovrà essere emesso
alcun segnale acustico in prossimità del margine destro dello schermo.
È l'impostazione predefinita.marginBellRisorsa
associata:marginBell.-mscolore_puntatoreQuesta opzione specifica il colore di primo piano da utilizzare
per il puntatore della finestra di terminale (X11). L'impostazione predefinita
è l'uso del colore di primo piano della finestra di terminale. Vedere
l'opzioneforegrounddescritta in precedenza.colore_puntatorespecifica il colore
di primo piano da utilizzare per il puntatore.Risorsa associata:pointerColor.-namenome_progQuesta opzione specifica il nome X11 della finestradtterm.nome_progspecifica il nome da utilizzare.name-nbnumeroQuesta opzione specifica la distanza in caratteri dal
margine destro raggiunta la quale deve essere emesso il segnale acustico di
avviso (se abilitato). Il valore predefinito è 10.nMarginBellRisorsa
associata:nMarginBell.-rQuesta opzione fa sì che la finestradttermvenga visualizzata con i colori di primo piano e di
sfondo invertiti. È identica alle opzioni-re-reverse.+rQuesta opzione fa sì che la finestradttermvenga visualizzata con i colori di primo piano e di
sfondo normali. È l'impostazione predefinita, ed è identica
all'opzione+rv.-reverseQuesta opzione fa sì che la finestradttermvenga visualizzata con i colori di primo piano e di
sfondo invertiti. È identica alle opzioni-re-rv.-rvQuesta opzione fa sì che la finestradttermvenga visualizzata con i colori di primo piano e di
sfondo invertiti. Lo stesso effetto si ottiene scegliendo Opzioni | Globali
e quindi cambiando l'impostazione "Sfondo/caratteri" in "Colori invertiti".
Nelle finestredttermavviate con questa
opzione, il menu "Sfondo/caratteri" viene preimpostato su "Colori invertiti".
Vedere"Opzioni globali".+rvQuesta opzione fa sì che la finestradttermvenga visualizzata con i colori di primo piano e di
sfondo normali. È l'impostazione predefinita.-rwQuesta opzione abilita il ritorno del cursore alla riga
precedente.reverseWrapRisorsa
associata:reverseWrap.+rwQuesta opzione disabilita il ritorno del cursore alla
riga precedente. È l'impostazione predefinita.reverseWrapRisorsa
associata:reverseWrap.-SccnQuesta opzione specifica che l'emulatore di terminale
dovrà essere eseguito su un'unità pty o STREAMS aperta in precedenza.
È utile nei casi in cui il nome slave dell'unità pty o STREAMS
ha la formatty??(presenta cioè esattamente
due caratteri dopotty), e viene normalmente
usata quandodttermviene richiamato da un'altra
applicazione.ccspecifica gli ultimi due caratteri
del nome slave dell'unità pty o STREAMS, dove il nome slave ha la formatty??. Questo valore viene ignorato, ma deve avere esattamente
una lunghezza di due caratteri.nspecifica il numero
del descrittore del file che corrisponde al lato master già aperto
dell'unità pty o STREAMS.-Sc.nQuesta opzione è identica all'opzione-Sccndescritta in precedenza, ma è destinata ai sistemi
con maggiore spazio per il nome pty.cspecifica l'ultimo componente del
nome slave dell'unità pty. Questo valore viene ignorato e può
essere vuoto.nspecifica il numero
del descrittore del file che corrisponde al lato master già aperto
dell'unità pty.-sbQuesta opzione abilita una barra di scorrimento. È
l'impostazione predefinita.scrollBarRisorsa
associata:scrollBar.+sbQuesta opzione indica che la barra di scorrimentononverrà abilitata.scrollBarRisorsa
associata:scrollBar.-sfQuesta opzione indica che, per i tasti funzione, dovranno
essere generati i codici di escape Sun anziché le sequenze di escape
standard dei VT220.sunFunctionKeysRisorsa
associata:sunFunctionKeys.+sfQuesta opzione indica che, per i tasti funzione, dovranno
essere generate le sequenze di escape standard anziché i codici di
escape dei tasti funzione Sun.
È l'impostazione predefinita.sunFunctionKeysRisorsa
associata:sunFunctionKeys.-slschermi[s|l]Questa opzione specifica il numero di righe da includere
nel buffer del terminale oltre la lunghezza della finestra. L'opzione è
composta da un numero seguito da un suffisso opzionale. Se non viene indicato
alcun suffisso, o viene specificato il suffisso "l" (elle), la lunghezza totale
del buffer del terminale saràschermipiù
la lunghezza della finestra di terminale. Se il suffisso è "s" (esse),
la lunghezza totale del buffer del terminale sarà (schermipiù uno) per la lunghezza della finestra di terminale.
Aumentando le dimensioni della finestra,dttermcercherà di mantenere lo stesso rapporto buffer-finestra. Il valore
predefinito è "4s".schermispecifica il numero di schermi
o di righe da salvare.saveLinesRisorsa
associata:saveLines.-tiid_termQuesta opzione fornisce il nome usato per selezionare
la risposta corretta alle richieste di ID del terminale. I valori validi sonovt100, vt101, vt102evt220. Il valore predefinito èvt220.term_idspecifica l'ID del terminale
da utilizzare.-titletitoloQuesta opzione specifica il titolo della finestra. Se
viene usata l'opzione-e, l'impostazione
predefinita sarà l'ultimo componente del percorso del programma. Se
non viene usata l'opzione-e, l'impostazione
predefinita sarà l'ultimo componente del nome usato per eseguiredtterm(cioèargv[0]).titolospecifica il titolo da utilizzare.titleRisorsa
associata:title.-tmmodi_terminaleQuesta opzione specifica una stringa contenente una o
più parole chiave per l'impostazione del terminale e i caratteri a
cui queste possono essere associate. Le parole chiave ammesse sonointr,quit,erase,kill,eof,eol,swtch,start,stop,brk,susp,dsusp,rprnt,flush,weraselnext. Le parole chiave che non si applicano a un'architettura
specifica saranno analizzate correttamente e ignorate. I caratteri di controllo
possono essere specificati con il simbolo^seguito dalcarattere(ad esempio^co^u), mentre per indicare
la cancellazione si potrà usare^?.
Questa opzione è utile per sostituire le impostazioni predefinite del
terminale senza dover eseguire unstty(1)ogni volta che si avvia un processo di un terminale. Il valore predefinito
è NULL.modi_terminalespecifica la stringa
contenente i modi del terminale.ttyModesRisorsa
associata:ttyModes.-tnnome_terminaleQuesta opzione specifica l'impostazione della variabile
d'ambiente$TERM. Il valore predefinito è"vt220".nome_terminalespecifica il nome
di terminale da utilizzare.termNameRisorsa
associata:termName.-usageVisualizza sullo schermo la sintassi del comando.-vbQuesta opzione abilita l'utilizzo di un segnale visivo
al posto del segnale acustico. Anziché emettere un segnale acustico
ad ogni ricevimento di un Control-G, la finestra di terminale lampeggerà.visualBellRisorsa
associata:visualBell.+vbQuesta opzione indica l'abilitazione di un segnale acustico
anziché visivo.
È l'impostazione predefinita.visualBellRisorsa
associata:visualBell.-wlarghezza_bordoQuesta opzione è identica a-borderwidth.larghezza_bordospecifica la larghezza
del bordo della finestra in pixel.-xrmrisorsaQuesta opzione permette di specificare risorse di tipo
X11 sulla riga comandi.risorsaspecifica
una stringa relativa a una risorsa X11.RISORSEallowSendEventsQuesta risorsa specifica che l'emulatore di terminale
abiliterà gli eventi artificiali (generati e inviati da un'altra applicazione).
L'abilitazione di questa risorsa apre un varco nel meccanismo di sicurezza.
L'impostazione predefinita è False.appCursorDefaultSe l'impostazione è True, i tasti del cursore operano
inizialmente nel modo associato all'applicazione. Se l'impostazione è
False, essi operano inizialmente nel modo standard del cursore. L'impostazione
predefinita è False.appKeypadDefaultSe l'impostazione è True, i tasti del tastierino
numerico operano inizialmente nel modo associato all'applicazione. Se l'impostazione
è False, essi operano inizialmente nel modo numerico standard. L'impostazione
predefinita è False.autoWrapQuesta risorsa specifica se il ritorno del cursore alla
riga precedente è inizialmente abilitato o disabilitato. L'impostazione
predefinita è True.backgroundQuesta risorsa specifica il colore di sfondo della finestra
di terminale e il colore di sfondo predefinito usato per la barra di scorrimento.
Nel CDE, l'impostazione predefinita di questa risorsa è il pixel di
selezione del set di colori primario o il pixel di sfondo del set di colori
primario (vederebackgroundIsSelect). L'impostazione
predefinita è il pixel di sfondo del set di colori primario. Al di
fuori del CDE, l'impostazione predefinita di questa risorsa è il colore
nero.backgroundIsSelectSe l'impostazione è True, questa risorsa specifica
che la finestra di terminale utilizzerà, per il suo sfondo, il colore
di selezione Motif anziché il colore di sfondo. L'impostazione predefinita
è False.blinkRateQuesta risorsa specifica il numero di millisecondi nei
quali il cursore si trova in stato"on"e"off"quando lampeggia. Ad esempio, il valore 250 fa lampeggiare
il cursore due volte al secondo, mentre il valore 0 si disabilita il lampeggiamento.
L'impostazione predefinita è 250.borderColorQuesta risorsa definisce il colore per il bordo della
finestra. Il bordo della finestra può non essere visibile quando si
utilizzano gestori di finestre con funzioni di "reparenting" comedtwmemwm. L'impostazione
predefinita è"black".borderWidthQuesta risorsa specifica la larghezza del bordo della
finestra del widget della shell.
Su questo valore possono prevalere le impostazioni di gestori di finestre
con funzioni di "reparenting" comedtwmemwm. L'impostazione predefinita è0.c132Questa risorsa specifica l'abilitazione o meno della sequenza
di escape DECCOLM, usata per commutare la larghezza delle finestre tra 80
e 132 colonne. L'impostazione predefinita è False.charCursorStyleQuesta risorsa specifica la forma del cursore di testo.
Il valorechar_cursor_boxspecifica un cursore
con la larghezza e l'altezza del riquadro che racchiude il font di base. Il
valorechar_cursor_barspecifica un cursore
con la larghezza del riquadro che racchiude il font di base e un'altezza di
due pixel disegnato con il bordo superiore in corrispondenza della linea base.
L'impostazione predefinita èchar_cursor_box.consoleModeQuesta risorsa specifica che l'output diretto a/dev/consoledeve essere invece indirizzato alla finestra
di terminale. Viene usata come metodo per impedire che l'output normalmente
visualizzato sull'ITE sovrascriva lo schermo del server X. Non è destinato
all'uso come meccanismo generale per dirigere l'output di una/dev/consoledi un sistema arbitrario a un server X arbitrario.
Si noti che, per poter usare questa opzione, occorre essere proprietari e
avere accesso in lettura e scrittura a/dev/console.
L'impostazione predefinita è False.foregroundQuesta risorsa specifica il colore di primo piano della
finestra di terminale, il colore di primo piano predefinito usato per la barra
di scorrimento e il colore usato per il puntatore. Nel CDE, l'impostazione
predefinita di questa risorsa è il colore di primo piano del set di
colori primario. Diversamente, l'impostazione predefinita è"white".geometryQuesta risorsa specifica la dimensione e la posizione
preferite per la finestra di terminale. L'impostazione predefinita per le
dimensioni è di 24 righe di 80 caratteri ciascuna. Non è prevista
alcuna posizione predefinita.iconGeometryQuesta risorsa specifica la posizione preferita per l'icona
dell'emulatore di terminale. I gestori di finestre potrebbero ignorare questo
valore. Non è previsto alcun valore predefinito.iconicSe l'impostazione è True, questa risorsa specifica
che l'emulatore di terminale deve apparire inizialmente sullo schermo in forma
di icona. I gestori di finestre (inclusidtwmemwm) potrebbero ignorare questo valore.
L'impostazione predefinita è False.iconicNameQuesta risorsa specifica il nome per l'icona. Se viene
usata l'opzione-e, il nome predefinito sarà
l'ultimo componente del percorso del programma. Se non si utilizza l'opzione-e, il valore predefinito sarà il nome base usato
per eseguiredtterm(cioèargv[0]).jumpScrollQuesta risorsa abilita dello scorrimento di più
righe. In questa modalità, il contenuto dello schermo scorre di più
righe alla volta, consentendo un aggiornamento più rapido quando si
inviano al terminale molte righe di testo. Il numero massimo di righe per
questo tipo di scorrimento corrisponde alle righe della finestra di terminale.
In questo modo è sempre garantita la visualizzazione di tutte le righe.
L'impostazione predefinita è True.kshModeQuesta opzione specifica l'abilitazione del modo ksh.
In questa modalità, premendo un tasto con il bit di modifica per caratteri
estesi impostato verrà generato un carattere di escape seguito dal
carattere generato dal tasto non-esteso. Questa opzione è destinata
all'uso con emacs e con il modo di editazione delle righe comandi emacs diksh(1)oied(1). Essa è
in conflitto con l'uso normale del tasto Meta per la generazione di caratteri
estesi a un byte e per la generazione di caratteri asiatici multibyte. L'impostazione
predefinita è False.logFileQuesta risorsa specifica il nome del file di log in cui
deve essere scritto l'output descritto più avanti. Se il nome del file
inizia con un simbolo di pipe (|), il resto
della stringa viene interpretato come un comando da utilizzare come elemento
finale di una pipe. Il nome predefinito per il file èDttermLogXXXXX(doveXXXXXè una stringa di caratteri unica), e
viene creato nella directory da cui è stato avviato il processo secondario.
Se gli ultimi cinque caratteri sono "XXXXX", essi vengono sostituiti da una
stringa di caratteri unica.loggingQuesta risorsa abilita la registrazione dell'output in
un log. Se il logging è abilitato, tutto l'output ricevuto dal processo
secondario viene registrato in un file o in una pipeline di comandi (a seconda
di quanto specificato con l'opzionelogFiledescritta in precedenza). Poiché i dati vengono registrati direttamente
dal processo secondario, essi includono anche tutti i caratteri di escape
e le coppie return/nuova riga caratteristiche del terminale. L'output può
essere abilitato e disabilitato mediante sequenze di escape. L'impostazione
predefinita è False.logInhibitQuesta risorsa disabilita la registrazione in un log delle
unità e dei file. L'impostazione predefinita è False.loginShellQuesta risorsa specifica che la shell avviata dovrà
essere una shell di login (il primo carattere diargv[0]sarà un trattino, per indicare alla shell di leggere il fileprofiledel sistema e il file$HOME/.profiledell'utente (perkshesh), oppure il filecsh.logindel sistema e il file$HOME/.logindell'utente
(percsh). L'impostazione predefinita è
False.mapOnOutputQuesta risorsa indica che la finestra dell'emulatore di
terminale, se chiusa ad icona (non mappata), dovrà aprirsi (mapparsi)
automaticamente all'output di un processo secondario. È possibile specificare
un ritardo iniziale per l'apertura dell'icona utilizzando la risorsamapOnOutputDelay(vedere più avanti). L'impostazione
predefinita è False.mapOnOutputDelayQuesta risorsa specifica il ritardo dopo il qualedttermdarà esecuzione alla risorsamapOnOutput(vedere sopra). Questo consente l'invio di un
output iniziale (ad esempio di una richiesta della shell) al terminale senza
determinare l'apertura (mappatura) automatica della finestra. L'impostazione
predefinita è0(nessun ritardo).marginBellSpecifica l'abilitazione o meno di un segnale acustico
a una determinata distanza dal margine destro dello schermo. L'impostazione
predefinita è False.menuBarQuesta risorsa specifica la visualizzazione o meno di
un menu a tendina. L'impostazione predefinita è True.menuPopupQuesta risorsa specifica la visualizzazione di un menu
a scomparsa. L'impostazione predefinita è True.nMarginBellSpecifica la distanza in caratteri dal margine destro
raggiunta la quale deve essere emesso il segnale acustico (se abilitato).
L'impostazione predefinita è 10.pointerBlankQuesta risorsa abilita la modalità di scomparsa
del puntatore. In questa modalità, il puntatore verrà visualizzato
allo spostamento del mouse, mentre scomparirà dallo schermo dopo un
determinato numero di secondi oppure mentre si inseriscono i caratteri dalla
tastiera. Il ritardo viene impostato mediante la risorsapointerBlankDelay(vedere più avanti). L'impostazione
predefinita è False.pointerBlankDelayQuesta risorsa specifica i secondi che dovranno trascorrere
dall'ultimo movimento del mouse prima che il puntatore scompaia dallo schermo.
Il valore 0 specifica la scomparsa del puntatore solo in corrispondenza di
un input da tastiera. Il valore predefinito è di 2 secondi.pointerColorQuesta risorsa specifica il colore di primo piano da utilizzare
per il puntatore della finestra di terminale (X11). L'impostazione predefinita
è il colore di primo piano della finestra di terminale. Vedere l'opzioneforegrounddescritta in precedenza.pointerColorBackgroundQuesta risorsa specifica il colore di sfondo da utilizzare
per il puntatore della finestra di terminale (X11). L'impostazione predefinita
è il colore di sfondo della finestra di terminale. Vedere l'opzionebackgrounddescritta in precedenza.pointerShapeQuesta risorsa specifica il font da utilizzare per il
puntatore tra quelli disponibili per i cursori X. Deve essere specificata
usando una delle stringhe del file include<X11/cursorfont.h>eliminando i caratteri inizialiXC_.
L'impostazione predefinita èxterm.reverseVideoQuesta risorsa specifica l'inversione o meno dei colori
dello schermo. l'impostazione predefinita è False.reverseWrapQuesta risorsa abilita o disabilita il ritorno del cursore
alla riga precedente. L'impostazione predefinita è False.saveLinesQuesta risorsa specifica il numero di righe da includere
nel buffer del terminale oltre la lunghezza della finestra. Il valore consiste
in un numero seguito da un suffisso opzionale. Se non viene indicato alcun
suffisso, o viene specificato il suffisso "l" (elle), la lunghezza totale
del buffer del terminale saràschermipiù
la lunghezza della finestra di terminale. Se il suffisso è "s" (esse),
la lunghezza totale del buffer del terminale sarà (schermipiù uno) moltiplicato per la lunghezza della finestra di
terminale. Aumentando le dimensioni della finestra,dttermcercherà di mantenere lo stesso rapporto buffer-finestra.
Il valore predefinito è "4s."scrollBarQuesta risorsa specifica l'abilitazione o meno della barra
di scorrimento.
L'impostazione predefinita è True.sunFunctionKeysSpecifica la generazione o meno, per i tasti funzione,
di codici di escape con tasti funzione Sun anziché con sequenze di
escape standard del VT220. L'impostazione predefinita è False.termIdQuesta risorsa indica il nome usato per selezionare la
risposta corretta alle richieste di ID del terminale. I valori ammessi sonovt100,vt101,vt102evt220. L'impostazione
predefinita èvt220.termNameQuesta risorsa specifica il nome per la variabile d'ambiente$TERM. L'impostazione predefinita èvt220.titleQuesta risorsa specifica il titolo della finestra. Se
viene usata l'opzione-e, l'impostazione
predefinita sarà l'ultimo componente del percorso del programma. Se
non viene usata l'opzione-e, l'impostazione
predefinita sarà l'ultimo componente del nome usato per eseguiredtterm(cioèargv[0]).ttyModesQuesta risorsa specifica una stringa contenente una o
più parole chiave per l'impostazione del terminale e i caratteri a
cui queste possono essere associate. Le parole chiave ammesse sono:intr, quit, erase, kill, eof, eol, swtch, start, stop, brk, susp, dsusp, rprnt,
flush, weraseInext. Le
parole chiave che non si applicano a un'architettura specifica saranno analizzate
correttamente e ignorate. I caratteri di controllo possono essere specificati
con il simbolo^seguito da uncarattere(ad esempio^co^u), mentre per indicare la cancellazione
si potrà usare^?. Questa opzione
è utile per sostituire le impostazioni predefinite del terminale senza
dover eseguiresttyogni volta che si avvia
un processo di un terminale. L'impostazione predefinita è NULL.userBoldFontQuesta risorsa specifica un set di font X (XFontSet) da
utilizzare per visualizzare sul terminale il testo in grassetto. Deve essere
specificata comeXmFontListMotif. Sono supportati
solo i font non proporzionali. L'uso di un font proporzionale può produrre
effetti indeterminati. Verrà generato un font grassetto predefinito
basato sul nome XLFD dellouserFont. Se quel
font non è disponibile, il testo in grassetto verrà generato
sovrascrivendo (con uno sfasamento di un pixel) louserFont.userFontQuesta risorsa specifica un set di font X (XFontSet) da
utilizzare per la visualizzazione del testo sul terminale. Deve essere specificata
comeXmFontListMotif. Sono supportati solo
i font non proporzionali. L'uso di un font proporzionale può produrre
effetti indeterminati. Questo font non verrà utilizzato per visualizzare
altri tipi di testo (barre dei menu, menu a scomparsa, riquadri di dialogo,
ecc.). L'impostazione predefinita prevede l'uso del valoreXmNtextFontListdel bulletin board principale (vedere
XmBulletinBoard(3X)) allo stesso modo del widgetXmText.visualBellQuesta risorsa specifica l'abilitazione di un segnale
visivo al posto del segnale acustico. Anziché emettere un segnale acustico
ad ogni ricevimento di un CTRL-G, la finestra di terminale lampeggerà.
L'impostazione predefinita è False.USO DEL PUNTATOREdttermpermette di selezionare parti
di testo. La selezione si basa sul modello specificato nel volumeInter-Client Communication Conventions Manual(ICCCM).dttermsupporta solo la selezione
primaria, e consente di copiare o incollare il testo selezionato mediante
un trasferimento primario. L'input viene trattato come un input da tastiera,
e viene inserito in corrispondenza del cursore. Le operazioni di selezione/inserimento
e i relativi valori predefiniti sono descritti più avanti.selezionePer selezionare il testo da copiare si utilizza il pulsante sinistro
del mouse. Posizionare il puntatore all'inizio del testo da copiare, premere
il pulsante sinistro, spostare il cursore alla fine del testo da copiare e
rilasciare il pulsante. È possibile annullare la selezione del testo
selezionato facendo clic con il pulsante sinistro senza spostare il mouse.inserimentoPer incollare il testo della selezione primaria si utilizza il pulsante
centrale. Il testo viene gestito come un input da tastiera.AZIONIQuesta sezione descrive le routine di azioni didtterm.bell([Percentuale])Questa azione emette il segnale acustico della tastiera
ad un volume superiore o inferiore dellapercentualespecificata al volume di base.break ()Questa azione invia un segnale di interruzione al processo
figlio.cancel ()Questa azione invia un carattereCAN(di annullamento) al processo figlio.do ()Questa azione invia al processo figlio la sequenza di
escape associata al tastoDo.edit-key(stringa)Questa azione invia al processo figlio la sequenza di
escape associata al tasto di editazione corrispondente. L'interpretazione
di questi tasti dipende dall'applicazione. I valori ammessi perstringasonofind,insert,next,prior,remove, eselect.extend-start()Avvia l'estensione del testo correntemente selezionato.extend-end ()Estende la selezione corrente. L'estensione del testo
selezionato dipende dal numero dei clic del mouse.function-key-execute(num[,tipo])Questa azione invia al processo figlio la sequenza di
escape associata al tasto funzionenum. I valori ammessi
pernumsono i numeri da1a35. Setipoviene
impostato sufunction(o non viene impostato
affatto), viene inviata la sequenza di escape associata al tasto funzionenum. Setipoviene impostato suUDK, al processo figlio viene inviata la stringa associata
al tastonumdefinito dall'utente.grab-focus ()Questa azione esegue una delle operazioni seguenti, a
seconda del numero dei clic del mouse. Un solo clic deseleziona il testo selezionato
e imposta il punto di ancoraggio della selezione in corrispondenza del puntatore;
un doppio clic seleziona una parola, un triplo clic seleziona una riga di
testo, mentre un quadruplo clic seleziona l'intero testo.hard-reset ()Questa azione produce un ripristino hard dell'emulatore
di terminale.help ()Questa azione invia al processo figlio la sequenza di
escape associata al tasto Help delDEC VT220.
L'interpretazione di questo tasto dipende dall'applicazione.keymap(nome)Questa azione definisce in modo dinamico una nuova tabella
di conversione, il cui nome di risorsa equivale al nome con il suffisso Keymap
(le maiuscole e le minuscole sono significative). Il nomeNoneripristina la tabella di traslazione originale.keypad-key-execute(stringa)Questa azione invia al processo figlio la sequenza di
escape associata al tasto del tastierino numerico corrispondente. L'interpretazione
di questi tasti dipende dall'applicazione. I valori ammessi perstringasono:f1-f4,space,tab,enter,equal,multiply,add,separator,subtract,decimal,divide, e0-9.move-cursor(direzione)Questa azione invia al processo figlio la sequenza di
escape associata al movimento del cursore. L'interpretazione di questi tasti
dipende dall'applicazione. I valori ammessi perdirezionesono:up,down,backward, eforward.redraw-display ()Questa azione ridisegna il contenuto della finestra di
testo.scroll(contatore[,unità])Questa azione produce uno scorrimento della memoria dello
schermo verso il basso secontatoreè minore di
zero, o verso l'alto secontatoreè maggiore di
zero. Il numero di righe dello scorrimento si basa sul contatore e sulle unità.
I valori ammessi per leunitàsonopage,halfpage, oline.
Il valore predefinito per le unità èline.select-adjust ()Questa azione estende la selezione. L'estensione del testo
selezionato dipende dal numero dei clic del mouse:1 clic = carattere2 clic = parola3 clic = riga4 clic = bufferselect-all ()Questa azione seleziona l'intero testo.select-page ()Questa azione seleziona tutto il testo che compare sullo
schermo.self-insert ()Questa azione invia al processo figlio il carattere associato
al tasto premuto.soft-reset ()Questa azione esegue un ripristino soft del terminale.stop(stato)Questa azione abilita/disabilita, avvia o interrompe il
processo di lettura dei dati dal processo figlio. I valori ammessi perstatosonotoggle,on, eoff.string(stringa)Questa azione inserisce la stringa di testo specificata
come se fosse stata digitata. Se la stringa contiene spazi vuoti o caratteri
non alfanumerici, essa deve essere racchiusa tra virgolette. Se inizia con
i caratteri0x, viene interpretata come una
costante in caratteri esadecimali.tab ()Questa azione invia un carattere di tabulazione al processo
figlio.visual-bell ()Questa azione produce un rapido lampeggio della finestra.Collegamenti virtualiI collegamenti ai tasti virtuali dipendono dal produttore del
software. I collegamenti virtuali non vengono applicati quando il widgetdttermè attivo e pronto per l'immissione. Per maggiori
informazioni sui collegamenti a pulsanti e tasti virtuali, vedereVirtualBindings(3X).CONVERSIONIdtterminclude una serie di conversioni
da Primitive.
Si noti che l'alterazione delle conversioni in modalità#overrideo#augmentnon è definita.Shift~Ctrl<Key>KP_Multiply:XtDisplayInstalledAccelerators()~ShiftCtrl<Key>KP_Multiply:XtDisplayAccelerators()ShiftCtrl<Key>KP_Multiply:XtDisplayTranslations()<Key>osfCancel:process-cancel()<Key>osfCopy:copy-clipboard()<Key>osfCut:copy-clipboard()<Key>osfPaste:paste-clipboard()<Key>osfBeginLine:beginning-of-buffer()<Key>osfEndLine:end-of-buffer()Shift<Key>osfUp:scroll(1,line)Shift<Key>osfDown:scroll(-1,line)<Key>osfUp:move-cursor(up)<Key>osfDown:move-cursor(down)<Key>osfLeft:move-cursor(backward)<Key>osfRight:move-cursor(forward)<Key>Find:vt-edit-key(find)<Key>Insert:vt-edit-key(insert)<Key>Select:vt-edit-key(select)<Key>Do:vt-edit-key(do)<Key>Help:vt-edit-key(help)<Key>Menu:vt-edit-key(menu)~Shift<Key>osfPageUp:vt-edit-key(prior)~Shift<Key>osfPageDown:vt-edit-key(next)<Key>osfPageUp:scroll(-1,page)<Key>osfPageDown:scroll(1,page)Mod1<Key>Break:soft-reset()Shift<Key>Break:hard-reset()~Shift ~Mod1<Key>Break:vt-break()Ctrl<Key>Cancel:stop(long)~Ctrl<Key>Cancel:stop()~Shift<Key>Tab:tab()~Mod1<Key>KP_Space:keypad-key-execute(space)~Mod1<Key>KP_Tab:keypad-key-execute(tab)~Mod1<Key>KP_Enter:keypad-key-execute(enter)~Mod1<Key>KP_F1:keypad-key-execute(f1)~Mod1<Key>KP_F2:keypad-key-execute(f2)~Mod1<Key>KP_F3:keypad-key-execute(f3)~Mod1<Key>KP_F4:keypad-key-execute(f4)~Mod1<Key>KP_Equal:keypad-key-execute(equal)~Mod1<Key>KP_Multiply:keypad-key-execute(multiply)~Mod1<Key>KP_Add:keypad-key-execute(add)~Mod1<Key>KP_Separator:keypad-key-execute(separator)~Mod1<Key>KP_Subtract:keypad-key-execute(subtract)~Mod1<Key>KP_Decimal:keypad-key-execute(decimal)~Mod1<Key>KP_Divide:keypad-key-execute(divide)~Mod1<Key>KP_0:keypad-key-execute(0)~Mod1<Key>KP_1:keypad-key-execute(1)~Mod1<Key>KP_2:keypad-key-execute(2)~Mod1<Key>KP_3:keypad-key-execute(3)~Mod1<Key>KP_4:keypad-key-execute(4)~Mod1<Key>KP_5:keypad-key-execute(5)~Mod1<Key>KP_6:keypad-key-execute(6)~Mod1<Key>KP_7:keypad-key-execute(7)~Mod1<Key>KP_8:keypad-key-execute(8)~Mod1<Key>KP_9:keypad-key-execute(9)Shift<Key>F1:vt-function-key-execute(1, UDK)Shift<Key>F2:vt-function-key-execute(2, UDK)Shift<Key>F3:vt-function-key-execute(3, UDK)Shift<Key>F4:vt-function-key-execute(4, UDK)Shift<Key>F5:vt-function-key-execute(5, UDK)Shift<Key>F6:vt-function-key-execute(6, UDK)Shift<Key>F7:vt-function-key-execute(7, UDK)Shift<Key>F8:vt-function-key-execute(8, UDK)Shift<Key>F9:vt-function-key-execute(9, UDK)Shift<Key>F10:vt-function-key-execute(10, UDK)Shift<Key>F11:vt-function-key-execute(11, UDK)Shift<Key>F12:vt-function-key-execute(12, UDK)Shift<Key>F13:vt-function-key-execute(13, UDK)Shift<Key>F14:vt-function-key-execute(14, UDK)Shift<Key>F15:vt-function-key-execute(15, UDK)Shift<Key>F16:vt-function-key-execute(16, UDK)Shift<Key>F17:vt-function-key-execute(17, UDK)Shift<Key>F18:vt-function-key-execute(18, UDK)Shift<Key>F19:vt-function-key-execute(19, UDK)Shift<Key>F20:vt-function-key-execute(20, UDK)Shift<Key>F21:vt-function-key-execute(21, UDK)Shift<Key>F22:vt-function-key-execute(22, UDK)Shift<Key>F23:vt-function-key-execute(23, UDK)Shift<Key>F24:vt-function-key-execute(24, UDK)Shift<Key>F25:vt-function-key-execute(25, UDK)Shift<Key>F26:vt-function-key-execute(26, UDK)Shift<Key>F27:vt-function-key-execute(27, UDK)Shift<Key>F28:vt-function-key-execute(28, UDK)Shift<Key>F29:vt-function-key-execute(29, UDK)Shift<Key>F30:vt-function-key-execute(30, UDK)Shift<Key>F31:vt-function-key-execute(31, UDK)Shift<Key>F32:vt-function-key-execute(32, UDK)Shift<Key>F33:vt-function-key-execute(33, UDK)Shift<Key>F34:vt-function-key-execute(34, UDK)Shift<Key>F35:vt-function-key-execute(35, UDK)~Shift<Key>F1:vt-function-key-execute(1, function)~Shift<Key>F2:vt-function-key-execute(2, function)~Shift<Key>F3:vt-function-key-execute(3, function)~Shift<Key>F4:vt-function-key-execute(4, function)~Shift<Key>F5:vt-function-key-execute(5, function)~Shift<Key>F6:vt-function-key-execute(6, function)~Shift<Key>F7:vt-function-key-execute(7, function)~Shift<Key>F8:vt-function-key-execute(8, function)~Shift<Key>F9:vt-function-key-execute(9, function)~Shift<Key>F10:vt-function-key-execute(10, function)~Shift<Key>F11:vt-function-key-execute(11, function)~Shift<Key>F12:vt-function-key-execute(12, function)~Shift<Key>F13:vt-function-key-execute(13, function)~Shift<Key>F14:vt-function-key-execute(14, function)~Shift<Key>F15:vt-function-key-execute(15, function)~Shift<Key>F16:vt-function-key-execute(16, function)~Shift<Key>F17:vt-function-key-execute(17, function)~Shift<Key>F18:vt-function-key-execute(18, function)~Shift<Key>F19:vt-function-key-execute(19, function)~Shift<Key>F20:vt-function-key-execute(20, function)~Shift<Key>F21:vt-function-key-execute(21, function)~Shift<Key>F22:vt-function-key-execute(22, function)~Shift<Key>F23:vt-function-key-execute(23, function)~Shift<Key>F24:vt-function-key-execute(24, function)~Shift<Key>F25:vt-function-key-execute(25, function)~Shift<Key>F26:vt-function-key-execute(26, function)~Shift<Key>F27:vt-function-key-execute(27, function)~Shift<Key>F28:vt-function-key-execute(28, function)~Shift<Key>F29:vt-function-key-execute(29, function)~Shift<Key>F30:vt-function-key-execute(30, function)~Shift<Key>F31:vt-function-key-execute(31, function)~Shift<Key>F32:vt-function-key-execute(32, function)~Shift<Key>F33:vt-function-key-execute(33, function)~Shift<Key>F34:vt-function-key-execute(34, function)~Shift<Key>F35:vt-function-key-execute(35, function)<KeyRelease>:key-release()<KeyPress>:insert()~Shift~Ctrl<Btn1Down>:grab-focus()Shift~Ctrl<Btn1Down>:extend-start()~Ctrl<Btn1Motion>:select-adjust()~Ctrl<Btn1Up>:extend-end()~Shift<Btn2Down>:process-bdrag()~Shift<Btn2Up>:copy-to()<EnterWindow>:enter()<LeaveWindow>:leave()<FocusIn>:focus-in()<FocusOut>:focus-out()Sequenze di escape di dttermOgnuno degli argomenti sotto elencati contiene un elenco di sequenze
di escape valide. Per informazioni più dettagliate, vedere la pagina
di spiegazionidtterm(5x).Tasti cursore, modalità VT220La tabella seguente mostra le sequenze di escape associate ai tasti
cursore in modalità normale e in modalità applicazione.Modalità tasti cursore
      TASTO            Normale   Applicazione
==================     =======   ============ 
Cursore su              Esc[A       EscOA
Cursore giù             Esc[B       EscOB
Cursore a destra        Esc[C       EscOC
Cursore a sinistra      Esc[D       EscODTastierino numerico ausiliario, modalità ANSILa tabella seguente mostra le sequenze di escape associate ai tasti
del tastierino numerico in modalità numerica e in modalità applicazione.Modalità applicazione
  TASTO       Normale   Applicazione
=========     =======   ============ 
spazio         space      EscOA 
tab            tab        EscOI 
Enter          CR/CR-LF   EscOM    
PF1            EscOP      EscOP 
PF2            EscOQ      EscOQ 
PF3            EscOR      EscOR 
PF4            EscOS      EscOS 
* (per)          *        EscOj 
+ (più)       +        EscOk 
, (virgola)      ,        EscOl 
- (meno)         -        EscOm 
. (punto)        .        EscOn 
/ (diviso)       /        EscOo 
0                0        EscOp 
1                1        EscOq 
2                2        EscOr 
3                3        EscOs 
4                4        EscOt 
5                5        EscOu 
6                6        EscOv 
7                7        EscOw 
8                8        EscOx 
9                9        EscOy 
=(uguale)        =        EscOXTasti funzione, modalità VT220La tabella seguente mostra le sequenze di escape corrispondenti ai tasti
funzione.TASTO          Sequenza di escape
=======        ==================== 
 F1                   Esc[11~
 F2                   Esc[12~
 F3                   Esc[13~
 F4                   Esc[14~
 F5                   Esc[15~
 F6                   Esc[17~
 F7                   Esc[18~
 F8                   Esc[19~
 F9                   Esc[20~
 F10                  Esc[21~
 F11                  Esc[23~
 F12                  Esc[24~
 F13                  Esc[25~
 F14                  Esc[26~
 F15                  Esc[28~
 F16                  Esc[29~
 F17                  Esc[31~
 F18                  Esc[32~
 F19                  Esc[33~
 F20                  Esc[34~
 Help                 Esc[28~
 Menu                 Esc[29~
 Find                 Esc[1~
 Insert               Esc[2~
 Remove               Esc[3~
 Select               Esc[4~
 Prior                Esc[5~
 Next                 Esc[6~Tasti funzione, modalità sunFunctionKeysLa tabella seguente mostra le sequenze di escape corrispondenti ai tasti
funzione.TASTO           Sequenza di escape
========        ==================== 
 F1                   Esc[224z
 F2                   Esc[225z
 F3                   Esc[226z
 F4                   Esc[227z
 F5                   Esc[228z
 F6                   Esc[229z
 F7                   Esc[230z
 F8                   Esc[231z
 F9                   Esc[232z
 F10                  Esc[233z
 F11                  Esc[192z
 F12                  Esc[193z
 F13                  Esc[194z
 F14                  Esc[195z
 F15                  Esc[196z
 F16                  Esc[197z
 F17                  Esc[198z
 F18                  Esc[199z
 F19                  Esc[200z
 F20                  Esc[201z
 F21 (R1)             Esc[208z
 F22 (R2)             Esc[209z
 F23 (R3)             Esc[210z
 F24 (R4)             Esc[211z
 F25 (R5)             Esc[212z
 F26 (R6)             Esc[213z
 F27 (R7)             Esc[214z
 F28 (R8)             Esc[215z
 F29 (R9)             Esc[216z
 F30 (R10)            Esc[217z
 F31 (R11)            Esc[218z
 F32 (R12)            Esc[219z
 F33 (R13)            Esc[220z
 F34 (R14)            Esc[221z
 F35 (R15)            Esc[222z
 Help                 Esc[196z
 Menu                 Esc[197z
 Find                 Esc[1z
 Insert               Esc[2z
 Remove               Esc[3z
 Select               Esc[4z
 Prior                Esc[5z
 Next                 Esc[6zSequenze di escape ricevuteLa tabella seguente descrive le sequenze di escape ricevute supportate
da dtterm.Sequenza
di escape      Descrizione
=========      ===========
Ctrl-G         Segnale acustico (Ctrl-G)
Ctrl-H         Cancellazione all'indietro (Ctrl-H)
Ctrl-I         Tabulazione orizzontale (HT) (Ctrl-I)
Ctrl-J         Avanzamento riga (NL) (Ctrl-J)
Ctrl-K         Tabulazione verticale, uguale ad avanzamento riga
Ctrl-L         Avanzamento pagina, uguale ad avanzamento riga
Ctrl-M         Ritorno a capo (Ctrl-M)
Esc ( B        Designazione (font base) ASCII come G0.
Esc ( 0        Designazione grafica speciale DEC (disegno) come G0.
Esc ) B        Designazione (font base) ASCII come G1.
Esc ) 0        Designazione grafica speciale DEC (disegno) come G1.
Esc * B        Designazione (font base) ASCII come G2.
Esc * 0        Designazione grafica speciale DEC (disegno) come G2.
Esc + B        Designazione (font base) ASCII come G3.
Esc + 0        Designazione grafica speciale DEC (disegno) come G3.
Ctrl-N         Mappatura di G1 in GL.
Ctrl-O         Mappatura di G0 in GL.
Esc n          Mappatura di G2 in GL.
Esc o          Mappatura di G3 in GL.
Esc N          Mappatura di G2 in GL per il carattere successivo.
Esc O          Mappatura di G3 in GL per il carattere successivo.
Esc Spazio F   Selezione dei caratteri di controllo C1 a 7 bit. In questo 
               modo, l'utility dtterm invia tutti i caratteri di controllo C1  
               all'host come sequenze di escape a 7 bit. In altre parole, 
               il CSI viene inviato all'host come&newline;f2Esc&newline;fP [.
Esc Spazio G   Selezione dei caratteri di controllo C1 a 8 bit. In questo 
               modo, l'utility dtterm invia tutti i caratteri di controllo C1
               all'host come codici di controllo a 8 bit. In altre parole,  
               il CSI viene rinviato come il valore esadecimale 0x9B.
Esc#8          Test di allineamento dello schermo DEC (DECALN)
Esc7           Cursore di salvataggio (DECSC)
Esc8           Cursore di ripristino (DECRC)
Esc=           Tastierino in modalità applicazione (DECPAM)
Esc>           Tastierino in modalità normale (DECPNM)
EscD           Indice (IND)
EscE           Riga successiva (NEL)
EscH           Impostazione tabulazioni (HTS)
EscM           Indice invertito (RI)
EscPpi;pi|pi/cifre esadecimali;pi/cifre esadecimali;...Esc&newline;
               Stringa di controllo unità (DCS)
EscZ           Restituzione dell'ID del terminale (DECID)
Escc           Ripristino totale (RIS)
Escn           Selezione del set di caratteri G2 (LS2)
Esco           Selezione del set di caratteri G3 (LS3)
Esc[pi"p       Selezione del livello di compatibilità (DECSCL)
Esc[pi@        Inserimento di caratteri vuoti (ICH)
Esc[piA        Cursore in su (CUU)
Esc[piB        Cursore in giù (CUD)
Esc[piC        Cursore in avanti (CUF)
Esc[piD        Cursore all'indietro (CUB)
Esc[piF        Cursore allapiesima riga precedente (CPL)
Esc[piG        Cursore alla colonna p (CHA)
Esc[pi;piH     Posizione del cursore (CUP)
Esc[piJ        Cancellazione nello schermo (ED)
Esc[piK        Cancellazione nella riga (EL)
Esc[piL        Inserimento di righe (IL)
Esc[piM        Cancellazione di righe (DL)
Esc[piP        Cancellazione di caratteri (DCH)
Esc[piS        Scorrimento in alto di p righe (SU)
Esc[piT        Scorrimento in basso (SD)
Esc[piX        Cancellazione dipicaratteri (ECH)
Esc[pic        Invio degli attributi dell'unità
Esc[pi;pif     Posizione orizzontale e verticale (HVP)
Esc[pig        Cancellazione tabulazioni (TBC)
Esc[pih        Impostazione della modalità (SM)
Esc[pil        Ripristino della modalità (RM)
Esc[pim        Attributi dei caratteri (SGR)
Esc[pin        Status report dell'unità (DSR)
Esc[pi;pir     Impostazione dell'area di scorrimento (DECSTBM)
Esc[pix        Richiesta dei parametri del terminale
Esc[?pih       Impostazione del modo privato DEC (DECSET)
Esc[?pil       Ripristino del modo privato DEC (DECRSET)
Esc[?pin       Stato del modo privato DEC (DSR)
Esc[?pir       Ripristino dei valori del modo privato DEC
Esc[?pis       Salvataggio dei valori del modo privato DEC
Esc]?pi;piCtrl-G
               Impostazione dei parametri di testo
Esc]p1;p2;p3tSequenze di escape Sun
Esc_piEsc\     Programma applicativo
Esc[?piK       Cancellazione selettiva su una riga (DECSEL)
Esc[?piJ       Cancellazione selettiva sullo schermo (DECSED)
Esc!p          Ripristino soft del terminale (DECSTR)Informazioni sulla tastiera di dttermfunzioni della tastieraQuesta sezione contiene una lista delle funzionalità della tastiera
associate a dtterm. Essa include solo i tasti che hanno una funzione particolare
in dtterm. Le funzioni descritte sono disponibili solo quando l'area di testo
del terminale è pronta per l'inserimento da tastiera.Nonsono disponibili quando l'area pronta per l'immissione è
un riquadro di dialogo, un menu a tendina o un menu a scomparsa. In alcuni
casi, i modificatori aggiuntivi vengono ignorati. Ad esempio, la descrizione
relativa a F1 non si applica a Shift F1, ma la descrizione di Tab si applica
sia a Tab che a Shift Tab.<Key>Home          Scorre all'inizio del buffer
Shift<Key>Home     Scorre alla fine del buffer
Shift<Key>Prior    Scorre in su di una pagina
Shift<Key>Next     Scorre in giù di una pagina
<Key>Up            Invia la sequenza di escape come descritto
                   in dtterm(5x)
<Key>Down
<Key>Left
<Key>Right
<Key>Prior
<Key>Next
<Key>Find
<Key>Insert
<Key>Select

<Key>Cancel        Abilita/disabilita l'output del processo secondario.

<Key>Tab           Invia un carattere di tabulazione.

<Key>Break         Invia un'interruzione RS232 al processo secondario.
Meta<Key>Break     Ripristino soft come descritto in dtterm(5x).
Shift<Key>Break    Ripristino hard come descritto in dtterm(5x).

<Key>F1 -          Invia la sequenza di escape con tasti funzione
<Key>F35           come descritto in dtterm(5x). 


Shift<Key>F1 -     Invia la sequenza definita dall'utente (se presente)
Shift<Key>F35      per quel tasto.


Ctrl<Key>F10       Attiva la barra dei menu.

ShiftCtrl<Key>F10  Attiva il menu a scomparsa.

Esc                 Invia il carattere di escape.

<Key>KP_F1 -       Invia la sequenza di escape come descritto
<Key>KP_F4         in dtterm(5x)


<Key>KP_0 -        Invia la sequenza di caratteri o di escape
<Key>KP_9          come descritto in dtterm(5x).


<Key>KP_Equal
<Key>KP_Multiply
<Key>KP_Add
<Key>KP_Separator
<Key>KP_Subtract
<Key>KP_Decimal
<Key>KP_Divide
<Key>KP_Space
<Key>KP_Tab
<Key>KP_Enter