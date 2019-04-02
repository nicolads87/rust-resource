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



use serde::Deserialize;
use serde_json::Error;



#[derive(Debug)]
pub struct Get<T> {
    data: T
}

#[derive(Debug)]
pub struct Query<T> {

    data: Vec<T>

}



pub fn get<'a, T>(url: &str) -> Get<T> where  T: Deserialize<'a> {

    println!("url: {}", url);
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let t: T  = serde_json::from_str(data).unwrap();

    Get {
        data: t
    }
}

pub fn query<'a, T>(url: &str) -> Query<T> where  T: Deserialize<'a> {

    println!("url: {}", url);
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let t: T  = serde_json::from_str(data).unwrap();

    Query {
        data: vec![t]
    }
}

///A resource "class" object with methods for the default set of resource actions optionally extended with custom actions.
/// The default set contains these actions:

pub trait Resource<ParamsDefault=Vec<String>, Actions=Vec<String>, Options=Vec<String>> {

    type Url;

    fn get<'a, T>(&self) -> Result<T, Error> where  T: Deserialize<'a> {


        //println!("{}", Self::Url);
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let t: T  = serde_json::from_str(data).unwrap();
        Ok(t)
    }


    //fn save(&self) -> Self::Item;

    fn query<'a, T>(&self) -> Result<Vec<T>, Error> where  T: Deserialize<'a> {

        let data = r#"
        {
            "name": "Foo Bar",
            "age": 30,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        let t: T  = serde_json::from_str(data).unwrap();
        Ok(vec![t])
    }


}



