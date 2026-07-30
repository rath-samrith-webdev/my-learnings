let mut limit i32 = 1;

loop {
  println("Hello {}", limit);
  if limit == 3{
    break limit;
  }

  limit += 1
}

// **loop** here will go on forever
// **Hovever you can also using break keywords to exit the loop under a certain condiations
