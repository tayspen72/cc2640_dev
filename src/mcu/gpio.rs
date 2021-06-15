//==============================================================================
// Notes
//==============================================================================
// mcu::gpio.rs
// Basic control over gpio pins

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use cc2640r2f;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinDirection{
	Input = 0,
	Output = 1
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinPull{
	PullDown = 1,
	PullUp = 2,
	PullDisabled =3
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinState{
	PinLow = 0,
	PinHigh = 1
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PinConfig{
	pub pin: u8,
	pub direction: PinDirection,
	pub pull: PinPull,
	pub state: PinState,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq)]
pub enum PortId{
	RFC_SMI_CL_IN = 56,
	RFC_SMI_CL_OUT = 55,
	RFC_SMI_DL_IN = 54,
	RFC_SMI_DL_OUT = 53,
	RFC_GPI1 = 52,
	RFC_GPI0 = 51,
	RFC_GPO3 = 50,
	RFC_GPO2 = 49,
	RFC_GPO1 = 48,
	RFC_GPO0 = 47,
	RFC_TRC = 46,
	I2S_MCLK = 41,
	I2S_BCLK = 40,
	I2S_WCLK = 39,
	I2S_AD1 = 38,
	I2S_AD0 = 37,
	SSI1_CLK = 36,
	SSI1_FSS = 35,
	SSI1_TX = 34,
	SSI1_RX = 33,
	CPU_SWV = 32,
	PORT_EVENT7 = 30,
	PORT_EVENT6 = 29,
	PORT_EVENT5 = 28,
	PORT_EVENT4 = 27,
	PORT_EVENT3 = 26,
	PORT_EVENT2 = 25,
	PORT_EVENT1 = 24,
	PORT_EVENT0 = 23,
	UART0_RTS = 18,
	UART0_CTS = 17,
	UART0_TX = 16,
	UART0_RX = 15,
	I2C_MSSCL = 14,
	I2C_MSSDA = 13,
	SSI0_CLK = 12,
	SSI0_FSS = 11,
	SSI0_TX = 10,
	SSI0_RX = 9,
	AUX_IO = 8,
	AON_CLK32K = 7,
	GPIO = 0,
}

//==============================================================================
// Variables
//==============================================================================
static GPIO_HANDLE: Mutex<RefCell<Option<cc2640r2f::GPIO>>> = 
	Mutex::new(RefCell::new(None));
static IOC_HANDLE: Mutex<RefCell<Option<cc2640r2f::IOC>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(gpio: cc2640r2f::GPIO, ioc: cc2640r2f::IOC){
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| GPIO_HANDLE.borrow(cs).replace(Some(gpio)));
	free(|cs| IOC_HANDLE.borrow(cs).replace(Some(ioc)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn get_pin_state(pin: u8) -> PinState {
	unsafe { if !INITIALIZED {
		return PinState::PinLow;
	}}
	
	let read = free(|cs|
		if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			gpio.din31_0.read().bits()
		}
		else {
			0
		}
	);
	
	match read & (1 << pin) {
		0 => PinState::PinLow,
		_ => PinState::PinHigh
	}
}

// #[allow(dead_code)]
// pub fn pin_disable(_config: &PinConfig) {
// 	unsafe { if !INITIALIZED {
// 		return;
// 	}}
// }

#[allow(dead_code)]
pub fn pin_setup(config: &PinConfig){
	unsafe { if !INITIALIZED {
		return;
	}}
	
	// Set input pin config
	free(|cs| {
		if let Some(ref mut ioc) = IOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			match config.pin {
				0 => {
					ioc.iocfg0.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				1 => {
					ioc.iocfg1.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				2 => {
					ioc.iocfg2.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				3 => {
					ioc.iocfg3.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				4 => {
					ioc.iocfg4.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				5 => {
					ioc.iocfg5.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				6 => {
					ioc.iocfg6.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				7 => {
					ioc.iocfg7.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				8 => {
					ioc.iocfg8.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				9 => {
					ioc.iocfg9.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				10 => {
					ioc.iocfg10.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				11 => {
					ioc.iocfg11.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				12 => {
					ioc.iocfg12.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				13 => {
					ioc.iocfg13.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				14 => {
					ioc.iocfg14.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				15 => {
					ioc.iocfg15.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				16 => {
					ioc.iocfg16.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				17 => {
					ioc.iocfg17.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				18 => {
					ioc.iocfg18.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				19 => {
					ioc.iocfg19.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				20 => {
					ioc.iocfg20.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				21 => {
					ioc.iocfg21.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				22 => {
					ioc.iocfg22.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				23 => {
					ioc.iocfg23.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				24 => {
					ioc.iocfg24.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				25 => {
					ioc.iocfg25.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				26 => {
					ioc.iocfg26.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				27 => {
					ioc.iocfg27.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				28 => {
					ioc.iocfg28.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				29 => {
					ioc.iocfg29.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				30 => {
					ioc.iocfg30.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				31 => {
					ioc.iocfg31.write(|w| unsafe { w
						.ie().bit(config.direction == PinDirection::Input)
						.pull_ctl().bits(config.pull as u8) }
					);
				},
				_ => ()
			}
		}
	});

	// Set output pin enable
	free(|cs| {
		if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if config.pin < 31 {
				let mut val = gpio.doe31_0.read().bits();
				if config.direction == PinDirection::Input {
					val = val & !(1 << config.pin)
				}
				else {
					val = val | (1 << config.pin);
				}
				gpio.doe31_0.write(|w| unsafe { w.bits(val) });
			}
		}
	});

	// Set the port id
	set_pin_port_id(config.pin, PortId::GPIO);
}

#[allow(dead_code)]
pub fn set_pin_state(pin: u8, state: PinState){
	unsafe { if !INITIALIZED {
		return;
	}}
	
	free(|cs| {
		if let Some(ref mut gpio) = GPIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if let PinState::PinLow = state {
				gpio.doutclr31_0.write(|w| unsafe { w.bits( 1 << pin) });
			}
			else {
				gpio.doutset31_0.write(|w| unsafe { w.bits( 1 << pin) });
			}
		}
	});
}

#[allow(dead_code)]
pub fn set_pin_port_id(pin: u8, id: PortId){
	unsafe { if !INITIALIZED {
		return;
	}}

	free(|cs| {
		if let Some(ref mut ioc) = IOC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			match pin {
				0 => ioc.iocfg0.modify(|_, w| unsafe { w.port_id().bits(id as u8) }),
				// TODO: Finish this copy paste
				_ => (),
			}
		}
	});
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
