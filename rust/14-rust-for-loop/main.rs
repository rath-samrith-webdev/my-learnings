
//**loop from 1 to n -1

for i in 1..5{
  println("the i val is {}", i);
}

//** loop from 1 to n

for j in 1..=5{
  println("the j val is {}", j);
}

//** break and continue in loop
for i in 1..=10 {
  if i == 3 {
    continue; // skip 3
  }
  if i == 5 {
    break; // stop before printing 5
  }
  println!("i is: {}", i);
}
