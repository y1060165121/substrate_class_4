fn main() {
   let light = TrafficLight::Red;
   println!("light is {}", light.time());
   let light = TrafficLight::Yellow;
   println!("light is {}", light.time());
   let light = TrafficLight::Green;
   println!("light is {}", light.time());
}

 enum TrafficLight {
        Red,
        Green,
        Yellow
    }

pub trait GetTime {
    fn time(&self) -> u8;
}

impl GetTime for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 70,
            TrafficLight::Yellow => 10,
            TrafficLight::Green => 60,
        }
    }
}
