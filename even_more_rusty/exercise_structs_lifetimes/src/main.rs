
fn main() {
    let mut car = Car { mpg: 12.4, color: String::from("Red"), top_speed: 100 };
    println!("base car {:?}", car);
    car.set_mpg(15.1);
    car.set_color(String::from("Green"));
    car.set_top_speed(120);
    println!("new car {:?}", car);
    let fn_car = update_car(&mut car);
    println!("{:?}", fn_car);
}

#[derive(Debug)]
struct Car {
    mpg: f32,
    color: String,
    top_speed: i8,
}

impl Car {
    fn set_mpg(&mut self, new_mpg: f32) {
        self.mpg = new_mpg
    }
    
    fn set_color(&mut self, new_color: String) {
        self.color = new_color
    }
    
    fn set_top_speed(&mut self, new_top_speed: i8) {
        self.top_speed = new_top_speed;
    }
}

fn update_car(c: &mut Car) -> &mut Car {
    c.set_mpg(15.8);
    c.set_color(String::from("Teal"));
    c.set_top_speed(125);
    c
}
