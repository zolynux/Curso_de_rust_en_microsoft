# Curso de Rust en Microsoft 🦀

**Primeros pasos con Rust**
¿Está interesado en aprender en un nuevo lenguaje de programación que está creciendo en uso y popularidad? ¡Empiece por aquí! Siente las bases del conocimiento que necesita para compilar programas rápidos y eficaces en Rust.

En esta ruta de aprendizaje, hará lo siguiente:

- Instalar las herramientas necesarios para escribir sus primeras líneas de código de Rust.
- Aprender los conceptos básicos de Rust.
- Aprender a administrar los errores.
- Administrar la memoria en Rust.
- Usar tipos y rasgos genéricos.
- Configurar módulos para paquetes y contenedores.
- Escribir y ejecutar pruebas automatizadas.
- Crear una herramienta de línea de comandos.

---

# Tabla de contenido

- [Curso de Rust en Microsoft 🦀](#curso-de-rust-en-microsoft-)
- [Tabla de contenido](#tabla-de-contenido)
- [Qué es Rust 1](#qué-es-rust-1)
  - [Introducción](#introducción)
    - [¿Cuál es la mejor manera de aprender Rust?](#cuál-es-la-mejor-manera-de-aprender-rust)
    - [Objetivos de aprendizaje en Qué es Rust](#objetivos-de-aprendizaje-en-qué-es-rust)
  - [¿Qué es Rust?](#qué-es-rust)
  - [Características únicas de Rust](#características-únicas-de-rust)
    - [Administración de código con el sistema de módulo de Rust](#administración-de-código-con-el-sistema-de-módulo-de-rust)
    - [Uso de crates y bibliotecas de Rust](#uso-de-crates-y-bibliotecas-de-rust)
    - [Creación y administración de proyectos con Cargo](#creación-y-administración-de-proyectos-con-cargo)
    - [Cuándo se debe usar Rust](#cuándo-se-debe-usar-rust)
    - [Comprobación de conocimientos](#comprobación-de-conocimientos)
  - [El área de juegos de Rust](#el-área-de-juegos-de-rust)
    - [Herramientas y características](#herramientas-y-características)
    - [Opciones de compilación](#opciones-de-compilación)
    - [Límites de protección](#límites-de-protección)
  - [Ejercicio](#ejercicio)
    - [Escritura de código en el área de juego](#escritura-de-código-en-el-área-de-juego)
    - [Compilación y ejecución de código en el sitio de prueba](#compilación-y-ejecución-de-código-en-el-sitio-de-prueba)
    - [Guardado y uso compartido de código en el sitio de prueba](#guardado-y-uso-compartido-de-código-en-el-sitio-de-prueba)
  - [Resumen](#resumen)
    - [Prueba de las recetas en la guía paso a paso de Rust](#prueba-de-las-recetas-en-la-guía-paso-a-paso-de-rust)
    - [Referencias](#referencias)
- [Configuración el entorno de desarrollo Rust](#configuración-el-entorno-de-desarrollo-rust)
  - [Introducción sobre el entorno de desarrollo](#introducción-sobre-el-entorno-de-desarrollo)
    - [Objetivos de aprendizaje](#objetivos-de-aprendizaje)
  - [Instalar Visual Studio Code](#instalar-visual-studio-code)
    - [Herramientas para escribir código de Rust](#herramientas-para-escribir-código-de-rust)
    - [Comprobación de conocimientos en configuración el entorno de desarrollo Rust](#comprobación-de-conocimientos-en-configuración-el-entorno-de-desarrollo-rust)
  - [Instalación de las herramientas de compilación de Visual C++](#instalación-de-las-herramientas-de-compilación-de-visual-c)
  - [Instalación de Rust](#instalación-de-rust)
    - [Comprobación de la instalación de Rust](#comprobación-de-la-instalación-de-rust)
    - [Comprobación de conocimientos en Instalación de Rust](#comprobación-de-conocimientos-en-instalación-de-rust)
  - [Ejercicio: Hola mundo](#ejercicio-hola-mundo)
    - [Creación de un nuevo directorio para organizar el código](#creación-de-un-nuevo-directorio-para-organizar-el-código)
    - [Escritura del primer programa con Rust](#escritura-del-primer-programa-con-rust)
    - [Compilación y ejecución del programa](#compilación-y-ejecución-del-programa)
    - [Creación de un proyecto con Cargo](#creación-de-un-proyecto-con-cargo)
    - [Compilación y ejecución del programa con Cargo](#compilación-y-ejecución-del-programa-con-cargo)
  - [Resumen 1](#resumen-1)
- [Creación del primer programa de Rust](#creación-del-primer-programa-de-rust)
  - [Introducción sobre creación del primer programa](#introducción-sobre-creación-del-primer-programa)
    - [Área de juegos de Rust](#área-de-juegos-de-rust)
    - [Objetivos de aprendizaje en creación del primer programa de Rust](#objetivos-de-aprendizaje-en-creación-del-primer-programa-de-rust)
  - [Descripción de la estructura básica de programas de Rust](#descripción-de-la-estructura-básica-de-programas-de-rust)
    - [Funciones en Rust](#funciones-en-rust)
    - [Sangría del código](#sangría-del-código)
    - [Macro todo! macro](#macro-todo-macro)
    - [El println! macro](#el-println-macro)
    - [Sustitución de valores para argumentos {}](#sustitución-de-valores-para-argumentos-)
    - [Comprobación de conocimiento](#comprobación-de-conocimiento)
  - [Creación y uso de variables en Rust](#creación-y-uso-de-variables-en-rust)
    - [Variable](#variable)
    - [Inmutable frente a mutable](#inmutable-frente-a-mutable)
    - [Propiedad reemplazada de variables](#propiedad-reemplazada-de-variables)
    - [Comprobación de conocimientos Creación y uso de variables en Rust](#comprobación-de-conocimientos-creación-y-uso-de-variables-en-rust)
  - [Exploración de tipos de datos para números, texto y valores true/false](#exploración-de-tipos-de-datos-para-números-texto-y-valores-truefalse)
    - [Tipos de datos integrados](#tipos-de-datos-integrados)
    - [Números: valores enteros y de puntos flotante](#números-valores-enteros-y-de-puntos-flotante)
    - [Valores booleanos: true o false](#valores-booleanos-true-o-false)
    - [Texto: caracteres y cadenas](#texto-caracteres-y-cadenas)
    - [Characters](#characters)
    - [Cadenas](#cadenas)
    - [Ejemplo de texto](#ejemplo-de-texto)
    - [Comprobación de conocimientos: Exploración de tipos de datos para números, texto y valores true/false](#comprobación-de-conocimientos-exploración-de-tipos-de-datos-para-números-texto-y-valores-truefalse)
  - [Definición de colecciones de datos mediante tuplas y estructuras](#definición-de-colecciones-de-datos-mediante-tuplas-y-estructuras)
    - [Tuplas](#tuplas)
    - [Definición de una tupla](#definición-de-una-tupla)
    - [Acceso a elementos de una tupla](#acceso-a-elementos-de-una-tupla)
    - [Estructuras](#estructuras)
    - [Definición de una estructura](#definición-de-una-estructura)
    - [Estructura clásica](#estructura-clásica)
    - [Estructura de tupla](#estructura-de-tupla)
    - [Creación de una instancia de una estructura](#creación-de-una-instancia-de-una-estructura)
    - [Conversión de un literal de cadena en un tipo String](#conversión-de-un-literal-de-cadena-en-un-tipo-string)
    - [Comprobación de conocimientos en definición de colecciones de datos mediante tuplas y estructuras](#comprobación-de-conocimientos-en-definición-de-colecciones-de-datos-mediante-tuplas-y-estructuras)

---

# Qué es Rust 1

Una introducción rápida a las características del lenguaje Rust y comparación de Rust con otros lenguajes de programación.

## Introducción

con el lenguaje de programación de Rust, puede compilador software de sistemas confiable y eficaz. Los desarrolladores usan Rust para software de red como servidores web, servidores de correo y exploradores web. Rust también está presente en compiladores e intérpretes, contenedores de virtualización y software, bases de datos, sistemas operativos y criptografía. También puede usar Rust para compilar juego, programas de línea de comandos, programas de ensamblado web y aplicaciones diseñadas para dispositivos incrustados.

Rust es una alternativa segura a los lenguajes de software de sistemas existentes como C y C++. Al igual que C y C++, Rust no tiene un recolector de elementos no utilizados o del entorno de ejecución de gran tamaño, lo que lo diferencia de casi todos los demás lenguajes modernos. Sin embargo, a diferencia de C y C++, Rust garantiza la seguridad de la memoria; Rust evita muchos de los errores relacionados con el uso incorrecto de la memoria que podría encontrar en C y C++.

Rust logra un equilibrio único entre expresiones de rendimiento, seguridad e implementación. Sea cual sea su experiencia en programación, descubrirá que Rust tiene algo que ofrecerle.

### ¿Cuál es la mejor manera de aprender Rust?

Rust requiere algunos conocimientos teóricos para poder escribir código de Rust por su cuenta de forma productiva. Siga este curso u otros recursos de aprendizaje de Rust antes de comenzar su desarrollo. Una vez que cuente con unas nociones básicas del lenguaje, practique la escritura de código lo máximo posible. Escriba al realizar los ejercicios de este módulo y de todos los demás de esta ruta de aprendizaje.

Comenzaremos aprendiendo los pequeños conceptos fundamentales del lenguaje. A continuación, con basaremos en los ejercicios prácticos y la exploración. Creará unos cuantos proyectos a lo largo del proceso y, al final de la lección, tendrá una idea sódica del lenguaje.

### Objetivos de aprendizaje en Qué es Rust

En este módulo, aprenderá lo siguiente:

- Algunas de las características únicas de Rust.
- Por qué los desarrolladores eligen Rust en lugar de otros lenguajes de programación.
- Componentes y herramientas básicas para crear, compilar y ejecutar programas de Rust.
- Uso del área de juegos de Rust.

---

## ¿Qué es Rust?

Rust es un lenguaje de programación de sistemas de código abierto que puede usar para desarrollar software seguro y eficaz. Con Rust, puede administrar la memoria y controlar otros detalles de bajo nivel. Pero también puede aprovechar los conceptos de alto nivel, como la iteración y las interfaces. Estas características distinguen a Rust de los lenguajes de bajo nivel, como C y C++.

Rust también ofrece las siguientes ventajas que lo hacen ideal para una amplia gama de aplicaciones.

- **Seguridad de tipos:** el compilador garantiza que no se aplicará ninguna operación a una variable de un tipo incorrecto.
- **Seguridad de memoria:** los punteros de Rust (conocidos como *referencia*) siempre hacen referencia a la memoria válida.
- **Sin carrera de datos:** el comprobador de préstamos de Rust garantiza la seguridad para subprocesos asegurándose de que varias partes de un programa no puedan mutar el mismo valor al mismo tiempo.
- **Abstracciones de costo cero:** Rust permite el uso de conceptos generales, como la iteración, las interfaces y la programación funcional, con un costo de rendimiento mínimo o nulo. Las abstracciones funcionan tan bien como si hubiera escrito el código subyacente a mano.
- **Tiempo de ejecución mínimo:** Rust tiene un tiempo de ejecución mínimo y opcional. Con el fin de administrar la memoria de forma eficaz, el lenguaje tampoco tiene ningún recolector de elementos no utilizados. De este modo, Rust se parece más a lenguajes como C y C++.
- **Destinos sin sistema operativo:** Rust puede tener como destino la programación insertada y sin sistema operativo, lo que lo hace adecuado para escribir un kernel de sistema operativo o controladores de dispositivo.

Según la [encuesta de desarrolladores de Stack Overflow de 2022](https://survey.stackoverflow.co/2022/#overview?azure-portal=true), Rust ha sido el lenguaje más apreciado durante varios años seguidos. Los desarrolladores disfrutarán de la programación con Rust. Muchos tipos de organizaciones, desde las startup hasta las grandes empresas, usan Rust en sus casos de uso exclusivos. Desde la creación de herramientas, hasta la escritura de aplicaciones web, el trabajo en servidores o la creación de sistemas insertados, las posibilidades son infinitas.

---

## Características únicas de Rust

Para averiguar si un lenguaje de programación es adecuado para un proyecto, debe conocer las características y las limitaciones. A continuación, puede comparar los lenguajes posibles y elegir el que mejor funcione.

En esta unidad, revisaremos algunas de las características y limitaciones de Rust:

- El sistema de módulo de Rust: módulo, crates y rutas.
- Bibliotecas estándar de Rust y crates de terceros.
- La herramienta Cargo de Rust y el administrador de dependencias
- Cuándo se debe usar Rust.

### Administración de código con el sistema de módulo de Rust

Rust ofrece un colección de características que le ayudarán a administrar y organizar el código. Estas características se conocen como **sistema de módulos de Rust**. El sistema se compone de *crates, módulos y rutas*, as+i como herramientas para trabajar con esos elementos.

- **Crates**: Un crate de Rust es una unidad de compilación. Es el fragmento de código más pequeño que puede ejecutar el compilador de Rust. El código de un crate se compila en conjunto para crear un archivo ejecutable binario o una biblioteca. En Rust, solo los crates se compilan como unidades reutilizables. Un crate contiene una jerarquía de módulos de Rust con un módulo implícito de nivel superior sin nombre.
- **Módulos:** Los módulos de Rust ayudan a organizar el programa, ya que permiten administrar el ámbito de los elementos de código individuales dentro de un crate. Los elementos de código relacionado o los elementos que se usan juntos se pueden agrupar en el mismo módulo. Las definiciones de código recursivas pueden abarcar otros módulos.
- **Rutas:** En Rust, puede usar rutas para dar nombre a los elementos del código. Por ejemplo, una ruta puede ser una definición de datos, como un vector, una función de código o incluso un módulo. La característica de módulo también le ayuda a controlar la privacidad de las rutas. Puede especificar las partes del código a las que se puede acceder públicamente frente a las partes privadas. Esta características le permite ocultar los detalles de implementación.

### Uso de crates y bibliotecas de Rust

La biblioteca estándar de Rust, `std`, contiene código reutilizable para las definiciones y operaciones fundamentales de los programas de Rust. Esta biblioteca tiene definiciones para tipos de datos principales como, por ejemplo, `String` y `Vec<T>`, operaciones para primitivas de Rust, código para funciones de macro usadas con frecuencia, compatibilidad con acciones de entrada y salida, y muchas otras áreas de funcionalidad.

Hay decenas de miles de bibliotecas y crates de terceros disponibles para su uso en los programas Rust; para acceder a la mayoría de ellas se puede usar el repositorio de crates terceros de Rust, [crates.io](https://crates.io). Más adelante veremos cómo acceder a estos create desde nuestro proyecto, pero por ahora estas son algunas de las crates que se usan en los ejercicios de programación:

- [std](https://doc.rust-lang.org/std): Biblioteca estándar de Rust. En los ejercicios de Rust, verá que aparecen los siguientes módulos:
  - std::collections: definiciones de tipos de colección, como `HashMap`.
  - std::env: Funciones para trabajar con el entorno.
  - std::fmt: Funcionalidad para controlar el formato de salida.
  - std::fs: Funciones para trabajar con el sistema de archivos.
  - std::io: Definiciones y funcionalidad para trabajar con entradas y salidas.
  - std::path: Definiciones y funciones que permiten trabajar con datos de ruta de acceso del sistema de archivo.
- [structopt]((https://crates.io/crates/structopt)): crate de terceros para analizar argumentos de línea de comandos fácilmente.
- [chrono](https://crates.io/crates/chrono): crate de terceros para controlar los datos de fecha y hora.
- [regex](https://crates.io/crates/regex): crate de terceros para trabajar con expresiones regulares.
- [serde](https://crates.io/crates/serde): crate de terceros con operaciones de serialización y deserialización de estructuras de datos de Rust.

De manera predeterminada, la biblioteca `std` está disponible para todos los crates de Rust. Para acceder al código reutilizable en un crate o biblioteca, implementamos la palabra clave `use`. Con la palabra clave `use`, el código del crate o biblioteca se "incluye en el ámbito" para que pueda acceder a las definiciones y funciones en el programa. Se accede a la biblioteca estándar en instrucciones `use` con la ruta `std`, como en `use std::fmt`. Se accede a otros crates o bibliotecas con su nombre, como `use regex::Regex`.

### Creación y administración de proyectos con Cargo

Aunque se puede usar el compilador de Rust (`rustc`) directamente para crear crates, en la mayoría de los proyectos se usa la herramienta de compilación de Rust y un administrador de dependencias llamando **Cargo**.

Cargo hace gran cantidad de cosas, entre las que se incluyen las siguientes:

- Crear nuevas plantillas de proyecto con el comando `cargo new`.
- Compilar un proyecto con el comando `cargo build`.
- Compilar y ejecutar un proyecto con el comando `cargo run`.
- Probar un proyecto con el comando `cargo test`.
- Comprobar los tipos de proyectos con el comando `cargo check`.
- Compilar la documentación de un proyecto con el comando `cargo doc`.
- Publique una biblioteca para `crates.io` con el comando `cargo publish`.
- Para agregar crates dependientes a un proyecto, agregue el nombre del crate al archivo Cargo.toml.

### Cuándo se debe usar Rust

El lenguaje Rust tiene numerosos puntos a favor que se deben tener en cuenta al elegir el mejor lenguaje para un proyecto:

- Rust permite controlar el rendimiento y el consumo de recursos de los programas y bibliotecas escritos en el lenguaje en el mismo nivel que C y C++, al tiempo que mantiene la memoria protegida. Este nivel de control elimina todas las clases de errores comunes.
- Rust tiene características de abstracción muy completas que permiten a los desarrolladores codificar muchos de los aspectos invariables de sus programas en código, que luego el compilador se encarga de comprobar en lugar de depender de convenciones o documentaciones. Esta característica suele dar lugar a la impresión de que "si se compila, funciona".
- Rust tiene herramientas integradas para compilar, probar, documentar y compartir código, así como un ecosistema completo de herramientas y bibliotecas de terceros. Gracias a estas herramientas, algunas tareas que son difíciles en algunos lenguajes, como crear dependencias, resultan fáciles de llevar a cabo y productivas en Rust.

### Comprobación de conocimientos

Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione **Comprobar las respuestas**.

1. **¿Cuál es una ventaja atractiva de trabajar con Rust?**

- [x] Rust es un lenguaje con seguridad de tipos, con seguridad de memoria y sin carrera de datos.
- [ ] Rust está optimizado para el desarrollo sin sistema operativo, como los sistemas operativos.
- [ ] Rust tiene un recolector de elementos no utilizados sólido y permite administrar la memoria de forma eficaz.

2. **¿Cómo se ejecuta el código de Rust?**

- [ ] Los programas de Rust no se compilan en un script, sino en un archivo ejecutable.
- [ ] El código de Rust debe estar incluido en un archivo de código fuente de C++.
- [x] A través de la compilación seguida de la ejecución directa.

3. **¿Cuál sería un ejemplo de algo que no se puede hacer con Cargo?**

- [ ] Compilar un proyecto de Rust existente.
- [x] Actualizar la versión del compilador de Rust instalada.
- [ ] Publicar una biblioteca en Crates.io.

---

## El área de juegos de Rust

En ocasiones, solo quiere probar un poco del código de Rust o comprobar la sintaxis de una definición en una biblioteca de Rust. También podría estar buscando una manera de compartir rápidamente código con otros. El lenguaje Rust permite estas tareas en el área de juegos de Rust.

El área de juegos es un IDE para el desarrollo en Rust, que está disponible en Internet en `https://play.rust-lang.org/`. Cualquier puede accede al área dde juegos. Puede escribir el código y, luego, compilado y ejecutarlo en el mismo entorno. En la captura de pantalla siguiente se muestra el entorno del área de juegos. En el extremo derecho de la barra de herramientas, el menú **CONFIG** tiene opciones para establecer las preferencias del entorno.

![area-de-juegos](image.png)

En el área de juegos, puede acceder a los métodos y funciones de la biblioteca estándar de Rust, `std`. Los 100 crates principales más descargados de la biblioteca `crates.io` también están disponibles junto con sus dependencias.

### Herramientas y características

El sitio de prueba de Rust tiene varias herramientas y características de desarrollo integradas:

- Código de formato: la herramienta **Rustfmt** da formato al código para seguir los estilos oficiales de Rust. La herramienta ajusta el código y aplica la sangría y el espaciado recomendados entre los elementos y operadores.
- Probar código: la herramienta **Clippy** comprueba si hay errores en el código. La herramienta ejecuta pruebas de *lint* en el código para buscar errores y áreas de mejora.
- Guardar código: a medida que se trabaja en el sitio de prueba de Rust, el código se guarda automáticamente en el almacenamiento local del explorador. Esta característica facilita la recuperación del trabajo más reciente, en especial si cierra la ventana del explorador.
- Compartir código: la característica **Compartir** crea un gist de GitHub que se puede compartir para el código del sitio de prueba. Puede guardar esta dirección URL para acceder al código más adelante. La dirección URL carga el gist del código específico en el área de juegos.

> **Nota:**
> El almacenamiento local de un explorador es un recurso singleton. Si tiene más de una ventana del explorador abierta en el área de juegos de Rust y está trabajando con código diferente en cada ventana, solo el código guardado más recientemente entre todas las ventanas se conservará en el almacenamiento local.

### Opciones de compilación

Hay varias opciones para compilar y ejecutar código en el sitio de prueba de Rust:

- **Run** (Ejecutar): Compile y ejecute el código y vea la salida. La opción **Run** es lo mismo que usar el comando `cargo run`.
- **Build** (Compilar): Compile el código, pero no lo ejecute. La opción **Build** es lo mismo que usar el comando `cargo build`.
- **Test** (Probar): Compile el código y ejecute todas las pruebas en el código. La opción **Test** es lo mismo que usar el comando `cargo test`.

### Límites de protección

Hay algunas limitaciones en el área de juegos para evitar que el sitio se utilice de forma malintencionada. Las restricciones ayudan a garantizar que el sitio sigue estando disponible para todos los usuarios.

- **Red:** Al compilar o ejecutar código en el área juegos, no hay disponible una conexión de red.
- **Memoria:** El área de juegos limita la memoria disponible para compilar código y ejecutar un programa compilado.
- **Tiempo de ejecución:** El área de juegos establece una cantidad máxima de tiempo para compilar código y ejecutar un programa compilado.
- **Disco:** La cantidad de espacio disponible en disco para compilar código y ejecutar un programa compilado es limitada.

Puede obtener más información sobre las características del área de juegos de Rust en el [sitio web de Rust](https://play.rust-lang.org/help).

1. ¿Qué herramienta del área de juegos de Rust se puede usar para encontrar errores en el código?

- [ ] rustfmt
- [x] Clippy
- [ ] Depuración

2. ¿Cuándo no está disponible una conexión de red en el sitio de pruebas de Rust?

- [ ] Cuando se edita código
- [ ] Cuando se ejecuta un programa
- [x] Al compilar código o ejecutar un programa.

## Ejercicio

El área de juegos de Rust es práctica para probar pequeños programas, probar nuevos crates y bibliotecas, y compartir código con otros usuarios. En este ejercicio, crearemos un pequeño programa en el área de juegos para familiarizamos con el entorno.

### Escritura de código en el área de juego

Para empezar, vamos a escribir código para un programa básico.

1. Conéctese al [Área de juegos de Rust](https://play.rust-lang.org/).
2. Escriba el código siguiente en el editor del área de juegos:

~~~rust
fn main(){println!(Welcome to Rust!);}
~~~

3. Seleccione **Herramientas** > **Rustfmt** para dar formato al código:

![alt text](image-1.png)

La herramienta ajusta el código para seguir los estilos oficiales de Rust:

![alt text](image-2.png)

4. Seleccione **Tools** (Herramientas)>**Clippy** para comprobar si hay errores en el código. Los resultados se muestran en el editor:

![alt text](image-3.png)

5. Para corregir el código de ejemplo, agregue comillas alrededor del texto "Welcome to Rust!":

![alt text](image-4.png)

### Compilación y ejecución de código en el sitio de prueba

Ahora se compilará el código y se ejecutará el programa.

  1. Para elegir cómo compilar y ejecutar el código en el sitio de prueba, abra el menú desplegable **Run** (Ejecución) en la parte superior de la interfaz de usuario:
  ![alt text](image-5.png)
  2. Seleccione **Run** para compilar y ejecutar el programa de ejemplo. La salida del programa se muestra en el editor:
  ![alt text](image-6.png)

### Guardado y uso compartido de código en el sitio de prueba

A medida que trabaje en el sitio de prueba, el código se guardará automáticamente en el almacenamiento del explorador. Si cierra la ventana del explorador, puede perder el código que ha escrito. Para que el código esté siempre disponible, puede crear un dirección URL compartible.

  1. Seleccione la característica **Share** (Compartir) en la barra de herramientas para crear un gist de GitHub para el código en el sitio de prueba:
  ![alt text](image-7.png)
  2. Seleccione el icono de papel junto al texto **Permalink to the playground** (Vinculo permanente al área de juegos) para obtener un gist que se pueda compartir para el código.

Ahora se puede guardar la dirección URL para acceder al código más tarde, o bien compartir la URL para que otros usuarios vean el código.

## Resumen

En este módulo, ha aprendido sobre los tipos de aplicaciones que puede compilar mediante el lenguaje de programación Rust. Por qué Rust es útil para los tipos de desarrollo de bajo nivel y alto nivel.

Ha revisado los comandos de Rust para trabajar con el código. Que el comando `rustc` se usa para escribir y compilar programas de Rust.

Ha descubierto la característica Cargo de Rust y ha aprendido sobre el sistema de módulo para la organización del código. Para crear, compilar y ejecutar un proyecto, se usa Cargo.

Hemos analizado el entorno del área de juego de Rust y hemos visto cómo escribir, compilar, probar y ejecutar código.

### Prueba de las recetas en la guía paso a paso de Rust

La Rust Cookbook contiene *recetas* para el código que sigue las prácticas recomendadas para tareas de programación comunes. Al seguir las recetas, puede obtener información sobre cómo trabajar con los crates usados con frecuencia en Rust. Las recetas abarcan una amplia variedad de temas, incluido el procesamiento de texto y números, el trabajo con bases de datos, la aplicación de algoritmo comunes y la depuración de programas. Puede leer Rust Cookbook en el [sitio web de Rust](https://rust-lang-nursery.github.io/rust-cookbook/).

### Referencias

- [Biblioteca estándar std de Rust](https://doc.rust-lang.org/std/)
- [Repositorio de bibliotecas crates.io de Rust](https://crates.io/)
- [Recetas de Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)
- [Ayuda del área de juegos de Rust](https://play.rust-lang.org/help)
- [Encuesta para desarrolladores de Stack Overflow de 2022](https://survey.stackoverflow.co/2022/#overview?azure-portal=true)

---

# Configuración el entorno de desarrollo Rust

Obtenga información sobre cómo configurar el entorno de desarrollo de Rust, escribir un programa y usar el sistema de compilación Cargo.

## Introducción sobre el entorno de desarrollo

En esta unidad, se describirán los pasos necesarios para instalar y configurar el entorno de desarrollo para que pueda empezar a programar en Rust.

Para programar en Rust, instalará el editor de Visual Studio Code, las herramientas de compilación de Microsoft C++ para Visual Studio Code y los archivos del lenguaje Rust.

Una vez configurado el entorno, probaremos un programa básica "Hola mundo" para confirmar que está listo para empezar.

### Objetivos de aprendizaje

En este módulo, ha aprendido a hacer lo siguiente:

- Configuración del entorno de desarrollo para usar Rust
- Compile y ejecute un programa básico "Hola mundo"
- Usar Cargo, la herramienta de compilación de Rust y el administrador de dependencias.

## Instalar Visual Studio Code

Como aspirante a desarrollador de Rust, deberá escribir código fuente de Rust en archivos de texto.

Un archivo de código fuente de Rust es un archivo de texto con una extensión .rs en el que se escribe todo el código de Rust. Después de guardar el código en el archivo de texto, utilice el compilador de Rust (`rustc`) o Cargo para compilar el código en un programa.

### Herramientas para escribir código de Rust

Normalmente, la sintaxis de Rust se escribe en un archivo de texto y se guarda en la unidad de disco duro local. Se puede escribir código mediante un editor de archivos de texto simple, como el Bloc de notas de Windows. El Bloc de notas edita texto ASCII, un formato de archivo de texto estándar simple.

> **Sugerencia:**
> Evite el uso de editores de texto que incluyan opciones de formato, como negrita, subrayado o cursiva, o de cualquier otro programa que tenga características de procesamiento de texto. Por ejemplo, no escriba código en Microsoft Word ni en TextEdit en macOS. Estos programas tienen instrucciones de formato adicionales que el compilador de Rust no entenderá.

Aunque puede usar un editor de texto, normalmente se suele usar una herramienta que se adapte mejor a los desafíos asociados a la escritura de código. Hay gran cantidad de opciones, pero muchos desarrolladores confían en Visual Studio Code para este propósito. Visual Studio Code es gratis y está disponible en Windows, macOS y Linux. Tiene muchas características que permiten navegar fácilmente por el código, independientemente del lenguaje de programación con el que quiera trabajar.

Elija uno de los siguientes procedimientos de instalación, en función del sistema operativo.

### Comprobación de conocimientos en configuración el entorno de desarrollo Rust

Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione Comprobar respuestas.

1. ¿Qué dos maneras se pueden usar para compilar un programa de Rust?

- [ ] compile y rustc
- [ ] build y rustc
- [x] Cargo y rustc

## Instalación de las herramientas de compilación de Visual C++

Rust requiere las herramientas de compilación de Microsoft C++ para Visual Studio 2013 o versiones posteriores. Estas herramientas de compilación deben instalarse antes de instalar Rust.

Si no tiene instaladas las herramientas de compilación, siga estos pasos:

  1. Vaya a la [página de descarga de Microsoft Visual Studio](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
  2. Seleccione **Descargar Build Tools**.
  3. Una vez finalizada la descarga, ejecute el archivo del instalador. Se abre la ventana del instalador de Visual Studio.
  4. En el cuadro de diálogo emergente, seleccione Sí. En el siguiente cuadro de diálogo emergente, seleccione **Continuar**.
  5. En la ventana del instalador, en **Móviles y de escritorio**, active la casilla de la opción **Desarrollo de escritorio con C++**.
  6. En el panel **Detalles de la instalación**, asegúrese de que estén seleccionadas las siguientes opciones:
  
  > **Nota:**
  > Asegúrese de seleccionar el SDK correcto para el sistema operativo.
  
  ![alt text](image-8.png)
  7. Seleccione **Instalar**

Una vez completa la instalación, puede continuar con la instalación de Rust.

## Instalación de Rust

La manera recomendada de instalar Rust es usar `rustup`, el instalador de la cadena de herramientas de Rust. Vaya al sitio web [rustup.rs](https://rustup.rs/) para encontrar las instrucciones adecuadas correspondientes a su sistema operativo.

![alt text](image-9.png)

En Linux o macOS, copie el comando curl seleccionando el icono del portapapeles. Después, abra el terminal del equipo o el símbolo del sistema para pegar el comando y siga las instrucciones que aparecen en pantalla. En Windows, siga las instrucciones del instalador.

> **Importante:**
> Rust requiere las herramientas de compilación de Microsoft C++ para Visual Studio 2013 o versiones posteriores. Las herramientas de compilación deben instalarse antes de instalar Rust. Si tiene que instalar las herramientas de compilación, vea los pasos de la unidad anterior.

Rust tiene un proceso de lanzamiento rápido de seis semanas y admite un gran número de plataformas, por lo que hay muchas compilaciones de Rust disponibles en cualquier momento. Si ha instalado `rustup` en el pasado, puede actualizar a la versión estable más reciente de Rust ejecutando el comando `rustup update`.

### Comprobación de la instalación de Rust

Una vez completada la instalación de Rust, debe tener disponibles los comandos `rustc` y `cargo`.

> **Nota:**
> Los comandos siguientes funcionan en todas las plataformas.

Ejecute el siguiente comando en el terminal o símbolo del sistema:

~~~bash
rustc --version
~~~

Debería ver una salida como la de este ejemplo:

~~~bash
rustc 1.76.0 (07dca489a 2024-02-04)
~~~

Luego, ejecute el siguiente comando:

~~~bash
cargo --version
~~~

Debería ver una salida similar a esta:

~~~bash
cargo 1.50.0 (f04e7fab7 2021-02-04)
~~~

Ambas líneas de salida contienen la siguiente información sobre las versiones estables más recientes de Rust y Cargo que están disponibles:

- Número de versión
- Hash de confirmación
- Fecha de confirmación

Esta información aparece en el formato siguiente:

`<executable-name> <three-part-release-number> (<9-character-hash-code> <4-digit-year>-<2-digit-month>-<2-digit-day>)`

Si ve este tipo de salida, significa que ambas instalaciones se han realizado correctamente. Si no ve esta información, compruebe la variable de entorno `PATH`. Asegúrese de que incluye una carpeta que contiene los archivos ejecutables `rustc.exe` y `cargo.exe`.

### Comprobación de conocimientos en Instalación de Rust

Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione Comprobar respuestas.

1. ¿Cuál es el comando recomendado que se debe usar para instalar Rust?

- [ ] rinstall
- [x] rustup
- [ ] rupdate

2. ¿Con qué frecuencia se actualizan las bibliotecas de Rust?

- [ ] Cada seis meses
- [ ] Cada tres meses
- [x] Cada seis semanas

## Ejercicio: Hola mundo

Con Rust instalado, está listo para empezar a crear código. Vamos a escribir un programa que imprima "Hola mundo" en la consola.

### Creación de un nuevo directorio para organizar el código

Empiece por crear un directorio para almacenar todo el código en esta ruta de aprendizaje (`rust-learning-path`) y luego cree un nuevo subdirectorio para mantener el código fuente de este ejercicio.

Para Linux y macOS, ejecute los siguientes comandos:

~~~bash
mkdir ~/rust-learning-path
cd ~/rust-learning-path
mkdir hello-world
cd hello-world
~~~

Para PowerShell en Windows, ejecute los siguientes comandos:

~~~powershell
mkdir "%USERPROFILE%\rust-learning-path"
cd /d "%USERPROFILE%\rust-learning-path"
mkdir hello-world
cd hello-world
~~~

### Escritura del primer programa con Rust

A continuación, cree un nuevo archivo denominado main.rs y use el editor para escribir el código siguiente:

~~~rust
fn main() {
  println!("Hello, world!");
}
~~~

### Compilación y ejecución del programa

El código fuente está listo. Ahora es el momento de compilar el programa en un archivo ejecutable. Vuelva a la ventana de terminal y escriba los siguientes comandos para compilar y ejecutar el archivo.

En Windows, ejecute los siguientes comandos:

~~~powershell
rustc main.rs
.\main.exe
~~~

Si está en Linux o macOS, ejecute los siguientes comandos:

~~~bash
rustc main.rs
./main
~~~

Debería ver la siguiente salida:

~~~bash
Hello, world!
~~~

### Creación de un proyecto con Cargo

Ahora se usará Cargo para escribir y ejecutar el mismo programa.

> **Nota:**
> Los comandos de las secciones siguientes funcionan en todas las plataformas

Para empezar, se use Cargo para crear un proyecto.

Asegúrese de que el terminal está en el directorio `rust-learning-path` y ejecute el siguiente comando:

~~~bash
cargo new hello-cargo
~~~

Este comando generado un nuevo directorio denominado *hello-cargo* con un subdirectorio *src* y agrega dos archivos:

> hello-cargo/
>      Cargo.toml
>      src/
>          main.rs

- El archivo *Cargo.toml* es el archivo de manifiesto de Rust. Es donde se conservan los metadatos para el proyecto, así como las dependencias.
- El archivo *main.rs* en el subdirectorio *src* es donde escribirá el código de la aplicación.

Observe que el comando `cargo new` generó un proyecto "Hola mundo" reutilizable automáticamente.

### Compilación y ejecución del programa con Cargo

Para ejecutar el programa reutilizable, pasaremos al nuevo directorio hello-cargo y, a continuación, usaremos el comando `cargo run`.

Ejecute los siguientes comandos en el terminal:

~~~bash
cd hello-cargo
cargo run
~~~

Debería aparecer la salida siguiente en el terminal:

~~~bash
Compiling hello-cargo v0.1.0 (/tmp/.OFUp/hello-cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.59s
      Running `target/debug/hello-cargo`

Hello, world!
~~~

Cargo ha compilado y ejecutado el archivo ejecutable.

Enhorabuena, ha escrito el primer programa de Rust y ha aprendido a inicializar el primer proyecto de Rust con Cargo.

## Resumen 1

En este módulo, ha instalado Rust y las herramientas de Visual Studio Code que necesita para escribir y ejecutar programas de Rust. Una vez configurado el entorno, ha creado un programa básico "Hola mundo" y lo ha modificado para que use Cargo para iniciar una nueva plantilla de proyecto.

En el siguiente módulo de esta ruta de aprendizaje, recorrerá algunos conceptos de programación comunes en Rust, como funciones, tipos de datos y flujo de control.

# Creación del primer programa de Rust

Obtenga información sobre los conceptos de Rust, como variables, tipos de datos y funciones.

**Objetivos de aprendizaje**

En este módulo, aprenderá a:

- Explorar los conceptos básicos del lenguaje Rust, incluidas las funciones, los tipos de datos y las variables
- Comprender los tipos de Rust básicos para texto, números, valores booleanos y datos compuestos
- Crear, compilar y ejecutar un programa básico de Rust
- Descubrir cómo imprimir la salida del programa

## Introducción sobre creación del primer programa

En este módulo, obtendrá información sobre conceptos comunes en los lenguajes de programación y descubrirá cómo se implementan en Rust. Los conceptos no son exclusivos de Rust, pero proporcionan una base para todos los programas con Rust. Al obtener información sobre estos conceptos, podrás entender cómo admitir el desarrollo en cualquier lenguaje de programación.

### Área de juegos de Rust

El [área de juegos de Rust](https://play.rust-lang.org/) es una interfaz de explorador para el compilador de Rust. Puede usar el área de juegos para experimentar con la escritura de código de Rust antes de instalar el lenguaje localmente o cuando no tenga el compilador disponible. A lo largo de este curso, se proporcionarán vínculos del área de juegos al código y a los ejercicios de ejemplo. Puede interactuar con el código, aunque no tenga la cadena de herramientas de Rust disponible en ese momento.

Todo el código que se ejecuta en el Área de juegos de Rust también se puede compilar y ejecutar en el entorno de desarrollo local. No dude en interactuar con el compilador de Rust del equipo. Puede obtener más información sobre el área de juegos de Rust en el módulo [¿Qué es Rust?](#¿qué-es-rust?).

### Objetivos de aprendizaje en creación del primer programa de Rust

En este módulo, aprenderá a:

- Explore los conceptos básicos del lenguaje Rust, incluidas las funcionas, los tipos de datos y las variables.
- Comprenda los tipos de Rust básicos para texto, números, valores booleanos y datos compuestos.
- Cree, compile y ejecute un programa básico de Rust.
- Descubra cómo imprimir la salida del programa.

## Descripción de la estructura básica de programas de Rust

En esta unidad, se  revisa cómo se estructura un programa simple de Rust.

### Funciones en Rust

Una función es un bloque de código que realiza una tarea específica. Separamos el código de nuestro programa en bloques basados en tareas. Esta separación hace que el código sea más fácil de entender y mantener. Después de definir una función para una tarea, podemos llamar a la función cuando sea necesario realizar esa tarea.

Cada programa de Rust debe tener una función llamada `main`, El código de la función `main` siempre es el primer código que se ejecuta en un programa con Rust. Podemos llamar a otras funciones desde la función `main` o desde otras funciones.

~~~rust
fn main() {
  println!("Hello, world!");
}
~~~

Para declarar una función en Rust, usamos la palabra clave `fn`. Después del nombre de la función, se le indica al compilador cuántos parámetros o *argumentos* espera la función como entrada. Los argumentos se enumeran entre paréntesis `()`. El *cuerpo de la función* es el código que realiza la tarea de una función y se define entre llaves `{}`. Un procedimiento recomendado consiste en aplicar formato al código para que la llave de apertura del cuerpo de la función aparezca justo después de la lista de argumentos entre paréntesis.

### Sangría del código

En el cuerpo de la función, la mayoría de las instrucciones de código terminan con un punto y coma `;`. Rust procesa estas instrucciones una tras otra, por orden. Cuando una instrucción de código no termina con un punto y coma, Rust sabe que la línea de código siguiente debe ejecutarse antes de que se pueda completar la instrucción inicial.

Para ayudar a ver las relaciones de ejecución en el código, usamos la sangría. Este formato muestra cómo se organiza el código y revela el flujo de pasos necesarios para completar la tarea de la función. A una instrucción de código inicial se le aplica una sangría de cuatro espacios desde el margen izquierdo. Cuando el código no termina con un punto y coma, a la siguiente línea de código que se va a ejecutar se le aplica una sangría de cuatro espacios más.

Veamos un ejemplo:

~~~rust
fn main() { // The function declaration is not indented

    // First step in function body
        // Substep: execute before First step can be complete

    // Second step in function body
        // Substep A: execute before Second step can be complete
        // Substep B: execute before Second step can be complete
            // Sub-substep 1: execute before Substep B can be complete

    // Third step in function body, and so on...
}
~~~

### Macro todo! macro

Cuando trabaje en los ejercicios de los módulos de Rust, observará que en el código de ejemplo se suele usar la macro `todo!`. En Rust, una macro es como una función y toma un numero variable de argumentos de entrada. La macro `todo!` se usa para identificar código sin terminar en el programa comportamiento que no está completo.

Este es un ejemplo de cómo se usa la macro `todo!` en los ejercicios:

~~~rust
fn main() {
  // Display the message "Hello, world!"
  todo!("Display the message by using the println!() macro");
}
~~~

Al compilar código en el que se usa la macro `todo!`, el compilador puede devolver un mensaje de alarma en el que espera encontrar la funcionalidad completada:

~~~cmd
Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/playground`
thread 'main' panicked at 'not yet implemented: Display the message by using the println!() macro', src/main.rs:3:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
~~~

### El println! macro

Nuestra función `main` realiza una tarea. Llama a la macro `println!` predefinida en Rust. La macro `println!` espera uno o varios argumentos de entrada, que se muestran en la pantalla o en la *salida estándar*. En nuestro ejemplo, pasamos un argumento de entrada a la macro, la cadena de texto "Hello, world!".

~~~rust
fn main() {
  // Our main function does one task: show a message
  // println! displays the input "Hello, world!" to the screen
  println!("Hello, world!");
}
~~~

### Sustitución de valores para argumentos {}

En las lecciones del módulo de Learn de Rust, a menudo llamamos a la macro `println!` con una lista de argumentos que incluye cadenas de texto con instancias de corchetes `{}` y otros valores. La macro `println!` reemplaza cada instancia de llaves `{}` dentro de una cadena de texto por el valor del argumento siguiente de la lista.

Veamos un ejemplo:

~~~rust
fn main() {
  // Call println! with three arguments: a string, a value, a value
  println!("The first letter of the English alphabet is {} and the last letter is {}", 'A', 'Z');
}
~~~

Llamamos a la macro `println!` con tres argumentos: una cadena, un valor y otro valor. La macro procesa los argumentos por orden. Cada instancia de llaves `{}` dentro de una cadena de texto se reemplaza por el valor del argumentos siguiente de la lista.

La salida es la siguiente:

~~~cmd
The first letter of the English alphabet is A and the last letter is Z
~~~

### Comprobación de conocimiento

Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cade pregunta y, después, seleccione **Comprobar las respuestas**.

1. ¿Cuántas funciones principales puede haber en un programa de Rust?
   - [ ] Un programa de Rust puede tener tantas funciones `main` como sea necesario.
   - [ ] Cualquier función de Rust puede tener una subfunción denominada `main`.
   - [x] Cada programa de Rust solo debe tener una función llamada `main`.
2. ¿Qué palabra clave de Rust se usa para declarar una función nueva?
   - [ ] `function`
   - [x] `fn`
   - [ ] `func`
3. ¿Cuál es la salida de esta llamada a la macro `println!`? `println!("{} is a number. {} is a word", 1, "Two");`
   - [x] 1 is a number. Two is a word
   - [ ] {} is a number. {} is a word.
   - [ ] {1} is a number. {"Two"} is a word.

## Creación y uso de variables en Rust

Los desarrolladores escriben para trabajar con datos. Los datos se recopilan, analizan, almacenan, procesan, comparten y notifican. Usamos *variables* para almacenar nuestros datos en una referencia con nombre, que podemos consultar más adelante en el código.

### Variable

En Rust, una variable se declara con la palabra clave `let`.  Cada variable tiene un nombre único. Cuando se declara una variable, se puede enlazar a un valor o el valor se puede enlazar más adelante en el programa. El código siguiente declara una variable denominada `a_number`.

~~~rust
let a_number;
~~~

La variable `a_number` aún no está enlazado a un valor. podemos modificar esta instrucción para enlazar un valor a la variable:

~~~rust
let a_number = 10;
~~~

> **Nota**
>
> **Palabras clave** Al igual que con otros lenguajes de programación, determinadas *palabra clave*, como `fn` y `let`, están reservadas para que las use Rust únicamente. Las palabras clave no se pueden usar como nombres de funciones o variables.

Veamos otro ejemplo. El código siguiente declara dos variables. La primera variable se declara, pero no se enlaza a un valor. La segunda variable se declara y enlaza a un valor. Más adelante en el programa, la primera variable se enlaza a un valor. El código llama a la macro `println!` para mostrar los valores de la variable.

~~~rust
// Declara una variable
let a_number;

// Declara una segunda variable y enlaza el valor.
let a_word = "Ten";

// Enlaza un valor para el primer variable.
a_number = 10;

println!("The number is {}.", a_number);
println!("The word is {}.", a_word);
~~~

Nuestro ejemplo imprime la siguiente salida:

~~~cmd
The number is 10.
The word is Ten.
~~~

Si llamamos a la macro `println!` e intentamos mostrar el valor de la variable `a_number` antes de enlazarla, el compilador devuelve un error.

Puede comprobar este mensaje de error en el [área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7619f3a57e95b1c161d410641f9e88fb%3Fazure-portal%3Dtrue). Seleccione el botón **Ejecutar** para ejecutar el código.

### Inmutable frente a mutable

En Rust, los enlaces de variables son inmutables de manera predeterminada. Cuando una variable es inmutable, después de enlazar un valor a un nombre, no se puede cambiar ese valor.

Por ejemplo, si intentamos cambiar el valor de la variable `a_number` del ejemplo anterior, recibiremos un mensaje de error del compilar.

~~~rust
// Cambia el valor de una inmutable variable
a_number = 15;
~~~

Puede probar este cambio y ver el mensaje de erro en el [área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7619f3a57e95b1c161d410641f9e88fb%3Fazure-portal%3Dtrue).

Para mutar un valor, debemos usar en primer lugar la palabra clave `mut` para convertir en mutable el enlace de una variable.

~~~rust
// el palabra clave `mut` se permite la variable ser cambiado
let mut a_number = 10;
println!("El número es {}.", a_number);

// Cambio el valor de una variable mutable
a_number = 15;
println!("Ahora el número es {}.", a_number);
~~~

Este ejemplo imprime la salida siguiente:

~~~cmd
El número es 10.
Ahora el número es 15.
~~~

Este código se compila sin errores porque la variable `a_number` ahora puede mutarse.

### Propiedad reemplazada de variables

Puede declarar una variable nueva que use el nombre de una existente. La declaración nueva crea un enlace. En Rust, esta operación se denomina "propiedad reemplazada" porque la nueva variable prevalece sobre la anterior. La antigua variable sigue existiendo, pero ya no se puede hacer referencia a ella en este ámbito.

En el código siguiente se muestra cómo usar la propiedad reemplazada. Declaramos una variable denominada `shadow_num` mientras se reemplaza la propiedad del enlace de la variable anterior.

~~~rust
// Declare first variable binding with name "shadow_num"
let shadow_num = 5;

// Declare second variable binding, shadows existing variable "shadow_num"
let shadow_num = shadow_num + 5;

// Declare third variable binding, shadows second binding of variable "shadow_num"
let shadow_num = shadow_num * 2;

println!("The number is {}.", shadow_num);
~~~

¿Puede adivinar la salida? visite el [Área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=ece8cff9611f109439db2645d75e98d6%3Fazure-portal%3Dtrue) para ejecutar este ejemplo.

### Comprobación de conocimientos Creación y uso de variables en Rust

Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione **Comprobar las respuestas.**

1. ¿Qué instrucción de Rust declara una variable y enlaza un valor?
   - [x] `let continents = 7;`
   - [ ] `continents = 7;`
   - [ ] `let continents;`

2. ¿Qué palabra clave de Rust se usa para hacer que el valor de una variable sea modificable?
   - [ ] `mutable`
   - [ ] `inmutable`
   - [ ] `mut`

## Exploración de tipos de datos para números, texto y valores true/false

Rust es un lenguaje con establecimiento de tipos en modo estático. El compilador debe conocer el tipo de datos exacto  de todas las variables del código para que el programa se compile y ejecute. Normalmente, el compilador puede inferir el tipo de datos de una variable en función del valor enlazado. No siempre es necesario indicar de forma explícita el tipo en el código. Cuando son posibles muchos tipos, debe informar al compilador del tipo específico mediante *anotaciones de tipo*.

En el ejemplo siguiente, se le indica al compilador que cree la variable `number` como un entero de 32 bits. Especificamos el tipo de datos `u32` después del nombre de la variable. Observe que después del nombre de la variable se usa el carácter de dos puntos `:`.

~~~rust
let number: u32 = 14;
println!("The number is {}.", number);
~~~

Si ponemos el valor de la variable entre comillas dobles, el compilador interpreta el valor como texto en lugar de como un número. El tipo de datos deducido del valor no coincide con el tipo de datos `u32` especificado para la variable, por lo que el compilador emite un error:

~~~rust
let number: u32 = "14";
~~~

Error del compilador:

~~~cmd
Compiling playground v0.0.1 (/playground)
error[E0308]: mismatched types
 --> src/main.rs:2:23
  |
2 |     let number: u32 = "14";
  |                 ---   ^^^^ expected `u32`, found `&str`
  |                 |
  |                 expected due to this

error: aborting due to previous error
~~~

Se puede interactuar con el código anterior en esta [área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d1635823974d9456858611266a32ffa6%3Fazure-portal%3Dtrue).

### Tipos de datos integrados

Rust incluye algunos tipos de datos primitivos integrados para expresar números, texto y veracidad. Algunos de estos tipos se conocen como *escalares*, porque representan un solo valor:

- Números enteros
- Números de punto flotante
- Valores booleanos
- Characters

Rust también ofrece tipos de datos más complejos para trabajar con series de datos, como valores de cadena y de tupla.

### Números: valores enteros y de puntos flotante

Los enteros en Rust se identifican por el tamaño en bits y la propiedad *signed*. Un entero con **signo** puede ser un número positivo o negativo. Un entero **sin signo** solo puede ser un número positivo.

| **Length**                       | **Signo** | **Sin signo** |
| -------------------------------- | --------- | ------------- |
| 8 bits                           | `i8`      | `u8`          |
| 16 bits                          | `i16`     | `u16`         |
| 32 bits                          | `i32`     | `u32`         |
| 64 bits                          | `i64`     | `u64`         |
| 128 bits                         | `i128`    | `u128`        |
| *dependiente de la arquitectura* | `isize`   | `usize`       |

Los tipos `isize` y `usize` dependen del tipo de equipo en el que se ejecuta el programa. El tipo de 64 bits se usa en una arquitectura de 64 bits y el tipo de 32 bits, en una arquitectura de 32 bits. Si no especifica el tipo para un entero, y el sistema no puede deducir el tipo, asigna el tipo `i32` (un entero de 32 bits con signo) de forma predeterminada.

Rust tiene dos tipos de datos de punto flotante para los valores decimales: `f32` (32 bits) y `f64` (64 bits). El tipo de punto flotante predeterminado es `f64`. En las CPU modernas, el tipo `f64` tiene aproximadamente la misma velocidad que el tipo `f32`, pero cuenta con una mayor precisión.

~~~rust
let number_64 = 4.0; // compiler infers the value to use the default type f64
let number_32: f32 = 5.0; // type f32 specified via annotation
~~~

Todos los tipos de números primitivos en Rust admiten operaciones matemáticas como suma, resta, multiplicación y división.

~~~rust
// Addition, subtraction, and multiplication
println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);
~~~

> **Nota:**
>  
> Cuando llamamos a la macro `println`, agregamos el sufijo de tipos de datos a cada número literal para informar a Rust sobre el tipo de datos. La sintaxis `1u32` indica al compilador que el valor es el número 1 y que interprete el valor como un entero de 32 bits sin signo.
>
> Si no se proporcionan anotaciones de tipo, Rust intenta deducir el tipo a partir del contexto. Cuando el contexto es ambiguo, asigna el tipo `i32` (un entero de 32 bits con signo) de forma predeterminada.

Se puede intentar ejecutar este ejemplo en el [área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d683842bd8cedd949ed3c56b27f6f0eb%3Fazure-portal%3Dtrue).

### Valores booleanos: true o false

El tipo booleano en Rust se usa para almacenar la veracidad. El tipo `bool` tiene dos valores posibles: `true` o `false`. Los valores booleanos se usan de forma generalizada en expresiones condicionales. Si una instrucción `bool` o un valor es true, realice esta acción; de los contrario (la instrucción o el valor es false), realice una acción distinta. Una comprobación de comparación suele devolver un valor booleano.

En el ejemplo siguiente, usamos el operador mayor que `>` para probar dos valores. El operador devuelve un valor booleano que muestra el resultado de la prueba.

~~~rust
// Declare variable to store result of "greater than" test, Is 1 > 4? -- false
let is_bigger = 1 > 4;
println!("Is 1 > 4? {}", is_bigger);
~~~

### Texto: caracteres y cadenas

Rust admite valores de texto con dos tipos de cadena básico y un tipo de carácter. Un carácter es un elemento único, mientras que una cadena es una serie de caracteres. Todos los tipos de texto son representaciones UTF-8 válida.

### Characters

El tipo `char` es le más primitivo de los tipos de texto. El valor se especifica poniendo el elemento entre comillas simples:

~~~rust
let uppercase_s = 'S';
let lowercase_f = 'f';
let smiley_face = '😀';
~~~

> **Nota**
>
> Algunos lenguajes tratan sus tipos `char` como enteros de 8 bits sin signo, que es el equivalente del tipo `u8` de Rust. El tipo `char` de Rust contiene puntos de código Unicode, pero no usa la codificación UTF-8. `char` en Rust es un entero de 21 bits que se ha agregado para ampliar a 32 bits. `char` contiene directamente el valor de punto de código sin formato. Puede obtener más información sobre el tipo `char` de Rust en la [**documentación de Rust**](https://doc.rust-lang.org/std/primitive.char.html).

### Cadenas

El tipo `str`, también conocido como *segmento de cadena*, es una vista de los datos de la cadena. La mayoría de las veces, se hace referencia a estos tipos usando la sintaxis del estilo de referencia que precede al tipo con el símbolo de y comercial `&str`. Trataremos las referencias en los siguientes módulos. Por ahora, puede imaginarse `&str` como un puntero a datos de cadena inmutables. Los literales de cadena son todos de tipo `&str`.

Aunque los literales de cadena son convenientes para usarlos un ejemplos de introducción de Rust, no son adecuados para todas las situaciones en las que podríamos querer usar texto. No todas las cadenas pueden conocerse en tiempo de compilación. Un ejemplo se da cuando un usuario interactúa con un programa en tiempo de ejecución y envía texto mediante un terminal.

En estos escenarios. Rust tiene un segundo tipo de cadena denominado `String`. Este tipo se asigna en el montón. Cuando se usa el tipo `String`, no es necesario conocer la longitud de la cadena (número de caracteres) antes de compilar el código.

> **Nota:**
>
> Si está familiarizado con un lenguaje de recolección de elementos no utilizados, es posible que se pregunte por qué Rust tiene dos tipos de cadena. 1 Las cadenas son tipos de datos extremadamente complejos. La mayoría de los lenguajes usan sus recolectores de elementos no utilizados para atenuar esta complejidad. Rust, como lenguaje de un sistema, expone parte de la complejidad inherente de las cadenas. La complejidad adicional conlleva una cantidad de control muy especifica sobre cómo se usa la memoria en el programa.
>
>1 _En realidad, Rust tiene más de dos tipos de cadena. En este módulo, solo se describen los tipos `String` y `&str`. Puede obtener más información sobre los tipos de cadena que se ofrecen en la [**documentación de Rust**](https://doc.rust-lang.org/book/ch08-02-strings.html).

No va a obtener una idea completa de la diferencia entre `String` y `&str` hasta que conozca la propiedad y el sistema de préstamos de Rust. Hasta entonces, puede pensar en los datos de tipo `String` como datos de texto que pueden cambiar a medida que se ejecuta el programa. Las referencias `&str` son vistas inmutables en los datos de texto que no cambian a medida que se ejecuta el programa.

### Ejemplo de texto

En el ejemplo siguiente se muestra cómo usar los tipos de datos `char` y `&str` en Rust.

- Se declaran dos variables de caracteres con la sintaxis de anotación `: char`. Los valores se especifican usando comillas simples.
- Se declara una tercera variable de caracteres y se enlaza a una sola imagen. Para esta variable, se permite que el compilador deduzca el tipo de datos.
- Se declaran dos variables de cadena y se enlazan a sus valores respectivos. Las cadenas se ponen entre comillas dobles.
- Una de las variables de cadena se declara con la sintaxis de anotación `: &str` para especificar el tipo de datos. El tipo de datos de la otra variable se deja sin especificar. El compilador deducirá el tipo de datos de esta variable en función del contexto.

Observe que la variable `string_1` incluye un espacio vacío al final de la serie de caracteres.

~~~rust
// Specify the data type "char"
let character_1: char = 'S';
let character_2: char = 'f';
   
// Compiler interprets a single item in quotations as the "char" data type
let smiley_face = '😃';

// Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
let string_1 = "miley ";

// Specify the data type "str" with the reference syntax "&str"
let string_2: &str = "ace";

println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
~~~

Esta es la salida de nuestro ejemplo:

~~~cmd
😃 is a Smiley face.
~~~

¿Qué ocurre si no se especifica el símbolo de y comercial `&` antes de `str` en este ejemplo? Para averiguarlo, intente ejecutar este ejemplo en el [Área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=dafd8dfd9159b2c6db51fdf02d5cb096%3Fazure-portal%3Dtrue).

### Comprobación de conocimientos: Exploración de tipos de datos para números, texto y valores true/false

Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione **Comprobar las respuestas**.

1. ¿Qué afirmación describe cómo se definen los valores numéricos enteros en Rust?

- [ ] En Rust, los enteros se identifican principalmente por su tamaño de bits, por ejemplo 8 bits, 16 bits, y así sucesivamente.
- [x] En Rust, los enteros se identifican por su tamaño en bits y la propiedad con signo.
- [ ] En Rust, un entero positivo o negativo se puede definir como un valor sin signo (`u`) o con signo (`i`).

2. ¿Qué afirmación describe correctamente cómo se admiten en Rust los valores de caracteres de texto?

- [ ] Rust tiene un tipo de datos que se puede usar tanto para caracteres únicos como para cadenas de texto de varios caracteres.
- [ ] Un carácter (char) solo puede ser una sola letra alfa, como "A" o "z". Una cadena puede ser una serie de caracteres cualquiera: letras, números, imágenes, entre otros.
- [x] En Rust, todos los tipos de texto son representaciones UTF-8 válidas.

## Definición de colecciones de datos mediante tuplas y estructuras

En esta unidad, se explorarán dos tipos de datos que son útiles para trabajar con colecciones de datos o datos compuestos: tuplas y estructuras.

### Tuplas

Una tupla es una agrupación de valores de distintos tipos recopilados en un valor compuesto. Los valores individuales de una tupla se denominan *elementos*. Los valores se especifican como una lista separada por comas entre paréntesis `(<value>, <value>, ...)`.

Una tupla tiene una longitud fija, que es igual a su número de elementos. Una vez declarada una tupla, no puede aumentar ni reducir su tamaño. No se pueden agregar ni quitar elementos. El tipo de datos de una tupla se define mediante la secuencia de los tipos de datos de los elementos.

### Definición de una tupla

Este es un ejemplo de una tupla con tres elementos:

~~~rust
// Tuple of length 3
let tuple_e = ('E', 5i32, true);
~~~

En la tabla siguiente se muestra el índice, el valor y el tipo de datos de cada elemento de la tupla:

| **Elemento** | **Value** | **Tipo de datos** |
| ------------ | --------- | ----------------- |
| 0            | E         | `char`            |
| 1            | 5         | `i32`             |
| 2            | true      | `bool`            |

La firma de tipo de esta tupla se define mediante la secuencia de los tipos de los tres elementos: `(char, i32, bool)`.

### Acceso a elementos de una tupla

Se puede acceder a los elementos de una tupla por la posición del índice, a partir de cero. Este proceso se conoce como *indexación de tupla*. Para acceder a un elemento de una tupla, usamos la sintaxis `<tuple>.<index>`.

En el ejemplo siguiente se muestra cómo acceder a los elementos de la tupla usando la indexación:

~~~rust
// Declare a tuple of three elements
let tuple_e = ('E', 5i32, true);

// Use tuple indexing and show the values of the elements in the tuple
println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0,tuple_e.1,tuple_e.2);
~~~

En el ejemplo se muestra la salida siguiente:

~~~cmd
Is 'E' the 5th letter of the alphabet? true
~~~

Este ejemplo se puede explorar en el [Área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f0c1c833543c9f58af5e49efb77a9fdd%3Fazure-portal%3Dtrue).

Las tuplas resultan útiles cuando se quieren combinar tipos distintos en un único valor. Las funciones pueden utilizar tuplas para devolver varios valores porque las tuplas pueden contener cualquier número de valores.

### Estructuras

Una estructura es un tipo compuesto por otros tipos. Los elementos de una estructura se denominan *campos*. Al igual que las tuplas, los campos de una estructura pueden tener tipos de datos diferentes. Una ventaja importante del tipo de estructura es que puede asignar un nombre a cada campo, por lo que queda claro lo que significa el valor.

Para trabajar con estructuras en un programa con Rust, en primer lugar debe definir la estructura por nombre y especificar el tipo de datos de cada campo. Después, debe crear una instancia de la estructura con otro nombre. Al declarar la instancia, se proporcionan los valores específicos para los campos.

Rust admite tres tipos de estructura: clásicas, de tupla y de unidad. Estos tipos de estructura admiten diferentes maneras de agrupar y trabajar con los datos.

- **Las [estructuras de C](https://wikipedia.org/wiki/Struct_(C_programming_language)) clásicas** son las más utilizadas. Cada campo de la estructura tiene un nombre y un tipo de datos. Una vez definida una estructura clásica, se puede acceder a los campos de la estructura usando la sintaxis `<struct>.<field>`.
- **Las estructuras de tupla** son parecidas a las clásicas, pero sus campos no tienen nombres. A fin de acceder a los campos de una estructura de tupla, usamos la misma sintaxis que para indexar una tupla: `<tuple>.<index>`. Al igual que con las tuplas, los valores de índice de la estructura de tupla empiezan por cero.
- **Las estructuras de unidad** suelen usarse como marcadores. Obtendremos más información sobre por qué las estructuras pueden resultar útiles cuando descubramos la característica *rasgos* de Rust.

En el código siguiente se muestran definiciones de ejemplo para las tres variedades de tipos de estructura:

~~~rust
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;
~~~

### Definición de una estructura

Para definir una estructura, se escribe la palabra clave `struct` seguida de un nombre de la estructura. Elija un nombre para el tipo de estructura que describa la característica significativa de los datos agrupados. A diferencia de la convención de nomenclatura que hemos usado hasta ahora, el nombre de un tipo de estructura se escribe en mayúsculas.

Los tipos de estructura se definen a menudo fuera de la función `main` y de otras funciones en el programa con Rust. Por este motivo, al inicio de la definición de la estructura no se le aplica sangría desde el margen izquierdo. Solo se le aplica sangría a la parte interna de la definición para mostrar cómo se organizan los datos.

### Estructura clásica

Al igual que una función, el cuerpo de una estructura clásica se define entre llaves `{}`. A cada campo de la estructura clásica se le asigna un nombre único dentro de la estructura. El tipo de cada campo se especifica con la sintaxis `: <type>`. Los campos de la estructura clásica se especifican como una lista separada por comas `<field>, <field>, ....` Una definición de estructura clásica **no** termina con un punto y coma.

~~~rust
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }
~~~

Una ventaja de la definición de estructura clásica es que se puede acceder al valor de un campo de estructura por el nombre. Para acceder al valor de campo, usamos la sintaxis `<struct>.<field>`.

### Estructura de tupla

Al igual que una tupla, el cuerpo de una estructura de tupla se define entre paréntesis `()`. Los paréntesis van inmediatamente después del nombre de la estructura. No hay espacio entre el nombre de la estructura y el paréntesis de apertura.

A diferencia de una tupla, la definición de estructura de tupla incluye solo el tipo de datos de cada campo. Los tipos de datos de la estructura de tupla se especifican como una lista separada por comas `<type>, <type>, ...`.

~~~rust
// Tuple struct with data types only
struct Grades(char, char, char, char, f32);
~~~

### Creación de una instancia de una estructura

Después de definir un tipo de estructura, para usar la estructura se crea una instancia del tipo y se especifican valores para cada campo. Al establecer los valores de campo, no es necesario especificar los campos con el mismo orden con el que están definidos.

En el ejemplo siguiente se usan las definiciones que hemos creado para los tipos de estructura Student y Grades.

~~~rust
// Instantiate classic struct, specify fields in random order, or in specified order
let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };

// Instantiate tuple structs, pass values in same order as types defined
let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
         user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
         user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
~~~

### Conversión de un literal de cadena en un tipo String

Los datos de cadena que se almacenan dentro de otra estructura de datos, como una estructura o un vector, se deben convertir de una referencia literal de cadena (`&str`) a un tipo `String`. Para realizar la conversión, se usa el método `String::from(&str)` estándar. Observe cómo se usa este método en este ejemplo:

~~~rust
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }
...
let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };
~~~

Si no se convierte el tipo antes de asignar el valor, el compilador emite un error:

~~~cmd
error[E0308]: mismatched types
  --> src/main.rs:24:15
   |
24 |         name: "Dyson Tan",
   |               ^^^^^^^^^^^
   |               |
   |               expected struct `String`, found `&str`
   |               help: try using a conversion method: `"Dyson Tan".to_string()`

error: aborting due to previous error
~~~

El compilador sugiere que se puede usar la función `.to_string()` para realizar la conversión. En los ejemplos, se usa el método `String::from(&str)`.

Se puede interactuar con el código de ejemplo en esta [Área de juegos de Rust](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=1a09be796adc0020c4f868a92b4084f4%3Fazure-portal%3Dtrue).

### Comprobación de conocimientos en definición de colecciones de datos mediante tuplas y estructuras

Responda a las preguntas siguientes para ver lo que ha aprendido. Elija una respuesta para cada pregunta y, después, seleccione **Comprobar las respuestas**.

1. ¿Qué es tuple en Rust?

- [x] Una tupla es una colección de valores de diferentes tipos. El tipo de datos se basa en los tipos de datos de sus elementos y la longitud se fija en función del número de elementos.
- [ ] Una tupla es una colección de valores de diferentes tipos. El tipo de datos se basa en los tipos de datos de sus elementos. La longitud puede aumentar y reducirse a medida que se agregan o se quitan elementos.
- [ ] Una tupla es una colección de valores del mismo tipo de datos. Todos los elementos de la tupla deben tener el mismo tipo de datos. La longitud de la tupla se fija en función del número de sus elementos.

2. ¿Cuál es la principal diferencia entre una estructura clásica y una estructura de tupla en Rust?

- [ ] Todos los campos de una estructura clásica deben ser del mismo tipo de datos. Los campos de una estructura de tupla pueden ser tipos de datos diferentes.
- [ ] Se puede acceder a los valores de una estructura de tupla mediante la indexación. Solo se puede acceder a los valores de una estructura clásica por nombre de campo.
- [x] Cada campo de una estructura clásica tiene un nombre y un tipo de datos. Los campos de una estructura de tupla no tienen nombres.
