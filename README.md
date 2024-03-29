# ddc-rs

`ddc-rs` is a cross-platform Node package for controlling monitors with
[DDC/CI](https://en.wikipedia.org/wiki/Display_Data_Channel).

## **Warning**

This repository will be unmaintained in favor of the async version, please check: https://github.com/ThalusA/ddc-node 

## Documentation

https://github.com/ThalusA/ddc-rs/blob/master/index.ts

## Examples

```javascript
import { Display, DisplayManager, VCPFeatures } from "ddc-rs";

const displays = new DisplayManager().collect();

for (const display of displays) {
    const vcp_feature = display.getVcpFeature(VCPFeatures.ImageAdjustment.Luminance);
    console.info(`Display at index ${display.index} have a brightness value of`);
    console.info(vcp_feature);
    display.setVcpFeature(VCPFeatures.ImageAdjustment.Luminance,
                          vcp_feature.currentValue + 5);
}
```
