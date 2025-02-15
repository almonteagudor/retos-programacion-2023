// autor: @SergioRibera
//
// Este breve curso estara separado en secciones dentro
// de este mismo archivo, cada seccion cubre un apartado
// basico con explicacion acerca de la sintaxis de Rust
//
//  * - Haz un "Hola, mundo!"
//  * - Crea variables de tipo String, numéricas (enteras y decimales)
//  *   y Booleanas (o cualquier tipo de dato primitivo).
//  * - Crea una constante.
//  * - Usa un if, else if y else.
//  * - Crea estructuras como un array, lista, tupla, set y diccionario.
//  * - Usa un for, foreach y un while.
//  * - Crea diferentes funciones (con/sin parámetros y con/sin retorno).
//  * - Crea una clase.
//  * - Muestra el control de excepciones.
//  *
//  * Así, cualquier persona podrá consultar rápidamente diferentes ejemplos
//  * de sintaxis básica de muchos lenguajes.
//  *

//******************************************
//      Seccion 1: Hola mundo!
//******************************************
//
// partimos de la funcion `main` ya que es la
// funcion de entrada para cualquier programa
// de rust
fn main() {
    // la funcion `println` es como cualquier otra
    // funcion, recibe parametros y realiza una ejecucion
    // su unica diferencia es el `!` que encontramos
    // esto es porque es una macro
    // las macros son un tema aparte del cual hablaremos
    // en otro momento
    println!("Hola mundo!");
}

//******************************************
//      Seccion 2: Variables y tipos de datos
//******************************************
#[allow(unused)] // Esta linea de aca permite tener funciones declaradas sin usar
                 // Solo esta aqui para que no llore el linter o compilador de rust XD
fn variables_typos_de_datos() {
    // declarar variables en rust es sencillo y hay dos maneras
    //
    // Este tipo de variables pertenecen a un bloque de codigo
    // esto es importante ya que si tratamos de usarlas fuera del bloque de codigo
    // en donde fueron creadas, tendremos problemas
    let a = 21;
    // en rust se usa la palabra reservada `let` para decir que crearemos una variable
    // seguido del nombre de la variable (NOTA: la nomencratura en rust es camell_case para
    // variables, funciones y todo en general)
    // y por ultimo asignamos un valor
    // Como te diste cuenta no hace falta decirle el tipo de dato al que pertenece la variable
    // esto debido a que el compilador es capaz de inferir el tipo en el momento de la compilacion
    // aunque nosotros tambien podemos decirle el tipo de datos que queremos usar, para esto lo
    // hacemos con una sintaxis muy similar a TypeScript
    let a: u32 = 21;
    // especificamos el tipo de dato luego del nombre y separando por `:`
    //
    // Sencillo verdad?
    // Ahora veremos los tipos de datos que tiene Rust

    // Este es un caracter, es un tipo de dato de 8 bits que puede ser un caracter ASCII o
    // un caracter UTF-8, como los emojis o caracteres mas raros
    let c: char = 'S';

    let s: &str = "Sergio Ribera"; // Este es el "String nativo" por asi decirlo, y aunque en
                                   // realidad es distinto, no profundizaremos mucho
                                   // puedes encontrar mas informacion aqui: https://rustlanges.github.io/rust-book-es/ch04-03-slices.html
    let s: String = String::from("Sergio Ribera"); // Este es mas un objeto de texto que maneja su
                                                   // contenido en memoria, suele tener la
                                                   // capacidad de crecer

    // Estos son los enteros con signo (Pueden ser negativos o positivos)
    // varian en tamaño dependiendo lo que necesitemos
    let i: i8 = 1; // 8 bit
    let i: i16 = -12; // 16 bit
    let i: i32 = 123; // 32 bit
    let i: i64 = -1234; // 64 bit
    let i: i128 = 12345; // 128 bit
    let i: isize = 12345; // toma el tamaño de la arquiectura del procesador

    // Estos son los enteros sin signo (Solo pueden ser positivos)
    // varian en tamaño dependiendo lo que necesitemos
    let u: u8 = 1; // 8 bit
    let u: u16 = 12; // 16 bit
    let u: u32 = 123; // 32 bit
    let u: u64 = 1234; // 64 bit
    let u: u128 = 12345; // 128 bit
    let u: usize = 12345; // toma el tamaño de la arquiectura del procesador

    // Estos son los flotantes y a diferencia de los enteros solo tenemos dos tipos
    // Y ambos pueden ser negativos o positivos
    let f: f32 = -1.123;
    let f: f64 = 2.234;

    // Estos son los booleanos y como en el resto de lenguajes solo pueden ser true o false
    let b: bool = true | false;

    // Un tip, hay ocaciones en las que queremos declarar una variable pero aun no usarlas
    // para este caso se coloca un `_` al inicio del nombre de la variable
    let _u = 254;

    // Otro detalle que podemos hacer es definir un tipo de dato sin declararlo de la anterior
    // manera
    // la sintaxis sigue la siguiente estructura
    // Tarch
    // por ejemplo:
    let _a = 21u8;
    let _a = 21f32;

    //
    //
    //  NOTA: como pudiste notar declare multiples variables con los mismos nombres, esto en rust
    //  es posible y bastante optimo, lo que pasa es que en cada `reemplazo` rust elimina el
    //  anterior dato y lo reemplza por el nuevo, esto se conoce como shadowing
    //  Y puedes encontrar mas informacion aqui: https://rustlanges.github.io/rust-book-es/ch03-01-variables-and-mutability.html#shadowing
    //
    //
}

//******************************************
//      Seccion 3: Constantes
//******************************************

// Las constantes son un tipo de datos que nunca se modifica y por ese motivo suelen ir dentro del
// binario, cargandose en el stack de inicio
// para declarar una constante es muy sencillo y similar a las variables que ya vimos
// la diferencia esta en que no puede inferir el tipo de dato y si o si tenemos que especificar
// reemplazamos el `let` por un `const` y tenemos una constante

const _MI_CONSTANTE: f32 = 21.256; // Fijate que el nombre ahora esta todo en uppercase
                                   // esto es debido a que las constantes se las suele nombrar
                                   // de esa manera para diferenciarlas

#[allow(unused)]
fn variables_constantes() {
    // las constantes tambien pueden ser declaradas dentro de bloques de codigo
    // En el bloque de esta funcion por ejemplo podemos crear una constante
    const OTRA_CONSTANTE: bool = true;
}

//******************************************
//      Seccion 4: Condicionales if/else if/else
//******************************************

#[allow(unused)]
fn condicionales() {
    // Algo bonito de la sintaxis de Rust es que no tiene parentesis en los consroles de flujo :D
    // Por el resto sigue la misma sintaxis que en otros lenguajes
    let a = 5;

    if a < 10 {
        println!("a es menor que 10");
    } else if a > 10 {
        println!("a es mayor que 10");
    } else {
        println!("a es igual a 10");
    }

    // Llegados a este punto es importante concoer tambien los operadores logicos que tiene rust
    // Y aunque basicamente los tiene todos, no esta demas
    //
    // == != > < >= <= && ||

    // continuando con los if/else
    // en Rust los if pueden devolver un valor
    // Debido a esto se usan como ternarios ya que debido a la filosofia de legibilidad en Rust la
    // sintaxis clasica de un ternario no ha sido admitida :(
    // pero tenemos esto
    let a = if a < 10 { "menor" } else { "mayor" };
    println!("a ahora vale: {a}"); // NOTA: println puede imprimir variables y aunque esto puede
                                   // parecer complicado mas adelante, no lo es tanto
                                   // para mas informacion visita (no esta en español y pido perdon,
                                   // pero es muy util): https://doc.rust-lang.org/rust-by-example/hello/print.html

    // para el control de flujo Rust no solo cuenta con el clasico if/else
    // Sino que tiene algo mas avanzado llamado `match` su uso es similar a los switch case, pero
    // mas bonito :3
    let a = 7;
    match a {
        1 => {
            println!("a vale 1");
        } // Comparamos valores puntuales
        5 => {
            println!("a vale 5");
        } // Comparamos valores puntuales
        x if x > 5 => {
            println!("a es mayor a 5");
        } // almacenamos el valor en una variable `x`
        // para luego compararla mas detalladamente
        x if x <= 5 => {
            println!("a es menor o igual a 5");
        } // almacenamos el valor en una variable `x`
        // para luego compararla mas detalladamente
        _ => {
            println!("a vale {a}")
        } // para todos los casos que no hicieron match anteriormente
    }

    // al igual que con el if, el match igual puede retornar datos
    let a = match a {
        1 => "a vale 1",                // Comparamos valores puntuales
        5 => "a vale 5",                // Comparamos valores puntuales
        x if x > 5 => "a es mayor a 5", // almacenamos el valor en una variable `x`
        // para luego compararla mas detalladamente
        x if x <= 5 => "a es menor o igual a 5", // almacenamos el valor en una variable `x`
        // para luego compararla mas detalladamente
        _ => "No se cuanto vale a :(", // para todos los casos que no hicieron match anteriormente
    };
    println!("a vale {a}");
}

//******************************************
//      Seccion 5: Arreglos, Listas, Tupla, Set y Diccionario
//******************************************

#[allow(unused)]
fn arreglos() {
    //  * - Crea estructuras como un array, lista, tupla, set y diccionario.
    //
    //******************************************
    //      Arreglos
    //******************************************
    //Rust tiene los arreglos que conocemos en otros lenguajes
    // usando la misma sintaxis para declararlo
    let a = [1, 2, 3, 4]; // esto nos da un tipo de array [i32; 4]
                          //                                ^   ^
                          //                              tipo  longitud
                          // En caso de querer especificar el tipo de dato, tambien podemos hacerlo
    let a: [u8; 3] = [232, 128, 255];
    //      ^   ^
    //    tipo  longitud

    // tambien podemos rellevar un arreglo de datos vacios
    let a = [0; 100]; // nos da el tipo de dato [i32; 100]

    //******************************************
    //      Listas
    //******************************************
    // al igual que con los string, tenemos un objeto que se encarga de administrar de mejor manera
    // los items del arreglo
    let a: Vec<i32> = Vec::new();

    // Este tiene una manera de crearlo muy facil usando una macro
    let a = vec![1, 2, 3];

    // Si no usaramos la macro tendriamos que hacer esto
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    //******************************************
    //      Tupla
    //******************************************
    // Las tuplas son una manera sencilla de agrupar datos
    // Su sintaxis es similar a esto (T, T1, T2, T3, ...)
    let (_a, _b, _c) = (1, "Hola", 3.4);

    // La ventaja con respecto a los arreglos o Vec<T>, es que pueden agrupar distintos tipos de
    // datos en una sola variable
    // Otra diferencia es su manera de acceder a los datos
    // ya que tenemos que hacerlo mediante indices, asi:
    let t = (1, "Hola", 3.4);
    let _s = t.1;
    let _f = t.2;


    // No nos olvidemos de los tipo de datos vacio
    // O void en otros lenguajes
    // en rust es una tupla vacia :)
    let void = ();

    //******************************************
    //      Set
    //******************************************
    // El set de datos suele ser una estructura similar al Vec<T> con la diferencia que los datos
    // no se repiten
    // en Rust existe un tipo de dato pero pertenece a una libreria parte de rust que tenemos que
    // importar, la libreria `std::collections`
    // puedes encontrar mas informacion al respecto aqui: https://doc.rust-lang.org/std/collections/index.html
    use std::collections::HashSet;

    // para este tipo de estructuras lastimosamente no contamos con alguna macro que nos haga facil
    // la vida por lo que solo podemos crear la variable e ir insertando datos
    let mut s = HashSet::new(); // esto nos da un dato de tipo `HashSet<&str>`

    // Add some books.
    s.insert("A Dance With Dragons");
    s.insert("To Kill a Mockingbird");
    s.insert("The Odyssey");
    s.insert("The Great Gatsby");

    //NOTA: puedes encontrar mas informacion acerca del HashSet aqui: https://doc.rust-lang.org/std/collections/struct.HashSet.html

    //******************************************
    //      Diccionario
    //******************************************
    // El Diccionario de datos es una estructura basada en Llave y Valor, de tal manera que los
    // datos no se repiten
    // en Rust existe un tipo de dato pero pertenece a una libreria parte de rust que tenemos que
    // importar, la libreria `std::collections` nuevamente
    // puedes encontrar mas informacion al respecto aqui: https://doc.rust-lang.org/std/collections/index.html
    use std::collections::HashMap;

    let mut a = HashMap::new(); // esto nos da un tipo HashMap<&str, f32>
    a.insert("Mercury", 0.4);
    a.insert("Venus", 0.7);
    a.insert("Earth", 1.0);
    a.insert("Mars", 1.5);
}

//******************************************
//      Seccion 6: Usa un for, foreach y un while.
//******************************************

#[allow(unused)]
fn ciclos() {
    //******************************************
    //      For && Foreach
    //******************************************
    // La sintaxis de los ciclos en Rust son similares a python
    //
    // Para entenderlos los leemos de la siguiente manera
    // por cada i en el rango de 0 a 10
    for i in 0..=10 {
        println!("{i}");
    }

    // Rust funciona a base de Iterator<T, Item = S>
    // por ejemplo, si tenemos un Vec<T> el iterator seria Iterator<Vec<T>, Item = T>
    // Esto es importante saber porque permite que rust tenga multiples maneras de iterar en ciclos

    // Aqui tenemos algunos iterables "nativos"
    let a = 0..5; // esto es un rango entre 0 y 4
    let a = 0..=5; // En este rango indicamos que sea de 0 hasta 5
    let a = ..5; // inicia en el minimo valor del tipo hasta el 5
    let a = 2..; // inicia en 2 y termina en el maximo del tipo
                 // puedes encontrar mas informacion acerca de esto aqui: https://doc.rust-lang.org/reference/expressions/range-expr.html

    // Tambien podemos iterar los Arreglos
    let a = vec![1, 2, 3, 4, 5];

    //
    // for i in a { // <- se movio aqui
    //     println!("{i}");
    // }
    //
    // println!("{a:?}");
    //              ^
    //              panic here
    //
    // El problema con este codigo es que luego quires ocupar `a` no podras porque se movio en el
    // for, entonces la manera de iterarlo seria de la siguiente manera
    //
    // Esto es debido al tema de Borrowing y Ownership que tiene Rust, para mas detalles
    // revisa la documentacion aqui:
    //
    for i in &a {
        // Los for se pueden `romper` dada ciertas circuntancias
        if i == &5 {
            break;
        }
        // Y tambien se puede saltar el resto de codigo en ciertas circuntancias
        if i > &8 {
            continue;
        }
        // Si a es 8 entonces se omite esta parte del codigo ya que el `continue` le dice al bucle
        // que salte a la siguiente iteracion
        println!("{i}");
    }
    // Si funciona :D
    println!("{a:?}");

    // El tema de los rangos es super util porque podemos tomar una parte de un arreglo, similar
    // a python
    let _a = &a[2..5];

    //******************************************
    //      While
    //******************************************

    // Los whiles tienen una sintaxis similar como en todos los lenguajes solo que sin parentesis
    let a = 2;
    while a < 5 {
        println!("Estoy en un while");
    }

    while a < 10 {
        // Los while se pueden `romper` dada ciertas circuntancias
        if a == 5 {
            break;
        }
        // Y tambien se puede saltar el resto de codigo en ciertas circuntancias
        if a > 8 {
            continue;
        }
        // Si a es 8 entonces se omite esta parte del codigo ya que el `continue` le dice al bucle
        // que salte a la siguiente iteracion
        println!("{a}");
    }

    //******************************************
    //      Loop
    //******************************************
    //
    // En Rust existe otro tipo de ciclo llamado loop, y es un bucle infinito que solo puede parar
    // de manera manual

    let mut i = 0;

    loop {
        println!("{i}");
        i += 1;

        // Al igual que los anteriores bucles, se puede parar el ciclo y saltar la iteracion
        // Y tambien se puede saltar el resto de codigo en ciertas circuntancias
        if a > 8 {
            continue;
        }
        if i == 5 {
            break;
        }
    }

    // Otra cosa espectacular de los loops es que estos pueden devolver un valor en el break
    // Por ejemplo
    let mut i = 0;
    let a = loop {
        i += 1;

        if i == 10 {
            break i;
        }
    };

    println!("{a}");

    // Otra cosa hermosa es que se pueden nombrar por lo que si tenemos varios bucles anidados
    // podemos detener uno u otro, ejemplo
    let mut i = 0;
    'first: loop {
        let mut x = 0;
        let y = 'second: loop {
            if x == 15 {
                break 'second x; // incluso devolver datos al romper :O
            }
        };

        if i == 10 {
            break 'first;
        }
    }
}

//******************************************
//      Seccion 7: funciones (con/sin parámetros y con/sin retorno)
//******************************************
// cabe recalcar que tanto en parametros como en valores de retorno se pueden usar todos los tipos
// de datos que tiene rust, incluso las tuplas

fn _sin_parametros() {
    println!("Esta es una funcion sin parametros");
}
// La declaracion de los parametros de la funcion se hacen dentro del parentesis y sigue la misma
// sintaxis de la declaracion de variables separando los argumentos por `,`
fn _con_parametros(name: &str, age: i32, year: u64) {
    println!("Esta es una funcion con parametros");
    println!("name: {name}");
    println!("age: {age}");
    println!("year: {year}");
}
fn _sin_retorno() {
    println!("Esta es una funcion sin retorno :p");
}
// La sintaxis al devolver un dato sigue este patron
//
// fn name_function (param: T) -> U {}
//
// Una cosa que diferencia mucho a las funciones de rust es que para devolver un valor no hace
// falta tener un `return`, siempre el ultimo bloque de codigo va a ser lo que se va a devolver
// siempre y cuando no tenga `;`
fn _con_retorno_simple(name: &str) -> String {
    // No tiene `;` y es la ultima linea de la funcion
    format!("Hola {name}!").to_string()
}

fn _con_retorno_simple_2(name: &str, age: i32) -> String {
    // si recordamos la seccion de los condicionales, podiamos retornar valores
    // en esta ocacion al ser el ultimo bloque, se verifica que devuelva un valor y ese valor es lo
    // que se retorna a la funcion
    if age >= 18 {
        format!("Hola {name}, puedes pasar!").to_string()
    } else {
        format!("Hola {name}, No puedes pasar! :C").to_string()
    }
}

fn _con_retorno_simple_3(name: &str, age: i32) -> String {
    // lo mismo sucede con los loop o con cualquier otro bloque que retorne un dato
    let mut i = 0;
    loop {
        if i >= age {
            break format!("Hola {name}!").to_string();
        }
        i += 1;
    }
}

//******************************************
//      Seccion 8: estructuras/clases
//******************************************

// En rust no existen las `clases` como se conocen en otros lenguajes
// aqui tenemos estructuras y estas tienen implementaciones

// Podemos tener estructuras que no contengan datos, solo existen
struct EstructuraSinParametros;

// Tambien podemos tener estructuras con formato de tupla
// estas estructuras no pueden nombrar a sus parametros
struct EstructuraTupla(String, u32);
// Por defecto los parametros de las estructura son privados por lo que no podremos acceder a ellos
// desde otro modulo o fichero ageno en el que este declarada
// para hacerlos publicos tiene que llevar la palabra reservada `pub`
pub struct EstructuraTupla2(pub String, pub u32);
// ^
// Expone la estructura a otros modulos

// Y por ultimo tenemos las estructuras con datos nombrados
struct Estructura {
    pub name: String,
    age: i32,
}

#[allow(unused)]
fn structs() {
    // tampoco tenemos la palabra reservada `new` para instanciar estructuras
    // aqui se instancian dependiendo del tipo de estructura

    // Instanciando las diferentes estructuras

    let a = EstructuraSinParametros; // Y ya, no necesitamos nada mas
    let a = EstructuraTupla("Sergio".to_string(), 21);
    let a = Estructura {
        name: "Sergio".to_string(),
        age: 21,
    };

    // algo que tiene muy bueno rust, es que podemos desestructurar las estructuras en caso de que
    // lo necesitemos
    let Estructura { name, age } = &a; // el `&` no siempre es necesario, pero en este caso lo
                                       // necesitamos porque seguiremos usando `a`
    let Estructura {
        name: nombre,
        age: _,
    } = &a; // podemos tambien renombrar las variables
            // Y si no queremos usar una variable la renombramos a `_`
            //
    let Estructura { name, .. } = a; // Tambien podemos obviar el resto de parametros

    // Incluso podemos desestructurar las estructuras de Tupla
    // Y como en las tuplas los parametros no tienen nombre, no es necesario renombrar ya que al
    // desestructurar automaticamente nombramos
    let EstructuraTupla(name, age) = EstructuraTupla("Sergio".to_string(), 21);
    let EstructuraTupla(_name, _) = EstructuraTupla("Ribera".to_string(), 21); // en el caso de
                                                                               // querer ignorar
                                                                               // un valor hay que
                                                                               // renombrar a `_`

    // Algo que tambien se puede hacer es crear una structura con datos de otra estructura, similar
    // a js
    let base = Estructura {
        name: "Sergio".to_string(),
        age: 18,
    };
    let _me = Estructura {
        age: 21,
        ..base
    };
}

// Y si pero donde pongo mis funciones, ya para eso rust usa implementaciones
impl Estructura {
    // aqui metemos las funciones, constantes y lo que necesitemos
    const _A: &str = "Algo";

    // aqui tenemos la funcion new que recibe parametros los cuales usamos para construir la
    // instancia de la estructura
    //
    // Nota esta funcion es una funcion estatica debido a que no requiere una instancia
    // Veremos mas de esto mas abajo
    pub fn new(name: &str, age: i32) -> Estructura {
        // Nota: puedes ver que al igual que en js, si una variable tiene el mismo nombre que un
        // parametro en la estructura no hace falta hacer `variable: variable`
        Estructura { name: name.to_string(), age }
    }

    // Tambien podemos usar Self para referirnos a esta Estructura pero como tipo
    pub fn build(name: &str, age: i32) -> Self {
        // Nota: puedes ver que al igual que en js, si una variable tiene el mismo nombre que un
        // parametro en la estructura no hace falta hacer `variable: variable`
        Self::new(name, age + 10)
    }

    // Para obtener datos de la estructura necesitamos que el metodo sea parte de una instancia
    // para eso usamos `&self` para obtener datos y `&mut self` para modificar datos
    pub fn es_mayor(&self) -> bool {
        // usamos `self` como si fuera `this` en otros lenguajes
        // devolvemos la comparacion
        self.age >= 18
    }

    // Para modificar datos necesitamos una referencia mutable
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

fn _usando_funciones_estructuras() {
    // Llamando a una funcion estatica
    let _a = Estructura::new("Sergio", 11);
    let a = Estructura::build("Sergio", 11);

    // Accediendo a la constante interna
    let _c = Estructura::_A;

    // llamando a una funcion para obtener un dato
    let _es_mayor = a.es_mayor();

    // En el caso de que querramos modificar la instancia no se puede a menos que la instancia sea
    // mutable
    let _a = Estructura::new("Sergio", 21);
    // a.set_name("Sergio Ribera"); // Error => Cannot mutate inmutable instance of `a`

    // por lo que nuestra instancia tiene que ser mutable
    let mut a = Estructura::new("Sergio", 11);
    a.set_name("Sergio Ribera");
}

// Un tema mas avanzado es el de los traits, pero eso no corresponde a esta guia por lo que puedes
// ver mas detalles aqui: https://rustlanges.github.io/rust-book-es/ch10-02-traits.html


//******************************************
//      Seccion 9: Manejo de Exceptions
//******************************************
//
// Enlace a la documentacion: https://rustlanges.github.io/rust-book-es/ch09-02-recoverable-errors-with-result.html

#[allow(unused)]
fn exceptions() {
    // Existen los errores irrecuperables, es decir que no hay manera de controlarlo
    // Estos se conocen como panics
    // y para lanzarlos, usamos la macro panic!
    let a = 0;
    if a == 0 {
        panic!("a is 0");
    }

    // Un ejemplo de error irrecuperable puede ser este
    let v = vec![1, 2, 3];
    v[99]; // panic porque el arreglo no es tan grande


    // Los errores controlables se manejan atravez de los Result<T, E>
    // En donde T es la respuesta correcta
    // Y E es una implementacion de error
    //
    // La manera de usar un Result es mediante sus 2 variantes
    // Ok(T) y Err(E)
    //
    // Un ejemplo de como usar ambos casos
    // En estos casos tengo que especificar explicitamente los tipos de datos para T y E
    let a: Result<&str, &str> = Ok("Hola");
    let a: Result<&str, &str> = Err("Esto es un Error");

    // Para usar el contenido del Result, tenemos varias maneras
    let _ = a.unwrap(); // saca el dato del Ok y lanza un panic! en caso de que sea un Err
                        // Esta manera es algo peligrosa en runtime y nada controlada

    // Por eso tenemos otras maneras para verificar el contenido del dato
    if let Ok(_) = a { }
    // Tambien podemos obtener el error
    if let Err(_) = a { }

    // tambien podemos usar un match para observar ambos casos
    match a {
        Ok(v) => { println!("{v}"); },
        Err(e) => { println!("{e}"); },
    }

    // Y recientemente se agrego una manera mas de como acceder a estos datos mas "sencillamente"
    // Esto almacena en una variable el interior del Ok disponible para todo el contexto
    // y en caso de que sea error, nos devolvemos y no continuamos
    // Esto es debido a que Ok y Err son opciones de Result y Result es un `enum` por lo que
    // basicamente esto es desestructurar el enum
    let Ok(_) = a else { return; };

    // Una "alternativa" interesante al Result es el Option<T>
    // El Option tiene 2 variantes, el Some(T) y el None
    // el Some(T) contiene un valor
    // y el None no contiene nada, es similar al null en otros lenguajes
    let a = Some(21);
    let a: Option<&str> = None;

    // Obtenemos el valor
    if let Some(_) = a { }
    if let None = a { } // esto es posible pero algo tonto
    // Es mejor usar:
    if a.is_none() {} // Se ve menos feo y mas profesional

    // tambien podemos usar un match para observar ambos casos
    match a {
        Some(v) => { println!("{v}"); },
        None => { println!("No hay valor que mostrar"); },
    }

    let Some(_) = a else { return; };
}

fn _sin_unwrap() -> Result<String, ()> {
    // Solo en funciones que retornan un Result<T, E>
    // podemos usar esta sintaxis
    let a: Result<&str, ()> = Ok("Hola");

    let _value = a?; // notese el `?`

    let a: Result<&str, &str> = Err("algo");

    // let _err = a?; // Este caso se rompre porque como puedes ver el tipo Err no son del mismo tipo
    
    // Peeeero, podemos mapear un result
    // let _value = a.map(|x| x.to_string())?; // mapeamos en caso de ser Ok
    let _err = a.map_err(|_| ())?; // mapeamos en caso de ser Ok
    
    Ok("Todo Bien".to_string())
}
