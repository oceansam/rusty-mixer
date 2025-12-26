const {
  getAudioDevices,
  setDeviceVolume,
  setDeviceAsDefault,
  setGlobalVolume,
} = require(".");

console.log("Getting audio devices...");
console.log(getAudioDevices());
setDeviceVolume("{0.0.0.00000000}.{b16d4e45-f6eb-437e-a5e2-9039a8be9269}", 0.5);
setDeviceAsDefault("{0.0.0.00000000}.{b16d4e45-f6eb-437e-a5e2-9039a8be9269}");
