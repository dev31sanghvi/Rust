
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

//refrences

fn main(){
    let s1 = String::from("yo yo");
    let s2=&s1;
    // let len = calculate_length(&s1); // passing a reference to the function
    // println!("The length of '{}' is {}.", s1, len);
    print!("s1: {}, s2: {}", s1, s2);
}

fn main(){
    let mut s1:String=String::from("sup");
    update_word(&mut s1); // passing a mutable reference to the function
    print("{)",s1);
}

fn update_word(word: &mut String) {
    word.push_str(" world");
}


fn main() {
    let mut s1 = String::from("Hello, world!");
    let s2= &mut s1;
let s3:&String = &s1; // now this will error as single refrence is allowed , cant have more than one borrower .
    update_word(word: &mut s1); // passing ref.
    print!("{}",s1);
    print!("{}",s2);
}

fn update_word(word: &mut String) {
    word.push_str(" world");
}


 struct User {
    active:bool,
    username:String,
    email:String,
    sign_in_count:u64,
 }
 fn main() {
    let user1 = User {
        active: true,
        username: String::from("devsa@gmail.com"),
        email: String::from("devsa@gmail.com"),
        sign_in_count: 1,
    };
    print!("User 1 username: {:?}", user1.username);
}

// implementing structs
//similar to classes in ts

struct Rect {
    width: u32,
    height: u32,
}

impl Rect{
    // you can attact any kind of funtion to this
    fn area(&self) -> u32 {
      return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

}
fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

print!("area is {:?}", rect);
}

struct Point {
    x: i32,
    y: i32,
}

let a = Point { x: 1, y: 2 };
let b = a;
let c = a.clone();

//pattern matching and stuff


enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}


fn calculate_area(shape: Shape) -> f64 {
    let ans: f64 =match shape{
  Shaoe::Circle(radius) => std::f64::consts::3.14 * radius * radius,
        // Shape::Square(side) => side * side,
        Shape::Rectangle(width, height) => width * height,
    };
    // Shape::Square(side) => side * side
    return  ans;
}

fn main() {

    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);


    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));
}

//error handling
//enum with generic types(can be any capital letter )
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs;

fn main() {
    let greeting_file_result = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        },
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}