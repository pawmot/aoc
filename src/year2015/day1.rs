#[cfg(test)]
mod day1 {
  use crate::data::{
      common::DatasetType::{FULL, SAMPLE},
      year2015::day1::get_data,
  };
  use std::io;

  fn a(data: String) -> i64 {
      let mut current_floor = 0i64;

      for c in data.chars() {
          match c {
              '(' => current_floor += 1,
              ')' => current_floor -= 1,
              unexp => panic!("Did not expect {}", unexp),
          }
      }

      return current_floor;
  }

  macro_rules! a_sample_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
      #[test]
      fn $name() -> io::Result<()> {
        let (filename, expected) = $value;
        let data = get_data(SAMPLE(Some(filename)))?;
        assert_eq!(a(data), expected);
        Ok(())
      }
    )*
    }
  }

  a_sample_tests! {
    a_sample1: ("sample1", 0),
    a_sample2: ("sample2", 0),
    a_sample3: ("sample3", 3),
    a_sample4: ("sample4", 3),
    a_sample5: ("sample5", 3),
    a_sample6: ("sample6", -1),
    a_sample7: ("sample7", -1),
    a_sample8: ("sample8", -3),
    a_sample9: ("sample9", -3),
  }

  #[test]
  fn a_full() -> io::Result<()> {
      let data = get_data(FULL)?;
      let result = a(data);

      assert_eq!(result, 138);
      Ok(())
  }

  fn b(data: String) -> usize {
      let mut current_floor = 0i64;
      let mut result: usize = 0;

      for (idx, c) in data.char_indices() {
          match c {
              '(' => current_floor += 1,
              ')' => current_floor -= 1,
              unexp => panic!("Did not expect {}", unexp),
          }

          if current_floor == -1 {
              result = idx + 1;
              break;
          }
      }

      return result;
  }

  #[test]
  fn b_sample10() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample10")))?;
      let result = b(data);

      assert_eq!(result, 1);
      Ok(())
  }

  #[test]
  fn b_sample11() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample11")))?;
      let result = b(data);

      assert_eq!(result, 5);
      Ok(())
  }

  #[test]
  fn b_full() -> io::Result<()> {
      let data = get_data(FULL)?;
      let result = b(data);

      assert_eq!(result, 1771);
      Ok(())
  }
}
