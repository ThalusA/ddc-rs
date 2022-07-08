# ddc-node-rs

`ddc-node-rs` is a cross-platform Node package for controlling monitors with
[DDC/CI](https://en.wikipedia.org/wiki/Display_Data_Channel).

## Documentation

https://github.com/ThalusA/ddc-node-rs/blob/master/index.d.ts

## Examples

```javascript
import { Display, DisplayManager, VCPFeatures } from "ddc-node-rs";

const displays = new DisplayManager().collect();

for (const display of displays) {
    const vcp_feature = display.getVcpFeature(VCPFeatures.ImageAdjustment.Luminance);
    console.info(`Display at index ${display.index} have a brightness value of`);
    console.info(vcp_feature);
    display.setVcpFeature(VCPFeatures.ImageAdjustment.Luminance,
                          vcp_feature.currentValue + 5);
}
```
