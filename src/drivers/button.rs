//==============================================================================
// Notes
//==============================================================================
// drivers::button.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::gpio;
use super::led;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const PRESSED_STATE: gpio::PinState = gpio::PinState::PinLow;
const PUSH_BUTTON: gpio::PinConfig = gpio::PinConfig {
	pin: config::PUSH_BUTTON_PIN,
	direction: gpio::PinDirection::Input,
	pull: gpio::PinPull::PullUp,
	state: gpio::PinState::PinHigh
};

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(){
	gpio::pin_setup(&PUSH_BUTTON);

	unsafe { INITIALIZED = true };
}

#[allow(dead_code)]
pub fn get_button_state() -> bool {
	if gpio::get_pin_state(PUSH_BUTTON.pin) == PRESSED_STATE {
		true
	}
	else {
		false
	}
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
pub fn task_handler() {
	if get_button_state() {
		led::set_led_state(0, true);
	}
	else {
		led::set_led_state(0, false);
	}
}