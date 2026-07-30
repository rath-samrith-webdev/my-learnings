//declaring a functions

fn function_name(){
  //** executable codes
}

// ** calling a function
function_name();

//Example

fn say_hi(){
  println("Hello / Hi from a function");
}

say_hi(); //calling a functions

//function with parametres

fn greet_someone(name &str){
  println("Hello {}!", name);
}

greet("Jonh");

//function with return value

fn add (a:i32, b:i32) -> i32{
  return a + b;
}

let sum = add(3,4);
println("The sum is: {}", sum);


