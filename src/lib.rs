use neon::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait};
use windows::{
    core::HSTRING,
    Win32::{
        Media::Audio::*,
        System::Com::*,
        Media::Audio::Endpoints::*
    }
};

fn get_audio_devices(mut cx: FunctionContext) -> JsResult<JsArray> {
    println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {available_hosts:?}");

    let host = cpal::default_host();
    let devices = host.devices().unwrap();
    
    // Create the JS array to return
    let js_array = cx.empty_array();

    for (device_index, device) in devices.enumerate() {
        let id = device
            .id()
            .map_or_else(
                |_| "Unknown ID".to_string(),
                |id| id.to_string(),
            );
        let name = device
            .description()
            .map_or_else(
                |_| "Unknown Name".to_string(),
                |desc| desc.to_string()
            );

        // Create a JS object for each device
        let js_object = cx.empty_object();
        let formatted_id = id.strip_prefix("wasapi:").unwrap();

        let js_id = cx.string(&formatted_id.to_string());
        let js_name = cx.string(&name);
        
        js_object.set(&mut cx, "id", js_id)?;
        js_object.set(&mut cx, "name", js_name)?;
        
        js_array.set(&mut cx, device_index as u32, js_object)?;
    }
    
    Ok(js_array)
}

pub fn set_global_volume(volume: f32) -> windows::core::Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let endpoint_volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;
        endpoint_volume.SetMasterVolumeLevelScalar(volume, std::ptr::null())?;
        Ok(())
    }
}

pub fn set_device_volume(device_id: &str, volume: f32) -> windows::core::Result<()> {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let device_id = HSTRING::from(device_id);
        let device_to_update = enumerator.GetDevice(&device_id)?;

        let endpoint_volume: IAudioEndpointVolume = device_to_update.Activate(CLSCTX_ALL, None)?;
        let v = volume.clamp(0.0, 1.0);
        endpoint_volume.SetMasterVolumeLevelScalar(v, std::ptr::null())?;
        Ok(())
    }
}

pub fn set_device_as_default(_device_id: &str) -> windows::core::Result<()> {
    unsafe {
        // TODO @sam
        Ok(())
    }
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
