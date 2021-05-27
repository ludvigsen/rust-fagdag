#[cfg(test)]
mod error_tests {

  fn some_string() -> Option<Vec<i32>>{
    Some(vec![1])
  }

  #[test]
  fn test_option () {
    let vec = vec![1,2,3];
    let fourth = vec.get(3); // None
    let third = vec.get(2); // Some(3)

    // TODO: assert the content of third and fourth element
  }

  #[test]
  fn test_result () {
    let res = "2".parse::<i32>();
  
    // TODO: match result in different ways
  }

  #[test]
  fn test_try () -> Result<(), std::num::ParseIntError> {
    let res = "2".parse::<i32>()?;

    assert_eq!(res, 2);

    // TODO: show try operator 
    Ok(())
  }
}









// Solutions 
/*
  fn test_option () {
    // let array = [1,2,3];
    // let out_of_bounds = array[4]; // will panic
    
    
    let vec = vec![1,2,3];
    let fourth = vec.get(3); // None
    let third = vec.get(2); // Some(3)

    assert_eq!(None, fourth);

    let third = vec.get(2);
    assert_eq!(*((third).unwrap()), 3);
  }

  fn test_result () {
    let res = "2".parse::<i32>();
    match res {
      Ok(num) => assert_eq!(num, 2),
      Err(..) => assert!(false)
    }

    let res2 = "asdf".parse::<i32>();
    let error = match res2 {
      Ok(..) => "ok".to_string(),
      Err(err) => err.to_string()
    };
    assert_eq!(error, "invalid digit found in string");
  }

  fn test_try () -> Result<(), std::num::ParseIntError> {
    let res = "2".parse::<i32>()?;
    assert_eq!(2, res);
    let res2 = "123".parse::<i32>()?;
    assert_eq!(123, res2);
    Ok(())
  }

*/
