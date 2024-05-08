fn main() {
    let tri = Shape::Triangle;
    let sq = Shape::Square;
    let pen = Shape::Pentagon;
    let oct = Shape::Octagon;
    println!("{}", tri.corners());
    println!("{}", sq.corners());
    println!("{}", pen.corners());
    println!("{}", oct.corners());
}

#[derive(Debug)]
enum Shape {
    Triangle,
    Square, 
    Pentagon, 
    Octagon,
}

impl Shape {
    fn corners(self) -> i8 {
        match self { 
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
        }
    }
}
