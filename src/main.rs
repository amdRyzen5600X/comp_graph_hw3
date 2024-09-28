#[macro_use]
extern crate glium;
use glium::Surface;
use hw2::teapot;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("hw2")
        .build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &teapot::INDICES,
    )
    .unwrap();

    let vertex_shader_src = r#"
            #version 150

            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;

            uniform mat4 matrix;

            void main() {
                v_normal = transpose(inverse(mat3(matrix))) * normal;
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

    let fragment_shader_src = r#"
            #version 150

            in vec3 v_normal;
            out vec4 color;
            uniform vec3 u_light;

            void main() {
                float brightness = dot(normalize(v_normal), normalize(u_light));
                vec3 dark_color = vec3(0.6, 0.0, 0.0);
                vec3 regular_color = vec3(1.0, 0.0, 0.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);
            }
        "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    let mut t: f32 = 0.0;
    let s: f32 = 0.02;

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| {
            match ev {
                glium::winit::event::Event::WindowEvent { event, .. } => match event {
                    glium::winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    glium::winit::event::WindowEvent::RedrawRequested => {
                        let mut target = display.draw();
                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                        t += s;

                        // let matrix = [
                        //     [0.01, 0.0, 0.0, 0.0],
                        //     [0.0, 0.01, 0.0, 0.0],
                        //     [0.0, 0.0, 0.01, 0.0],
                        //     [0.0, 0.0, 0.0, 1.0f32],
                        // ];
                        let matrix = [
                            [0.01 * t.cos(), 0.0, 0.01 * t.sin(), 0.0],
                            [0.0, 0.01, 0.0, 0.0],
                            [-0.01 * t.sin(), 0.0, 0.01 * t.cos(), 0.0],
                            [0.0, 0.0 + 0.5 * t.sin(), 0.0, 1.0f32],
                        ];
                        // let matrix = [
                        //     [0.01, 0.0, 0.0, 0.0],
                        //     [0.0, 0.01*t.cos(), -0.01*t.sin(), 0.0],
                        //     [0.0, 0.01*t.sin(), 0.01*t.cos(), 0.0],
                        //     [0.0, 0.0, 0.0, 2.0f32],
                        // ];
                        let light = [-1.0, 0.4, 0.9f32];

                        let params = glium::DrawParameters {
                            depth: glium::Depth {
                                test: glium::draw_parameters::DepthTest::IfLess,
                                write: true,
                                ..Default::default()
                            },
                            ..Default::default()
                        };

                        target
                            .draw(
                                (&positions, &normals),
                                &indices,
                                &program,
                                &uniform! { matrix: matrix, u_light: light },
                                &params,
                            )
                            .unwrap();
                        target.finish().unwrap();
                    }
                    glium::winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                glium::winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
