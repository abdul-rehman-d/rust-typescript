enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self  {
            return true;
        }
        return false;
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Green | Color::Yellow => return true,
            _ => return false,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::Yellow => println!("yellow"),
    }
}

fn main() {
    let green = Color::Green;
    let yellow = Color::Yellow;

    println!("green.is_green() {}", green.is_green());
    println!("green.is_green_parts() {}", green.is_green_parts());

    println!("yellow.is_green() {}", yellow.is_green());
    println!("yellow.is_green_parts() {}", yellow.is_green_parts());

    print_color(Color::Red)
}
