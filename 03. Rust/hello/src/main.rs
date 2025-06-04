fn main() {
    println!("Hello, world!");
    // Variables
    let bunnies = 16;
    let rabbits: i32 = 24;
    let (gorrion,pez) = (8,14);
    // Por defecto, las variables las hace inmutables
    // por seguridad, concurrencia y rendimiento
    //bunnies = 2; // Error!
    let mut chickens = 32;
    // Constantes
    const WARP_FACTOR: f64 = 9.9;
    // const son globales y dan un mejor rendimiento a tu codigo


    // BLOCKS
    let x = 5;
    {
        let y = 99;
        println("{}, {}", x, y);
    } // Como no existe un garbage collector las variables fuera del scope se borran ni bien acaba el bloque
    // println("{}, {}", x, y); // Error

    let a = 4;
    {
        // No se esta mutando el valor de la variable a
        // esta tecnica se llama shadowing y es para asignar otro valor a una variable de manera local
        let a = 32;
        println("{}", x); // print 32
    }
    println("{}",x); // print 5

    // Se puede usar la misma tecnica en el mismo scope
    let mut x = 5; // mutable
    let x = x; // es inmutable

    // el shadowing sirve tambien para cambiar el tipo de variable que es
    let meme = "More cowbell";
    let meme = make_image(meme);

    // En Rust no se puede declarar variables sin inicializarlas
    // Esto se hace para mejorar el el tiempo de compilacion
    let enigma: i32;
    //if true {
    //    enigma = 32; // Esto no garantiza nada, ya que se lee la condicional en tiempo de ejecucion y nada garantiza que valor tendra true
    //}                // Por ende no se sabe que valor tendra enigma
    if true {
        enigma = 32; // Este bloque, sin embargo si garantiza que se inicializara la variable
    } else {
        enigma = 7;
    }
    println!("{}",enigma);
    //println!("{}",enigma); // error
}
