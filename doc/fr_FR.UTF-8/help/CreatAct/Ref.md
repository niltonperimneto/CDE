
# Créer une action - Référence

# Créer une action - Généralités





# Créer une action - Fenêtres









# Fichiers créés
Créer une action:fichiers créésNom d'action

Créer une action génère les éléments suivants:

Un fichier de définition permettant de lancer l'action:`Dossier_personnel`/.dt/types/`nom_action`.dt.

Icône d'action

Une icône d'action, installée dans le répertoire personnel
de l'utilisateur; lorsque vous cliquez deux fois dessus, l'action est
lancée. Pour configurer l'icône en tant que zone de pose, définissez
des types de données posables.

Les fonctions avancées de l'utilitaire permettent également de créer
les éléments suivants:

un ou plusieurs types de données associés aux fichiers de données
de l'application;

des actions Ouvrir et Imprimer pour le type de données.

Ces éléments sont également écrits dans le fichier`Dossier _personnel`/.dt/types/`nom_action`.dt.
# Restrictions d'utilisation

# Actions
Créer une action:restrictions d'utilisation

La création d'actions est impossible dans les cas suivants:

La ligne de commande requiert un argument qui n'est pas un fichier.
La commande suivante, par exemple, ne peut pas faire l'objet d'une
création d'action:lp -d`unité``nom_fichier`

En effet, un nom d'unité doit être indiqué pour l'exécution de la
commande; l'action correspondante doit être créée manuellement.

Le libellé de l'icône doit être différent du nom de l'action.
Par exemple, Créer une action ne permet pas de modifier le nom
d'une action existante.

L'action requiert certaines fonctions étendues de la base de données
des actions, telles que:

lancement de commandes sur des systèmes éloignés,

appel d'autres actions,

exécution sous un ID utilisateur différent (root, par exemple),

utilisation étendue de la fonction "map",

comportements variés, en fonction du nombre d'arguments
indiqués pour l'action.
# Types de données
Créer une action:restrictions d'utilisation

La création de types de données est impossible dans les cas suivants:

Le type de données doit être associé à des actions autres qu'Ouvrir
et Imprimer.

L'action Ouvrir du type de données ne correspond pas à la commande de
lancement de l'action. Par exemple, vous ne pouvez pas créer un type
de données qui associe une icône unique au répertoire représentant
le groupe d'applications de l'application.
# Fenêtre principale
Créer une action:fenêtre principale

Pour plus de détails sur les tâches pouvant être effectuées à
partir de cette fenêtre, reportez-vous à la section.

* **Nom (libellé de l'icône)** 

Tapez le nom de l'action. La distinction
majuscules/ minuscules est respectée et les espaces ne sont pas autorisés.
* **Icônes d'action** 

Affiche l'image qui sera utilisée pour l'icône de l'action.
* **Rech. ensemble** 

Affiche une boîte de dialogue qui permet de sélectionner
un autre bitmap ou pixmap existant.
* **Editer une icône** 

Ouvre l'Editeur d'icônes, qui permet de créer une
nouvelle icône ou de modifier une icône existante.
* **Commande exécutée à l'ouverture de l'action** 

Tapez la commande. Utilisez
la syntaxe$`n`pour indiquer qu'un fichier doit être spécifié
comme argument.
* **Aide sur l'icône** 

Tapez le texte de l'aide associée à l'icône.
* **Type de fenêtre** 

Sélectionnez le type de fenêtre créé par l'action:

Graphique: L'application affiche sa propre fenêtre.

Terminal (fermeture auto.): L'action affiche une fenêtre de terminal
qui se ferme automatiquement lorsque l'exécution de l'action prend fin.

Terminal (fermeture manuelle): L'action affiche une fenêtre de terminal
dont l'utilisateur doit demander explicitement la fermeture.

Pas de sortie: L'application ne requiert aucune fenêtre.

# Fonctions étendues


Ces fonctions ne sont disponibles que si la commande exécutée lors du
double-clic requiert un fichier.

* **A l'ouverture de l'action, demander aux utilisateurs** 

Tapez le message qui
s'affichera lorsque vous cliquerez deux fois sur l'action.
* **Types de données utilisant l'action** 

Liste des types de données associés
à l'action, accessible en lecture seulement. Les entrées sont ajoutées à
mesure que vous créez des types de données à l'aide de la boîte de
dialogue Ajouter un type de données.
* **Ajouter** 

Ouvre la fenêtre Ajouter un type de données, qui permet de créer
un type de données.
* **Supprimer** 

Supprime les types de données sélectionnés dans la liste des
types de données utilisant l'action.
* **Editer** 

Permet d'éditer les types de données sélectionnés dans la liste
des types de données utilisant l'action.
* **Types de données pouvant être posés** 

Indiquez les types de données pris
en charge par l'action (tous ou seulement ceux de la liste des types de
données utilisant l'action).

# Boîte de dialogue Ajouter un type de données
Boîte de dialogue Ajouter un type de données

Pour plus de détails, reportez-vous à la section.

* **Nom de la famille du type de données** 

Un nom est attribué par défaut;
vous pouvez le modifier en respectant la distinction majuscules/minuscules
et en n'indiquant pas d'espaces.
* **Caractéristiques** 

Liste des critères définissant le type de fichier;
pour les modifier, utilisez l'option Editer.
* **Editer** 

Affiche la boîte de dialogue Caractéristiques.
* **Aide sur l'icône de type de données** 

Tapez l'aide associée aux fichiers
appartenant à ce type de données.
* **Icônes du type de données** 

Affiche l'image associée au type de données.
* **Rech. ensemble** 

Affiche la boîte de dialogue correspondante, qui permet de
sélectionner un autre bitmap ou pixmap existant.
* **Editer une icône** 

Ouvre l'Editeur d'icônes, qui permet de créer une
nouvelle icône ou de modifier une icône existante.
* **Commande d'ouverture du type de données** 

Affiche la commande exécutée
lorsque l'utilisateur clique deux fois sur le type de données (identique
à la commande exécutée lors du double-clic).
* **Commande d'impression du type de données** 

Tapez la commande permettant
d'imprimer le type de données.
* **OK** 

Enregistre les informations associées au type de données, ajoute
ce dernier à la liste des types de données utilisant l'action et ferme
la boîte de dialogue.
* **Appliquer** 

Enregistre les informations associées au type de données, ajoute
ce dernier à la liste des types de données utilisant l'action (la boîte
de dialogue n'est pas fermée).
* **Annuler** 

Ferme la boîte de dialogue sans créer le type de données.
* **Aide** 

Affiche l'aide en ligne.

# Boîte de dialogue Caractéristiques
Boîte de dialogue Caractéristiques

Pour plus de détails sur les tâches pouvant être effectuées à partir
de cette fenêtre, reportez-vous à la section.

* **Tout inclure** 

Fichiers (types de données s'appliquant aux fichiers uniquement).

Dossiers (types de données s'appliquant aux répertoires uniquement).
* **Modèle de nom** 

Cochez la case et tapez le modèle voulu. Les caractères
spéciaux suivants sont autorisés:

*: correspond à une chaîne de caractères.

?: correspond à un caractère.
* **Modèle d'autorisations** 

Cochez la case et sélectionnez les boutons radio
appropriés (si les autorisations n'ont pas d'importance, sélectionnez
Indifféremment).
* **Contenu** 

Cochez la case et tapez les données contenues dans le fichier.
* **Type** 

Sélectionnez le type de données (Chaîne pour le texte ASCII).
* **Octet de départ** 

Tapez l'emplacement du fichier à partir duquel la recherche
doit commencer (1indique le début).
* **OK** 

Applique les caractéristiques à la zone Caractéristiques de la boîte
de dialogue Ajouter un type de données et ferme la fenêtre.
* **Annuler** 

Ferme la fenêtre sans sauvegarder les modifications.
* **Effacer** 

Rétablit les valeurs par défaut.
* **Aide** 

Affiche l'aide en ligne.

# Boîte de dialogue Rech. ensemble
Boîte de dialogue Rech. ensemble

Cette boîte de dialogue permet d'indiquer l'image à utiliser pour
une action ou un type de données. Pour plus de détails, reportez-vous
à la section.

* **Dossiers d'icônes** 

Affiche la liste des dossiers faisant partie du chemin
de recherche des icônes. Cliquez deux fois sur un répertoire pour afficher
les icônes qu'il contient.
* **Fichiers d'icônes** 

Affiche les icônes du répertoire. Pour sélectionner un
fichier d'icône, cliquez dessus; son chemin d'accès apparaît dans la zone
Entrez un nom de fichier d'icône.
* **Entrez un nom de fichier d'icône** 

Indiquez le nom de base
du fichier d'icône. Pour modifier le contenu de cette zone, cliquez sur
un élément de la liste Fichiers d'icônes.
* **OK** 

Applique les modifications et ferme la boîte de dialogue.
* **Annuler** 

Ferme la boîte de dialogue sans appliquer les modifications.
* **Aide** 

Affiche l'aide en ligne.

# Boîte de dialogue Ouvrir
Créer une action:ouverture d'un fichier d'actionAction:ouvertureAction:édition

* **Entrez le chemin d'accès ou le nom du dossier** 

Indique le chemin d'accès du dossier en cours.
* **Filtre** 

L'astérisque (*) correspond à tous les fichiers. Vous pouvez
utiliser des caractères génériques, pour n'afficher que les
fichiers portant un suffixe donné; par exemple, si vous
indiquez *.doc, seuls les fichiers dont le suffixe est .doc
s'affichent.
* **Fichiers** 

Affiche les fichiers du dossier en cours.
* **Dossiers** 

Affiche les sous-dossiers du dossier en cours.

Vous pouvez ouvrir un fichier situé dans le dossier en cours, un
sous-dossier ou un dossier différent.
* **Entrez le nom du fichier** 

Affiche le nom du fichier sélectionné dans la liste
(vous pouvez également l'entrer directement au clavier).
Pour l'ouvrir, appuyez sur Entrée ou cliquez sur OK.
* **OK** 

Ouvre le fichier indiqué dans la zone Entrez le nom du
fichier.
* **Mettre à jour** 

Régénère la liste de fichiers, après modification du
critère de filtrage ou ouverture d'un autre dossier.
* **Annuler** 

Annule l'ouverture.
* **Aide** 

Affiche l'aide sur les zones d'entrée, les listes de
sélection et les boutons de la boîte de dialogue.
