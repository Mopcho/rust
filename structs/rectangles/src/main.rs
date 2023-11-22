struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return (2 * self.width) + (2 * self.heigth);
    }

    fn can_fit_rectangle(&self, rect: &Rectangle) -> bool {
        if self.width > rect.width && self.heigth > rect.heigth {
            return true;
        }

        return false;
    }
}


fn main() {
    let rec1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    let rec2 = Rectangle {
        width: 20,
        heigth: 30,
    };

    let area1 = rec1.area();

    let answer = rec1.can_fit_rectangle(&rec2);

    println!("{answer}");

    println!("Area of rec1 {area1}");
}
