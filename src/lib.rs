use neon::prelude::*;
use cpal::traits::{DeviceTrait, HostTrait};

mod structs;
use structs::index::{DeviceTrait, AudioDevice};

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
        let js_id = cx.string(&id);
        let js_name = cx.string(&name);
        
        js_object.set(&mut cx, "id", js_id)?;
        js_object.set(&mut cx, "name", js_name)?;
        
        js_array.set(&mut cx, device_index as u32, js_object)?;
    }
    
    Ok(js_array)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getAudioDevices", get_audio_devices)?;
    Ok(())
}
