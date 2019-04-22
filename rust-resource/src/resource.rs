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








pub mod tests {
    use std::io;
    use serde::Deserialize;
    use serde_json::Error;
    extern crate tokio;
    extern crate futures;
    extern crate reqwest;
    extern crate serde;
    extern crate serde_json;
    // `Poll` is a type alias for `Result<Async<T>, E>`
    use futures::{Future, Async, Poll};
    use std::collections::HashMap;

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
}


pub mod resource {
    use serde::Deserialize;
    use std::fmt::Debug;
    use reqwest::Response;

    pub struct Resource {
        url: String
    }

    impl Resource {

        pub fn get<T>(&self, params: Vec<(&'static str,&'static str)>) -> Result<T, String>

            where  for<'de> T: Deserialize<'de> + Debug {

            let mut url = self.url.clone();

            //Given a template /path/:verb and parameter {verb: 'greet', salutation: 'Hello'} results in URL /path/greet?salutation=Hello.
            for param in params {

                let key = format!("{}{}", ":", param.0);
                url = url.replace(&key, param.1);
                println!("{:?}, {}", param, url);
            }


            let result: Result<Response, _> = reqwest::get(&url);
            println!("{:#?}", result);

            //let aaa = result.unwrap().text();

            let json: Result<T, reqwest::Error> = result.unwrap().json();

            let data: T = json.unwrap();
            println!("{:#?}", data);

            Ok(data)

        }


    }

    pub fn resource(url: &str) -> Resource {

        Resource {
            url: String::from(url)
        }
    }
}








