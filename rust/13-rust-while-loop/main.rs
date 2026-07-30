//** while loop could run forever base on the condition true.
let mut loopCount = 1;
while loopCount<=5 {
  println("Do something bro");
  loopCount+=1;
}

//** loop under false condition

while loopCount <= 10 {

  // ** stop the loop when count = 6
  if loopCount == 6 {
    break;
  }
  println("Current number {}", loopCount);
  loopCount+=1;
}

let count = 10;

while count <= 5 {
  println!("This won't be printed.");
}


//**skip loop

let mut num = 1;

while num <= 10 {
  if num == 6 {
    num += 1;
    continue;
  }

  println!("Number: {}", num);
  num += 1;
}

