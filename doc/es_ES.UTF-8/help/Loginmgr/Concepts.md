
# Conceptos acerca del Gestor de inicio de sesiones





# Introducción para sesiones de Escritorio
iniciar: sesión de Escritorioiniciar sesión de EscritorioEscritorio: iniciar sesiónsesión: iniciar de Escritorioutilización: sesión de iniciosesión de inicio: definidasesión actual definida

Una sesión es la colección de aplicaciones, valores y recursos
que están presentes en su escritorio.
Una sesión de escritorio tiene lugar entre el momento en que inicia la
sesión y el momento en que la finaliza. La pantalla de inicio de
sesión, creada por el Gestor de inicio de sesiones, es su puerta
para el escritorio. Le proporciona un lugar para
escribir el ID de usuario y la contraseña.

El menú Opciones de la pantalla de inicio de sesión lista sus opciones.
Además de ejecutar una sesión de escritorio, puede elegir ejecutar
una sesión de seguridad contra anomalía.
También puede seleccionar el idioma para la sesión.

Después de que el gestor de inicio de sesiones autentifica el ID de
usuario y la contraseña, el Gestor de sesiones entra en función.
La gestión de sesiones es un conjunto de convenios y
protocolos que permiten que un gestor de sesiones especial,
por ejemplo el Gestor de Sesiones de Entorno de Escritorio Común,
guarde y restaure su sesión. Puede iniciar la sesión en el sistema
y tener presente el mismo conjunto de aplicaciones de ejecución,
valores y recursos que estaban presentes cuando finalizó la
sesión.
El Gestor de sesiones guarda y restaura:

Los valores del comportamiento y el aspecto&emdash;por ejemplo,
valores de ratón, fonts y colores.

Las aplicaciones de ventana que se estaban ejecutando&emdash;por
ejemplo, las ventanas del Gestor de archivos y el Editor de textos.
El Gestor de sesiones no puede guardar y restaurar ciertos
tipos de aplicaciones. Por ejemplo, si inicia el editorvidesde una línea de mandatos en una ventana de Terminal, el Gestor de
sesiones no puede restaurar la sesión de edición.

Cuando se inicia la sesión en el escritorio por primera vez, se carga una
sesión inicial predeterminada. A partir de ese momento,
el Gestor de sesiones soporta la noción de una sesión actual y una
sesión de inicio.
# Sesión inicial


Cuando inicie la sesión en el escritorio por primera vez,
el Gestor de sesiones generará la sesión inicial utilizando
valores predeterminados del sistema.
De forma predeterminada, se inician automáticamente el Gestor de
archivos y una Introducción al Entorno de Escritorio Común.
# Sesión actual


De forma ordinaria, el escritorio guarda información de sesión
cuando finaliza la sesión y utiliza dicha
información para iniciar la siguiente sesión. Si inicia o detiene
aplicaciones durante la sesión, o utiliza el Gestor de estilos para
cambiar el aspecto y el comportamiento del sistema, las modificaciones
que realiza se reflejan en la siguiente sesión.

La sesión en ejecución siempre se considera lasesión
actual, tanto si se restaura en el momento de iniciar la sesión a
partir de una sesión de inicio guardada, una sesión actual guardada o
la sesión inicial predeterminada del sistema. Basándose en los
valores de Arranque del Gestor de estilos, el Gestor de sesiones
guarda la sesión actual de manera automática cuando el usuario
sale de la sesión. Cuando se inicia la sesión en el escritorio
la siguiente vez, se restaura la sesión actual que se ha guardado
con anterioridad. Esto significa que el escritorio se restaurará
al mismo estado que tenía cuando finalizó la sesión por última vez.
# Sesión de inicio


El escritorio también proporciona unasesión de inicio. Una
sesión de inicio es una sesión que se guarda de forma explícita.
Es como tomar una instantánea de la sesión actual en un
momento determinado. Una vez se ha guardado una sesión inicial,
se puede especificar que el escritorio restaure dicha sesión en lugar
de la sesión actual la siguiente vez que se inicie la sesión.

Consulte las siguientes tareas:






# Otras formas de iniciar la sesión
sesión: tipos diferentes a Regulariniciar: sesión de seguridad contra anomalíainiciar: sesión en modalidad Inicio de sesión de línea de mandatossesión de seguridad contra anomalía: iniciarmodalidad Inicio de sesión de línea de mandatos: iniciar sesión

Además de la sesión de escritorio normal, existen tipos adicionales
de sesiones:

* **Seguridad contra anomalía** 

Una sesión de seguridad contra anomalía
proporciona una ventana de Terminal y un Gestor de ventanas.
Es útil para ejecutar varios mandatos antes de iniciar la
sesión en otra sesión de escritorio.
(Consulte.)
* **Inicio de sesión de línea de mandatos** 

Le permite dejar el
escritorio de manera temporal para trabajar en la consola del
sistema.
(Consulte.)
