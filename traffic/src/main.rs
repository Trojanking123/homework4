enum TrafficLight {
    Red,
    Green,
    Yellow
}

trait WaitTime {
        fn wait_time(self) -> u32 ;
}


impl WaitTime for TrafficLight {
    fn wait_time(self) -> u32 {
        match self {
            TrafficLight::Green => 30,
            TrafficLight::Red => 40,
            TrafficLight::Yellow => 50,
        }
    }
}


fn main() {
    let g = TrafficLight::Green;
    let r = TrafficLight::Red;
    let y = TrafficLight::Yellow;
    println!("wait time: {} {} {}", r.wait_time(), g.wait_time(), y.wait_time());
}
