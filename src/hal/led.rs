pub enum LEDColor {
    Off = 0x00,
    Green = 0x01,
    Red = 0x10,
    Orange = 0x11,
}

/// Restore system LED control.
pub fn xreset_led() {
    unsafe {
        nxdk_sys::hal::led::XResetLED();
    }
}

/// Set the LED to use a four step loop.
/// For more characteristics, please see https://xboxdevwiki.net/LED.
pub fn xset_custom_led(color1: LEDColor, color2: LEDColor, color3: LEDColor, color4: LEDColor) {
    unsafe {
        nxdk_sys::hal::led::XSetCustomLED(
            color1 as i32,
            color2 as i32,
            color3 as i32,
            color4 as i32,
        );
    }
}
