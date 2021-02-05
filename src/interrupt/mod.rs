



mod handler;
mod context;

pub fn init()
{
	println!("In the begin of mod.rs => init()");
	handler::init();
	println!("mod interrupt initialized!");
	println!("In the end of mod.rs => init()");
}
