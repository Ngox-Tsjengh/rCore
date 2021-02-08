


mod timer;
mod handler;
mod context;

pub fn init()
{
	println!("Beginnng of interrupt/mod.rs => init()");
	handler::init();
	println!("Interrupt initialized after interrupt/mod.rs => handler::init()");
	timer::init();
	println!("Timer initialized after interrupt/mod.rs => timer::init()");
	println!("End of interrupt/mod.rs => init()");
}
