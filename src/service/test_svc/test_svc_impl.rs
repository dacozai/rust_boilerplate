use crate::db::sql::mysql::MyPpl;

pub struct TestSvc {
  pub name: String,
}

impl TestSvc {
  pub fn new(name: String) -> Self {
    Self { 
      name: name,
    }
  }

  pub fn hello(&self) -> String {
    println!("Hello {}", self.name);
    format!("TestSvc say hi to {}", self.name)
  }

  pub fn sum_up(&self, a: i16, b: i16) -> i16 {
    a + b
  }

  pub fn read_db(&self) -> Vec<MyPpl> {
    let res = MyPpl::find_all();
    res.unwrap()
  }
}
