
mod cake; // a mod can live here or in a separate file
// if defined in a separate file, we only need the mod here

fn main() {
    // VALUES ONLY LIVE AS LONG AS THE SCOPE THEY ARE IN
    let number = 10;
    {
        let number = 22;
        println!("{}", number);// number is 22 in this scope
    }
    println!("{}", number); // number is 10 again in this scope
    
    // RE_ASSIGNMENT WILL INVALIDATE THE PROVIOUS OWNER, UNLESS THE VALUE IS CLONED
    fn abc() -> String {
        "abc".to_string()
    }
    
    let letters = abc(); // ownership of abc is moved to letters
    let cloned_letters = abc().clone(); //we explicitly clone abc to cloned_letters
    
    println!("{}", letters); // these are now two distincs variables with different ownership
    println!("{}", cloned_letters);

    // LIKE C, RUST HAS A POINTER TYPE
    let pi = 3.14159265359;
    let funny_number = &pi;
    
    println!("{funny_number}"); // println automatically dereferences the pointer

    let planet = "Earth";
    let earth = &&&&planet;
 
    assert_eq!("EARTH", earth.to_uppercase());
    println!("{}", ****earth); // these are all the same variable,, .to_uppercase() dereferences the pointer

    // SLICES ARE POINTERS TO A PART OF A STRING
    let s = String::from("hello world");
 
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");    


    // CLOSURES ARE SIMILAR TO FUNCTIONS, BUT ARE NOT NAMED
    let double = |d| d * 2; // |d| is the closure, d is the argument, d * 2 is the return value
 
    // This is the outcome of calling the closure
    let var = double(10);
    println!("{}", var); // 20¨


    // THE COMPILER CAN DEDUCE THE TYPE OF A CLOSURE
    fn double_fn(num: u128) -> u128 {
        num * 2
    }
       
    let _stars:u128 = double_fn(10); // This is of type `u128`

    // SHADOWING IS THE PROCESS OF RE-DECLARING A VARIABLE
    // let VARIABLES ARE IMMUTABLE BY DEFAULT, BUT CAN BE SHADOWED
    // SHADOWING IS OFTEN PREFERRED OVER MUTABILITY
    let favorite = "orange";
    println!("{favorite}");
    
    let favorite = "cerulean";
    println!("{favorite}");
    
    let favorite = "yellow";
    println!("{favorite}");

    // PATTERN BINDING
    let [noun, verb, adjective] = ["arrays", "are", "homogeneous"];
    println!("{noun} {verb} {adjective}");


    // MUTABILITY
    fn question(s: &mut String) { // s is a mutable reference to a String
        s.pop();
        s.push('?');
    }
     
    let mut sentence = String::from("I am.");
    println!("{sentence}");

    question(&mut sentence);
     
    println!("{sentence}");

    let mut sentence = String::from("Take care, take care.");
    let immutable_reference = &mut sentence;
    
    // Swapping the order of these statements will cause our code to not compile.
    // this is because println!() create an immutable reference to sentence, which is
    // already borrowed by immutable_reference
    println!("{}", immutable_reference);
    println!("{}", sentence);
    // A struct can be mutable. Then its fields will be mutable.
    // If a struct is immutable, we can declare its fields as mutable, this is called interior mutability.
    // Interior mutability, however, needs to be used more carefully. 

    //CONSTANTS are compile-time constants. They are always immutable.
    // they need to use the const keyword, and their type must be annotated.
    const _ANIMAL: &str = "penguin";

    //CONSTANT FUNCTIONS
    // Constant functions are functions that can be evaluated at compile time.
    // they are guranteed to not have any side effects by the Rust compiler.
    const fn days_to_seconds(days: usize) -> usize {
        days * 60 * 60 * 24
    }
    
    // We can utilize constant functions within other constant declarations
    const WEEK_IN_SECONDS: usize = days_to_seconds(7);
    
    let february_in_seconds = days_to_seconds(28);
    
    println!("{WEEK_IN_SECONDS}");
    println!("{february_in_seconds}");

    // TRAITS
    // A trait is a collection of methods that are implemented for a type.
    // A type can implement multiple traits.
    // A trait is a contract that the consumer of a type must adhere to.
    // A trait is a bit like an interface in other languages.
    trait Greeting {
        fn greet(&self);
    }
    // Define a type named `Person`
    struct Person {
        name: String,
    }

    // Implement the `Greeting` trait for the `Person` type
    impl Greeting for Person {
        // Implement the `greet` method for `Person`
        fn greet(&self) {
            println!("Hello, my name is {}.", self.name);
        }
    }

    Person { name: "John".to_string() }.greet();

    // MODULES
    // rust has its own module system. 
    // pieces of code can be namespaced into modules using the mod keyword.
    // modules can be nested.
    // entities in a module can be accessed using the :: operator.
    // entities within a module are private by default, but can be made public using the pub keyword.
    let favorite = cake::taster::is_favorite("Coconut"); // I personally prefer this, for readibility
    println!("{}", favorite);

    // we could also use the use keyword to bring a module into scope.
    use cake::taster::is_favorite;
    let favorite = is_favorite("Coconut");
    println!("{}", favorite);
    // rust also allows us to define more restricive pubs. 
    // pub(crate) makes an entity public to the current crate.
    // pub(super) makes an entity public to the current module.
    // pub(in path) makes an entity public to the current module and any child modules.
    // to navigate our modules, we can also use ::super and ::self, and ::crate.
    // ::super refers to the parent module.
    // ::self refers to the current module.
    // ::crate refers to the root module of the current crate.
    // just like in python, we can use the as keyword to rename a module or a type.

    // MACROS 
    // macros are similar to functions, but they can be used to define patterns.
    // macros are defined using the macro_rules! macro.
    // macros are expanded before the compiler interprets the code.
    // macros are often used to define DSLs.
    // macros can also be used to define attributes.
    #[derive(Debug)]
    struct Chapter(String);
    
    let number = 10;
 
    if number <= 5 {
        todo!("we will handle this outcome soon.")
    } 
    else if number > 5 {
        unimplemented!("we might do something here eventually")
    } 
    else {
        unreachable!()
    }

    panic!("we should use panics sparingly.");
}

// Attributes are used to annotate code and add metadata or functionality to it.
// for example tests
#[test] // this is an outer attribute. It is applied to the item that follows it.
pub fn is_true() {
    assert!(true, "successful test")
}
// attributes can also be outer attributes, applied to the scope they are in.

#[derive(Debug, Clone)]
struct Chair {
    legs: u32,
    wooden: bool,
}
 
let chair = Chair {
    legs: 4,
    wooden: true,
};
 
// Now we can print the debug output of our `Chair` type:
println!("{chair:#?}")