fn main(){
    let my_name = "Bob";

    match my_name {
        "George" => println!("that is my name"),
        "Bob" => println!("not my name"),
        "Alice" => println!("hello alice"),
        _ => println!("nice to meet you!"),
    }


}