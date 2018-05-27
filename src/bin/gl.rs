#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Mandelbrot!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // Render 2 triangles covering the whole screen
    let vertices = [
        // Top-left corner
        Vertex {
            position: [-1.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0],
        },
        // Bottom-right corner
        Vertex {
            position: [-1.0, -1.0],
        },
        Vertex {
            position: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
        },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let program = glium::Program::from_source(
        &display,
        include_str!("mandelbrot.glslv"),
        include_str!("mandelbrot.glslf"),
        None,
    ).unwrap();
    let indicies = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut closed = false;
    let mut center = (0.5f32, 0.0f32);
    let mut move_step = 0.02f32;
    let zoom_step = 0.05f32;
    let mut zoom = 1.0f32;
    let mut max_iterations = 100u16;
    while !closed {
        let mut target = display.draw();
        let (width, height) = display.gl_window().get_inner_size().unwrap_or((1, 1));
        // Draw the vertices
        target
            .draw(
                &vertex_buffer,
                &indicies,
                &program,
                &uniform!{
                    height: height as f32,
                    width: width as f32,
                    zoom: zoom,
                    center: center,
                    max_iterations: max_iterations as i32,
                    },
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        // listing the events produced by application and waiting to be received
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event: window_event, .. } => match window_event {
                glutin::WindowEvent::Closed => closed = true,
                glutin::WindowEvent::KeyboardInput{input: keyboard_input, ..} => match keyboard_input {
                    glutin::KeyboardInput {virtual_keycode, state, .. } => if state == glutin::ElementState::Pressed {
                        match virtual_keycode {
                            Some(glutin::VirtualKeyCode::Z) => {
                                zoom = zoom * (1.0 + zoom_step);
                                move_step = 0.05 / zoom;
                            },
                            Some(glutin::VirtualKeyCode::X) => {
                                zoom = zoom / (1.0 + zoom_step);
                                move_step = 0.05 / zoom;
                            },
                            Some(glutin::VirtualKeyCode::Down) => center = (center.0, center.1 + move_step),
                            Some(glutin::VirtualKeyCode::Up) => center = (center.0, center.1 - move_step),
                            Some(glutin::VirtualKeyCode::Right) => center = (center.0 - move_step, center.1),
                            Some(glutin::VirtualKeyCode::Left) => center = (center.0 + move_step, center.1),
                            Some(glutin::VirtualKeyCode::Equals) => max_iterations = max_iterations + 10,
                            Some(glutin::VirtualKeyCode::Minus) => max_iterations = max_iterations - 10,
                            Some(glutin::VirtualKeyCode::Escape) => closed = true,
                            something => println!("{:?}", something),
                        }
                    },
                },
                _ => (),
            },
            _ => (),
        });
    }
}
