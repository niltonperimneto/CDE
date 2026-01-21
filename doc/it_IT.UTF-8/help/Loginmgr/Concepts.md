
# Introduzione alla Gestione del login





# Introduzione alle sessioni del desktop
avvio di una sessione del desktoplogin in una sessione del desktopDesktop: avvio di una sessionesessioni: avvio dal desktopuso della sessione inizialesessione iniziale: definizionesessione corrente: definizione

Una sessione è un insieme di applicazioni, impostazioni e risorse presenti sul desktop, che ha inizio al login e termina con il logout. Lo schermo di login, creato dalla Gestione del login, costituisce la "porta" di accesso al desktop. Fornisce un interfaccia per inserire l'ID utente e la parola chiave.

Il menu Opzioni dello schermo di login elenca le opzioni disponibili per l'accesso al desktop: ad esempio, è possibile avviare una sessione protetta o impostare una lingua diversa da quella predefinita.

Dopo l'autenticazione dell'ID utente e della parola chiave, la Gestione del login viene sostituita dalla Gestione delle sessioni, un insieme di convenzioni e protocolli che consente di salvare e ripristinare le sessioni degli utenti. In questo modo, al login nel sistema ogni utente può ritrovare esattamente le stesse applicazioni, impostazioni e risorse presenti sul desktop al momento dell'ultimo logout. La Gestione delle sessioni permette di salvare e ripristinare:

L'aspetto e le impostazioni operative del desktop &emdash; ad esempio, font, colori e impostazioni del mouse.

Le applicazioni in esecuzione al momento del logout &emdash; ad esempio, le finestre della Gestione di file e dell'Editor di testo. Alcuni tipi di applicazioni non possono essere salvati e ripristinati. Ad esempio, se si avvia l'editorvida una riga comandi in una finestra di Terminale, la Gestione delle applicazioni non sarà in grado di ripristinarlo.

Quando si effettua il primo login, il desktop carica una sessione predefinita. Successivamente, la Gestione delle sessioni supporta le cosiddette sessioni correnti e sessioni iniziali.
# Sessione iniziale


Quando si effettua il primo login nel desktop, la Gestione delle sessioni genera una sessione iniziale usando i valori predefiniti nel sistema. Questa sessione comprende l'avvio automatico della Gestione di file e dell'Introduzione al desktop.
# Sessione corrente


Normalmente, il desktop salva le informazioni sulla sessione al momento del logout e le utilizza all'avvio della sessione successiva. Se nel corso della sessione si avviano o chiudono delle applicazioni, o si utilizza la Gestione degli stili per cambiare l'aspetto e il comportamento del sistema, le modifiche apportate verranno riprodotte nella sessione successiva.

La sessione in esecuzione viene sempre considerata lasessione corrente, si tratti di una sessione iniziale ripristinata al login, di una sessione corrente salvata o della sessione iniziale predefinita del sistema. In base alle impostazioni della finestra Avvio della Gestione degli stili, all'uscita dal desktop la Gestione delle sessioni salva automaticamente la sessione corrente. Al login successivo nel desktop, la sessione corrente così salvata verrà automaticamente ripristinata; in altre parole, il desktop verrà ripresentato esattamente con lo stesso contenuto e con le stesse impostazioni in uso al momento dell'ultimo logout.
# Sessione iniziale


Il desktop supporta anche unasessione iniziale. Con questo termine si indica una sessione esplicitamente salvata dall'utente, una "fotografia" della sessione corrente in un determinato momento. Dopo avere salvato una sessione iniziale, è possibile specificare al desktop di ripristinare quella sessione al login successivo anziché la sessione corrente.

Vedere le attività seguenti:






# Altri modi per effettuare il login
sessioni: tipiavvio di una sessione protettaavvio di una sessione dalla riga comandisessione protetta: avvioLogin dalla riga comandi: uso per l'avvio di una sessione

Oltre alla sessione normale del desktop, esistono altri tipi di sessioni:

* **Sessione protetta** 

Una sessione protetta avvia una finestra di Terminale e una Gestione delle finestre. Può essere usata per eseguire una serie di comandi prima del login in una sessione del desktop.
(Vedere.)
* **Login della riga comandi** 

Il login della riga comandi permette di uscire temporaneamente dal desktop per operare nella console del sistema.
(Vedere.)
