use riscv::register::{sstatus::Sstatus, scause::Scause};


#[repr(C)]
#[derive(Debug)]
pub struct Context
{
	pub x:[usize; 32],
	pub sstatus: Sstatus,
	pub spec:usize
}
