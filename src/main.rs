extern crate glium;
extern crate winit;

use winit::{event::*, event_loop::*, window::WindowBuilder};
use glium::glutin::{self, event::Event, event_loop::ControlFlow, window::WindowBuilder as OtherWindowBuilder};
use glium::Surface;
use glium::uniform;
use glium::implement_vertex;
use k_board::{Keyboard, Keys};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    // Set up the window and OpenGL context
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = OtherWindowBuilder::new().with_title("Cube Example");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    // Define cube vertices
    let vertices: [Vertex; 8] = [
        Vertex { position: [-0.5, -0.5, -0.5] },
        Vertex { position: [0.5, -0.5, -0.5] },
        Vertex { position: [0.5, 0.5, -0.5] },
        Vertex { position: [-0.5, 0.2, -0.5] },
        Vertex { position: [-1.0, -0.5, 0.5] },
        Vertex { position: [0.5, -0.5, 0.5] },
        Vertex { position: [0.5, 0.5, 0.5] },
        Vertex { position: [-0.5, 0.5, 0.5] },
    ];

    // Define cube indices for rendering
    let indices: [u16; 6] = [
        0, 1, 2, 2, 3, 0,  // Front face
        // Define other faces here...
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();

    // Define shaders (vertex and fragment)
    let vertex_shader_src = r#"
        #version 140
        in vec3 position;
        out vec3 color;
        uniform mat4 perspective;
        uniform mat4 view;

        void main() {
            gl_Position = perspective * view * vec4(position, 1.0);
            color = position * 0.5 + 0.5;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 color;
        out vec4 fragColor;

        void main() {
            fragColor = vec4(color, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Set up perspective and view matrices
    //let perspective = glium::uniforms::UniformBuffer::<[[f32; 4]; 4]>::new(&display, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]).unwrap();
    //let view = glium::uniforms::UniformBuffer::<[[f32; 4]; 4]>::new(&display, [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]).unwrap();
    let asspect_ratio: f32 = 16.0 / 9.0;
    let perspective: [[f32; 4]; 4] = [
        [1.0 /* / asspect_ratio */, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let view: [[f32; 4]; 4] = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, -4.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    event_loop.run(move |ev, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match ev {
            Event::RedrawRequested(_) => {
                let mut target = display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                // Draw the cube
                let uniforms = uniform! {
                    perspective: perspective,
                    view: view,
                };
                target
                    .draw(
                        &vertex_buffer,
                        &index_buffer,
                        &program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();

                target.finish().unwrap();
            }
            Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        };
    });
}
