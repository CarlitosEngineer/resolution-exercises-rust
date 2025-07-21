pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let palabra = "epale"; // tipo: &str
    let resultado = reverse(palabra); // pasa &str directamente
    println!("{}", resultado);
}

// cargo run --bin Reverse String
