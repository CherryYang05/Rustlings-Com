// TODO: Add the missing type of the argument `num` after the colon `:`.
fn main() {
    call_me(3);
}

fn call_me(num: usize) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
