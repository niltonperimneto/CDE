
# Conceptos del Editor de iconos







# Introducción al Editor de iconos
uso: Editor de iconosEditor de iconos: usoedición: iconosiconos: edición

El Editor de iconos sirve para crear y editar imágenes en dos formatos:

Mapa de pixels X (formato XPM)-- Imágenes de varios colores que
incluyen colores estáticos y dinámicos. Los archivos de mapas de pixels
se identifican normalmente por la extensión.pm.

Mapa de bits X (formato XBM)-- Imágenes en blanco y negro. Los
archivos se identifican normalmente por la extensión.bm.

Para dibujar imágenes se selecciona una herramienta y un color y, a
continuación, se dibuja en el área de trabajo. A medida que
dibuja, el Editor de iconos muestra una copia en tamaño real del
icono en ambos formatos. Incluso si dibuja una imagen para utilizarla
en sistemas en color, debe asegurarse de que la versión en blanco
y negro es legible, porque si no hay suficientes colores libres para
mostrar la versión en color completa, los iconos pueden convertirse al
formato bitmap.
# Consejos para el diseño de iconos


Trate de utilizar un tema común entre iconos relacionados. Por
ejemplo, si está diseñando iconos para una aplicación, establezca
semejanzas en cuanto a la finalidad entre el icono de la aplicación y
los iconos para archivos de datos relacionados.

Asegúrese de que la versión en blanco y negro de cualquier icono en
color que diseñe sea aceptable.
Si el icono se visualiza en una pantalla monocromo o en gama de grises (o
si no hay suficientes colores disponibles), el icono aparecerá
automáticamente en blanco y negro.




# Utilización del color
color: uso en iconosiconos: uso del color

Los iconos del escritorio usan una paleta de 22 colores:

Ocho grises estáticos

Ocho colores estáticos: rojo, azul, verde, cian, magenta, amarillo,
negro y blanco

Seis colores dinámicos: primer plano, fondo, sombra superior,
sombra inferior, seleccionar y transparente

Esta paleta es lo bastante amplia como para crear iconos atractivos y de
fácil lectura, sin emplear demasiados recursos para el color que a veces
necesitan otras aplicaciones.
Principalmente, los iconos que se proporcionan en el escritorio emplean
grises con color para su resaltado.

Los iconos dinámicos son útiles para iconos que cambian el color a
medida que se seleccionan distintas paletas de color en el Gestor de
estilos.

El color transparente es útil para crear iconos que parecen ser no
rectangulares, ya que permite visualizar el color que hay detrás del icono.
# Consejos para el tamaño de los iconos
iconos: consejos sobre el tamañotamaño: iconos

Los tamaños recomendados para crear nuevos iconos son los
siguientes -- en pixels, ancho&times;alto.

Gestor de archivos (grande):

Alta resolución: 32&times;32&newline;Resolución media: 32&times;32&newline;Baja resolución: 32&times;32

Gestor de archivos (pequeño):

Alta resolución: 16&times;16&newline;Resolución media: 16&times;16&newline;Baja resolución: 16&times;16

Gestor de aplicaciones (grande):

Alta resolución: 32&times;32&newline;Resolución media: 32&times;32&newline;Baja resolución: 32&times;32

Gestor de aplicaciones (pequeño):

Alta resolución: 16&times;16&newline;Resolución media: 16&times;16&newline;Baja resolución: 16&times;16

Panel Frontal:

Alta resolución: 48&times;48&newline;Resolución media: 48&times;48&newline;Baja resolución: 32&times;32

Subpaneles del Panel Frontal:

Alta resolución: 32&times;32&newline;Resolución media: 32&times;32&newline;Baja resolución: 16&times;16

Ventanas minimizadas:

Alta resolución: 48&times;48&newline;Resolución media: 48&times;48&newline;Baja resolución: 32&times;32

Escritorio:

Alta resolución: 32&times;32&newline;Resolución media: 32&times;32&newline;Baja resolución: 32&times;32

Las imágenes de fondo pueden ser de cualquier tamaño. El patrón se repite
para completar todo el espacio de trabajo.
# Consulte también



# Convenios de nomenclatura de los archivos de iconos
iconos: cómo se encuentran los archivosarchivos de imagen: ver iconosiconos: convenios de nomenclatura.pmnombre de archivo.bmnombre de archivomapas de bits: cómo se encuentran los archivosmapas de pixels: cómo se encuentran los archivosconvenios, nomenclatura de archivos de imagen

Cada icono y cada imagen de fondo se almacena a modo de archivo separado.
Normalmente, un icono se especifica con la parte básica de su
nombre de archivo.
Por ejemplo, un icono podría denominarse justo con el nombremailcuando el archivo esté realmente almacenado como:/usr/dt/appconfig/icons/`idioma`/mail.l.pm

El convenio de adición a la nomenclatura de archivo pone sufijos a los
iconos de grupo de ayudas según el tamaño y tipo. Para los componentes del
escritorio, muchos nombres de icono se encuentran en estos formatos
generales:`nombre_base_datos`.`tamaño`.`formato``nombre_base_datos`.`formato`

donde`nombre_base_datos`es el nombre de imagen que se emplea para hacer
referencia a la imagen,`tamaño`es una sola letra que indica el tamaño
del icono y`formato`espmpara los mapas de pixels X obmpara los mapas de bits X.

Los tamaños válidos posibles para los iconos son:

* **Nombre** 

Tamaño Sufijo
* **Tiny** 

16&times;16`t`
* **Small** 

24&times;24`s`
* **Medium** 

32&times;32`m`
* **Large** 

48&times;48`l`


Por ejemplo, supongamos que se especifica un icono denominadomailpara un tipo de archivo que se haya escrito.
Si dispone de una pantalla en color y ha establecido las preferencias
del Gestor de archivos para utilizar iconos pequeños, el nombre de icono
que se adopta esmail.s.pm(sequivale a pequeño ypmamapa de pixels, el formato del icono en color).
# Rutas de búsqueda


El directorio donde se guarda una imagen se determina buscando el
archivo en una lista de directorios. Dicha lista se define mediante
una "ruta de búsqueda" para iconos.

de utilizar una pantalla en color, el escritorio primero efectúa
una búsqueda de los archivos que acaban en.pm. En caso de no encontrarlos, busca
archivos que acaben en.bm.
Si una imagen se especifica con un nombre de ruta completo, la ruta
de búsqueda no se utiliza.
# Consulte también


muestra las especificaciones de la ruta de
búsqueda.
# Almacenamiento de archivos de iconos
almacenamiento de archivos de iconosarchivo de iconos: almacenamiento

Como valor predeterminado, los componentes del escritorio buscan en
estos directorios los archivos de iconos:

Iconos personales:/`DirectorioInicio`/.dt/icons

Iconos accesibles en el sistema:/etc/dt/appconfig/icons/`idioma`

Iconos incorporados:/usr/dt/appconfig/icons/`idioma`

Para los iconos en inglés, utiliceCpara`idioma`.
# Almacenamiento de archivos de imágenes de fondo
fondo: almacenamiento de archivos de imágenes de fondoarchivo imagen: almacenamiento de fondo

Cada fondo listado en el cuadro de diálogo de fondo del Gestor de estilos
representa un archivo imagen. Como valor predeterminado, el Gestor de
estilos efectúa una búsqueda de los fondos en estos directorios:

Imágenes accesibles en el sistema:/etc/dt/appconfig/backdrops/`idioma`

Imágenes incorporadas:/usr/dt/backdrops/`idioma`

Si crea una imagen de fondo que desea dejar accesible a todos los usuarios,
coloque dicha imagen en/etc/dt/appconfig/backdrops/`idioma`.
Para restringir el acceso a una imagen de fondo por parte de unos
usuarios determinados, coloque la imagen de fondo en un directorio
diferente y añada ese directorio al recurso*backdropDirectoriespara otros usuarios.