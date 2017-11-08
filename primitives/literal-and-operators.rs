fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;

    let default_float = 5i32;
    let default_integer = 5i32;

    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error!
    // mutable = true;

    let mutable = true;
}
