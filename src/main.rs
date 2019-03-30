
mod resource;
use resource::Resource;
fn main() {


    #[derive(Debug)]
    pub struct User {

        username: String,
        id: u32
    }

    impl User {
        fn print(&self) {
            println!("{:?}", self);
        }
    }


    impl Resource for User {

        type Item = User;

        fn get(&self) -> User {
            self.print();

            User {username: String::from("Foo"), id: 0}
        }

        fn save(&self) -> User {
            self.print();

            User {username: String::from("Foo"), id: 0}
        }



    };

    let user = User {username: String::from("Bar"), id: 9};
    user.print();

    let get_user = user.get();
    get_user.print();


    println!("Hello, world!");

}
