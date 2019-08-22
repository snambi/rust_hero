use rocket::http::{Status,ContentType};
use rocket::response::{Responder, Body};
use rocket::{Request, Response};
use std::io::Cursor;
//use rocket::request::Request;
//use rocket::response::Result;

#[derive(Serialize, Deserialize)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32,
}

impl Hero {
    pub fn new(
        num: Option<i32>,
        name: String,
        identity: String,
        hometown: String,
        age: i32,
    ) -> Hero {
        Hero {
            id: num,
            name: name,
            identity: identity,
            hometown: hometown,
            age: 0,
        }
    }
}

impl<'r> Responder<'r> for Hero {

    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {

        let stat = Status::Ok;

        let response = Response::build()
            .status(Status::ImATeapot)
            .header(ContentType::Plain)
            .raw_header("X-Teapot-Make", "Rocket")
            .raw_header("X-Teapot-Model", "Utopia")
            .raw_header_adjoin("X-Teapot-Model", "Series 1")
            .sized_body(Cursor::new("Brewing the best coffee!"))
            .finalize();

        Result::Ok(response)
    }

}
