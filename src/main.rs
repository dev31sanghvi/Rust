fn main() {
    // println!("Hello, world!");
let x:i8=10;


    print!("x:{}", x);

// for i:i32 in 0..1000  {
//  x=x+100;



// }

if is_male{
    print!("male ");
}else {
    print!("not male " );
}

if is_male && is_above_18{
    print!("you are legal male ");
}

// strings

let greetings : &str=  "Hello, world!";
for i in 0..100{
    // ax=ax+
}


let char1:Option<char> = greetings.chars().nth(0);

main();


fn main (){
    let is_even:bool=true ;

if is_even {
    println!("The number is even.");
} else {
    println!("The number is odd.");
}

use std::string;

fn main(){
    let sentence :String = String::from("Hello, world!");
    let fisrt_word:String=get_first_word(&sentence);
    println!("The first word is: {}", fisrt_word);
}
//iterating over and array

fn get_first_word(sentence: &String) -> String {
    let mut first_word = String::new();
    for c in sentence.chars() {
        if c == ' ' {
            break;
        }
        first_word.push(c);
    }
    first_word
}


// simply iterating over an string

fn main(){
    let s= "dev sanghvio";
    for c in s.chars(){ // char method ia basically an iterator that iterated over all elements .
        print!("{} ", c);
    }
}

fn main() {
    let x= define_x();
        println!("{}, world", x);

    }

    fn define_x() -> &'static str {
        let x = "hello";
        x
    }

    fn main() {
        let mut x = 1;
        x+= 2;

        assert_eq!(x, 3);
        println!("Success!");
    }


    fn main(){
        let value=hello();

        std::mem::drop(value); // this will drop the value and free the memory
        print!("{} ", value);l
    }

    //destructuring
    // Fix the error below with least amount of modification
fn main() {
    let  (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}


// memory management-> allocation and deallocation
//   immutables -> value cant be changed once assigned
//stack frame
fn main(){

    let mut x:String = String::from("Hello, world!"); // this cant be changed until we make it mut.
    x.push_str(string:"asd");// err bcz x is immutable
    print!("{}", x);

}

// stack & heap

fn main(){
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn(){
    // doing something
}

fn heap_fn(){
let s1: String = String::from("Hello, world!");
    let s2: &str = "Hello, world!";

    let combined =format!("{} {}", s1, s2);
    print!("heap function:combined string'{}'",combined);
}


fn update_string() {

    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

//updating the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
}



// ownership-> set of rules that tells how a rust program manages memory.in rust memory is managed through a system of ownership with a set of rules that the compiuler checks .


// scoping variables in the same function

fn main(){
    let x=1;
    {
        let y=5;
    }
    print!("{}",y); // this will throws error
}


fn main(){
    let s1=String::from("Hello, world!");
    let s2=s1; // this will move the ownership of s1 to s2, so s1 is no longer valid
print!("{} ", s1); // two things cant point to same things
}

fn main(){
    let my_string = String::from("what is up ");
    takes_ownership(my_string); // moving ownership
    print!("{}",my_string);
}
//=>last option should be cloning
fn takes_ownership(some_string;String){
    print!("{}",some_string) // now somestring owns the data
}