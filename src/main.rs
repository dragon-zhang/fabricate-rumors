use fabricate_rumors::set_jmp::{longjmp, setjmp, JmpBuf};
use std::cell::RefCell;

thread_local! {
    static JMP_BUF: Box<RefCell<JmpBuf>> = Box::new(RefCell::new(unsafe{std::mem::zeroed()}));
}

fn init_jmp_buf(buf: JmpBuf) {
    JMP_BUF.with(|boxed| {
        let _ = boxed.replace(buf);
    });
}

fn jmp_buf() -> Option<JmpBuf> {
    JMP_BUF.with(|boxed| {
        let buf = boxed.replace(unsafe { std::mem::zeroed() });
        if buf == unsafe { std::mem::zeroed::<JmpBuf>() } {
            None
        } else {
            Some(buf)
        }
    })
}

//arm64架构，有3个方法，m1,m2和m3，m1直接调用m2，要求在m2中用rust asm!更改LR来实现m2返回后只调用1次m3，要求在m3中用rust asm!更改LR来实现m3直接返回1作为m2的返回结果，禁止使用全局变量，要求rust版本为最新稳定版

//在my_func返回后调用m2，m2再跳回到my_func并返回
extern "C" fn my_func() {
    unsafe {
        let mut jmp_buf: JmpBuf = std::mem::zeroed();
        if setjmp(&mut jmp_buf) == 0 {
            init_jmp_buf(jmp_buf);
            println!("linked to m2");
            std::arch::asm!("mov lr, {}", in(reg) m2); // 将m2函数地址存储在变量中
            std::arch::asm!("ret", options(noreturn)); // 使用ret指令返回
            unreachable!("never happens");
        } else {
            println!("m2 returned");
        }
    }
}

#[no_mangle]
fn m2() {
    println!("m2");
    unsafe { longjmp(&mut jmp_buf().unwrap(), 1) }
}

//RUSTFLAGS="--emit asm" cargo build --release 获取优化后的汇编代码
fn main() {
    my_func();
}
