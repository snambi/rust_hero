

use rocket::response::Responder;
use rocket::{Request, Response};
use rocket::http::Status;
//use rocket::request::Request;
//use rocket::response::Result;

#[derive(Serialize, Deserialize)]
pub struct Hero{
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero{

    pub fn new( num:Option<i32>, name:String,
            identity:String, hometown:String, age:i32 ) -> Hero {

        Hero{ id:num,
            name:name,
            identity: identity,
            hometown: hometown,
            age: 0 }
    }
}

impl <'r> Responder<'r> for Hero {

    fn respond_to(self, request: &'r Request) -> Result<Response, Status> {
        unimplemented!()
    }


//    fn respond_to(self, request: &Request) -> response::Result<'r>{
//
//    }
}