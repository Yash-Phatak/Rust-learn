// fn main() {
//     let a = 4;
//     println!("{}",a);
// }   



/* Take user input */
// use std::io::{self, Read};
// fn main(){
//     let mut n = String::new();
//     io::stdin().read_line(&mut n).unwrap();
//     let number: i32 = n.trim().parse().expect("Enter a valid number");
// //OR
//      let number: i32 = n.trim().parse().unwrap();
//     print!("The entered number is {}",number);

// }



/* Take string input  */ 
// use std::io;
// fn main(){
//     let mut input = String:: new();
//     println!("Enter a space separated string of integers:");
//     io::stdin().read_line(&mut input).unwrap();

//     // Initialise a vector
//     let a : Vec<&str> = input.split_whitespace().collect();
//     let x : i32= a[0].trim().parse().unwrap();
//     let y : i32 = a[1].trim().parse().unwrap();
//     print!("Product of both the numbers is: {} ",x*y);
// }


/* How to use a for loop in a string or a regular for loop */ 
// fn main(){
//     let string : String = String::from("Yash Phatak is my name.");
//     let word = get_first_word(string);
//     for i in 0..10{
//         println!("{}",i);
//     }
//     println!("First word is: {}",word);
// }

// fn get_first_word(sentence : String) -> String{
//     let mut ans = String:: new();
//     for char in sentence.chars(){
//         ans.push_str(char.to_string().as_str());
//         if char==' '{
//             break;
//         }
//     }
//     return ans;
// }



/* Ownership in Rust */
// Rules:
// 1. Each value in Rust has an owner.
// 2. There can be only one owner at a time.
// 3. Once the owner goes out of the scope, the value is dropped.

// SCOPE : It is the range in the program within which the item is valid.

// Variable Scope
// fn main(){                      // s is not valid here, it’s not yet declared
//     let s = "hello";   // s is valid from this point forward
//     // do stuff with s
// }                      // this scope is now over, and s is no longer valid


/* Stack Variables */
// Whenever the function is called, a copy of variables is being created here and stored in a new stack frame. 
// Once the execution of the function is completed, it is popped out of the stack memory.
// fn main(){
//     let x = 1; // created on stack, owner is main fn
//     let y = 2;// created on stack, owner is main fn
//     println!("{}",sum(x,y)); // function is called, so a new memory is being alloted 
//     println!("Hello World."); // function execution is over => popped out
// }

// fn sum(a:i32,b:i32)->i32{
//     let c = a + b; // owner of a,b,c is sum fn
//     return c;
// }



/* Scoping Variable in same function */
// fn main(){
//     let x = 1; // created on stack with owner, as main fn
//     {
//         let y = 3; // created on stack but owner is {}
//     } // y is popped off
//     print!("{}",y); // throws an error  
// }


/* HEAP VARIABLES */
// They always want to have a single owner, if that owner goes out of scope, they are deallocated.
// fn main(){
//     let s1:String = String :: from("Hello"); // here, s1 is a pointer in stack frame with pointer to hello in heap. s1 owns hello.
//     println!("s1 is {}",s1);
//     let mut s2 = s1; // once s2 = s1,usually in c both of them point to the same heap memory, leading to lot of errors -> dangling pointers, double free errors.
//     // Now, once s2 points to hello; by ownership rules s1 pointer is removed and s1 gets killed. Now, s2 owns hello!
//     println!("s2 is {}",s2);
//     // println!("{}",s1); // says the value is moved => hello ditched s1 and moved with s2.
//     let s3 = s2.clone(); // this will create a new copy 
//     s2.push_str(" World");
//     println!("s2 is {}",s2);
//     println!("s3 is {}",s3);
// } 


/*  ANOTHER IMPORTANT EXAMPLE */
// When we call integers as functions, a copy is created in the stack.
// But when Strings are called, a reference is called as they are stored in a heap. As a result, the ownership of the variables change.
// fn main(){
//     let my_string = String  :: from("Hello, World."); // my_string is the current owner of "Hello, World"
//     take_ownership(my_string);
//     /*println!("{}",my_string); */ // This will throw an error as some_string takes the ownership of "Hello, World" when the function was called.
// }

// fn take_ownership(some_string: String){ 
//     println!("Some String is: {}",some_string);
// } // As the scope gets over, both some_string and hello,world get killed.


// Take away : Numbers get cloned easily, but strings arent as they are present in heap and not stack.


/* Now there is a way with which, Hello,World can move back to my_string, instead of getting killed. */
// Answer => Return it back to the owner.

// fn main(){
//     let mut my_string = String:: from("Hello,World");
//     my_string = take_ownership(my_string); // some_string takes the ownership of Hello,World, prints it and returns it back to my_string
//     println!("My String is: {}",my_string);
// }

// fn take_ownership(some_string: String)->String{
//     println!("Some string is: {}",some_string);
//     return some_string; // ownership is returned
// } // second bf dies and she returns back to ex ie the first bf

/* BORROWING AND REFERENCING */
// Referencing : Giving the address of a string rather than the ownership of a string to a function
// fn main(){
//     let s1 = String::from("Hello, World");
//     let s2 = &s1; // s2 is accessing s1's address. It is dependent on s1 => it is not the owner of Hello, World.
//     println!("String s1 is {}",s1); // won't throw error 
//     println!("String s2 is {}",s2);
// }

// Borrowing
// Earlier we were transferring the ownership of variables to functions for strings. 
// Here, if we pass by reference, the function is borrowing the string and the ownership still stays with the original owner.
// All these borrowers are dependent on owner (&owner).So, if the owner goes out of scope, all the borrowers are killed.

// Three cases allowed for Borrowing:
// 1. Completely loyal to owner, and no borrowers.
// 2. Can have multiple borrowers but they can't mutate/update.
// 3. Can have a single borrower who can mutate the variable.

// CASE-2
// fn main(){
//     let s1 = String :: from("Hello, World.");
//     let _s2 = &s1; // borrowed s1
//     let _s3 = &s1; // borrowed s1
//     dont_take_ownership(&s1); // pass by reference, just borrowing it 
//     println!("Owner of  '{}' is still s1.",s1);
// }

// fn dont_take_ownership(some_string: &String){
//     println!("Some string has borrowed {}",some_string);
// }


// CASE-3 -> Mutable references &mut -> Only one borrower -> not even immutable references are allowed.
// Eg: Update a string 
// fn main(){
//     let mut s1 = String::from("Hello, ");
//     println!("Old String: {}",s1);
//     // Even another string can mutably borrow. let s2 = &mut s1;
//     update_string(&mut s1); // mutable reference invoked
//     println!("New String: {}",s1);
// }

// fn update_string(s: &mut String){
//     s.push_str("Yash");
// }


/* Structs */
// struct User{
//     active: bool,
//     username: String,
//     email : String,
//     sign_in_count : u64,
// }
// fn main(){
//     let user = User{
//         active: true,
//         username : String:: from("Yash"),
//         email : String:: from("abc@gmail.com"),
//         sign_in_count: 4,
//     };
//     println!("The {} has signed in {} times via {}",user.username,user.sign_in_count,user.email);
// }

/* Implementing Structs */
// Attach functions to structs like classes
// struct Rect{
//     width : i32,
//     height : i32,
// }

// impl  Rect {
//     fn get_area(&self)-> i32{
//         return self.height * self.width;
//     }
// }

// fn main(){
//     let rect = Rect{width : 2,height :4};
//     print!("The area of the rectangle is {}",rect.get_area());
// }

// Unit Structs
// Structs with no attributes but only implementations 
// struct NoShape;

// impl NoShape {
//     fn area(&self)->u32{
//         return 0;
//     }
// }

// fn main(){
//     let noshape = NoShape;
//     print!("{}",noshape.area());
// }


/* Enums */
// Allows you to define a type by enumerating its possible variants 
// // Adds more strictness to your codebase
// enum Direction{
//     North,
//     West,
//     South,
//     East,
// }

// fn main(){
//     move_around(Direction:: East);
// }

// fn move_around(direction: Direction){
//     // implements logic to move around
// }


// /* Enums with Values */
// enum Shape{
//     Circle(f64),
//     Rectangle(f64,f64),
//     Square(f64),
// }

// fn main(){
//     let circle = Shape:: Circle(7.0);
//     let _square: Shape = Shape:: Square(2.0);
//     let _rectangle: Shape = Shape:: Rectangle(3.7, 4.5);
//     println!("{}",calculate_area(circle));
// }
/* Pattern Matching Enums */
// fn calculate_area(shape:Shape)->f64{
//     let ans: f64 = match  shape{
//         Shape::Circle(radius)  => 3.14*radius*radius,
//         Shape::Rectangle(width, height) => width*height,
//         Shape::Square(side) => side*side,
//     };
//     return ans;
// }

/* Error Handling */
// Enum Generics 
//<T,E> T is basically like auto in cpp that checks if the data type is same.[A,B,C,T.. any capital letter works]
// struct Point<T>{ 
//     x: T,
//     y: T,
// } // it means both x and y must have the same type => generic

// struct Point2<A,B>{
//     x:A,
//     y:B,
//     z:B,
// } // it means any two data types work but y and z must be the same
// fn main(){
//     let integer_point = Point{x:5,y:6};
//     let float_point = Point{x:5.0,y:7.0};
//     let string_point = Point{x:"1",y:"2"};
//     println!("Integer Point: {},{}",integer_point.x,integer_point.y);
//     println!("String Point: {} {}",string_point.x,string_point.y);
//     let random_point = Point2{x:2,y:"Hello",z:"World"};
// }

// Result Enum -> provided by rust -> inbuilt
// enum Result<A,B>{
//     Ok(A),
//     Err(B),
// }

// fn main(){
//     // there is a function that can error out or stop the execution of the thread.
//     // fs:: read_to_string("example.txt"); // will throw an error if path doesnt exist
//     let res = fs:: read_to_string("example.txt");
//     // Here we know , res can either be a string (Ok) or has an error
//     // We will pattern match it 
//     match res {
//         Ok(content)=>{
//             println!("The content is {}",content);
//         },
//         Err(content)=>{
//             println!("Error is {}",content);
//         }
//     }
//     print!("This code still runs even after the error.");
// }



/* Option Enum */
// this basically handles null values. There is no null keyword in rust.
// if you ever have a function that returns null, return an Option 
// pub enum Option<T>{
//     None,
//     Some(T),
// }

// fn find_first_a(s:String)-> Option<i32>{
//     for (index,character) in s.chars().enumerate(){
//         if character=='a'{
//             return Some(index as i32);
//         }
//     }
//     return None;
// }
// fn main(){
//     let string:String = String::from("hello, yash");
//     let opt = find_first_a(string);
//     match opt{
//         Some(index)=>{
//             println!("The first a is at the index: {}",index);
//         }
//         None => {println!("a doesn't exist in the string");}
//     }
// }


/* Collections */

// Vectors
// fn main(){
//     let mut vec = Vec::new();
//     vec.push(1);
//     vec.push(2);
//     vec.push(3);
//     vec.push(4);
//     println!("{:?}",vec); // :? is an debug trait
//     println!("{}",vec[0]); 
//     println!("{:?}",even_filter(&vec));
// }

// // Print even values from the vector
// fn even_filter(v : &Vec<i32>)-> Vec<i32>{
//     let mut ans : Vec<i32> = Vec::new();
//     for val in v{ // iterating a vector
//         if(val%2==0) {
//             ans.push(*val); // need to dereference a number to get its values
//         }
//     }
//     return ans;
// }


// Another way on initialising a vector using macros
fn main(){
    let vec: Vec<i32> = vec![1,2,3,4,5,6];
    for number in vec{
        print!("{} ",number);
    }
}