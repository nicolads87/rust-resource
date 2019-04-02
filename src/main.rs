
mod resource;
use resource::{Resource, Get};
extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use crate::resource::{get, query, Query};


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}


struct Service {

}

fn main() {

    impl Resource for Service {
        type Url = String;

    }


    impl Person {
        fn print(&self) {
            println!("{:?}", self);
        }
    }




    let user = Person {name: String::from("Bar"), age: 9, phones: vec![]};
    user.print();



    let service = Service{};

    let u: Person = service.get().unwrap();

    let u2:Vec<Person> = service.query().unwrap();
    println!("u: {:?}", u);
    println!("u2: {:?}", u2);


    let u3: Get<Person> = get("localhost:8000");
    let u4: Query<Person> = query("http:localhost:8080/all");

    println!("u3: {:?}", u3);
    println!("u4: {:?}", u4);

    println!("Hello, world!");


}
