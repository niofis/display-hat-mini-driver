// ST7789 functions taken from:
// https://github.com/pimoroni/st7789-python/blob/3e63dc34042c0d12bf2950ed5b88d3a8dc060018/library/ST7789/__init__.py

use rppal::gpio::OutputPin;
use rppal::spi::Spi;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Commands {
    SWRESET = 0x01,
    MADCTL = 0x36,
    FRMCTR2 = 0xB2,
    COLMOD = 0x3A,
    GCTRL = 0xB7,
    VCOMS = 0xBB,
    LCMCTRL = 0xC0,
    VDVVRHEN = 0xC2,
    VRHS = 0xC3,
    VDVS = 0xC4,
    FRCTRL2 = 0xC6,
    GMCTRP1 = 0xE0,
    GMCTRN1 = 0xE1,
    INVON = 0x21,
    INVOFF = 0x20,
    SLPOUT = 0x11,
    DISPON = 0x29,
    CASET = 0x2A,
    RASET = 0x2B,
    RAMWR = 0x2C,
    TEON = 0x35,
    PWCTRL1 = 0xD0,
}

impl Commands {
    pub fn u8(self) -> u8 {
        self as u8
    }
}

pub fn send_buffer(
    spi: &mut Spi,
    data_pin: &mut OutputPin,
    buffer: &[u8],
) -> Result<(), Box<dyn Error>> {
    data_pin.set_high();
    const CHUNK: usize = 4096;
    let mut start = 0;
    let size = buffer.len();
    while start < size {
        let end = if start + CHUNK >= size {
            size
        } else {
            start + CHUNK
        };
        spi.write(&buffer[start..end])?;
        start += CHUNK;
    }
    Ok(())
}

fn send_data(spi: &mut Spi, data_pin: &mut OutputPin, data: u8) -> Result<(), Box<dyn Error>> {
    data_pin.set_high();
    spi.write(&[data])?;
    Ok(())
}

fn send_command(
    spi: &mut Spi,
    data_pin: &mut OutputPin,
    command: u8,
) -> Result<(), Box<dyn Error>> {
    data_pin.set_low();
    spi.write(&[command])?;
    Ok(())
}

pub fn set_window(
    spi: &mut Spi,
    data_pin: &mut OutputPin,
    width: u32,
    height: u32,
) -> Result<(), Box<dyn Error>> {
    send_command(spi, data_pin, Commands::CASET.u8())?;
    send_data(spi, data_pin, 0x00)?;
    send_data(spi, data_pin, 0x00)?;
    send_data(spi, data_pin, ((width - 1) >> 8) as u8)?;
    send_data(spi, data_pin, ((width - 1) & 0xFF) as u8)?;
    send_command(spi, data_pin, Commands::RASET.u8())?;
    send_data(spi, data_pin, 0)?;
    send_data(spi, data_pin, 0)?;
    send_data(spi, data_pin, ((height - 1) >> 8) as u8)?;
    send_data(spi, data_pin, ((height - 1) & 0xFF) as u8)?;
    send_command(spi, data_pin, Commands::RAMWR.u8())?;
    Ok(())
}

pub fn init_display(
    spi: &mut Spi,
    data_pin: &mut OutputPin,
    invert_display: bool,
) -> Result<(), Box<dyn Error>> {
    send_command(spi, data_pin, Commands::SWRESET.u8())?;
    thread::sleep(Duration::from_millis(150));

    send_command(spi, data_pin, Commands::MADCTL.u8())?;
    send_data(spi, data_pin, 0xB0)?;

    send_command(spi, data_pin, Commands::FRMCTR2.u8())?;
    send_data(spi, data_pin, 0x0C)?;
    send_data(spi, data_pin, 0x0C)?;
    send_data(spi, data_pin, 0x00)?;
    send_data(spi, data_pin, 0x33)?;
    send_data(spi, data_pin, 0x33)?;

    send_command(spi, data_pin, Commands::COLMOD.u8())?;
    send_data(spi, data_pin, 0x05)?;

    send_command(spi, data_pin, Commands::GCTRL.u8())?;
    send_data(spi, data_pin, 0x14)?;

    send_command(spi, data_pin, Commands::VCOMS.u8())?;
    send_data(spi, data_pin, 0x37)?;

    send_command(spi, data_pin, Commands::LCMCTRL.u8())?;
    send_data(spi, data_pin, 0x2C)?;

    send_command(spi, data_pin, Commands::VDVVRHEN.u8())?;
    send_data(spi, data_pin, 0x01)?;

    send_command(spi, data_pin, Commands::VRHS.u8())?;
    send_data(spi, data_pin, 0x12)?;

    send_command(spi, data_pin, Commands::VDVS.u8())?;
    send_data(spi, data_pin, 0x20)?;

    send_command(spi, data_pin, Commands::PWCTRL1.u8())?;
    send_data(spi, data_pin, 0xA4)?;
    send_data(spi, data_pin, 0xA1)?;

    send_command(spi, data_pin, Commands::FRCTRL2.u8())?;
    send_data(spi, data_pin, 0x0F)?; // 60Hz
                                     // send_data(spi, data_pin, 0x05)?; // 90Hz1Eh
                                     // send_data(spi, data_pin, 0x00)?; // 119Hz
                                     // send_data(spi, data_pin, 0x1F)?; // 39hz

    send_command(spi, data_pin, Commands::GMCTRP1.u8())?;
    send_data(spi, data_pin, 0xD0)?;
    send_data(spi, data_pin, 0x04)?;
    send_data(spi, data_pin, 0x0D)?;
    send_data(spi, data_pin, 0x11)?;
    send_data(spi, data_pin, 0x13)?;
    send_data(spi, data_pin, 0x2B)?;
    send_data(spi, data_pin, 0x3F)?;
    send_data(spi, data_pin, 0x54)?;
    send_data(spi, data_pin, 0x4C)?;
    send_data(spi, data_pin, 0x18)?;
    send_data(spi, data_pin, 0x0D)?;
    send_data(spi, data_pin, 0x0B)?;
    send_data(spi, data_pin, 0x1F)?;
    send_data(spi, data_pin, 0x23)?;

    send_command(spi, data_pin, Commands::GMCTRN1.u8())?;
    send_data(spi, data_pin, 0xD0)?;
    send_data(spi, data_pin, 0x04)?;
    send_data(spi, data_pin, 0x0C)?;
    send_data(spi, data_pin, 0x11)?;
    send_data(spi, data_pin, 0x13)?;
    send_data(spi, data_pin, 0x2C)?;
    send_data(spi, data_pin, 0x3F)?;
    send_data(spi, data_pin, 0x44)?;
    send_data(spi, data_pin, 0x51)?;
    send_data(spi, data_pin, 0x2F)?;
    send_data(spi, data_pin, 0x1F)?;
    send_data(spi, data_pin, 0x1F)?;
    send_data(spi, data_pin, 0x20)?;
    send_data(spi, data_pin, 0x23)?;

    if invert_display {
        send_command(spi, data_pin, Commands::INVON.u8())?;
    } else {
        send_command(spi, data_pin, Commands::INVOFF.u8())?;
    }

    send_command(spi, data_pin, Commands::SLPOUT.u8())?;
    send_command(spi, data_pin, Commands::TEON.u8())?;

    send_command(spi, data_pin, Commands::DISPON.u8())?;
    thread::sleep(Duration::from_millis(100));

    Ok(())
}
