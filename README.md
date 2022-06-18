# ddc-enhanced-rs

`ddc-enhanced-rs` is a cross platform Rust crate and Node package for controlling monitors with [DDC/CI](https://en.wikipedia.org/wiki/Display_Data_Channel).

## Documentation

Rust: https://docs.rs/ddc-enhanced-rs

NodeJS: https://github.com/ThalusA/ddc-enhanced-rs/blob/master/index.d.ts

## Examples

### Rust
```rust
use ddc_enhanced_rs::{get_enhanced_displays, get_brightness, set_brightness};

// index in array is the id of the display
fn main() -> Result<(), std::io::Error> {
    for id in 0..get_enhanced_displays()?.len() {
        let value = get_brightness(id)?.value();
        set_brightness(id, value + 2).unwrap();
    }
    Ok(())
}
```

### NodeJS
```javascript
import Display from "ddc-enhanced-rs";

for (const display_info of Display.list()) {
    const display = new Display(display_info.id);
    const { value } = display.brightness;
    display.set_brightness(value + 2);
}
```
