#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn setHeight(&mut self, height: u32) -> () {
        self.height = height;
    }

    fn setWidth(&mut self, width: u32) -> () {
        self.width = width;
    }

    fn can_hold(&self, rect: &Rect) -> bool {
        self.height> rect.height && self.width > rect.width
    }

    fn square(size: u32) -> Rect {
        Rect {
            height: size,
            width: size
        }
    }
}


fn main() {
    let mut rectangle = Rect{height: 30, width: 50};
    let rect2 = Rect{height: 10, width: 20};
    let rect3 = Rect{height: 100, width: 5};
    rectangle.setHeight(20);
    println!(
        "The area of the rectangle {:?}  is {} square pixels.",
        &rectangle,
        rectangle.area()
    );

    println!("Can Rect 1 {:?} hold Rect 2 {:?}: {}", &rectangle, &rect2, rectangle.can_hold(&rect2));
    println!("Can Rect 1 {:?} hold Rect 3 {:?}: {}", &rectangle, &rect3, rectangle.can_hold(&rect3));

    let square = Rect::square(5);
    println!("Rect that is a square: {:?}", &square);
}


