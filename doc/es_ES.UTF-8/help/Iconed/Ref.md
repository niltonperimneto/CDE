
# Referencia del Editor de iconos











# Rutas de búsqueda de iconos
rutas de búsqueda: para iconosiconos: rutas de búsqueda

La ruta de búsqueda para los archivos imagen se define con dos
variables de entorno:

El sistema utiliza DTUSERAPPSEARCHPATH y DTAPPSEARCHPATH a menos que
existan las variables siguientes:

* **DTUSERICONSEARCHPATH** 

Variable personal. Si se
utiliza, debe definirse en/`DirectorioInicio`/.dtprofile.
* **DTICONSEARCHPATH** 

Variable accesible en el sistema.

# Ruta de búsqueda predeterminada


El valor predeterminado para DTICONSEARCHPATH es:/`DirectorioInicio`/.dt/icons/etc/dt/appconfig/icons/C/usr/dt/appconfig/icons/C
# Para modificar la ruta de búsqueda de iconos personales
adición: directorio para ruta de búsqueda de accionesdirectorio: adición para ruta de búsqueda de accionesacción: ruta de búsqueda, adición de directorioruta de búsqueda, accionesruta: ruta de búsqueda de accionesvariable de entorno DTACTIONSEARCHPATHvariable de entorno: DTACTIONSEARCHPATH

Para añadir un directorio a la ruta de búsqueda,
edite/`DirectorioInicio`/.dtprofile. Añada una línea que defina un
valor para DTUSERICONSEARCHPATH:DTUSERICONSEARCHPATH=`ruta`[,`ruta`...]
# Para modificar la ruta de búsqueda de iconos accesibles en el sistema


Las variables de la ruta de búsqueda accesible en el sistema se
definen en archivos situados en el directorio/etc/dt/Xsession.d.

Si DTICONSEARCHPATH ya está definida en un archivo de este
directorio, edite el valor para añadir el nuevo directorio en la ruta
de búsqueda.

Si DTICONSEARCHPATH todavía no está definida en este directorio,
defínala para incluir la ruta de búsqueda predeterminada y otras
que desee añadir.
(La ruta de búsqueda predeterminada se lista en los comentarios del
archivo/usr/dt/bin/dtsearchpath.)
# Herramientas del Editor de iconos
herramientas: Editor de iconosEditor de iconos: herramientasherramientas: de dibujo

Al seleccionar una de las herramientas del Editor de iconos, permanece
seleccionada hasta haber seleccionado otra. Las herramientas se muestran
y describen a continuación:

Lápiz&newline;Se utiliza para trazar líneas a pulso y dibujar pixels individuales.

Línea&newline;Se utiliza para trazar líneas rectas.

Rectángulo&newline;Se utiliza para dibujar contornos de rectángulos y figuras de rectángulos.

Círculo&newline;Se utiliza para dibujar contornos de círculos y figuras de círculos.

Borrador&newline;Se utiliza para borrar áreas amplias de la imagen.

Rellenar&newline;Se utiliza para rellenar con el color seleccionado una zona de un
color.

Polilínea&newline;Se utiliza para trazar líneas rectas conectadas.

Polígono&newline;Se utiliza para trazar líneas rectas conectadas, donde la primera y
la última se utilizan para formar un polígono cerrado.

Elipse&newline;Se utiliza para dibujar contornos de elipses y figuras de elipses.

Seleccionar&newline;Se utiliza para efectuar una selección principal. Varios
mandatos del menú Editar requieren, en primer lugar, una
selección principal.


# Rellenar figuras


Seleccione la casilla de verificación de Rellenar figuras situado debajo
de la paleta de herramientas para cambiar en la paleta de herramientas
los contornos formados por las herramientas de rectángulos, polígonos,
círculos y elipses por figuras.
(Consulte también).
# Menús del Editor de iconos











# Menú Archivo del Editor de iconos




* **Nuevo** 

Solicita un tamaño (anchura y altura, en
pixels) y, a continuación, crea un área de dibujo en
blanco de ese tamaño.
(Consulte.)
* **Abrir** 

Abre un archivo bitmap o pixmap ya existente.
(Consulte.)
* **Guardar** 

Guarda el icono actual utilizando su nombre existente. Si
el icono no tiene nombre, el Editor de iconos le solicita un
nombre.
(Consulte.)
* **Guardar como** 

Le solicita un nombre nuevo y, a continuación,
guarda el icono actual.
(Consulte.)
* **Salir** 

Cierra el Editor de iconos. Si tiene cambios no
guardados, el Editor de iconos le avisa de que se
perderán los cambios si continúa y sale. Para guardar
los cambios, elija Cancelar y, a continuación, guarde
el icono.)

# Menú Editar del Editor de iconos




* **Deshacer** 

Cancela la última operación de dibujo y devuelve el
icono a su estado anterior. La mayoría de
operaciones del Editor de iconos pueden deshacerse.
(Consulte.)
* **Cortar área** 

Corta el área seleccionada del icono. La zona
extraída se rellena con color transparente. La zona
cortada se guarda en el portapapeles del Editor de
iconos.
(Consulte.)
* **Copiar área** 

Copia el área seleccionada en el portapapeles del
Editor de iconos.
(Consulte.)
* **Pegar área** 

Pega el contenido del portapapeles en el icono.
(Consulteo.)
* **Girar área** 

Gira el área seleccionada 90 grados hacia la
izquierda o hacia la derecha. Seleccione la
dirección del submenú.
(Consulte.)
* **Alternar área** 

Crea una imagen especular del área seleccionada
alternándola de forma vertical u horizontal. Seleccione la
dirección en el submenú.
(Consulte.)
* **Escalar área** 

Le permite redimensionar el tamaño (o "escalar") del área seleccionada.
(Consulte.)
* **Redimensionar icono** 

Solicita un nuevo tamaño para el icono actual.
(Consulte.)
* **Añadir punto de actuación** 

Le permite indicar un pixel único en el icono actual
como "punto de actuación". Después de seleccionar Añadir
punto de actuación, efectúe una pulsación sobre el pixel que
desee que sea la zona activa.

Las zonas activas se utilizan al crear imágenes para
punteros de ratón -- la zona activa representa la
ubicaciónrealdel puntero.
(Consulte.)
* **Suprimir punto de actuación** 

Suprime el punto de actuación del icono actual.
* **Tomar imagen pantalla** 

Toma un área de la pantalla y la carga
en el área de dibujo.
(Consulte.)
La pantalla X-Y situada encima del área de trabajo
muestra el tamaño en pixels del área que se controla.
* **Borrar Icono** 

Borra el área de dibujo actual.
(Consulte.)

# Menú Opciones del Editor de iconos




* **Cuadrícula visible** 

Activa o desactiva la conmutación de las líneas
de cuadrícula. El valor predeterminado es Activado.
* **Formato de salida** 

Determina el formato de salida en que se guardará
el icono:

XBM para el formato de mapa de bits X de dos
colores. Por lo general, los archivos de mapa
de bits se identifican por el sufijo.bm.

XPM para el formato pixmap X multicolor (el
predeterminado). Los archivos de mapas de pixels
se identifican normalmente por el sufijo.pm.

Consulte.
* **Ampliación** 

Cambia el tamaño de visualización de la imagen
del área de dibujo. Puede seleccionar factores
de ampliación de 2x (el doble del tamaño normal)
a 12x (doce veces el tamaño normal).

# Menú Ayuda del Editor de iconos




* **Visión general** 

Muestra el tema de ayuda de introducción para un
Editor de iconos
* **Tareas** 

Muestra las instrucciones de tareas para la
mayoría de operaciones del Editor de iconos
* **Referencia** 

Muestra los resúmenes de consulta para varias
funciones del Editor de iconos, como son las ventanas y
los cuadros de diálogo, los menús y los recursos
* **Sobre el tema** 

Presenta una descripción del elemento en una
ventana del Editor de iconos que se haya
seleccionado
* **Uso de la ayuda** 

Proporciona información sobre el uso de las ventanas
de ayuda
* **Acerca del Editor de iconos** 

Muestra la versión y la información de copyright del
Editor de iconos

# Ventanas y cuadros de diálogo del Editor de iconos

# Subtopics







# Ventana principal del Editor de iconos


La ventana principal del Editor de iconos tiene cinco áreas importantes:

Lalínea de estadoque aparece debajo de la barra de menús
describe la herramienta actualmente seleccionada y las coordenadas del
pixel al que está señalando el puntero.

Elárea de dibujoes la zona en que se dibuja la
imagen del icono.

Lapaleta de herramientasproporciona varias
herramientas de dibujo, incluida una herramienta para borrar y una
herramienta para seleccionar.

Lapaleta de coloresproporciona colores para
dibujar: ocho colores estáticos, ocho grises estáticos y seis
colores dinámicos.

Elárea de iconos de tamaño realmuestra el aspecto del
icono en tamaño real. Muestra la versión a todo color y la versión de
dos colores.
# Paletas de color del Editor de iconos




* **Colores estáticos** 

Ocho colores estándar; negro, blanco,
los tres colores primarios y los
tres colores secundarios
* **Grises estáticos** 

Ocho tonalidades de gris (del
10% al 90% de gris)
* **Colores dinámicos** 

Seis colores dinámicos que responden
al cambiar los colores con el
Gestor de estilos


En el Visor de ayuda del Gestor de estilos se pueden consultar estos temas:

Para
seleccionar una paleta

Para
cambiar el número de colores utilizados por el escritorio.
# Cuadros de diálogo Abrir y Guardar como del Editor de iconos


* **Entre la ruta o el nombre de la carpeta:** 

Escriba el nombre de ruta completo de
la carpeta que contenga el icono que desee abrir o
la carpeta en la que desee guardar el icono.
* **Carpetas** 

Visualiza una lista de las carpetas que hay dentro de la carpeta
que aparece en el campo de texto "Entre la ruta o el nombre de la carpeta".
Si efectúa una doble pulsación en una carpeta de esta lista, las listas de
Carpetas y Archivos cambiarán y mostrarán el contenido de dicha carpeta. También
se puede seleccionar una carpeta en la lista Carpetas y
a continuación pulsar el botón Actualizar.
* **Archivos** 

Visualiza una lista de los archivos contenidos en la
carpeta que aparezca en el campo "Entre la ruta o el nombre de la carpeta". Si
cambia el nombre de este campo, deberá pulsar el botón Actualizar
para que la lista Archivos muestre la nueva lista de archivos.
* **Entrar nombre de archivo:** 

Visualiza el nombre del icono que se
cargará o se guardará.
La manera más sencilla de especificar el icono deseado es efectuar
una doble pulsación en la lista Archivos. También se puede escribir
el nombre del icono y a continuación pulsar el botón Abrir.

Recuerde que el formato correcto de los
nombres de iconos es nombre.tamaño.formato. Para que los iconos
funcionen correctamente, la información referente al tamaño y al
formato debe estar en el nombre del icono. El editor de iconos
rellenará automáticamente los valores de formato y tamaño correctos
en función del formato de salida y tamaño seleccionado en la barra
de menús del editor.
* **Abrir o Guardar** 

Abre o guarda el archivo y cierra el cuadro de
diálogo.
* **Actualizar** 

Cambia las listas Carpetas y Archivos de modo que
muestran el contenido de la carpeta del campo Entre la ruta o el nombre de la
carpeta. Se puede escribir el nombre de carpeta en el
campo y a continuación pulsar el botón Actualizar. También se puede
efectuar una doble pulsación en el nombre de la carpeta que se desee
abrir si ésta se encuentra en la lista Carpetas.
* **Cancelar** 

Cancela la operación de abrir o guardar y cierra el
cuadro de diálogo.
* **Ayuda** 

Muestra ayuda acerca de este cuadro de diálogo.

# Cuadro de diálogo Guardar como del Editor de Iconos




* **Entre la ruta o el nombre de la carpeta:** 

Escriba el nombre de ruta completo de la
carpeta en la que desee guardar el icono.
* **Carpetas** 

Visualiza una lista de las carpetas que hay dentro de la carpeta
que aparece en el campo de texto "Entre la ruta o el nombre de la carpeta".
Si efectúa una doble pulsación en una carpeta de esta lista, las listas de
Carpetas y Archivos cambiarán y mostrarán el contenido de dicha carpeta. También
se puede seleccionar una carpeta en la lista Carpetas y a continuación pulsar
el botón Actualizar.
* **Archivos** 

Muestra una lista de los archivos contenidos en la
carpeta que aparezca en el campo "Entre la ruta o el nombre de la carpeta". Si
cambia el nombre de este campo, deberá pulsar el botón Actualizar
para que la lista Archivos muestre la nueva lista de archivos.
* **Entrar nombre de archivo:** 

Escriba el nombre del icono que desee guardar. Recuerde
que el formato correcto de los nombres de iconos es nombre.tamaño.formato. Para
que los iconos funcionen correctamente, la información referente al tamaño y al
formato debe estar en el nombre del icono. El editor de iconos
rellenará automáticamente los valores de formato y tamaño correctos
en función del formato de salida y tamaño seleccionado en la barra
de menús del editor.
* **Guardar** 

Guarda el archivo y cierra el cuadro de diálogo.
* **Actualizar** 

Cambia las listas de Carpetas y Archivos de modo que
muestran el contenido de la carpeta del campo Entre la ruta o el nombre de la
carpeta. Se puede escribir el nombre de carpeta en el
campo y a continuación pulsar el botón Actualizar. También se puede
efectuar una doble pulsación en el nombre de la carpeta que se desee
abrir si ésta se encuentra en la lista Carpetas.
* **Cancelar** 

Cancela la operación de guardar y cierra el cuadro de diálogo.
* **Ayuda** 

Muestra ayuda acerca de este cuadro de diálogo.

# Cuadro de diálogo de confirmación del Editor de iconos


El cuadro de diálogo le evita la pérdida accidental de datos al solicitar
la comprobación de que se desea efectuar el mandato (como, por ejemplo, salir del
Editor de iconos sin guardar el icono).

Para continuar pulse OK, o pulse Cancelar para cancelar el mandato.