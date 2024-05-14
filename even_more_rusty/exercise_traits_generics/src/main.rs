
fn main() {
    let mut car = Car { mpg: 12.4, color: String::from("Red"), top_speed: 100 };
    println!("base car {:?}", car);
    car.set_mpg(15.1);
    car.set_color(String::from("Green"));
    car.set_top_speed(120);
    println!("new car {:?}", car);
    
    let mut motorcycle = MotorCycle { mpg: 15.2, color: String::from("Green"), top_speed: 180};
    println!("Base mc: {:?}", motorcycle);
    motorcycle.set_mpg(18.8);
    motorcycle.set_color(String::from("Black"));
    motorcycle.set_top_speed(200);
    println!("Upgraded mc: {:?}", motorcycle);
    
    simple_print(motorcycle);
}

#[derive(Debug)]
struct Car {
    mpg: f32,
    color: String,
    top_speed: i16,
}

#[derive(Debug)]
struct MotorCycle {
    mpg: f32,
    color: String,
    top_speed: i16,
}

trait ModifyVehicle {
    fn set_mpg(&mut self, new_mpg: f32);
    fn set_color(&mut self, new_color: String);
    fn set_top_speed(&mut self, new_top_speed: i16);
}

impl ModifyVehicle for MotorCycle {
    fn set_mpg(&mut self, new_mpg: f32) {
        self.mpg = new_mpg
    }

    fn set_color(&mut self, new_color: String) {
        self.color = new_color
    }

    fn set_top_speed(&mut self, new_top_speed: i16) {
        self.top_speed = new_top_speed 
    }
}

impl ModifyVehicle for Car {
    fn set_mpg(&mut self, new_mpg: f32) {
        self.mpg = new_mpg
    }

    fn set_color(&mut self, new_color: String) {
        self.color = new_color
    }

    fn set_top_speed(&mut self, new_top_speed: i16) {
        self.top_speed = new_top_speed;
    }
}

fn simple_print<T: std::fmt::Debug>(item: T) {
    print!("Simple print says: {:?}", item);
}