# Rusty Mixer

A Rust-based library for helpful native audio calls

## Usage

```js
const { getAudioDevices, setDeviceVolume, setGlobalVolume } = require(".");

console.log("Getting audio devices...");
const devices = getAudioDevices();
console.log(devices);

// Specify wasapi device id
setDeviceVolume("{X}.{X}", 0.5);

// Update default device
setGlobalVolume(0.5);
```
