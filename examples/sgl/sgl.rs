use std::ffi;

use sokol::{app as sapp, gfx as sg, gl as sgl, glue as sglue};

#[derive(Default)]
struct State {
    pass_action: sg::PassAction,
    tex_view: sg::View,
    smp: sg::Sampler,
    pip3d: sgl::Pipeline,
    quad: Quad,
    cube: Cube,
    texcube: Texcube,
}

#[derive(Default)]
struct Quad {
    rot: f32,
}

#[derive(Default)]
struct Cube {
    rot_x: f32,
    rot_y: f32,
}

#[derive(Default)]
struct Texcube {
    time_accum: f32,
}

extern "C" fn init(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    // setup sokol-gfx
    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });
    // setup sokol-gl
    sgl::setup(&sgl::Desc {
        logger: sgl::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });

    // create a pipeline object for 3d rendering, with less-equal
    // depth-test and cull-face enabled, note that we don't provide
    // a shader, vertex-layout, pixel formats and sample count here,
    // these are all filled in by sokol-gl
    state.pip3d = sgl::make_pipeline(&sg::PipelineDesc {
        depth: sg::DepthState {
            write_enabled: true,
            compare: sg::CompareFunc::LessEqual,
            ..Default::default()
        },
        cull_mode: sg::CullMode::Back,
        ..Default::default()
    });

    // pass-action to clear to black
    state.pass_action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };
}

extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut State) };

    // frame time 'normalized' to 60fps
    let dt = sapp::frame_duration() * 60.0;

    // compute viewport rectangles so that the views are horizontally
    // centered and keep a 1:1 aspect ratio
    let dw = sapp::widthf();
    let dh = sapp::heightf();
    let ww = dh * 0.5;
    let hh = dh * 0.5;
    let x0 = dw * 0.5 - hh;
    let x1 = dw * 0.5;
    let y0 = 0.0;
    let y1 = dh * 0.5;

    sgl::viewportf(x0, y0, ww, hh, true);
    // drawTriangle();
    sgl::viewportf(x1, y0, ww, hh, true);
    // drawQuad(dt);
    sgl::viewportf(x0, y1, ww, hh, true);
    // drawCubes(dt);
    sgl::viewportf(x1, y1, ww, hh, true);
    // drawTexCube(dt);
    sgl::viewportf(0.0, 0.0, dw, dh, true);

    sg::begin_pass(&sg::Pass {
        action: state.pass_action,
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sgl::draw();
    sg::end_pass();
    sg::commit();
}

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sgl::shutdown();
    sg::shutdown();

    let _ = unsafe { Box::from_raw(user_data as *mut State) };
}

fn main() {
    let state = Box::new(State::default());

    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame),
        cleanup_userdata_cb: Some(cleanup),
        user_data,
        window_title: c"sql.rs".as_ptr(),
        width: 512,
        height: 512,
        sample_count: 4,
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
        ..Default::default()
    });
}
