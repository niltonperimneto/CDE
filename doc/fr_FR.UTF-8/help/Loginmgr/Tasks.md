Gestionnaire de connexion - TâchesLancement et arrêt des sessionsAutres types de connexionPersonnalisation du lancement et de l'arrêt des sessionsMode de lancement de la session suivanteSauvegarde d'une session en coursDéfinition des paramètres de déconnexionOuverture d'une session de BureauConnexion à une session de BureauEntrez votre ID utilisateur
et appuyez sur Entrée ou cliquez sur OK.Entrez votre mot de passe
et appuyez sur Entrée ou cliquez sur OK.Si le Gestionnaire de connexion ne reconnaît pas votre ID utilisateur
ou
votre mot de passe, faites une nouvelle tentative.Lors de la connexion, le Gestionnaire de connexion lance une session:S'il
s'agit de votre première connexion, une nouvelle session est lancée.Sinon, la session précédente
est restaurée.Fermeture d'une session de BureauFermeture d'une session de BureauSession:arrêtTableau de bord:sortieMenu Espace de travail:commande de déconnexionCliquez
sur l'objet de commande de sortie du Tableau de bord.Ou,sélectionnez Déconnexion dans le menu Espace de travail (qui
s'affiche lorsque vous appuyez sur le bouton droit de la souris).Lorsque vous vous déconnectez, le Gestionnaire de sessions sauvegarde
la session en cours afin qu'elle puisse être restaurée lors de
la connexion suivante.Utilisation d'une session dans une langue différenteLangue:modificationSession:langue différenteSélectionnez
Langue dans le menu Options de l'écran de connexion.Sélectionnez le
groupe de langues qui comporte la langue voulue.Sélectionnez une
langue dans le groupe de langues. Une fois cette opération effectuée,
l'écran de connexion s'affiche de nouveau dans la langue sélectionnée.Connectez-vous.La langue par défaut du système est définie par
l'administrateur système. Le menu Options permet d'accéder à
d'autres langues. Le choix d'une langue définit la variable d'environnement
LANG pour la durée de la session. La langue par défaut est restaurée
à la fin de la session.Ouverture et fermeture d'une session monofenêtreOuverture et fermeture d'une session monofenêtreSession monofenêtreEmulateur de terminal:session monofenêtreUne session monofenêtre est une session simple qui lance le Gestionnaire
de
fenêtres et une fenêtre de terminal unique. Utilisez-la lorsque
vous devez
exécuter plusieurs commandes avant d'ouvrir une session de Bureau.ConnexionSur l'écran de
connexion, sélectionnez Session dans le menu Options.Sélectionnez Session
monofenêtre dans le sous-menu Session.Connectez-vous.DéconnexionEntrez
la commandeexitdans la fenêtre de
terminal.Mode de connexion à partir de la ligne de commandeServeur X:arrêt temporaireConnexion à partir de la ligne de commandeSur l'écran de connexion,
sélectionnez Connexion à partir de la ligne de commande dans
le menu Options; l'écran de connexion disparaît et est remplacé
par une invite de console.Entrez votre ID utilisateur et votre
mot de passe.Le mode de connexion à partir de la ligne de commande ne permet
pas de lancer une session du Bureau. Lorsque ce mode est activé, le
Bureau est en attente. La connexion se fait à partir du système
d'exploitation plutôt que du Gestionnaire de connexion. Aucune fenêtre
ne s'affiche car le serveur X n'est pas lancé.Dans certains types de configuration (terminal X, par exemple), l'option
Connexion à partir de la ligne de commande n'est pas disponible.Sortie du mode de connexion à partir de la ligne de commandeConnexion à partir de la ligne de comandeEntrezexità l'invite de la ligne de commande.Sauvegarde d'une session initialeSauvegarde:session initialeSession initiale:sauvegardeCliquez sur l'objet de commande Gestionnaire
de configuration du Tableau de bord.Cliquez sur l'objet de commande Lancement
du Gestionnaire de configuration.Cliquez sur Enregistrer la session
initiale, dans la boîte de dialogue Lancement.Cliquez sur OK dans la boîte
de dialogue de confirmation qui s'affiche.Cliquez sur OK.La session est sauvegardée "telle quelle".Voir aussiMode de lancement de la session suivanteSauvegarde d'une session en coursDéfinition des paramètres de déconnexionLancement automatique de la session initiale lors de la connexionLancement:session initiale à la connexionSession initiale:lancement automatique à la connexionCliquez sur l'objet de commande Gestionnaire
de configuration du Tableau de bord.Cliquez sur l'objet de commande Lancement
du Gestionnaire de configuration; la boîte de dialogue Lancement s'affiche.Sélectionnez Revenir à
la session initiale.Cliquez sur OK.Lorsque vous sélectionnez Revenir à la session initiale,
le Gestionnaire de sessionsne sauvegarde pasla session
en cours lors de la déconnexion.Voir aussi..Choix de la session à initialiserchoisir:entre la session en cours et la session initialesessions en cours et initiale:choisir entreDans l'écran de connexion, cliquez sur le bouton Options.sessionchoix à la connexionCliquez sur Session. Le menu Session liste toutes les sessions
disponibles:En coursLance la session la plus récente.InitialeLance votre session initiale (si vous en avez défini une).nom_d'affichage- En coursLance la session en cours pour votre affichage, s'il en existe une.
Sinon, le système créera une nouvelle session en cours pour
votre affichage. La première session existante parmi les suivantes
sera lancée: une session initiale spécifique à votre
affichage, la session initiale générique ou une nouvelle session
utilisateur.nom_d'affichage- InitialeLance la session initiale pour votre affichage, s'il en existe une.
Si non, le système créera une nouvelle session spécifique
à votre affichage. Le système lancera votre session initiale
générique (si elle existe) ou une nouvelle session utilisateur.Session de sauvegardeDémarre une session de sauvegarde.Cliquez sur la session à initialiser.Voir également..Variables personnelles d'environnementDéfinition:variables personnelles d'environnementVariables personnelles d'environnement:définitionVous pouvez définir des variables personnelles d'environnement
dans le fichier scriptRép_personnel/.dtprofile.Editez le fichierRép_personnel/.dtprofile.Ajoutez des lignes au fichier afin
de définir les variables d'environnement.Les syntaxes de commande sh ou ksh sont acceptées dans ce fichier.
Vous devez ajouter des définitions de variables d'environnement uniquement,
et non des commandes d'E-S de terminal, par exemple "tset" ou "stty".Les fichiers/etc/profileetRép_personnel/.profilene sont pas lus par le Bureau
car ils sont susceptibles de contenir des commandes d'E-S de terminal inappropriées
pour une interface graphique.Le Bureau définit automatiquement les variables d'environnement
suivantes pour chaque utilisateur:DISPLAYdéfinie en fonction de la valeur de la première zone du
fichier Xservers.EDITORdéfinie en fonction de l'éditeur par défaut du
Bureau.ENVprend la valeur "Rép_personnel/.kshrc".HOMEdéfinie en fonction du répertoire personnel de l'utilisateur
(dans /etc/passwd).KBD_LANGdéfinie en fonction de la valeur de $LANG pour certaines langues
(voir la commande Xsession).LANGdéfinie en fonction de la langue NLS de l'écran.LC_ALL, LC_MESSAGESdéfinies en fonction de la valeur de $LANG.LOGNAMEdéfinie en fonction du nom de l'utilisateur.MAILprend la valeur "/var/mail/$USER".PATHprend la valeur de la ressource Dtlogin "userPath".USERdéfinie en fonction du nom de l'utilisateur.SHELLdéfinie en fonction du shell par défaut de l'utilisateur
(dans /etc/passwd).TERMprend la valeur xterm.TZprend la valeur de la ressource Dtlogin "timeZone".Voir aussi.La page de manuel Dtlogin(1X), pour
plus de détails sur la définition des variables d'environnement.Utilisation du fichier .profile ou .login existantFichier d'environnement shell avec.dtprofileSi vous disposez d'un fichier d'environnement shell existant (.profileou.login), la
procédure ci-dessous vous permet de continuer à l'utiliser,
ce qui évite d'affecter deux fois des variables (dans.dtprofileet dans le fichier d'environnement shell). Vous
pouvez éditer le fichierRép_personnel/.profile
(ou .login)afin de pouvoir l'utiliser avec ET sans le Bureau.Créez deux sections dans le
fichier.profile ou .login:Une section contenant les commandes
qui ne s'appliquent pas au Bureau (par exemple, des commandes qui requièrent
des fonctions d'E-S de terminal, ou des variables pour lesquelles vous souhaitez
conserver les valeurs par défaut du Bureau). Accompagnez-les d'une
instruction conditionnelle qui recherchera la définition de la variable
d'environnement "DT".Une section contenant les variables
qui seront appliquées, que le Bureau soit lancé ou non.Dans le fichier.dtprofile, annulez la mise en commentaire de la ligne suivante,
afin qu'il prenne en compte les valeurs du fichier d'environnement shell:DTSOURCEPROFILE=trueReconnectez-vous.L'exemple suivant indique comment séparer le fichier en deux
sections, l'une destinée au Bureau et l'autre contenant des variables
qui s'appliquent aux deux environnements.Exemple pour sh/ksh#
  #  Commandes et variables d'environnement utilisées lorsque le Bureau
  #  n'est pas lancé.
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
  # Variables communes aux deux environnements.
  #
  PATH=$HOME/bin:$PATH
       ...Exemple pour csh#
  #  Commandes et variables d'environnement utilisées lorsque le Bureau
  #  n'est pas lancé.
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
  # Variables communes aux deux environnements.
  #
  setenv PATH $HOME/bin:$PATHUne erreur dans le fichier .dtprofile ou .profile (.login) peut empêcher
la connexion. Pour remédier à cet incident, ouvrez une session
monofenêtre et corrigez l'erreur.Si un émulateur de terminal est lancé avec l'option-ls, le fichier.loginou.profileest lu automatiquement.