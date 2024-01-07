mod shapes;

use crate::shapes::{rect::Rect, circle::Circle, area::Area, collisions::Collidable};

fn main() {
    let rect1 = Rect::default();
    let rect2 = Rect::default();

    let circ1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };
    let circ2 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 5.0,
    };

    println!("rect1.collide(&rect2) {}", rect1.collide(&rect2));
    println!("rect1.collide(&circ2) {}", rect1.collide(&circ2));
    println!("circ1.collide(&circ2) {}", circ1.collide(&circ2));
}
