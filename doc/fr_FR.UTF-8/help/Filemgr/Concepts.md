Gestionnaire de fichiers - ConceptsPour une meilleure compréhension du Gestionnaire de fichiers,
reportez-vous aux rubriques suivantes:Système de fichiers hiérarchiqueSystème de fichiers hiérarchiqueFichiersystème hiérarchiqueDéfinitionfichierSi vous débutez en informatique, la notion de système
de fichiers hiérarchique ne vous est peut-être pas familière.
Cette rubrique décrit les composants de base de ce type de système
de fichiers.Qu'est-ce qu'un fichier ?Unfichierest un conteneur renfermant des
informations. Les fichiers que vous utilisez contiennent des données
dans un format particulier&emdash;un document, une feuille de calcul, un graphique.
Le format correspond à la façon dont les données sont
disposées dans le fichier; il est indiqué par le type de données
du fichier.Lorsque l'un des modes de visualisation des icônes du Gestionnaire
de fichiers est activé, vous pouvez identifier le type de données
d'un fichier grâce à l'icône utilisée pour le représenter.
Chaque type de données est associé à une icône
différente.Les programmes d'application gèrent un nombre limité de
formats. Par exemple, un éditeur de documents ne peut pas lire une
feuille de calcul. Le Bureau vous aide à reconnaître les différents
types de fichiers grâce à une base de données detype de données. Le type de données identifie les
fichiers d'un format particulier et les associe aux applications correspondantes.
Lorsque vous cliquez deux fois sur un fichier, le Bureau lance automatiquement
l'application qui prend en charge son type de données.La longueur maximale autorisée pour le nom d'un fichier varie
d'un système à l'autre. Certains n'admettent pas une longueur
supérieure à 14 caractères. Si nécessaire, consultez
l'administrateur système.Qu'est-ce qu'un dossier ?Définition: dossierUndossierest un conteneur de fichiers,
semblable au dossier utilisé dans une armoire de rangement. En fait,
le Gestionnaire de fichiers utilise une icône particulière pour
représenter un dossier. Un dossier peut contenir dessous-dossiers. Les dossiers et sous-dossiers permettent de créer plusieurs
niveaux d'organisation formant une hiérarchie (dans d'autres contextes,
les dossiers sont souvent appelés répertoires).Si vous représentiez la structure hiérarchique d'un dossier
sous forme de schéma, en traçant des lignes entre les sous-dossiers
et leurs dossiers pères, votre dessin ressemblerait à un arbre
renversé (d'où le terme arborescence, souvent utilisé
pour faire référence à la hiérarchie des dossiers).Dans un dossier, chaque nom de fichier doit être unique. Cependant,
les fichiers stockés dans des dossiers différents peuvent porter
le même nom.Lorsque vous naviguez entre des dossiers, votre emplacement à
un moment donné est appelédossier courant.Qu'est-ce qu'un chemin d'accès ?Définition: chemin d'accèsChemin d'accèsdéfinitionL'emplacement d'un fichier est souvent indiqué par la liste
des dossiers et des sous-dossiers qui permettent d'y accéder; cette
liste est appeléechemin d'accès(pour
plus de détails, reportez-vous à).
Un chemin est visible à deux endroits du Gestionnaire de fichiers.
Premièrement, dans le chemin icônique, sous la forme d'une chaîne
de dossiers. Deuxièmement, sous forme de chaîne de caractères,
sur la ligne de chemin de texte située au-dessus de la zone de visualisation.Noms de chemins d'accèsLe chemin d'accès à un objet permet d'indiquer l'emplacement
de celui-ci dans le système de fichiers. Il existe trois façons
d'indiquer un chemin: chemin absolu, chemin relatif et chemin complet.Chemins absolusUn chemin estabsolulorsqu'il commence à
partir dudossier racine. Le dossier racine (en anglais
:root) est le dossier de plus haut niveau dans l'arborescence.
Si un chemin commence par une barre oblique (/),
il s'agit d'un chemin absolu. L'exemple suivant indique le chemin d'accès
absolu du fichierlettre:/usr/dt/config/lettreChemins relatifsUn cheminrelatifdécrit l'emplacement
d'un fichier ou d'un dossier par rapport au dossier courant. Si vous vous
trouvez dans un dossier et que vous voulez descendre dans l'arborescence,
vous n'avez pas besoin d'indiquer le chemin complet; il vous suffit de taper
le chemin, en commençant par le nom du dossier suivant de l'arborescence.
Lorsqu'un chemin ne commence pas par une barre oblique, il s'agit d'un chemin
relatif. Par exemple, si le dossier courant est/usr/dt,
et que vous souhaitez accéder au dossier "/usr/dt/config/lettres",
utilisez le chemin relatif suivant:config/lettres.. (dossier parent)Deux noms de dossiers spéciaux peuvent être utiles lorsque
vous indiquez des chemins relatifs. Le dossier dont l'extension commence par
"." représente le dossier courant. Le dossier dont l'extension commence
par ".." représente le dossierparent&emdash;
c'est-à-dire le dossier situé un niveau au-dessus dans la hiérarchie.
Par exemple, si le dossier courant est/usr/dt/config/panels, le chemin relatif du fichiersys.dtwmrcest:../sys.dtwmrcEn effet, le fichier se trouve dans le dossier/usr/dt/config, un niveau au-dessus du dossier courant.Voir aussiPropriété et sécurité des objetsTrois groupes d'utilisateurs peuvent accéder aux objets:le propriétaire,le groupeetles autresutilisateurs. Les droits d'accès sont de trois
types:lecture,écritureetexécution.Qui peut accéder aux objets ?Les trois catégories d'utilisateurs de base sont:PropriétaireIl s'agit du créateur du fichier.GroupePlusieurs utilisateurs qui ont été regroupés par
l'administrateur système. Par exemple, les membres d'un même
service peuvent appartenir au même groupe.AutreTous les autres utilisateurs du système.Quels sont les différents droits d'accès ?Les droits d'accès à un fichier définissent
les autorisations attribuées au propriétaire, aux membres du
groupe et aux autres utilisateurs.Droit d'accès en lecturePermet de copier ou de visualiser le contenu de l'objet.Droit d'accès en écriturePermet de modifier le contenu de l'objet ou de le supprimer.Droit d'accès en exécutionPermet d'exécuterun fichier (fichiers exécutables,
scripts et actions). Dans le cas d'un dossier, permet d'exécuter des
commandes, des scripts et des actions.Vous pouvez visualiser et modifier les droits d'accès aux fichiers
ou aux dossiers à partir du Gestionnaire de fichiers. Reportez-vous
àet.ExemplesPour rendre un dossier confidentiel:Modifiez la propriété
du dossier, en vous octroyant (propriétaire), les droits d'accès
en lecture, écriture et exécution, et en n'attribuant aucune
autorisation au groupe et aux autres utilisateurs. Ceci signifie que seuls
le propriétaire (vous) et l'utilisateur root peuvent visualiser le
contenu du dossier.Pour permettre à tous les utilisateurs d'accéder à
un objet que vous avez créé, tout en le protégeant contre
une éventuelle suppression:Modifiez la propriété
du fichier en attribuant les droits d'accès en lecture et en exécution
au propriétaire, au groupe et aux autres utilisateurs. N'attribuez
à personne le droit d'accès en écriture.Droits d'accès par défautLes droits d'accès par défaut définis lors
de la création d'un fichier ou d'un dossier peuvent être modifiés
par l'administrateur système. Pour les visualiser, créez un
fichier ou un dossier, puis sélectionnez l'option Modifier les droits
d'accès dans le menu Sélectionné(s).Faciliter l'accès aux objets - Introduction &newline; Objets
de l'espace de travailObjets de l'espace de travailEspace de travailobjetsLe Gestionnaire de fichiers permet de visualiser tous les objets du
système de fichiers. Cependant, ceux-ci sont uniquement visibles lorsque
vous visualisez le dossier dans lequel ils se trouvent.Pour faciliter l'accès à un objet, vous pouvez le placer
directement sur le fond de l'espace de travail. L'espace de travail est la
zone sur laquelle les fenêtres sont affichées (reportez-vous
à). Tout objet placé
sur cette zone est appeléobjet de l'espace de travail.Placer un objet sur l'espace de travail ne modifie pas le fichier ou
le dossier d'origine. En fait, l'icône qui apparaît sur le Bureau
représente simplement un raccourci (lien) pour accéder au fichier
ou dossier réel. Toute action effectuée sur l'objet de l'espace
de travail est en fait appliquée au fichier ou au dossier qu'il représente.Afficher des objets sur les espaces de travailLorsque vous placez un objet sur l'espace de travail, il apparaît
uniquement dans l'espace courant. Si vous souhaitez le voir apparaître
dans d'autres espaces, activez ces espaces pour l'y placer.Utiliser des objets de l'espace de travailUtilisez les objets de l'espace de travail exactement comme ceux
des fenêtres du Gestionnaire de fichiers ou d'applications. Pour exécuter
une action par défaut sur un objet, cliquez deux fois sur son icône
sur le Bureau.A chaque objet de l'espace de travail est associé un menu instantané
contenant des options et des actions. Pour afficher ce menu à l'aide
de la souris, amenez le pointeur sur l'icône, appuyez sur le bouton
3 de la souris et maintenez-le enfoncé. Pour l'afficher par l'intermédiaire
du clavier, appuyez sur Alt+Tab jusqu'à ce que l'icône soit mise
en évidence et appuyez sur Maj+F10.Formes de correspondance utilisées pour la recherche de fichiersCorrespondance des caractères génériquesLorsque vous indiquez un nom de fichier ou de dossier, vous pouvez taper
des caractères génériques tels que l'astérisque
(*) et le point d'interrogation (?).*correspond à
une chaîne de zéro ou plusieurs caractères et?, à un caractère unique.Exemples