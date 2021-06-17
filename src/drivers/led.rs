//==============================================================================
// Notes
//==============================================================================
// drivers::led.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::config;
use crate::mcu::gpio;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================


//==============================================================================
// Variables
//==============================================================================
const LED_OFF_STATE: gpio::PinState = gpio::PinState::PinLow;
const LED_ON_STATE: gpio::PinState = gpio::PinState::PinHigh;
const LEDS_COUNT: usize = 2;
const LEDS: [gpio::PinConfig; 2] = [
	gpio::PinConfig {
		pin: config::LED_1_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow
	},
	gpio::PinConfig {
		pin: config::LED_2_PIN,
		direction: gpio::PinDirection::Output,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow
	}
];

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(){
	for led in LEDS.iter() {
		gpio::pin_setup(&led);
	}

	unsafe { INITIALIZED = true };
}

#[allow(dead_code)]
pub fn set_led_state(led: usize, state: bool) {
	if led >= LEDS_COUNT {
		return;
	}

	let pin_state: gpio::PinState = if state {
		LED_ON_STATE
	}
	else {
		LED_OFF_STATE
	};

	gpio::set_pin_state(LEDS[led].pin, pin_state);
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

}