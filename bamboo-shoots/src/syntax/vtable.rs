trait Shape {
    fn area(&self) -> f32;
}

struct Rectangle {
    w: f32,
    h: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}

struct Circle {
    r: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        std::f32::consts::PI * self.r * self.r
    }
}

fn total_area(list: &[&dyn Shape]) -> f32 {
    list.iter().map(|x| x.area()).fold(0f32, |a, b| a + b)
}
