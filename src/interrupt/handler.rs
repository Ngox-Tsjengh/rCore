use	super::context::Context;
use super::timer;
use riscv::register::{scause::{Exception,Interrupt,Scause,Trap},
			stvec,sie,
};

global_asm!(include_str!("./interrupt.asm"));

pub fn init()
{
	println!("Beginning of interrupt/handler.rs => init()");
	unsafe{
		extern "C"{
			fn __interrupt();
		}
		stvec::write(__interrupt as usize,stvec::TrapMode::Direct);
	
		//sie::set_sext();
	}
	println!("End of interrupt/handler.rs => init()");
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context,scause:Scause,stval:usize){
	println!("Entered interrupt/handler.rs => handle_interrupt()");
	match scause.cause()
	{
		Trap::Exception(Exception::Breakpoint) => breakpoint(context),
		Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
		_ => fault(context,scause,stval),
	}
}

fn breakpoint(context: &mut Context)
{
	println!("Breakpoint at 0x{:x}",context.sepc);
	context.sepc += 2;
}

fn supervisor_timer(_: &Context)
{
	timer::tick();
}

fn fault(context: &mut Context, scause: Scause, stval: usize)
{
	panic!( 
		"Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
		scause.cause(),
		context,
		stval
	);
}
