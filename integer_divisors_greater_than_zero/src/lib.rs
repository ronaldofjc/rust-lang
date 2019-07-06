#[cfg(test)]
mod tests {
  #[test]    
  #[should_panic]
  fn zero_is_not_valid() {        
    dividers(0);    
  }

  #[test]    
  #[should_panic]
  fn negative_is_not_valid() {
    dividers(-10);
  }

  #[test]    
  #[should_panic]
  fn one_is_not_valid() {
    dividers(1);
  }

  #[test]
  fn two_divider_two() {
    assert_eq!(dividers(2), vec![2])
  }

  #[test]
  fn three_divider_three() {
    assert_eq!(dividers(3), vec![3])
  }

  #[test]
  fn dividers_four() {
    assert_eq!(dividers(4), vec![2, 4])
  }

  #[test]
  fn dividers_twenty_four() {
    assert_eq!(dividers(24), vec![2, 3, 4, 6, 8, 12, 24])
  }

  #[allow(dead_code)]
  fn dividers(value: i64) -> Vec<i64> {   
    if value <= 1 {
        panic!("{} <= 0 não é válido", value);   
    } else {
        (2..value + 1).filter(|x| value % x == 0).collect::<Vec<i64>>()
    }
  }
}

