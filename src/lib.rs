pub fn print_hello() {
    println!("Hello, world!");
}

#[test]
fn test_print(){
    print_hello();
    assert!(true);
}

