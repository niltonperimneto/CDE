
# Tareas de Crear Acción

# Crear y editar acciones





# Crear y editar tipos de datos





# Iconos para acciones y tipos de datos



# Crear una acción con Crear Acción
acción: crear

Abra el grupo de aplicaciones Apls_Escritorio del Gestor de
aplicaciones y efectúe una doble pulsación sobre Crear Acción.

Con ello se mostrará la ventana principal de Crear Acción.

En el campo de texto Nombre de Acción, escriba el nombre que
etiquetará al icono de la acción.
El nombre de acción puede incluir todos los caracteres excepto espacios.

Utilice los controles de Iconos de Acción para especificar el icono
para la aplicación. Al principio, aparece el icono
predeterminado.

Para usar un icono existente distinto, pulse Buscar Conjunto para
abrir el cuadro de diálogo Buscar Conjunto. Para obtener más
información, consulte.

Para crear iconos nuevos, pulse Editar Icono para ejecutar
el Editor de iconos.
Consulte.

En el campo Mandato Cuando Se Abre la Acción, escriba el mandato
para iniciar la aplicación. Utilice la sintaxis$`n`para un argumento de archivo.
Por ejemplo:emacs
bitmap $1
diff $1 $2
lp -oraw $1

Si la línea de mandatos incluye un argumento de archivo ($`n`),
entonces el icono de acción será una zona para soltar para archivos.

Las líneas de mandatos no se pasan a un shell a menos que especifique
de forma explícita la utilización de un shell.
Por ejemplo, estas líneas utilizan el proceso del shell:/bin/sh -c `ps | lp'
/bin/sh -c `spell $1 | more'

En el campo Texto de Ayuda para Icono de Acción, escriba la
información de ayuda que se mostrará cuando el usuario obtenga
la ayuda sobre el tema para elicono de acción.

El texto se ajustará automáticamente en el campo de texto. Sin
embargo, estas divisiones de la línea no se
mantienen en línea. Si desea forzar una línea de división,
utilice&bsol;n.

Utilice el menú de botones de Tipo de Ventana para seleccionar el
tipo de soporte necesario

* **Gráfica (X-Window)** 

La aplicación crea su propia ventana.
* **Terminal (Cierre Automático)** 

La aplicación se ejecutará en una
ventana de terminal que se cerrará automáticamente cuando el usuario
salga de la aplicación.
* **Terminal (Cierre Manual)** 

La aplicación se ejecutará en una ventana
de terminal que permanece abierta hasta que el usuario la cierra de forma
explícita.
* **Sin Salida** 

La aplicación no crea ninguna salida en la pantalla.


Haga lo siguiente:

Si la aplicación tiene archivos de datos, y desea crear uno o más
tipos de datos para ellos, vea.

Si no necesita crear ningún tipo de datos:

Guarde la acción seleccionando Guardar en el menú Archivo.

Pruebe la acción nueva efectuando una doble pulsación sobre su
icono en el directorio de inicio del usuario.
# Crear un tipo de datos con Crear Acción
tipo de datos: crear

Defina la acción para la aplicación. Consulte los pasos del 1 al 6
del tema.

Pulse el botón Mostrar Funciones Avanzadas para ampliar la ventana Crear Acción.

Si desea que elicono de acciónsolicite un argumento
de archivo cuando se pulse dos veces un icono, escriba el texto del
indicador de solicitud en el campo de texto "Cuando se Abra la
Acción, Solicitar a los Usuarios".

Debe utilizar este campo si la línea de mandatos de la aplicación
tiene un argumento de archivo obligatorio.

Debe dejar este campo en blanco si la línea de mandatos de la
aplicación no tiene ningún argumento de archivo.

Si el argumento de archivo de la línea de mandatos de la
aplicación es opcional, dispone de una opción.
Si proporciona el texto de la solicitud, el icono de acción
solicitará el archivo cuando se efectúe una
doble pulsación sobre el mismo.
Si no proporciona el texto de la solicitud, la acción se ejecutará
con una cadena nula como el argumento de archivo.

Especifique los tipos de archivos que la acción aceptará como
argumentos:

Si la acción puede aceptar cualquier tipo de datos, seleccione
Todos los Tipos de Datos.

Si la acción sólo puede aceptar el tipo o tipo de datos creados
para la aplicación, seleccione Sólo la Lista Anterior.

Inicialmente, la lista Tipos de Datos Que Usan Esta Acción está
vacía. A medida que
se crean tipos de datos para la
aplicación, se añaden a la lista.

Pulse Añadir para mostrar el cuadro de diálogo Añadir Tipo de
Datos.

Opcional: si no desea utilizar el nombre de tipo de datos
predeterminado, escriba un nombre nuevo para el tipo de datos en el
campo de texto Nombre de Tipo de Datos. El nombre
no puede incluir espacios.

Pulse el botón Editar, que se encuentra al lado del cuadro
Características de Identificación, para mostrar el cuadro de diálogo
Características de Identificación.

Utilice este cuadro de diálogo para determinar las características
usadas para diferenciar unos tipos de datos de otros. Puede elegir un criterio
(por ejemplo, Patrón de Nombre) o
combinar criterios (por ejemplo, Patrón de Nombre y
Patrón de Permiso).

Pulse uno de los botones de conmutación de Archivos o Carpetas para
especificar si el tipo de datos presenta un archivo o una carpeta.

Si el tipo de datos depende del nombre, pulse el cuadro de
comprobación Patrón de Nombre y escriba el patrón de nombre.
Puede utilizar*y?como comodines.

* ***** 

Coincide con cualquier secuencia de caracteres.
* **?** 

Coincide con cualquier carácter único.


Si el tipo de datos depende de los permisos, pulse el cuadro de
comprobación Patrón de Permiso y seleccione los permisos necesarios
para los tipos de datos.

* **Activar** 

El archivo debe tener el permiso especificado.
* **Desactivar** 

El archivo no debe tener el permiso especificado.
* **Ambos** 

(Predeterminado) El permiso no es relevante.


Si el tipo de datos depende del contenido, seleccione el cuadro de
comprobación Contenido y proporcione la información solicitada
relativa al Patrón de búsqueda y al Tipo de contenido. De forma opcional,
puede proporcionar la ubicación de byte donde debe iniciarse la búsqueda.

Pulse OK para cerrar el cuadro de diálogo Características de
Identificación.

Las características se mostrarán en el recuadro
Características de Identificación del cuadro de diálogo
Añadir Tipo de Datos.

La información de permisos del cuadro Características de
Identificación utiliza estos códigos:

* **d** 

Directorio
* **r** 

Permiso de lectura
* **w** 

Permiso de escritura
* **x** 

Ejecutable
* **!** 

NOT
* **&** 

AND


Escriba el texto de ayuda para el tipo de datos en el campo Texto
de Ayuda

Utilice los controles de Iconos de Tipo de Datos para especificar
el icono para la aplicación.

Al principio, se muestran los iconos predeterminados.

Para usar un icono existente distinto, pulse Buscar Conjunto para
abrir el cuadro de diálogo Buscar Conjunto. Para obtener más
información, consulte.

Para crear iconos nuevos, pulse Editar Icono para ejecutar
el Editor de iconos.
Véase.

Opcional: si la aplicación proporciona un mandato de impresión para
imprimir archivos de datos desde la línea de mandatos, escriba el
mandato en el campo de texto Mandato para Imprimir, utilizando
la sintaxis$`n`para un argumento de archivo.

Pulse OK para cerrar el cuadro de diálogo Añadir Tipo de Datos y
añada el tipo de datos a la lista de la ventana Crear Acción.
# Especificar iconos para acciones y tipos de datos


Tanto la ventana principal Crear Acción como el cuadro de diálogo
Añadir Tipo de Datos incluyen botones para especificar el icono que
las acciones y los tipos de datos van a utilizar.

Para utilizar un icono existente, pulse Buscar Conjunto. Vea.

Para crear un icono nuevo con el Editor de iconos, pulse Editar
Icono. Vea.
# Usar el cuadro de diálogo Buscar Conjunto


Al pulsar Buscar Conjunto en la ventana principal Crear Acción o en
el cuadro de diálogo Añadir Tipo de Datos, se muestra el cuadro de
diálogo Buscar Conjunto.

El cuadro de diálogo Buscar Conjunto sirve para especificar:

Un icono ubicado en una carpeta de la lista Carpetas de Iconos.

La lista Carpetas de Iconos incluye todas las carpetas de la ruta de
búsqueda de iconos.

Un icono que forme parte de un paquete de registro que se integrará
al escritorio mediantedtappintegrate.
# Para especificar un icono en una carpeta de iconos


En la lista Carpetas de Iconos, pulse dos veces sobre la ruta de la
carpeta que contiene el icono.
La lista Archivos de Iconos mostrará todos los archivos de iconos
en dicha carpeta.

En la lista Archivos de Iconos, pulse el icono que desee utilizar.

Pulse OK.
# Para especificar un icono en un paquete de registro


Si, como administrador del sistema o programador, está creando un
paquete de registro, los archivos de imágenes de iconos se ubicarán
en principio en el directorio del paquete de registro:`apl_root`/dt/appconfig/icons/`idioma`

Después del registro, los archivos de iconos se encontrarán en unacarpeta de iconos.
Por tanto, las definiciones de acciones y de tipo de datos
deben utilizar el nombre de archivo base.

Utilice este procedimiento para especificar un nombre base para un
icono que no se encuentre en una carpeta de iconos:

En el cuadro de texto Entrar Nombre Archivo Iconos,
escriba elnombre basedel archivo de iconos.

Pulse OK.

Aparecerá un cuadro de diálogo de error, que le informará de que el
archivo de iconos no se ha encontrado en ninguna carpeta de iconos.

En el cuadro de diálogo de error, pulse Nombre OK. Se cierra el
cuadro de diálogo y también el cuadro de diálogo Buscar Conjunto.

Ignore la ausencia de imágenes de iconos en el campo Iconos de
Acción o Iconos de Tipo de Datos. La imagen del icono se localizará
cuando se haya registrado la aplicación.
# Crear una nueva imagen de icono


Pulse Editar Icono en la ventana Crear Acción o Editar Tipo de Datos.
Con esto se ejecuta el Editor de iconos.

Utilice el Editor de iconos para dibujar un icono nuevo. Vea:

Tareas del Editor de
iconos.



Seleccione Guardar en el menú Archivo para guardar el archivo de iconos.

Guarde el archivo de iconos en la carpeta`CarpetaInicio`/.dt/icons.
Vea.

Si, como administrador del sistema o programador de aplicaciones,
está creando un paquete de registro, deberá guardar el archivo de
iconos en el directorio`apl_root`/dt/appconfig/icons/`idioma`.
# Tamaños y nombres de iconos


Los iconos del escritorio utilizan estos convenios de denominación:

* **Tamaño (Pixels)** 

Nombre
* **32 por 32** 

`nombrebase`.m.pmo`nombrebase`.m.bm
* **16 por 16** 

`nombrebase`.s.pmo`nombrebase`.s.bm


Para los iconos de acciones, utilice el nombre de acción como
nombre base.

Para los iconos de tipos de datos, utilice el nombre de tipo de
datos como nombre base.
# Editar una acción existente
acciones: editar

Crear Acción se puede utilizar para editar una acción ya existente
si:

La acción se ha creado mediante Crear Acción.

Y tambiénsi el archivo que contiene la definición de acción no
se ha editado manualmente (utilizando un editor de textos).

Abra el grupo de aplicaciones Apls_Escritorio del Gestor de
aplicaciones y pulse dos veces sobre Crear Acción.

Con esto se muestra la ventana principal Crear Acción.

Seleccione Abrir en el menú Archivo. Aparecerá el cuadro de diálogo
Abrir.

En la lista de Archivos, seleccione el archivo que contiene la
definición de acción.
Su nombre es`nombre_acción`.dt.

Seleccione OK.
# Editar un tipo de datos existente
tipo de datos: editar

Crear Acción puede utilizarse para editar un tipo de datos existente
si:

El tipo de datos se ha creado mediante Crear Acción.

Y tambiénsi el archivo que contiene el tipo de datos no se
ha editado manualmente (utilizando un editor de textos).

Abra el grupo de aplicaciones Apls_Escritorio del Gestor de
aplicaciones y pulse dos veces sobre Crear Acción.

Con esto se muestra la ventana principal Crear Acción.

Seleccione Abrir en el menú Archivo. Aparecerá el cuadro de diálogo
Abrir.

En la lista de Archivos, pulse dos veces el archivo que contiene la
definición del tipo de datos.
El nombre del archivo es`nombre_acción`.dt, siendonombre_acciónel nombre de la acción creada
simultáneamente al tipo de datos.

Pulse Avanzada.

En la lista Tipo de datos que usan esta acción, seleccione el
tipo de datos que se va a editar.

Pulse Editar para abrir el cuadro de diálogo Editar Tipo de Datos.

Realice modificaciones en el cuadro de diálogo Editar Tipo de
Datos. Cuando haya finalizado, pulse OK.

Para guardar la definición editada, seleccione Guardar en el menú
Archivo.