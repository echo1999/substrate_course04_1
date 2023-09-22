enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Duration {
    fn time_duration(&self) -> u32;  
}

impl Duration for TrafficLight {
    fn time_duration(&self) -> u32 {
        match *self {
            TrafficLight::Red => 60,    // 红灯持续60秒
            TrafficLight::Yellow => 5,  // 黄灯持续5秒
            TrafficLight::Green => 30,  // 绿灯持续30秒
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red;
    let yellow_light = TrafficLight::Yellow;
    let green_light = TrafficLight::Green;
    
    println!("Red light duration: {} seconds", red_light.time_duration());
    println!("Yellow light duration: {} seconds", yellow_light.time_duration());
    println!("Green light duration: {} seconds", green_light.time_duration());
}
