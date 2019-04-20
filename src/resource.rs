//!The MIT License
//!
//!Copyright (c) 2019 nicolads87
//!
//!Permission is hereby granted, free of charge, to any person obtaining a copy
//!of this software and associated documentation files (the "Software"), to deal
//!in the Software without restriction, including without limitation the rights
//!to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//!copies of the Software, and to permit persons to whom the Software is
//!furnished to do so, subject to the following conditions:
//!
//!The above copyright notice and this permission notice shall be included in
//!all copies or substantial portions of the Software.
//!
//!THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//!IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//!FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//!AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//!LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//!OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//!THE SOFTWARE.


use std::io;
use serde::Deserialize;
use serde_json::Error;
extern crate tokio;
extern crate futures;


// `Poll` is a type alias for `Result<Async<T>, E>`
use futures::{Future, Async, Poll};

#[derive(Debug)]
enum State {
    Consuming,
    None,
    Done
}

#[derive(Debug)]
pub struct Get<T> {

    state: State,
    data: Option<T>
}


#[derive(Debug)]
pub struct Query<T> {

    state: State,
    data: Option<Vec<T>>
}

impl<'a, T> Future for Get<T> where  T: Deserialize<'a> {
    type Item = T;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let data = raw_json();

        let t: T  = serde_json::from_str(data).unwrap();
        self.state = State::Done;
        self.data = Some(t);


        match &self.data {

            None => return Ok(Async::NotReady),
            Some(_) => {
                let data = self.data.take().unwrap();
                return Ok(Async::Ready(data))
            }

        };

    }
}

impl<'a, T> Future for Query<T> where  T: Deserialize<'a> {
    type Item = Vec<T>;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let data = raw_json();

        let t: T  = serde_json::from_str(data).unwrap();
        self.state = State::Done;
        self.data = Some(vec![t]);


        match &self.data {

            None => return Ok(Async::NotReady),
            Some(_) => {
                let data: Vec<T> = self.data.take().unwrap();
                return Ok(Async::Ready(data))
            }

        };


    }
}




fn raw_json() -> &'static str {

    let json = r#"
        {
            "name": "John Doe Poll",
            "age": 30,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    json
}

pub fn get<T>(url: &str) -> Get<T>

{
    println!("call get");
    println!("url: {}", url);

    Get {
        state: State::Consuming,
        data: None
    }
}

pub fn query<T>(url: &str) -> Query<T>

{
    println!("call query");
    println!("url: {}", url);

    Query {
        state: State::Consuming,
        data: None
    }
}




