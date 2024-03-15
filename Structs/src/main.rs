#[derive(Debug)]
struct Disc {
    x: f32,
    y: f32,
    radius: f32,
}

impl Disc {
    //Declare the scale method
    pub fn resize(&mut self, scale: f32) {
        //Write the code that implments the scale method
        self.radius *= scale;
    }
    pub fn offset(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }
    pub fn intersects_with(&self, other: &Disc) -> bool {
        return ((self.x-other.x).powf(2.0)+(self.y-other.y).powf(2.0)).powf(0.5) <= self.radius+other.radius;
    }
}

fn main() {
    let mut disc1: Disc = Disc {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    let mut disc2: Disc = Disc {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    let mut disc3: Disc = Disc {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    println!("disc1: {:?}", disc1);
    println!("disc2: {:?}", disc2);
    println!("disc3: {:?}", disc3);

    disc1.x += 5.0;
    println!("disc1: {:?}", disc1);

    disc1.resize(5.0);
    println!("disc1: {:?}", disc1);

    disc1.offset(-5.0, 5.0);
    println!("disc1: {:?}", disc1);

    let mut d1 = Disc{ x: 0.0, y: 0.0, radius:1.0 };
    let mut d2 = Disc{ x: 2.0, y: 0.0, radius:1.0 };
    println!("{:?} and {:?} intersect: {}", d1, d2, d1.intersects_with(&d2) );

    let mut d1 = Disc{ x: 0.0, y: 0.0, radius:1.0 };
    let mut d2 = Disc{ x: 2.0, y: 0.0, radius:2.0 };
    println!("{:?} and {:?} intersect: {}", d1, d2, d1.intersects_with(&d2) );

    let mut d1 = Disc{ x: 0.0, y: 0.0, radius:1.0 };
    let mut d2 = Disc{ x: 5.0, y: 0.0, radius:1.0 };
    println!("{:?} and {:?} intersect: {}", d1, d2, d1.intersects_with(&d2) );

}
