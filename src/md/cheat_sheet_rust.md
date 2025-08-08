# Rust Cheat Sheet - Language

## 🎯 VARIABLES & CONSTANTES

---
```rust
const VARIABLE_CONST1: u32 = 100; // [ NOT MUTABLE , UNSIGNED]
let variable_1 = -2147483648; // [ NOT MUTABLE , INFERIDO(INPLICITO), 32BITS POR DEFECTO]
let mut variable_2: i64 = -9223372036854775808; // [ MUTABLE, NOT INFERIDO(EXPLICITO) ].
let variable_3 = -20i16; // [ SUFIJO ]
```

## 🎯 DATA TYPES

##### ✅ INTEGERS (UNSIGNED)

---
| Tipo    | Rango                                                   |
| ------- | ------------------------------------------------------- |
| `u8`    | 0 a 255                                                 |
| `u16`   | 0 a 65,535                                              |
| `u32`   | 0 a 4,294,967,295                                       |
| `u64`   | 0 a 18,446,744,073,709,551,615                          |
| `u128`  | 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455 |
| `usize` | Depende del sistema: `u32` , `u64`                      |

##### ✅ INTEGERS (SIGNED)

---
| Tipo    | Rango                                                                                                      |
| ------- | ---------------------------------------------------------------------------------------------------------- |
| `i8`    | -128 a 127                                                                                                 |
| `i16`   | -32,768 a 32,767                                                                                           |
| `i32`   | -2,147,483,648 a 2,147,483,647                                                                             |
| `i64`   | -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807                                                     |
| `i128`  | -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727 |
| `usize` | Depende del sistema: `i32` , `i64`                                                                         |

##### ✅ FLOATS
---
| Tipo  | Bits | Precisión       | Rango aprox.                    |
| ----- | ---- | --------------- | ------------------------------- |
| `f32` | 32   | \~7 dígitos     | ±1.175494e-38 a ±3.402823e+38   |
| `f64` | 64   | \~15-17 dígitos | ±2.225073e-308 a ±1.797693e+308 |

```rust
    let variable_u1: u8 = 255;
    let variable_i1: i8 = -128;
    let variable_8 = 100u8; // SUFIJO VALOR:100 DE TIPO:u8
    let variable_9 = -20i16;
    let variable_7 = 42; // Default: i32
```

##### ✅ BOOLEANOS
---
* Un booleano es un tipo de dato que solo puede tener dos valores. { true , false }
    + Comparaciones (==, !=, <, >)
    + Condicionales (if, match)
    + Lógica (&&, ||, !)
```rust
    let a: bool = true; // tipo explícita
    let b = false; // anotación inferido
```

##### ✅ CHARACTERS & STRINGS

| Característica           | `String`                                         | `&str`                                                       |
| ------------------------ | ------------------------------------------------ | ------------------------------------------------------------ |
| **Tipo de dato**         | Tipo **propietario** (`owned`)                   | Tipo **prestado** (`borrowed`)                               |
| **Ubicación en memoria** | Vive en el **heap**                              | Generalmente vive en el **stack** o es una referencia a heap |
| **Mutable**              | Sí, si se declara `mut`                          | No                                                           |
| **Tamaño dinámico**      | Sí, puede crecer o modificarse                   | No, es de tamaño fijo                                        |
| **Conversión**           | Se puede convertir fácilmente desde `&str`       | Se obtiene fácilmente de un `String` como `&my_string`       |
| **Uso típico**           | Cuando necesitas **construir o modificar texto** | Cuando solo **lees texto inmutable**, como literales         |

---
```rust
let s1: String = String::from("Hola mundo"); // String: dueño de la cadena y es (UTF-8)
let s2: &str = "Hola mundo";                 // &str: slice estático (literal)
let s3: &str = &s1; // Podemos obtener un &str desde un String
let mut string2 = "Rust".to_string(); // using to_string and making it mutable
let value_c1: char = 'A'; // letra mayúscula
```

##### ✅ TUPLES
---
```rust
let tuple1: (i32, f64, u8) = (500, 6.4, 1);
let tuple2: (char, bool, f32) = ('R', true, 3.14);
let tuple1: (i32, f64, u8) = (500, 6.4, 1);
let tuple2: (char, bool, f32) = ('R', true, 3.14);
// Desestructuración (extraemos los valores de la tupla)
let (x, y, z) = tuple1;
let (a, b, c) = tuple2;
```

##### ✅ ARRAYS
---
* Colección de tamaño fijo
* Elementos del mismo tipo
* Se almacenan en la pila (stack)
* Usa array si el tamaño es fijo.
```rust
    // Declarando arrays de enteros
    let array1: [i32; 5] = [1, 2, 3, 4, 5]; // Sintaxis para Tipo y Longitud
    let array2: [i32; 5] = [0; 5]; // Inicializar con el Mismo Valor {// [3, 3, 3, 3, 3]}
    let primero = array1[0]; // acceder a un arreglo
    println!("testing_datatypes_array - array1: {:?}", array1);
```

##### ✅ VECTOR
---
```rust
```

```rust
```

```rust
```

```rust
```

```rust
```
