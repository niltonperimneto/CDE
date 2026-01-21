Attività di dttermAvviare dttermL'emulatore di terminaledttermdel &ProductName;
può essere avviato in diversi modi:Dal Pannello principaleDalla Gestione di fileDalla riga comandi di una finestra
di terminale già apertaDalla Gestione di applicazioniCon l'opzione Nuova del menu a tendina
Finestra didttermPer avviare dtterm dal Pannello principaleavvio di un emulatore di terminalechiusura di un emulatore di terminaleTerminale: controlloIl controllo del terminale si trova nel pannello secondario Applicazioni
personali.Fare clic sul controllo del terminale.
La spia di occupato lampeggerà per indicare che il terminale è
stato attivato.Dopo qualche istante comparirà
la finestra didtterm.Per avviare dtterm dalla Gestione di fileScegliere Aprire terminale dal menu
File.&sigspace;Questa opzione avviadttermnella stessa
directory
visualizzata nella Gestione di file.Per avviare un emulatore di terminale diverso da dttermPer usare un emulatore di terminale diverso dadttermoccorre avviarlo dalla riga comandi
di una finestra di terminale già aperta.Dalla riga comandi, digitare il nome
dell'emulatore di terminale seguito eventualmente dalle opzioni desiderate.
Ad esempio, per avviarextermdigitare:xterm  [opzioni] &opzionirappresenta le opzioni disponibili per personalizzare l'emulatore di
terminale.&specifica che l'emulatore di terminale verrà eseguito in background,
consentendo all'utente di continuare a lavorare nella
finestra originale.Se non specificato diversamente con un'opzione, l'emulatore di terminale
viene avviato nello spazio di lavoro corrente.Per avviare dtterm dal menu FinestraDa una finestra didttermgià aperta, scegliere Nuova dal menu Finestra.
Verrà avviata una copia della finestra didttermoriginale.EsempiIl comando seguente avvia una finestra didttermnello spazio di lavoroNote sul progetto:dtterm -xrm '*workspaceList: "Note sul progetto"' &Il comando seguente avvia una finestra didttermsu uno schermo del sistema "lgmcd":dtterm -display lgmcd:0.1 &Vedere anchePer informazioni dettagliate sulle
opzioni disponibili perdtterm, vedere la
pagina di spiegazionidtterm (1X).Chiudere dtterm