// Global variables need to be declared with the static or const keyword
// Constant variables are not just immutable by default, they're _only_ immutable.
const GLOBAL_VAR: u32 = 12;

fn main() {
    println!("The value of GLOBAL_VAR is: {}", GLOBAL_VAR);

    let immutable_var = "version 1";

    // The following is not going to work because variables are immutable
    // by default unless marked otherwise with the mut keyword
    // x = "version 2";
    println!("The value of immutable_var is: {}", immutable_var);

    let mut mutable_var = "version 1";
    println!("The value of mutable_var is: {}", mutable_var);
    mutable_var = "version 2";
    println!("The value of mutable_var is: {}", mutable_var);

    // Variable shadowing
    let shadowed_var = 1;
    let shadowed_var = shadowed_var * 2;
    println!("The value of shadowed_var is: {}", shadowed_var);
}
