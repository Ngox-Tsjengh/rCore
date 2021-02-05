#![feature(global_asm)]

use	super::context::Context;
use riscv::register::{scause::{Scause},stvec,};



global_asm!(include_str!("./interrupt.asm"));

pub fn init()
{
	println!("In the begin of handler.rs => init()");
	unsafe{
		extern "C"{
			fn __interrupt();
		}
		stvec::write(__interrupt as usize,stvec::TrapMode::Direct);
	}
	println!("In the end of handler.rs => init()");
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context,scause:Scause,stval:usize){
	println!("Entered handle_interrupt");
	panic!("Interrupted: {:?}",scause.cause());
}
