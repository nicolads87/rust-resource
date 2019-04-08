
mod resource;
//use resource::{Get};
extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use crate::resource::{get, Get};
use std::fmt;

extern crate tokio;
extern crate futures;


// `Poll` is a type alias for `Result<Async<T>, E>`
use futures::{Future, Async, Poll};


#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}




fn main() {

    struct HelloWorld;

    impl Future for HelloWorld {
        type Item = String;
        type Error = ();

        fn poll(&mut self) -> Poll<Self::Item, Self::Error> {

            Ok(Async::Ready("hello worlddddd".to_string()))
        }
    }

    struct Display<T>(T);



    impl<T> Future for Display<T>
        where
            T: Future,
            T::Item: fmt::Display,
    {
        type Item = ();
        type Error = T::Error;


        fn poll(&mut self) -> Poll<(), T::Error> {

            let value = match self.0.poll() {
                Ok(Async::Ready(value)) => value,
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(err) => return Err(err),
            };

            println!("{}", value);
            Ok(Async::Ready(()))
        }
    }

    //let future = Display(HelloWorld);
    let future = HelloWorld.map(|value| {
        println!("{}", value);
        println!("Future : {}", value);
    });


    let h = HelloWorld{};
    let future = h.and_then(|r| {
        println!("got {:?}", r);
        Ok(())


    });

    tokio::run(future);




    impl Person {
        fn print(&self) {
            println!("{:?}", self);
        }
    }




    let user = Person {name: String::from("Bar"), age: 9, phones: vec![]};
    user.print();










    let u3: Get<Person>  = get("localhost:8000");
    println!("u3: Get<Person> {:?}", u3);
    let future_u3 = u3.then(|person| {
        println!("Get<Person> {:?}", person);

        Ok(())

    });



    tokio::run(future_u3);




    println!("Hello, world!");


}
