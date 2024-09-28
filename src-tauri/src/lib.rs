use std::env;

use tauri::{AppHandle, Emitter, Runtime};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet<R: Runtime>(handle: AppHandle<R>, name: &str) -> String {
    handle
        .emit("greet", Some(name))
        .expect("failed to emit event");
    format!("Hello, {}!", name)
}

pub fn nvidia_workaround() {
    if has_nvidia() {
        // Workaround for Nvidia drivers on Linux
        env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
}

fn has_nvidia() -> bool {
    use wgpu::{
        Backends, DeviceType, Dx12Compiler, Gles3MinorVersion, Instance, InstanceDescriptor,
        InstanceFlags,
    };

    let instance = Instance::new(InstanceDescriptor {
        flags: InstanceFlags::empty(),
        backends: Backends::VULKAN | Backends::GL,
        gles_minor_version: Gles3MinorVersion::Automatic,
        dx12_shader_compiler: Dx12Compiler::default(),
    });
    for adapter in instance.enumerate_adapters(Backends::all()) {
        let info = adapter.get_info();
        match info.device_type {
            DeviceType::DiscreteGpu | DeviceType::IntegratedGpu | DeviceType::VirtualGpu => {
                // Nvidia PCI id
                if info.vendor == 0x10de {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(target_os = "linux")]
    nvidia_workaround();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
