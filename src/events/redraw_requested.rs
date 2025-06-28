use wgpu::{ Operations, LoadOp, StoreOp, Color, RenderPassColorAttachment };
use wgpu::{ Surface, SurfaceConfiguration, SurfaceTexture, SurfaceError, TextureView, CommandEncoder, Device, Queue, Limits };
// use crate::surface_state;
use crate::State;

// queue: &mut Queue, surface_texture: SurfaceTexture, mut encoder: CommandEncoder, view: TextureView
pub fn render(state: &mut State) -> Result<(), SurfaceError> {
    let surface_texture = state.surface.get_current_texture().unwrap();
    let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("Encoder"), });

    let red = 0.3;
    let green = 0.5;
    let blue = 0.6;
    let color = wgpu::Color { r: red, g: green, b: blue, a: 1.0, };

    {
        let _render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &color_attachments(&view, color),
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            }
        );
    }
    // submit will accept anything that implements IntoIter
    state.queue.submit(std::iter::once(encoder.finish()));
    surface_texture.present();

    Ok(())
}

fn color_attachments(view: &TextureView, color: wgpu::Color) -> Vec<Option<RenderPassColorAttachment>> {
    // NOTICE: Currently, we are clearing the screen with a bluish color
    vec![
        Some(
            RenderPassColorAttachment {
                view: view,
                resolve_target: None,
                // DOC: This tells wgpu what to do with the colors on the screen (specified by view)
                ops: Operations {
                    // DOC: The load field tells wgpu how to handle colors stored from the previous frame
                    load: LoadOp::Clear(color),
                    // DOC: The store field tells wgpu whether we want to store the rendered results to the Texture behind our
                    // TextureView (in this case, it's the SurfaceTexture) We use StoreOp::Store as we do want
                    // to store our render results.
                    store: StoreOp::Store,
                },
            }
        ),
    ]
}
