use serde::{Serialize, Deserialize};

struct SimpleNode {
    name: String,
    track_0: String,
    track_1: String,
}

struct EndNode {
    name: String,
    track: String,
}

struct PointsNode {
    name: String,
    facing_track: String,
    normal_track: String,
    reverse_track: String,
}

pub trait Node{

}

struct Track {
    name: String,
    length: f64,
}

fn main() {
    // let n1 = EndNode {
    //     name: String::from("E001"), 
    //     track: String::from("T001")
    // };
    // let n2 = PointsNode {
    //     name: String::from("P001"),
    //     facing_track: String::from("T001"),
    //     normal_track: String::from("T002"),
    //     reverse_track: String::from("T003"),
    // };
    // let n3 = SimpleNode {
    //     name: String::from("N001"),
    //     track_0: String::from("T001"),
    //     track_1: String::from("T002"),
    // };
    // let t1 = Track {
    //     name: String::from("T001"), 
    //     length: 10.0,
    // };
    println!("Hello, world!");
}
