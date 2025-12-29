export interface AudioDevice {
  id: string;
  name: string;
  isInput: boolean;
}

export function getAudioDevices(): AudioDevice[];
export function setGlobalVolume(volume: number): void;
export function setDeviceVolume(deviceId: string, volume: number): void;
export function setDeviceAsDefault(deviceId: string): void;
