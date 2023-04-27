use std::mem::MaybeUninit;

fn testtest() -> (i32, i32) {
    let mut x: (MaybeUninit<i32>, MaybeUninit<i32>) =
        (MaybeUninit::uninit(), MaybeUninit::uninit());
    x.0.write(1);
    x.1.write(2);
    unsafe { (x.0.assume_init(), x.1.assume_init()) }
}

fn testtest2() -> (i32, i32) {
    let x0: i32;
    let x1: i32;

    x0 = 1;
    x1 = 2;

    (x0, x1)
}

// fn testtest3() -> (i32, i32) {
//     let x: (i32, i32);
//
//     x.0 = 1;
//     x.1 = 2;
//
//     x
// }

// fn testtest4() {
//     let x: ();
//     x
// }

fn testtest5() {
    let x: i32;
}

fn main() {}
