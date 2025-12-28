use windows::Win32::Media::Audio::*;

pub fn collect_devices(collection: &IMMDeviceCollection, is_input: bool) -> Vec<(IMMDevice, bool)> {
    let mut devices = Vec::new();
    unsafe {
        let count = collection.GetCount().unwrap();
        for i in 0..count {
            let device: IMMDevice = collection.Item(i).unwrap();
            devices.push((device, is_input));
        }
    }
    devices
}
