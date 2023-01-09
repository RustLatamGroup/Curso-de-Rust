mod shapes;

fn main() {
    let rect = shapes::Rectangle::new(1.0, 2.0);
    let area = rect.get_feature(shapes::Feature::Area);
    println!("rect area is {}", area);

    let circ = shapes::Circle::new(3.0);
    let perimeter = circ.get_feature(shapes::Feature::Perimeter);
    println!("circ perimeter is {}", perimeter);
}
