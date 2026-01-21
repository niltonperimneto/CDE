
# Referencia del Gestor de aplicaciones
referencia

Esta es una lista de temas de referencia para el Gestor de
aplicaciones, ordenada en varias
categorías.
# Referencia general







# Menús del Gestor de aplicaciones











# Ventana y cuadros de diálogo del Gestor de aplicaciones





















# Referencia del ratón del Gestor de aplicaciones
botones del ratónselección de objetos
# Botón 1 del ratón&emdash;Seleccionar y arrastrar


El botón 1 del ratón (el botón izquierdo, predeterminado) se utiliza
para seleccionar y arrastrar
objetos.

Para seleccionar un objeto, efectúe una pulsación sobre su icono.

Para ampliar la selección, mantenga pulsada la tecla Control y
efectúe una pulsación sobre otro icono.
# Botón 2 del ratón&emdash;Arrastrar


Arrastrar objetos:

Mover objetos: mantenga pulsado el botón 1 del ratón mientras
arrastra un icono.

Copiar objetos: Pulse Control y arrastre.

Enlace simbólico: Pulse Despl y arrastre.

En un ratón de dos botones, pulse ambos botones a la vez.
# Botón 3 del ratón&emdash;Menús
menús de contexto

El botón 3 del ratón (el botón derecho, predeterminado) se utiliza
para mostrar menús emergentes.

Señale al objeto o al área donde esté disponible el menú,
pulse el botón 3 del ratón ymanténgalo pulsado.
# Nombres de archivo en el Gestor de aplicaciones
Gestor de aplicaciones:nombres de archivonombres de archivo:en el Gestor de aplicacionesetiquetas de acciónetiquetas para iconos de acción

En el Gestor de Archivos y el Gestor de aplicaciones, los archivos y
carpetas se representan como iconos,
y estos iconos se suelen etiquetar con el nombre de archivo.

Los iconos de acción en ocasiones constituyen una excepción a esta
regla. Por ejemplo, muestre el menú emergente para el icono de acción
en el grupo de aplicaciones de Herramientas_Escritorio etiquetado como
Reloj Digital. Observe que el nombre de archivo, que se muestra al
principio del menú emergente, es diferente a la etiqueta.

El nombre de archivo real está en los cuadros de diálogo Copiar
archivo y Enlazar archivo.
# Ubicación de la carpeta del Gestor de aplicaciones
Gestor de aplicaciones:ubicación de la carpeta

El Gestor de aplicaciones realiza unas funciones muy parecidas a las
del Gestor de Archivos. Esto es así porque el Gestor de aplicaciones
es una vista del Gestor de Archivos de una carpeta especial
del sistema que se utiliza para reunir aplicaciones registradas.
El Gestor de inicio de sesiones crea la carpeta del Gestor de
aplicaciones cada vez que el usuario inicia la sesión.

Normalmente, no es necesario que conozca la ubicación en la carpeta
del Gestor de aplicaciones. Sin embargo, puede serle de utilidad si
está intentando resolver problemas.

Su ubicación es:/var/dt/appconfig/appmanager/`nombre_especial_carpeta`

dondenombre_especial_carpetaes un nombre asignado por el sistema
que es exclusivo para el nombre de inicio de sesión y sistema.

No debenuncaintentar modificar directamente elnombre_especial_carpetadtappgatheracción Recargar Aplicaciones

Después de crear la carpeta, el Gestor de inicio de sesiones
ejecuta un programa de escritorio llamadodtappgather, que reúne todos los grupos de aplicaciones.

Durante una sesión, puede volver a ejecutardtappgatherefectuando una doble pulsación sobre
Recargar Aplicaciones en el grupo de aplicaciones
Herramientas_Escritorio.
# Barra de menús del Gestor de aplicaciones


La barra de menús del Gestor de aplicaciones contiene:








# Menú Archivo del Gestor de aplicaciones


Gestor de aplicaciones:menú Archivo

* **Carpeta nueva** 

Solicita un nombre de carpeta para crear una
carpeta nueva.
* **Archivo nuevo** 

Solicita un nombre de archivo para crear un
archivo nuevo.
* **Ir arriba** 

Sube un nivel en la jerarquía de carpetas.
* **Ir a** 

Muestra el cuadro de diálogo Ir a, que le permite
introducir un nombre de carpeta nuevo o seleccionar
uno en una lista de carpetas a la que ha ido con
anterioridad.
* **Buscar** 

Muestra el cuadro de diálogo Buscar, que le permite
buscar archivos y carpetas basándose en los
patrones de búsqueda de nombre de archivo o en el
contenido de archivo.
* **Cerrar** 

Cierra la vista actual del Gestor de aplicaciones.

# Menú Seleccionado del Gestor de aplicaciones


Gestor de aplicaciones:Menú Seleccionado

Muchos de los grupos de aplicaciones sólo puede modificarlos el
administrador del sistema. Por tanto, es posible que muchos de estos
elementos de menú no estén disponibles.

* **Mover a** 

Solicita el nuevo nombre de ruta completo del archivo
que desea mover.
* **Copiar en** 

Solicita un nuevo nombre de archivo para crear una
copia del archivo seleccionado.
El mandato Copiar sólo está disponible cuando hay
exactamente un archivo seleccionado.
* **Copiar como enlace** 

Solicita el nuevo nombre de ruta completo del enlace
que se creará para el objeto seleccionado.
* **Poner en espacio de trabajo** 

Coloca el objeto seleccionado en el ángulo derecho del
fondo del espacio de trabajo.
* **Echar en la papelera** 

Echa los objetos seleccionados a la Papelera.
* **Renombrar** 

Abre el campo editar nombre para el objeto
seleccionado.
* **Cambiar permisos** 

Muestra el cuadro de diálogo Cambiar permisos
para el objeto seleccionado.
Este cuadro de diálogo muestra los permisos para el
objeto.
* **Seleccionar todo** 

Selecciona todos los objetos de la vista actual del
Gestor de aplicaciones.
* **Deseleccionar todo** 

Deselecciona todos los objetos de la vista actual del
Gestor de aplicaciones.
* **acciones** 

Si el objeto seleccionado tiene acciones, se añaden al
final del menú. Son las mismas acciones que
aparecen en el menú emergente del objeto.

# Menú Vista del Gestor de aplicaciones


Gestor de aplicaciones:Menú Vistapreferencias de visualización en el Gestor de aplicaciones

* **Abrir en ventana nueva** 

Abre otra vista del Gestor de aplicaciones de la
carpeta actual.
* **Establecer opciones de vista** 

Muestra el cuadro de diálogo Establecer opciones de
vista para modificar el aspecto y las funciones de la
vista actual del Gestor de aplicaciones.
* **Guardar como opciones predeterminadas** 

Guarda el tamaño de ventana,
lista de filtro y opciones actuales como valores
predeterminados.
* **Mostrar objetos ocultos** 

Conmuta la visualización de archivos ocultos. Puede
especificar los datos que están ocultos seleccionando
Establecer opciones de filtro.
* **Establecer opciones de filtro** 

Muestra el cuadro de diálogo Establecer opciones de
filtro, que se utiliza para especificar archivos que
desea que estén ocultos (basándose en el tipo de datos
o nombre de archivo).
* **Actualizar** 

Refresca el contenido de la carpeta actual y vuelve a
mostrarla con todas las modificaciones.

Actualizar no reúne las aplicaciones que se han añadido desde que se
inició la sesión. Para volver a crear el contenido del
Gestor de aplicaciones, debe efectuar una doble pulsación
sobre Recargar Aplicaciones en el grupo de aplicaciones
Herramientas_Escritorio.

# Menú Ayuda del Gestor de aplicaciones


Gestor de aplicaciones:Menú Ayuda

* **Información general** 

Muestra información de introducción general acerca del
Gestor de aplicaciones.
* **Tareas** 

Muestra instrucciones de tarea del tipo "Cómo" específicas
para utilizar el Gestor de aplicaciones.
* **Referencia** 

Muestra información acerca de ventanas, menús y
cuadros de diálogo del Gestor de aplicaciones.
* **Sobre el tema** 

Cambia el cursor de pantalla por el cursor de
ayuda sobre el tema.
A continuación, pulse sobre un elemento de la
ventana del Gestor de aplicaciones para obtener
ayuda acerca de dicho elemento.
* **Uso de la ayuda** 

Proporciona ayuda acerca de como utilizar las ventanas
de ayuda.
* **Acerca del Gestor de aplicaciones** 

Muestra la versión e información de copyright del
Gestor de aplicaciones.

# Menú emergente para objetos del Gestor de aplicaciones


Gestor de aplicaciones:menús emergentesmenús emergentes:Gestor de aplicacionesLa mayor parte de objetos del Gestor de aplicaciones tienen sus
propios menús emergentes.

Al principio de cada menú emergente hay dos mandatos estándar:
Poner en el espacio de trabajo y Echar en la papelera.

Las acciones para el tipo de datos del objeto están al final
del menú emergente. Son las mismas acciones
que aparecen en el menú Acciones cuando se selecciona el icono del
objeto.

* **Poner en espacio de trabajo** 

Coloca el objeto en el escritorio del espacio de
trabajo actual.
La ubicación del objeto está determinada por el
recursoobjectPlacement, que toma el valor
predeterminado del ángulo superior derecho de la
pantalla.
* **Echar en la papelera** 

Elimina el objeto y lo tira a la Papelera.
* **Ayuda** 

Muestra la ayuda para los menús emergentes.
* **Acciones** 

Si el objeto tiene acciones, se añaden al final
del menú emergente.

# Ventana del Gestor de aplicaciones


Gestor de aplicaciones:ventana principalEl nivel superior del Gestor de aplicaciones contiene iconos que
representan grupos de aplicaciones.

Cada grupo de aplicaciones es una carpeta que contiene uno o másiconos de aplicación(los iconos de aplicación también se
denominaniconos de acción). Un grupo de aplicaciones también
puede contener otros tipos de archivos de aplicaciones, por
ejemplo plantillas y archivos "read me".
# Cuadro de diálogo Copiar Archivo del Gestor de aplicaciones


Gestor de aplicaciones:copia de objetoscopia de objetos

* **Objeto seleccionado** 

Muestra el objeto que se va a copiar.
* **Carpeta destino** 

Escriba el nuevo nombre de ruta completo de la
carpeta destino.
* **Nombre para copia** 

Escriba el nombre del nuevo objeto.
* **OK** 

Realiza la copia y cierra el
cuadro de diálogo.
* **Cancelar** 

Cancela el mandato Copiar y cierra el
cuadro de diálogo.
* **Mostrar icono** 

&newline;Muestra el icono que se utilizará para el nuevo
archivo.
* **Ayuda** 

Muestra la ayuda para este cuadro de diálogo.


También puede utilizar el ratón para copiar archivos y carpetas.
# Enlazar archivo del Gestor de aplicaciones/Cuadro de diálogo de carpeta




* **Objeto seleccionado** 

Muestra el objeto que se enlazará.
* **Carpeta destino** 

Escriba el nuevo nombre de ruta completo de la
carpeta destino.
* **Nombre para copia** 

Escriba el nombre del nuevo objeto.
* **OK** 

Realiza la copia y cierra el
cuadro de diálogo.
* **Cancelar** 

Cancela el mandato Copiar y cierra el
cuadro de diálogo.
* **Mostrar icono** 

&newline;Muestra el icono que se utilizará para el nuevo
archivo.
* **Ayuda** 

Muestra la ayuda para este cuadro de diálogo.


También puede utilizar el ratón para enlazar archivos o carpetas.
# Cuadro de diálogo Mover archivo del Gestor de aplicaciones


Gestor de aplicaciones:mover objetosmover objetos

* **Objeto seleccionado** 

Muestra el archivo o carpeta que se moverá.
* **Carpeta destino** 

Escriba el nuevo nombre de ruta completo de la
carpeta destino.
* **OK** 

Realiza la operación de mover y cierra el
cuadro de diálogo.
* **Cancelar** 

Cancela el mandato Mover y cierra el
cuadro de diálogo.
* **Ayuda** 

Muestra la ayuda acerca de cómo renombrar archivos y
carpetas.

# Cuadro de diálogo de Archivo nuevo del Gestor de aplicaciones




* **Nombre del nuevo archivo** 

&newline;Escriba el nombre del archivo o carpeta nuevos. Si
el archivo se está creando en una carpeta diferente,
debe escribir el nombre de ruta completo de la
carpeta o archivo nuevos que desea crear.
* **OK** 

Crea el archivo y cierra el cuadro de diálogo.
* **Aplicar** 

Crea el archivo y mantiene el cuadro de diálogo
para que pueda crear otro archivo nuevo.
* **Cancelar** 

Cancela el mandato Archivo nuevo y cierra el cuadro
de diálogo.
* **Mostrar icono** 

Si cambia el archivo por un tipo de archivo diferente
cuando introduce el Nombre de archivo nuevo, es
posible que cambie su icono. Para ver una vista previa
del icono nuevo, pulse Mostrar icono y se
actualizará el icono del interior del
cuadro de diálogo. Por ejemplo,
si escribe un nombre que termina en.tif,
seleccione Mostrar icono y verá el icono del
tipo de datos TIFF.
* **Ayuda** 

Muestra la ayuda acerca de este cuadro de diálogo.

# Cuadro de diálogo Carpeta nueva del Gestor de aplicaciones




* **Nombre de la carpeta nueva** 

&newline;Escriba el nombre de la carpeta nueva. Si la carpeta
se está creando en una carpeta diferente, debe
escribir el nombre de ruta completo de la carpeta
nueva que desea crear.
* **OK** 

Crea la carpeta y cierra el cuadro de diálogo.
* **Aplicar** 

Crea la carpeta y mantiene el cuadro de diálogo
para que pueda crear otra carpeta.
* **Cancelar** 

Cancela el mandato Carpeta nueva y cierra el cuadro
de diálogo.
* **Mostrar icono** 

Si cambia el tipo de datos cuando introduce
el Nombre de carpeta nueva, es posible que cambie su
icono. Para visualizar una vista previa
del icono nuevo, pulse Mostrar icono y se
actualizará el icono del interior del
cuadro de diálogo.
* **Ayuda** 

Muestra la ayuda acerca de este cuadro de diálogo.

# Cuadro de diálogo Establecer opciones de filtro del Gestor de aplicaciones


Gestor de aplicaciones:filtrado de objetosfiltrado de objetos

* **Seleccionar tipos de archivo a ocultar o mostrar** 

Este botón conmuta entre Ocultos y Mostrados.
Se muestra una lista de todos los tipos de datos
definidos en el sistema.
Los tipos de datos seleccionados se mostrarán o no
se mostrarán en el Gestor de aplicaciones,
dependiendo de si el botón conmutador muestra Ocultos
o Mostrados.
* **Seleccionar todo** 

Selecciona todos los tipos de datos. A menos que
a continuación deseleccione algunos, el área de
visualización del Gestor de aplicaciones estará vacía.
* **Deseleccionar todo** 

Deselecciona todos los tipos de datos.
* **Serie de filtro (Opcional)** 

Le permite filtrar por nombre de archivo. Por ejemplo,
si escribe*.o, el Gestor de aplicaciones no
mostrará archivos con nombres que terminen
en.o. Observe que cualquier tipo de datos que
introduce en este campo se añade a la lista de tipos
de datos seleccionados en la lista de iconos de la
parte superior del cuadro de diálogo.
* **OK** 

Aplica los valores de filtro actuales y cierra el
cuadro de diálogo.
* **Aplicar** 

Aplica los valores de filtro actuales sin cerrar el
cuadro de diálogo.
* **Predeterminados** 

Restaura la lista de filtro predeterminada (que incluye DOT_FILE,
DOT_FOLDER y CURRENT_FOLDER). La lista de filtro no se
aplicará hasta que seleccione Aplicar u OK.
* **Cancelar** 

Restaura los últimos valores aplicados y cierra el
cuadro de diálogo.
* **Ayuda** 

Muestra la ayuda para filtrar objetos.

# Cuadro de diálogo Buscar archivos o carpetas del Gestor de aplicaciones


Utilice el cuadro de diálogo Buscar archivos o carpetas para buscar
una carpeta y las carpetas que contiene, para archivos con un nombre
o contenido determinado.

* **Nombre de archivo o carpeta** 

Escriba el nombre del archivo o carpeta que desea
buscar. Puede utilizar caracteres comodín.
* **Contenido del archivo** 

Buscar efectuará una búsqueda
del texto que escriba en este campo dentro de
archivos y carpetas.
* **Carpeta de búsqueda** 

Escriba la ruta de la carpeta donde desea que se
inicie la búsqueda. La búsqueda se iniciará en
dicha carpeta e incluye todas sus subcarpetas.
* **Archivos encontrados** 

Lista los archivos o carpetas encontrados en la
búsqueda. Efectúe una doble pulsación en un
archivo o carpeta de la lista para abrir una vista
nueva del Gestor de aplicaciones que muestre dicho
archivo o carpeta.
* **Abrir nueva vista** 

Abre una vista del Gestor de aplicaciones de la
carpeta que contiene el archivo que se ha
seleccionado en Archivos encontrados. Si se ha
encontrado una carpeta, la vista es el contenido
de dicha carpeta.
* **Poner en espacio de trabajo** 

Coloca el archivo o carpeta seleccionados en el fondo
del espacio de trabajo actual.
* **Iniciar** 

Inicia la búsqueda.
* **Parar** 

Detiene una búsqueda que se está llevando a cabo.
Observe que este botón puede utilizarse incluso
cuando se muestra el cursor en forma de reloj de arena.
* **Cancelar** 

Detiene una búsqueda que se está llevando a cabo y
cierra el cuadro de diálogo.
* **Ayuda** 

Muestra la ayuda acerca de cómo buscar objetos.

# Cuadro de diálogo Cambiar permisos del Gestor de aplicaciones


Utilice el cuadro de diálogo Cambiar permisos para cambiar los
permisos de lectura, grabación y ejecución de los archivos o
carpetas que posee. El propietario del archivo o carpeta es el
único que puede cambiar los permisos. Si no es el propietario del
archivo o carpeta, el recuadro mostrará los valores actuales; no
se podrán cambiar. El cuadro de diálogo Cambiar permisos también
muestra el nombre de ruta completo, el tamaño, y la fecha y
hora en que se modificó por última vez el archivo o carpeta.

* **Nombre del propietario** 

El nombre del usuario que posee el objeto. El
administrador del sistema (usuario root) es el único
que puede cambiar el propietario de un objeto.
* **Nombre del grupo** 

Es el nombre del grupo de usuarios que reciben los
permisos mostrados en la fila Grupo
de la lista de permisos. Si usted es el propietario,
puede cambiar el grupo por otro grupo al que
pertenezca. Un usuario root puede cambiarlo por
cualquier otro grupo.
* **Permisos** 

Si es usted el propietario, puede cambiar los permisos
de lectura, escritura y ejecución. Seleccione una
casilla de verificación para proporcionar el permiso.
* **OK** 

Aplica los valores actuales y cierra el cuadro de
diálogo.
* **Cancelar** 

Cierra el cuadro de diálogo sin efectuar ninguna
modificación.
* **Ayuda** 

Muestra la ayuda acerca de cómo cambiar permisos.

# Cuadro de diálogo Establecer opciones de vista del Gestor de aplicaciones


Gestor de aplicaciones:opciones de vistaopciones de vista en el Gestor de aplicacionesUtilice el cuadro de diálogo Establecer opciones de vista para
modificar la manera en que los archivos y las carpetas se
representan en el Gestor de aplicaciones.


# Cabeceras


La preferencia de Cabeceras indica las líneas de cabecera que desea
que se muestren en la ventana del Gestor de aplicaciones.

* **Mostrar ruta con iconos** 

Muestra la carpeta actual como una cadena de
iconos de carpeta.
* **Ruta de texto** 

Muestra el nombre de ruta completo de la carpeta
actual en una línea de texto, justo encima del área
de visualización principal. Puede pulsar una vez
en el texto y editar el nombre de ruta o efectuar
una doble pulsación en uno de los nombres de carpeta
para cambiar a dicha carpeta.
* **Línea de mensajes** 

Muestra el número total de archivos, carpetas y
archivos ocultos de la carpeta actual.



# Colocación


La preferencia de Colocación indica cómo desea que estén ordenados
los iconos dentro de la vista del Gestor de aplicaciones.

* **Como está colocado** 

Los objetos se colocan exactamente donde los sitúa.
* **Ordenados en cuadrícula** 

Los objetos se vuelven a ordenar y se disponen
en filas y columnas.



# Mostrar
árbol de carpetavista de árbol (Gestor de aplicaciones)

* **Por carpeta única** 

Muestra el contenido de la carpeta actual.
* **Por árbol** 

Muestra el contenido de la carpeta actual
en forma de árbol.
* **Sólo carpetas** 

Si se ha seleccionado Por árbol, la vista sólo
muestra carpetas.
Este es el valor predeterminado.
* **Carpetas, después archivos** 

Si se ha seleccionado Por árbol, la vista muestra
carpetas, y a continuación archivos si efectúa una
pulsación en el signo +.
* **Carpetas y archivos** 

Si se ha seleccionado Por árbol, la vista muestra
todas las carpetas y archivos.



# Representación


* **Por nombre sólo** 

Se muestra cada uno de los objetos sólo como un nombre.
* **Por iconos grandes** 

Se muestra cada uno de los objetos con su nombre y un
icono grande.(Este es el valor predeterminado.)
* **Por iconos pequeños** 

Se muestra cada uno de los objetos con su nombre y un
icono pequeño.

formato de listado largo
* **Por nombre, fecha, tamaño, ...** 

Se muestra cada uno de los objetos en un listado largo
que muestra nombre, fecha de modificación, tamaño,
permisos, propietario y grupo.



# Ordenar


Seleccione el orden en que se mostrarán los archivos y carpetas:

* **Alfabéticamente** 

Seleccione Ascendente (de la A a la Z, por tanto de
la a a la z) o Descendente (de la Z a la A, por tanto
de la z a la a). (El valor predeterminado es
alfabético, de la A a la Z.)
* **Por tipo de archivo** 

Los archivos se agrupan juntos según el tipo de datos.
* **Por fecha** 

Seleccione Ascendente (del más antiguo al más reciente) o
Descendente (del más reciente al más antiguo).
* **Por tamaño** 

Seleccione Ascendente (del más pequeño al más grande)
o Descendente (del más grande al más pequeño).

# Dirección


Seleccione la dirección en que se mostrarán los archivos y carpetas:

* **Ascendente** 

Del más antiguo al más reciente, del más pequeño al más
grande.
* **Descendente** 

Del más reciente al más antiguo, del más grande al más
pequeño.
