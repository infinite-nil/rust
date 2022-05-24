fn main() {
    let mut x = 5;
    println!("Variables, {}!", x);

    x = 6;
    println!("Reassign variables, {}!", x);

    const COUNT: u32 = 1_000;
    println!("Constants, {}!", COUNT);

    let y = "String";
    println!("y, {}!", y);

    let y = 10;
    println!("Shadowing y, {}!", y);

    // Scalar types
    // ----------------------------------------------------

    // Integers
    let a = 10_000; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)

    println!("Int: {}, {}, {}, {}, {}", a, b, c, d, e);

    //   - Floating point numbers
    let f = 2.0; // f64 default
    let g: f32 = 3.0; // f32

    println!("Float: {}, {}", f, g);

    //   - Bool
    let h = true;
    let i = false;

    println!("Bool: {}, {}", h, i);

    //   - Chars
    let j = 'a';
    let k = 'A';
    let l = '😍';

    println!("Chars: {}, {}, {}", j, k, l);

    // Compound types
    // ----------------------------------------------------

    // Tuples
    let tuple = ("Some string", 100);
    let (ta, tb) = tuple;
    let tc = tuple.0;
    let td = tuple.1;

    println!("Tuple: {:?}, {}, {}, {}, {}", tuple, ta, tb, tc, td);

    // Arrays
    let array = [1, 2, 3];

    println!("Array: {:?}", array);

    // Functions
    let sum = custom_fn(10, 5);

    println!("Function sum: {}", sum);

    // Control flow Loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("Loop, {}", result);

    // Control flow while
    let mut counter = 5;

    while counter != 0 {
        counter -= 1;
    }

    println!("While, {}", counter);

    // Control flow for in
    let counter = [10, 20, 30, 40, 50];

    for element in counter.iter() {
        println!("For, {}", element);
    }

    // Control flow for in range

    for element in 1..10 {
        println!("For, {}", element);
    }
}

fn custom_fn(x: i32, y: i32) -> i32 {
    println!("X: {}", x);
    println!("Y: {}", y);

    x + y
}
