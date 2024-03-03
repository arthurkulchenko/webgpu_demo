use winit::{
    event::*,
    window::{Window},
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The graph_api_wrapper_instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let graph_api_wrapper_instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor { backends: wgpu::Backends::all(), ..Default::default() }
        );
        
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.
        let surface = unsafe { graph_api_wrapper_instance.create_surface(&window) }.unwrap();

        let graph_api_backend_adapter = graph_api_wrapper_instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                // NOTICE: Giving created surface to find better adapter
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        // let graph_api_backend_adapter = graph_api_wrapper_instance.enumerate_adapters(wgpu::Backends::all())
        //                       .filter(|graph_api_backend_adapter| {
        //                           // Check if this graph_api_backend_adapter supports our surface
        //                           graph_api_backend_adapter.is_surface_supported(&surface)
        //                       }).next().unwrap()


        let (device, queue) = graph_api_backend_adapter.request_device(
            &wgpu::DeviceDescriptor {
                // NOTICE: You can get a list of features supported by your device using adapter.features() or device.features().
                // here we no using specific features that might limit us in backends
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if we're building for the web, we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&graph_api_backend_adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter().copied().filter(|f| f.is_srgb()).next().unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        State { window, surface, size, device, queue, config }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        todo!()
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
