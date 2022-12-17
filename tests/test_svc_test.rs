#[cfg(test)]
mod tests {
  use rust_boilerplate::service::test_svc::TestSvc;

  #[test]
  fn it_works() {
    let s = TestSvc::new("hey".to_string());
    let result = s.sum_up(1, 3);
    assert_eq!(result, 4);
  }
}