#[no_mangle]
extern "C" fn pln(lr: u64) {
    println!("hello {}", lr);
}

extern "C" {
    //目标相当于在my_func返回后，在main函数调用pln
    fn my_func(new_lr: u64);
}

// extern "C" fn my_func(new_lr: u64) {
//     pln(new_lr);
// }

//RUSTFLAGS="--emit asm" cargo build --release 获取优化后的汇编代码
fn main() {
    unsafe { my_func(pln as u64) };
}
