use std::panic;
use std::fmt;

fn panic_function() {
  panic!("Panic!");
}

fn ok_func() {
  println!("OK FUNC!");
}

struct Test {
  x: i32
}

impl fmt::Debug for Test {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Test")
      .field("x", &self.x)
      .finish()
  }
}

trait Log {
  fn log(&self) -> ();
}

impl Log for Test {
  fn log(&self) {
    println!("LOG LOG");
  }
}

fn main2() {
  let test = Test {
    x: 6 
  };
  test.log();
}

enum IntOption {
  Some(i32),
  None,
}

enum CharOption {
  Some(char),
  None,
}

enum Option <T> {
  Some(T),
  None
}


fn main() {
    let tst = Test {
      x: 2
    };
    tst.log();


    println!("{:?}", tst);

    let panic_res = panic::catch_unwind(panic_function);

    println!("{:?}", panic_res);

    let ok_res = panic::catch_unwind(ok_func);
    
    println!("{:?}", ok_res);
}
