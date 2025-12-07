type Meters = u32;
type Kilometers = u32;


fn main() {
    #[allow(unused_variables)]
    let distance_in_meters: Meters = 5000;
    #[allow(unused_variables)]
    let distance_in_kilometers: Kilometers = distance_in_meters / 1000;
}
