mod utils;

use neon::prelude::*;
use windows::{
    core::HSTRING,
    Win32::{
        Devices::FunctionDiscovery::PKEY_Device_FriendlyName, Media::Audio::Endpoints::*,
        Media::Audio::*, System::Com::*,
    },
};

use crate::utils::collect_devices;

fn get_audio_devices(mut cx: FunctionContext) -> JsResult<JsArray> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).unwrap();

        let input_collection = enumerator
            .EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE)
            .unwrap();
        let output_collection = enumerator
            .EnumAudioEndpoints(eRender, DEVICE_STATE_ACTIVE)
            .unwrap();

        // Convert to native Rust Vec and chain them together
        let all_devices: Vec<(IMMDevice, bool)> = collect_devices(&input_collection, true)
            .into_iter()
            .chain(collect_devices(&output_collection, false))
            .collect();
        // Create the JS array to return
        let js_array = cx.empty_array();

        for (idx, (device, is_input)) in all_devices.iter().enumerate() {
            let id = device.GetId().unwrap().to_string().unwrap();
            let props = device.OpenPropertyStore(STGM_READ).unwrap();
            let name = props
                .GetValue(&PKEY_Device_FriendlyName)
                .unwrap()
                .to_string();
            // build js obj
            let js_object = cx.empty_object();
            let js_id = cx.string(&id);
            let js_name = cx.string(&name);
            let js_is_input = cx.boolean(*is_input);

            // update js obj
            js_object.set(&mut cx, "id", js_id)?;
            js_object.set(&mut cx, "name", js_name)?;
            js_object.set(&mut cx, "isInput", js_is_input)?;
            js_array.set(&mut cx, idx as u32, js_object)?;
        }

        Ok(js_array)
    }
}

pub fn set_global_volume(volume: f32) -> windows::core::Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let endpoint_volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;
        endpoint_volume.SetMasterVolumeLevelScalar(volume, std::ptr::null())?;
        Ok(())
    }
}

pub fn set_device_volume(device_id: &str, volume: f32) -> windows::core::Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device_id = HSTRING::from(device_id);
        let device_to_update = enumerator.GetDevice(&device_id)?;

        let endpoint_volume: IAudioEndpointVolume = device_to_update.Activate(CLSCTX_ALL, None)?;
        let v = volume.clamp(0.0, 1.0);
        endpoint_volume.SetMasterVolumeLevelScalar(v, std::ptr::null())?;
        Ok(())
    }
}

pub fn set_device_as_default(_device_id: &str) -> windows::core::Result<()> {
    // TODO @sam
    Ok(())
}
// ================ JS SHIT ================ \\
fn js_set_global_volume(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let volume = cx.argument::<JsNumber>(0)?.value(&mut cx) as f32;

    match set_global_volume(volume) {
        Ok(_) => Ok(cx.undefined()),
        Err(e) => cx.throw_error(format!("Failed to set volume: {}", e)),
    }
}

fn js_set_device_volume(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let device_id = cx.argument::<JsString>(0)?.value(&mut cx);
    let volume = cx.argument::<JsNumber>(1)?.value(&mut cx) as f32;

    // check if device_id contains "wasapi:"
    if device_id.contains("wasapi:") {
        return cx.throw_error(format!(
            "Invalid device ID: {}. You may have included a prefix or postfix to the id.",
            device_id
        ));
    }

    match set_device_volume(&device_id, volume) {
        Ok(_) => Ok(cx.undefined()),
        Err(e) => cx.throw_error(format!("Failed to set volume: {}", e)),
    }
}

fn js_set_device_as_default(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let device_id = cx.argument::<JsString>(0)?.value(&mut cx);
    match set_device_as_default(&device_id) {
        Ok(_) => Ok(cx.undefined()),
        Err(e) => cx.throw_error(format!("Failed to set device as default: {}", e)),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getAudioDevices", get_audio_devices)?;
    cx.export_function("setGlobalVolume", js_set_global_volume)?;
    cx.export_function("setDeviceVolume", js_set_device_volume)?;
    cx.export_function("setDeviceAsDefault", js_set_device_as_default)?;
    Ok(())
}
