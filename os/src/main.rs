#![no_std]
#![no_main]
#![feature(panic_info_message)]
// 否则： error[E0658]: use of unstable library feature 'panic_info_message'

#[macro_use]
mod console;
mod lang_items;
mod sbi;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}


// #[no_mangle]
// extern "C" fn _start() {
//   sbi::shutdown();
// }
// 因为下面汇编里定义了 _start 并且call rust_main，所以不能重复，否则会
// error: symbol '_start' is already defined

core::arch::global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

