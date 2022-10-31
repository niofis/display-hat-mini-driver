# display-hat-mini-driver

This is a very simple basic implementation of a driver for the [Pimoroni Display HAT Mini](https://shop.pimoroni.com/products/display-hat-mini?variant=39496084717651) ([pinout](https://pinout.xyz/pinout/display_hat_mini#)).

Implemented features:

- Bitmap drawing
- Tear effect pin
- Button states
- RGB Led

You can also use this driver from [node.js](https://nodejs.org), using this library: [display-hat-mini-node](https://github.com/niofis/display-hat-mini-node). 

## Quick example

```rust
use display_hat_mini_driver::DisplayHATMini;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dhm = DisplayHATMini::new()?;
    let width = dhm.width;
    let height = dhm.height;
    let bpp = 3;

    dhm.init()?;

    let mut buffer: Vec<u8> = vec![0; (width * height * bpp) as usize];

    for y in 0..height {
        for x in 0..width {
            let offset= ((y * width + x) * bpp) as usize;
            buffer[offset] = y as u8; //red
            buffer[offset + 1] = x as u8; //green
            buffer[offset + 2] = 0; //blue;
        }
    }

    dhm.display_rgb(&buffer[..])?;
    Ok(())
}
```

## Features

### Bitmap drawing

```rust
display_rgb(&mut self, rgb_buffer: &[u8]) -> Result<(), Box<dyn Error>>
```

This function draws the entire bitmap to the display, the buffer is expected to have a 320x240 pixels bitmap where each pixel has the format RGB24.

### Tear effect pin

```rust
set_vsync(&mut self, use_it: bool)
```

Call this with true if you want to wait for the tear effect pin to go high before drawing the bitmap to the display. It does not eliminate all tearing though.

### Button states

```rust
read_buttons(&mut self) -> Result<u8, Box<dyn Error>>
```

This function returns a byte with flags for pressed buttons. The lower nibble indicates the estate for the four buttons of the display.

|bits|7|6|5|4|3|2|1|0|
|------|-|-|-|-|-|-|-|-|
|buttons|-|-|-|-|Y|X|B|A|

For example, the byte 0x08 means that button Y is currently pressed.

### RGB LED

```rust
set_led(&mut self, red: u8, green: u8, blue: u8) -> Result<(), Box<dyn Error>>
```

Simply call this function to show the RGB in the integrated RGB LED.

