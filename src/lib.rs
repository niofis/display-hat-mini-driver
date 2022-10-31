mod st7789;
use crate::st7789::{init_display, send_buffer, set_window};
use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::error::Error;

#[repr(u8)]
#[derive(Copy, Clone)]
enum Pins {
    SpiDc = 9,
    BackLight = 13,
    TearEfect = 25,
    ButtonA = 5,
    ButtonB = 6,
    ButtonX = 16,
    ButtonY = 24,
    LEDRed = 17,
    LEDGreen = 27,
    LEDBlue = 22,
}

impl Pins {
    pub fn u8(self) -> u8 {
        self as u8
    }
}

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

pub struct DisplayHATMini {
    backlight: OutputPin,
    data_pin: OutputPin,
    tear_effect_pin: InputPin,
    button_a_pin: InputPin,
    button_b_pin: InputPin,
    button_x_pin: InputPin,
    button_y_pin: InputPin,
    led_r_pin: OutputPin,
    led_g_pin: OutputPin,
    led_b_pin: OutputPin,
    use_vsync: bool,
    spi: Spi,
    pub width: u32,
    pub height: u32,
}

impl DisplayHATMini {
    pub fn new() -> Result<DisplayHATMini, Box<dyn Error>> {
        Ok(DisplayHATMini {
            backlight: Gpio::new()?.get(Pins::BackLight.u8())?.into_output(),
            data_pin: Gpio::new()?.get(Pins::SpiDc.u8())?.into_output(),
            tear_effect_pin: Gpio::new()?.get(Pins::TearEfect.u8())?.into_input_pulldown(),
            button_a_pin: Gpio::new()?.get(Pins::ButtonA.u8())?.into_input_pullup(),
            button_b_pin: Gpio::new()?.get(Pins::ButtonB.u8())?.into_input_pullup(),
            button_x_pin: Gpio::new()?.get(Pins::ButtonX.u8())?.into_input_pullup(),
            button_y_pin: Gpio::new()?.get(Pins::ButtonY.u8())?.into_input_pullup(),
            led_r_pin: Gpio::new()?.get(Pins::LEDRed.u8())?.into_output(),
            led_g_pin: Gpio::new()?.get(Pins::LEDGreen.u8())?.into_output(),
            led_b_pin: Gpio::new()?.get(Pins::LEDBlue.u8())?.into_output(),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss1, 90_000_000, Mode::Mode0)?,
            width: WIDTH,
            height: HEIGHT,
            use_vsync: false,
        })
    }

    pub fn init(&mut self) -> Result<(), Box<dyn Error>> {
        self.backlight.set_high();
        init_display(&mut self.spi, &mut self.data_pin, true)?;
        set_window(&mut self.spi, &mut self.data_pin, self.width, self.height)?;

        self.led_r_pin.set_high();
        self.led_g_pin.set_high();
        self.led_b_pin.set_high();
        
        Ok(())
    }

    fn display(&mut self, buffer: &[u8]) -> Result<(), Box<dyn Error>> {
        if self.use_vsync {
            while self.tear_effect_pin.read() == Level::Low {}
        }
        send_buffer(&mut self.spi, &mut self.data_pin, buffer)?;
        Ok(())
    }

    pub fn set_vsync(&mut self, use_it: bool) {
        self.use_vsync = use_it;
    }

    pub fn read_buttons(&mut self) -> Result<u8, Box<dyn Error>> {
        let as_u8 = |lvl: Level| if lvl == Level::Low { 1 as u8 } else { 0 as u8 };
        let buttons = as_u8(self.button_y_pin.read()) << 3 |
            as_u8(self.button_x_pin.read()) << 2 |
            as_u8(self.button_b_pin.read()) << 1 |
            as_u8(self.button_a_pin.read());
        Ok(buttons)
    }

    pub fn set_led(&mut self, red: u8, green: u8, blue: u8) -> Result<(), Box<dyn Error>> {
        let set_pin = |pin: &mut OutputPin, value: u8|  -> Result<(), Box<dyn Error>> {
            if value >= 255 {
                pin.clear_pwm()?;
                pin.set_low();
            } else if value > 0 {
                let v = value as f64 / 255.0;
                pin.set_pwm_frequency(2000.0, 1.0 - v)?;
            } else {
                pin.clear_pwm()?;
                pin.set_high();
            }
            Ok(())
        };
        
        set_pin(&mut self.led_r_pin, red)?;
        set_pin(&mut self.led_g_pin, green)?;
        set_pin(&mut self.led_b_pin, blue)?;

        Ok(())
    }

    pub fn display_rgb(&mut self, rgb_buffer: &[u8]) -> Result<(), Box<dyn Error>> {
        let count = (self.width * self.height) as usize;
        let mut rgb565: Vec<u8> = vec![0; count * 2];
        let mut offset = 0;
        let mut offset_out = 0;

        for _ in 0..count {
            let r = rgb_buffer[offset];
            let g = rgb_buffer[offset + 1];
            let b = rgb_buffer[offset + 2];
            let b1 = (r & 0xF8) | ((g & 0xE0) >> 5);
            let b2 = ((g & 0x1C) << 3) | ((b) >> 3);
            
            rgb565[offset_out] = b1;
            rgb565[offset_out + 1] = b2;

            offset += 3;
            offset_out += 2;
        }

        self.display(&rgb565)?;

        Ok(())
    }
}
