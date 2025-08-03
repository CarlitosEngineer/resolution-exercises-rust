// CODIGO PARA EJECUTAR EN EXERCISM

// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

// CODIGO PARA PROBAR EL CODIGO LOCALMENTE
fn main() {
    let msg: &'static str = hello();
    println!("{}", msg);
}

// cargo run --bin hello_world
    