enum Location {
    Africa,
    America,
    Asia,
    Europe,
    Oceania,
}

fn main() {
    let loca = Location::Asia;
    if let Location::Asia = loca {
        println!("Asia.")
    }
}