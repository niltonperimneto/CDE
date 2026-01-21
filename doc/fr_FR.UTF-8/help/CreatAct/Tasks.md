
# Créer action - Tâches

# Création et édition d'actions





# Création et édition de types de données





# Icônes d'actions et de types de données



# Création d'une action à l'aide de Créer une action
Action:création

Ouvrez le Gestionnaire d'applications et cliquez deux fois sur Créer
une action, dans le groupe Applications du Bureau.

La fenêtre principale de l'utilitaire s'affiche.

Tapez le nom de l'action dans la zone Nom (libellé de l'icône).

Tous les caractères sont autorisés, à l'exception des espaces.

Indiquez l'icône de l'application dans la zone Icônes d'action
(l'icône par défaut est affichée).

Pour utiliser une autre icône, sélectionnez Rech. ensemble pour
ouvrir la boîte de dialogue correspondante (voir la section).

Pour créer une icône, sélectionnez Editer une icône pour lancer
l'Editeur d'icônes (voir la section).

Dans la zone Commande exécutée à l'ouverture de l'action, tapez la
commande de lancement de l'action.

Pour indiquer des fichiers comme arguments, utilisez la syntaxe$`n`. Par exemple:emacs
bitmap $1
diff $1 $2
lp -oraw $1

Si un argument de la commande est un fichier ($`n`), l'icône
d'action sera une zone de pose.

Les commandes ne sont pas transmises à un shell, à moins que vous
ne l'indiquiez explicitement. Par exemple:/bin/sh -c `ps | lp'
/bin/sh -c `spell $1 | more'

Dans la zone Aide sur l'icône, tapez le texte qui s'affichera lorsque
l'utilisateur demandera à visualiser l'aide sur l'icône
d'action.

Le retour à la ligne est automatique. Cependant, les coupures affichées
ici ne seront pas conservées dans l'aide en ligne. Si vous souhaitez
forcer une coupure, utilisez&bsol;n.

Les options Type de fenêtre permettent de sélectionner le support
graphique voulu:

* **Graphique (X-Window)** 

L'application crée sa propre fenêtre.
* **Terminal (fermeture auto.)** 

L'application s'exécute dans une fenêtre de terminal,
fermée automatiquement lorsque l'utilisateur quitte
l'application.
* **Terminal (fermeture manuelle)** 

L'application s'exécute dans une fenêtre de terminal,
qui reste ouverte jusqu'à ce que l'utilisateur demande
explicitement sa fermeture.
* **Pas de sortie** 

Aucune sortie associée à l'application ne s'affiche.


Exécutez la procédure suivante:

Si l'application contient des fichiers de données, auxquels vous
voulez associer des types de données, reportez-vous à la section.

Dans le cas contraire, procédez comme suit:

sauvegardez l'action à l'aide de l'option Sauvegarder du menu
Fichier;

testez la nouvelle action en cliquant deux fois sur l'icône
correspondante, située dans votre répertoire personnel.
# Création d'un type de données à l'aide de l'utilitaire Créer une action
Type de données:création

Définissez l'action associée à l'application. Pour ce faire, suivez
les étapes 1 à 6 de la section.

Sélectionnez l'option Affich. fonctions étendues du menu Options
afin d'agrandir la fenêtre principale de Créer une action.

Si vou souhaitez que l'icône d'action vous invite à indiquer
un nom de fichier lorsque vous cliquez deux fois dessus, tapez le texte
du message dans la zone A l'ouverture de l'action, demander aux
utilisateurs.

Cette zone doit être complétée si la syntaxe de la commande de
lancement de l'application comporte un nom de fichier.

Dans le cas contraire, elle doit rester à blanc.

Si l'argument est facultatif, deux possibilités se présentent:
vous pouvez taper un message, auquel cas vous serez invité à indiquer
un nom de fichier lorsque vous cliquerez deux fois sur l'icône, ou
laisser la zone à blanc, afin que l'action soit exécutée en utilisant
une chaîne nulle comme argument.

Définissez les types de fichiers pris en charge:

si l'action accepte n'importe quel type de données, sélectionnez
Tous;

si seul(s) le(s) type(s) de données créé(s) pour l'application
sont pris en charge, sélectionnez Liste ci-dessus seulement.

Au départ, la liste Types de données utilisant l'action est vide; elle
est complétée à mesure que vous créez les types de données associés à
l'application.

Sélectionnez l'option Ajouter pour afficher la boîte de dialogue
correspondante.

Eventuellement, si vous ne souhaitez pas utiliser le nom par défaut
du type de données, vous pouvez taper le nom de votre choix dans
la zone appropriée (les espaces ne sont pas autorisés).

Sélectionnez le bouton Editer, situé en regard de la zone Caractéristiques,
pour afficher la boîte de dialogue correspondante.

Cette boîte de dialogue permet de définir les spécificités du type de
données en cours de création. Vous pouvez choisir un ou plusieurs
critères (Modèle de nom seulement ou Modèle de nom et Modèle
d'autorisations, par exemple).

Cliquez sur Fichier ou Dossier pour indiquer si vous souhaitez que
le type de données représente un fichier ou un dossier.

Si le type de données est fonction du nom, cochez la case Modèle de nom
et remplissez la zone. Les caractères génériques '`

'' et?sont
autorisés:

* ***** 

correspond à une chaîne de caractères,
* **?** 

correspond à un caractère.


Si le type de données est fonction des autorisations, cochez la case
Modèle d'autorisations et indiquez les permissions associées au type
de données:

* **En fonction** 

Les autorisations spécifiées doivent être appliquées,
* **Hors fonction** 

Les autorisations spécifiées ne doivent pas être appliquées,
* **Indifféremment** 

Les autorisations ne sont pas prises en compte.


Si le type de données dépend du contenu, cochez la case Contenu et
indiquez les informations requises (forme à rechercher et type de
contenu). Vous pouvez également définir le point de départ (octet)
de la recherche.

Cliquez sur OK pour fermer la boîte de dialogue.

Les paramètres définis apparaissent dans la zone Caractéristiques
de la boîte de dialogue Ajouter un type de données.

Lors de la définition des autorisations, vous devez utiliser les
conventions suivantes:

* **d** 

Répertoire
* **r** 

Autorisation de lecture
* **w** 

Autorisation d'écriture
* **x** 

Exécutable
* **!** 

NOT
* **&** 

AND


Tapez l'aide associée au type de données dans la zone de texte
appropriée.

Utilisez les icônes de la zone Icônes du type de données pour associer
une icône à l'application.

Au départ, les icônes par défaut sont affichées.

Pour utiliser une autre icône, sélectionnez Rech. ensemble pour
ouvrir la boîte de dialogue correspondante (voir la section).

Pour créer une icône, sélectionnez Editer une icône pour lancer
l'Editeur d'icônes (voir la section).

Si l'application prend en charge une commande permettant d'imprimer les
fichiers de données à partir de la ligne de commande, tapez-la dans la
zone Commande d'impression. Pour indiquer un fichier comme argument,
utilisez la syntaxe$`n`.

Pour fermer la boîte de dialogue et ajouter le type de données créé à
la liste de la fenêtre Créer une action, cliquez sur OK.
# Association d'icônes aux actions et aux types de données


La fenêtre principale Créer une action et la boîte de dialogue Ajouter
un type de données contiennent des boutons permettant d'associer des
icônes aux actions et aux types de données.

Pour utiliser une icône existante, sélectionnez Rech. ensemble
(voir la section).

Pour créer une icône à l'aide de l'Editeur d'icônes, sélectionnez
Editer une icône (voir la section).
# Boîte de dialogue Rech. ensemble


Cette boîte de dialogue s'affiche lorsque vous sélectionnez l'option
Rech. ensemble de la fenêtre principale de Créer une action ou de la
boîte de dialogue Ajouter un type de données.

Elle permet d'indiquer les éléments suivants:

Une icône située dans un dossier de la liste des dossiers d'icônes,
qui contient tous les dossiers figurant dans le chemin de recherche
des icônes.

Une icône comprise dans un module d'enregistrement qui sera intégré
au Bureau à l'aide dedtappintegrate.
# Indication d'une icône située dans un dossier d'icônes


Dans la liste des dossiers d'icônes, cliquez deux fois sur le dossier
contenant l'icône.

Le contenu de ce dossier s'affiche dans la liste des fichiers d'icônes.

Cliquez alors sur l'icône à utiliser.

Cliquez sur OK.
# Indication d'une icône située dans un module d'enregistrement


Si vous êtes administrateur système ou programmeur et que vous créez
un module d'enregistrement, vous devez savoir que les fichiers d'images
se trouvent dans le répertoire suivant du module:`app_root`/dt/appconfig/icons/`langue`

Une fois l'enregistrement effectué, ces fichiers se trouvent dans un
dossier d'icônes; les définitions d'actions et de types
de données doivent donc utiliser le nom de base du fichier.

Pour définir le nom de base d'une icône qui ne se trouve pas dans un
dossier d'icônes:

Dans la zone Entrez un nom de fichier d'icône, tapez le nom de
base du fichier d'icône.

Cliquez sur OK.

Un message d'erreur s'affiche, indiquant que le fichier d'icône ne se
trouve dans aucun dossier d'icônes.

Cliquez sur OK pour supprimer le message et fermer la boîte de
dialogue Rech. ensemble.

Les zones Icônes d'action et Icônes du type de données ne contiennent
aucune image; elles ne seront disponibles qu'une fois l'application
enregistrée.
# Création d'une image d'icône


Sélectionnez Editer une icône dans la fenêtre Créer une action ou
Editer un type de données pour lancer l'Editeur d'icônes.

Créez la nouvelle icône. Pour plus de détails, reportez-vous aux
sections suivantes:

Editeur d'icônes - Tâches;

.

Pour sauvegarder l'icône que vous venez de créer, sélectionnez l'option
Sauvegarder du menu Fichier.

L'icône doit être sauvegardée dans le dossier`Dossier_personnel`/.dt/icons(voir la section).

Si vous êtes administrateur système ou programmeur et que vous créez
un module d'enregistrement, vous devez sauvegarder le fichier d'icône
dans le répertoire`approot`/dt/appconfig/icons/`langue`.
# Tailles et noms d'icônes


Les noms des icônes du Bureau respectent les conventions suivantes:

* **Taille (pixels)** 

Nom
* **32 x 32** 

`nom_base`.m.pmou`nom_base`.m.bm
* **16 x 16** 

`nom_base`.s.pmou`nom_base`.s.bm


Dans le cas des icônes d'actions, le nom de base est le nom de l'action.

Dans le cas des icônes de types de données, le nom de base est le nom
du type de données.
# Edition d'une action
Actions:édition

Vous pouvez éditer une action à l'aide de Créer une action si:

l'action a été créée à l'aide de cet utilitaire;

etle fichier contenant sa définition n'a jamais été édité
manuellement (à l'aide d'un éditeur de texte).

Ouvrez le Gestionnaire d'applications et cliquez deux fois sur
Créer une action, dans le groupe Applications du Bureau.

La fenêtre principale de l'utilitaire s'affiche.

Sélectionnez l'option Ouvrir du menu Fichier; la boîte de dialogue
correspondante s'affiche.

Sélectionnez le fichier`nom_action`.dtcontenant la définition
de l'action dans la liste de fichiers.

Cliquez sur OK.
# Edition d'un type de données
Type de données:édition

Vous pouvez éditer un type de données à l'aide de Créer une action si:

le type de données a été créé à l'aide de cet utilitaire;

etle fichier contenant le type de données n'a jamais été
édité manuellement (à l'aide d'un éditeur de texte).

Ouvrez le Gestionnaire d'applications et cliquez deux fois sur
Créer une action, dans le groupe Applications du Bureau.

La fenêtre principale de l'utilitaire s'affiche.

Sélectionnez l'option Ouvrir du menu Fichier; la boîte de dialogue
correspondante s'affiche.

Cliquez deux fois sur le fichier contenant la définition du type
de données.

Le nom de ce fichier est`nom_action`.dt, oùnom_actioncorrespond au nom de l'action créée simultanément avec le type de
données.

Cliquez sur Avancée.

Dans la liste des types de données utilisant l'action, sélectionnez
le type de données à éditer.

Sélectionnez Editer; la boîte de dialogue Editer un type de données
s'affiche.

Effectuez les modifications voulues puis cliquez sur OK.

Pour sauvegarder la définition, sélectionnez l'option Sauvegarder
du menu Fichier.