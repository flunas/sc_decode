fn main() {
    println!("hello world!");
    let arg = "123%40gmail.com";
    let val = sc_decode::decode(arg);
    assert_eq!("123@gmail.com", val);
}