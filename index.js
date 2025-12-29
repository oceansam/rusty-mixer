const native = require("./index.node");

module.exports = native;
module.exports.getAudioDevices = native.getAudioDevices;
module.exports.setGlobalVolume = native.setGlobalVolume;
module.exports.setDeviceVolume = native.setDeviceVolume;
module.exports.setDeviceAsDefault = native.setDeviceAsDefault;
