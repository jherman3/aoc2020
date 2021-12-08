use cached::proc_macro::cached;

#[cached]
fn foo(x: i32) -> i32 {
    let mut x = x;
    x += 1;
    x
}

fn main() {
    foo(3);
}