//==============================================================================
// Notes
//==============================================================================
// mcu::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use cortex_m;
use cc2640r2f;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
// pub enum McuState {
// 	Idle
// }

// #[allow(dead_code)]
// pub struct SystemClock{
// 	pub a_clk: u32,
// 	pub m_clk: u32,
// 	pub hsm_clk: u32,
// 	pub sm_clk: u32,
// 	pub b_clk: u32
// }

//==============================================================================
// Variables
//==============================================================================
// #[allow(dead_code)]
// #[derive(Copy, Clone, PartialEq)]
// pub enum Port{
// 	Port1,
// 	Port2,
// 	Port3,
// 	Port4,
// 	Port5,
// 	Port6,
// 	Port7,
// 	Port8,
// 	Port9,
// 	Port10,
// 	PortJ
// }

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	let peripherals = cc2640r2f::Peripherals::take().unwrap();
	// let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

	// let systick = cortex_peripherals.SYST;
	
	// systick::init(cortex_peripherals.SYST);

	// Disable the watchdog (indefinitely, for now)
	// peripherals.WDT_A.wdtctl.write(|w| unsafe { w
	// 	.wdtpw().bits(0x5A)
	// 	.wdthold().wdthold_1()
	// 	.wdtis().wdtis_0()
	// });

	// init_clock(peripherals.CS);
	
	// gpio::init(peripherals.DIO);
}

// #[allow(dead_code)]
// pub fn get_busy() -> McuState {
// 	McuState::Idle
// }

#[allow(dead_code)]
pub fn restart() {
	cortex_m::peripheral::SCB::sys_reset();
}

//==============================================================================
// Private Functions
//==============================================================================
// fn init_clock(clock: msp432p401r::CS) {
// 	clock.cskey.write(|w| unsafe { w.cskey().bits(0x695A) });


// }

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}