mod using_structs;

fn main() {
    using_structs::main();
    using_structs::User {
        _active: false,
        _username: "".to_string(),
        email: "".to_string(),
        _sign_in_count: 0,
    };
    println!("in main");
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple((width, height))
    );

    let rectangle = Rectangle {
        height: dbg!(height),
        width,
    };
    dbg!(&rectangle);
    println!(
        "The area of the rectangle {:?} is {} square pixels.",
        rectangle,
        area_struct(&rectangle)
    );
    println!(
        "The area of the rectangle {:#?} is {} square pixels.",
        rectangle,
        area_struct(&rectangle)
    );
    dbg!(rectangle.area());
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    dbg!(Rectangle::square(50));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

//method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn can_hold(&self, p0: &Rectangle) -> bool {
        p0.width < self.width && p0.height < self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// function
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
