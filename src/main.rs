#[macro_use]
extern crate reflect_test_proc;

use test_trait::TestTrait;

#[derive(TestTrait)]
struct TestStruct {
    int: i32,
    string: String,
}

fn main() {
    let test = TestStruct {
        int: 10,
        string: String::from("Hello world!"),
    };
    
    assert_eq!("intstring", test.do_something());
}
