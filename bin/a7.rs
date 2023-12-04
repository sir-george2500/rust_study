//Your mission is to write a function that take a
// Color Enumeration and return the color

enum Color{
    Red,
    Yellow,
    Blue,
}

fn display_color(my_color:Color){

    match my_color {
        Color::Red => println!("this is the color blue"),
        Color::Yellow => println!("this is the color red"),
        Color::Blue => println!("this is the color blue"),
    }


}

fn main(){
    display_color(Color::Blue);
}
