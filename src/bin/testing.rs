use time::{Duration, PrimitiveDateTime};

// Es una bibliote que combina en un tipo de varaible la fecha y la hora
// 

// Returns a DateTime one billion seconds after start.
pub fn after(start: PrimitiveDateTime) -> PrimitiveDateTime {
    const GIGASECOND: i64 = 1_000_000_000;
    let mut epale = start + Duration::seconds(GIGASECOND)
    println!("{}", Duration);
    println!("{}", start);
    println!("{}", epale);
}

fn main (){
    after()
}

// cargo run --bin testing