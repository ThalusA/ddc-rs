# ddc-enhanced-rs

`ddc-enhanced-rs` is a cross platform Rust crate and a Node package for controlling monitors with [DDC/CI](https://en.wikipedia.org/wiki/Display_Data_Channel).

## Documentation

Rust: https://docs.rs/ddc-enhanced-rs

NodeJS: https://github.com/ThalusA/ddc-enhanced-rs/blob/master/index.d.ts

## Examples

```rust
use ddc_enhanced_rs::EnhancedDisplay;

fn main() {
  for display_info in EnhancedDisplay::list_infos() {
    let mut display = EnhancedDisplay::get(display_info.id).unwrap();
    let value = display.get_brightness().unwrap().value();
    display.set_brightness(value + 2).unwrap();
  }
}
```

```javascript
import Display from "ddc-enhanced-rs";

for (const display_info of Display.list()) {
  const display = new Display(display_info.id);
  const { value } = display.brightness;
  display.set_brightness(value + 2);
}
```
