
# Editeur d'icônes - Concepts







# Editeur d'icônes - Initiation
Utilisation: Editeur d'icônesEditeur d'icônes: utilisationEdition: icônesIcônes: édition

L'Editeur d'icônes permet de créer et de modifier des images aux
formats suivants:

pixmap X (format XPM)-- Il s'agit de fichiers contenant des
images composées de couleurs statiques (prédéfinies) et de couleurs
dynamiques; généralement, leur extension est.pm.

bitmap X (format XBM)-- Il s'agit de fichiers contenant des
images en noir et blanc; généralement, leur extension est.bm.

Avant de dessiner une icône dans la zone de travail, vous devez sélectionner
un outil et une couleur. Deux exemplaires de votre icône, l'un au format
pixmap et l'autre au format bitmap, s'affichent à mesure que vous dessinez,
en taille réelle. Même si l'image est destinée à être utilisée sur un système
couleur, sa version en noir et blanc doit être lisible; en effet, le
format bitmap doit pouvoir être utilisé si la mémoire disponible ne permet
pas l'affichage de l'icône en couleurs.
# Aspect des icônes


Lorsque vous créez des icônes ayant un rapport entre elles (icônes d'une
application et icônes des fichiers de données associés, par exemple),
veillez à utiliser un concept de base commun.

Assurez-vous que la version en noir et blanc des icônes couleur est
correcte. Sur un écran monochrome ou monochrome gris (ou ne disposant
pas des couleurs requises), une icône couleur s'affiche en noir et
blanc.




# Utilisation des couleurs
Couleurs: dans une icôneIcônes: utilisation des couleurs

Les icônes disponibles sur le bureau utilisent une palette
de 22 couleurs:

huit nuances de gris;

huit couleurs statiques (prédéfinies): rouge, bleu, vert, cyan,
magenta, jaune, noir et blanc;

six couleurs dynamiques: premier plan, arrière-plan, ombrage du haut,
ombrage du bas, une couleur sélectionnée et une couleur transparente.

Cette palette permet de créer des icônes attrayantes et d'une grande
lisibilité, sans monopoliser les ressources dont d'autres applications
pourraient avoir besoin. Les icônes du Bureau utilisent principalement
les nuances de gris; les couleurs permettent uniquement de mettre
les éléments essentiels en évidence.

Les couleurs dynamiques sont utiles dans le cas des icônes dont
vous modifiez les couleurs à partir du Gestionnaire de configuration.

La couleur transparente permet de créer des icônes qui laissent
apparaître les couleurs d'arrière-plan, donnant ainsi l'illusion qu'elles
ne sont pas rectangulaires.
# Taille des icônes
Icônes: tailleTaille: icônes

Il est conseillé d'utiliser les tailles décrites ci-dessous
(en pixels, largeur&times;hauteur) lors de la création des icônes.

Gestionnaire de fichiers (icônes normales):

Haute résolution: 32&times;32&newline;Résolution moyenne: 32&times;32&newline;Basse résolution: 32&times;32

Gestionnaire de fichiers (icônes réduites):

Haute résolution: 16&times;16&newline;Résolution moyenne: 16&times;16&newline;Basse résolution: 16&times;16

Gestionnaire d'applications (icônes normales):

Haute résolution: 32&times;32&newline;Résolution moyenne: 32&times;32&newline;Basse résolution: 32&times;32

Gestionnaire d'applications (icônes réduites):

Haute résolution: 16&times;16&newline;Résolution moyenne: 16&times;16&newline;Basse résolution: 16&times;16

Tableau de bord:

Haute résolution: 48&times;48&newline;Résolution moyenne: 48&times;48&newline;Basse résolution: 32&times;32

Tableaux secondaires du tableau de bord:

Haute résolution: 32&times;32&newline;Résolution moyenne: 32&times;32&newline;Basse résolution: 16&times;16

Fenêtres réduites:

Haute résolution: 48&times;48&newline;Résolution moyenne: 48&times;48&newline;Basse résolution: 32&times;32

Bureau:

Haute résolution: 32&times;32&newline;Résolution moyenne: 32&times;32&newline;Basse résolution: 32&times;32

La taille des images de fond est indifférente; leur motif est répété
autant de fois que nécessaire jusqu'à ce que l'espace de travail soit
rempli.
# Voir aussi



# Noms des fichiers d'icônes
Icônes: détection des fichiersFichiers d'icônes: détectionIcônes: conventions de dénominationExtension.pmExtension.bmBitmaps: détection des fichiersPixmaps: détection des fichiersConventions de dénomination des fichiers d'icônes

Les icônes et les images de fond sont stockées dans des fichiers
distincts. Généralement, une icône est identifiée par la première
partie de son nom de fichier. Par exemple, l'icônemailest
stockée dans le fichier:/usr/dt/appconfig/icons/`langue`/mail.l.pm

L'utilisation d'extensions permet de regrouper les icônes en fonction de
leur taille et de leur type. Les noms des icônes du bureau ont
généralement le format suivant:`nom_base`.`taille`.`format``nom_base`.`format`

`nom_base`indique le nom de l'image identifiant l'icône,`taille`représente la taille de l'icône (une lettre) et`format`estpmpour les pixmaps X oubmpour les
bitmaps X.

Les tailles d'icônes disponibles sont les suivantes:

* **Nom** 

Taille Extension
* **Tiny (très petite)** 

16&times;16,`t`
* **Small (petite)** 

24&times;24,`s`
* **Medium (moyenne)** 

32&times;32,`m`
* **Large (normale)** 

48&times;48,`l`


Par exemple, si vous indiquez le nom d'icône "mail" pour un fichier
que vous avez créé, que vous disposez d'un écran couleur et que le
Gestionnaire de fichiers utilise des icônes "réduites", le nom par
défaut de l'icône est "mail.s.pm" ("s" indique qu'il s'agit d'une
icône réduite et "pm" correspond au format couleur`pixmap`).
# Chemins de recherche


Pour identifier le répertoire dans lequel une icône est stockée, il
suffit de rechercher un nom de fichier dans une liste de répertoires.
La liste des répertoires sur lesquels la recherche doit porter est
appelée "chemin de recherche".

Si vous utilisez un écran couleur, le Bureau recherche les fichiers
dont l'extension est.pm. Sinon, il recherche les fichiers dont
l'extension est.bm.

Dans le cas des images identifiées par un chemin d'accès complet, le
chemin de recherche n'est pas utilisé.
# Voir aussi


fournit la liste des chemins de recherche.
# Stockage des fichiers d'icônes
Stockage des fichiers d'icônesFichiers d'icônes: stockage

Par défaut, le bureau recherche les fichiers d'icônes dans les répertoires
suivants:

Icônes personnelles:/`Rép_personnel`/.dt/icons

Icônes système:/etc/dt/appconfig/icons/`langue`.

Icônes intégrées:/usr/dt/appconfig/icons/`langue`.

Pour les icônes en langue anglaise, indiquezCpour`langue`.
# Stockage des fichiers d'images de fond
Fond: stockage des fichiers d'imagesFichiers d'images de fond: stockage

Chaque élément de la liste affichée dans la boîte de dialogue
Fond du Gestionnaire de configuration correspond à un fichier d'image.
Par défaut, le Gestionnaire de configuration recherche les images de
fond dans les répertoires suivants:

Images système:/etc/dt/appconfig/backdrops/`langue`.

Images intégrées:/usr/dt/backdrops/`langue`.

Les images disponibles pour tous les utilisateurs du système doivent
être placées, lors de leur création, dans le répertoire/etc/dt/appconfig/backdrops/`langue`. Pour réserver une image
à un utilisateur donné, placez-la dans un autre répertoire et ajoutez
ce dernier à la ressource*backdropDirectoriesde l'utilisateur.