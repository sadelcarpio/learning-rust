use std::fmt;
use std::fmt::Formatter;

fn main() {
    println!("{} days", 31);
    // order of arguments on print
    println!("{0}, this is {1}. {1}, this is {0}", "Sergio", "Dora");
    // named parameters!
    println!("{subject} {verb} {object}", object="la tina", subject="Anita", verb="lava");
    // string formatting (like :.f and such in python)
    println!("Base 10: {}", 2048);
    println!("Base 2 (binary) {:b}", 2048);
    println!("Base 8 (octal) {:o}", 2048);
    println!("Base 16 (hexadecimal) {:x}", 2048);  // minuscule
    println!("Base 16 (hexadecimal) {:X}", 2048);  // caps
    // right justify :o
    println!("{number:>5}", number=1);
    // padding
    println!("{number:p>5}", number=1);
    // padding variable
    println!("{number:0>num_spaces$}", number=1, num_spaces=5);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // Formatting only works with types that implement fmt::Display
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct {:?} won't print", Structure(3));  // debug print

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f64 = 3.141592;
    println!("Pi's approximate value is {:.3}", pi);

    // Debug
    // All types can derive automatically from fmt::Debug but not from fmt::Display (needs to be
    // implemented)
    // not printable
    #[allow(dead_code)]
    struct UnPrintable(i32);
    // derive creates Debug implementation automatically
    #[derive(Debug)]
    struct Deep(Structure);

    println!("Now {:?} will print!", Deep(Structure(3)));

    #[derive(Debug)]
    #[allow(dead_code)]  // it's like a double decorator :o
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "George";
    let age = 25;
    let george = Person {name, age};
    println!("{:?}", george);

    // Implement display
    struct MyStructure(i32);
    impl fmt::Display for MyStructure {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            // Write the first element (.0) into output stream `f`.
            // Similar syntax to println!
            write!(f, "{}", self.0)
        }
    }
}
