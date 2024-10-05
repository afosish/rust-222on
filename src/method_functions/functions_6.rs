#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method
impl TrafficLightColor {
    pub fn color(&self) -> &str {
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

#[test]
fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}
