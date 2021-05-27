#[cfg(test)]
mod generics_tests {

  #[test]
  #[ignore]
  fn generics_test () {
    struct Animal {
      legs: i32,
    }

    let dog = Animal {
      legs: 4
    };
    assert_eq!(dog.legs, 4);

    // TODO
    // let snake = Animal {
    //   legs: None
    // };
    // assert_eq!(snake.legs, None);
  }

  #[test]
  #[ignore]
  fn trait_test () {
    trait Empty {
      fn is_empty(&self) -> bool;
    }

    struct Animal {
      name: String,
    }

    // TODO: Implement Empty for animal..

    let empty_animal = Animal {
      name: String::new()
    };

    let cat = Animal {
      name: String::from("Cat")
    };
   
    // TODO
    // assert_eq!(empty_animal.is_empty(), true);
    // assert_eq!(cat.is_empty(), false);
  }

  #[test]
  #[ignore]
  fn generic_trait_test () {

    pub trait Add <T> {
      type Output;
      fn add(&self, a: T) -> Self::Output; 
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    pub enum Opt <T> {
      Som(T),
      Non
    }
    use Opt::*;

    // TODO: Implement add
    // impl <T: std::ops::Add<Output = T> + std::marker::Copy> Add <T> for Opt <T> {

    // }

    // assert_eq!(Som(4), Som(2).add(2));
  }
}













































// Solutions: in case of emergency
/*
fn generics_test () {
  struct Animal <T> {
    legs: T,
  }


  let dog = Animal::<i32> {
    legs: 4
  };
  assert_eq!(dog.legs, 4);

  let snake = Animal::<Option<i32>> {
    legs: None
  };
  assert_eq!(snake.legs, None);
}


fn trait_test () {
  trait Empty {
    fn is_empty(&self) -> bool;
  }

  struct Animal {
    name: String,
  }

  impl Empty for Animal {
    fn is_empty(&self) -> bool {
      self.name.is_empty()
    }
  }

  let empty_animal = Animal {
    name: String::new()
  };

  let cat = Animal {
    name: String::from("Cat")
  };
  
  assert_eq!(empty_animal.is_empty(), true);
  assert_eq!(cat.is_empty(), false);
}

fn generic_trait_test () {

  pub trait Add <T> {
    type Output;
    fn add(&self, a: T) -> Self::Output; 
  }

  #[derive(Debug)]
  #[derive(PartialEq)]
  pub enum Opt <T> {
    Som(T),
    Non
  }
  use Opt::*;

  impl <T: std::ops::Add<Output = T> + std::marker::Copy> Add <T> for Opt <T> {
    type Output = Opt<T>;

    fn add(&self, a: T) -> Opt<T> {
       match self {
        Som(b) => Som(a + *b),
        Non => Non
       }
    }
  }

  assert_eq!(Som(4), Som(2).add(2));
}

*/

