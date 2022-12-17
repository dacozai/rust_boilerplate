use crate::db::sql::mysql;
use self::mysql::ppl::dsl::*;
use crate::err::CustomError;

use diesel::prelude::*;
use eyre::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct MyPpl {
    pub id: i32,
    pub surname: Option<String>,
    pub givenname: Option<String>,
}

impl MyPpl {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = &mut mysql::connection().unwrap();
        let results = ppl 
            .limit(5)
            .load::<MyPpl>(conn)
            .expect("Error loading ppl");
        println!("Displaying {} ppl", results.len());
        Ok(results)
    }
}
