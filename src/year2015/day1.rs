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

  /*macro_rules! a_sample_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
      #[test]
      fn a_$name() -> io::Result<()> {
        let data = get_data(SAMPLE(Some("$name")))?;
        assert_eq!(a(data), $value);
        Ok(())
      }
    )*
    }
  }

  a_sample_tests! {
    sample1: 0,
    sample2: 0,
    sample3: 3,
  }*/

  #[test]
  fn a_sample4() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample4")))?;
      let result = a(data);

      assert_eq!(result, 3);
      Ok(())
  }

  #[test]
  fn a_sample5() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample5")))?;
      let result = a(data);

      assert_eq!(result, 3);
      Ok(())
  }

  #[test]
  fn a_sample6() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample6")))?;
      let result = a(data);

      assert_eq!(result, -1);
      Ok(())
  }

  #[test]
  fn a_sample7() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample7")))?;
      let result = a(data);

      assert_eq!(result, -1);
      Ok(())
  }

  #[test]
  fn a_sample8() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample8")))?;
      let result = a(data);

      assert_eq!(result, -3);
      Ok(())
  }

  #[test]
  fn a_sample9() -> io::Result<()> {
      let data = get_data(SAMPLE(Some("sample9")))?;
      let result = a(data);

      assert_eq!(result, -3);
      Ok(())
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
