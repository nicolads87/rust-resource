
mod resource;
//use resource::{Get};
extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use crate::resource::resource::*;
use std::fmt;

extern crate tokio;
extern crate futures;


// `Poll` is a type alias for `Result<Async<T>, E>`
use futures::{Future, Async, Poll};

#[macro_use]
extern crate hello_macro;

#[macro_use]
extern crate rresource_derive;


//use rresource::RResource;

pub trait HelloMacro {
    fn hello_macro();
}



#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug, RResource)]
struct Wheater {
    base: String,
    name: String,
    id: u32
}
#[derive(HelloMacro, RResource, Debug)]
struct Pancakes;



fn main() {

    Pancakes::hello_macro();
    //Pancakes::save();

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



/*
    let u3: Get<Person>  = get("localhost:8000");
    println!("u3: Get<Person> {:?}", u3);
    let future_u3 = u3.then(|person| {
        println!("Get<Person> {:?}", person);

        Ok(())

    });



    tokio::run(future_u3);


    let u4: Query<Person>  = query("localhost:8000");
    println!("u4: Query<Person> {:?}", u4);
    let future_u4 = u4.then(|persons| {
        println!("Query<Person>  {:?}", persons);

        Ok(())

    });



    tokio::run(future_u4);


*/


    //let resp = reqwest::get("https://samples.openweathermap.org/data/2.5/weather?lat=35&lon=139&appid=b6907d289e10d714a6e88b30761fae22");

    let host = "https://samples.openweathermap.org";
    let path = "/data/2.5/weather?lat=:lat&lon=:lon&appid=:appid";

    let wheater_resource: Resource = Resource::new(host, path);



    let params = vec![("lat", "35"), ("lon", "139"), ("appid", "b6907d289e10d714a6e88b30761fae22")];

    let result: Result<Wheater, _> = wheater_resource.get(params);


    println!("Result<Wheater, _>  --> {:#?}", result);

    let mut weather: Wheater = result.unwrap();


    let post_params = vec![("lat", "35"), ("lon", "139"), ("appid", "b6907d289e10d714a6e88b30761fae22")];

    weather.name = String::from("London");
    weather.save();
    weather.test();


    let w = Wheater {
        id: 0,
        base: String::from("hsa8sabak"),
        name: String::from("daja77aba")

    };

    w.save();



}
