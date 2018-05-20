fn main() {
    let _logical: bool = true;

    let _a_float: f64 = 1.0;
    let _an_integer = 5i32;

    let _default_float = 5i32;
    let _default_integer = 5i32;

    let mut _inferred_type = 12; // Type i64 is inferred from another line
    _inferred_type = 4294967296i64;

    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;

    // Error!
    // mutable = true;

    let _mutable = true;
}
