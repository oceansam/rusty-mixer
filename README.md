# Rusty Mixer

A Rust-based library for helpful native audio calls

## Usage

```js
// ESM syntax (Electron with "type": "module" or in a bundler)
import { getAudioDevices, setDeviceVolume, setGlobalVolume } from "rusty-mixer";

// CommonJS (default Electron renderer/main)
const {
  getAudioDevices,
  setDeviceVolume,
  setGlobalVolume,
} = require("rusty-mixer");

console.log("Getting audio devices...");
const devices = getAudioDevices();
console.log(devices);

// Specify wasapi device id
setDeviceVolume("{X}.{X}", 0.5);

// Update default device
setGlobalVolume(0.5);
```
