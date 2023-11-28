use std::{fs, path::Path, result, sync::Arc};

use chrono::prelude::*;
use curl::easy::{Easy2, Handler, WriteError};
use lazy_static::lazy_static;
use mlua::{
    AnyUserData, Error, Function, Lua, LuaOptions, MultiValue, RegistryKey, Result, StdLib, Table,
    UserData, UserDataMethods, Value,
};

use std::time::Instant;

use glow::HasContext;
use glutin::{event_loop::EventLoop, WindowedContext};
use imgui_winit_support::WinitPlatform;

const TITLE: &str = "Path of Building";

type Window = WindowedContext<glutin::PossiblyCurrent>;

// FIXME(tatu): Runtime should actually save these globals but this whole crap will get refactored
// out eventually.
lazy_static! {
    static ref START: DateTime<Local> = Local::now();
}

pub struct PathOfBuilding {}

// TODO: Is this supposed to be connection execute or? Or console?
fn con_execute(_ctx: &Lua, command: String) -> Result<()> {
    println!("Execute received {}", command);
    Ok(())
}

// TODO: Is this supposed to be connection execute or?
fn set_window_title(_ctx: &Lua, title: String) -> Result<()> {
    println!("Title received {}", title);
    Ok(())
}

// TODO: Rename this to something like GetMillisecondsSinceStart.
//
// Sooooo... GetTime in SimpleGraphics is confusing as fuck. GetTime you'd expect to get something that
// represents time, as in the fucking hours, minutes, seconds and so on. But what it actually does is
// that during app startup the current time is saved, then GetTime fetches that date, subtracts it
// from current time and then finally converts the duration to millisecond duration!
fn get_time(_ctx: &Lua, _: ()) -> Result<i64> {
    println!("Get Time called {:?}", *START);

    let duration_since_start = Local::now() - *START;
    Ok(duration_since_start.num_milliseconds())
}

fn set_main_object(_ctx: &Lua, table: Table) -> Result<()> {
    println!("Got set main table {:?}", table);
    Ok(())
}

fn render_init(_ctx: &Lua, _: ()) -> Result<()> {
    println!("Render init called");
    Ok(())
}

fn con_printf(_ctx: &Lua, message: String) -> Result<()> {
    println!("Printing {}", message);
    Ok(())
}

// TODO: Not sure if pload_module actually returns multivalue, it was just simpler to implement
// pload_module is essiantially just a convoluted name for Lua loadfile
fn pload_module<'a>(
    lua: &'a Lua,
    (module, args): (String, MultiValue<'a>),
) -> Result<(Option<i32>, MultiValue<'a>)> {
    load_module(lua, (module, args)).map(|ret| (None, ret))
}

// Wrapper for loadfile that makes it work similar to require.
fn load_module<'a>(
    lua: &'a Lua,
    (module, args): (String, MultiValue<'a>),
) -> Result<MultiValue<'a>> {
    let mut path = Path::new("src").join(&module);

    if path.extension().is_none() {
        path = path.with_extension("lua");
    }

    println!("pload module called {:?} with args {:?}", path, args);
    lua.load(path).call(args)
}

fn show_err_message(_ctx: &Lua, message: String) -> Result<()> {
    println!("ShowErrMsg called {}", message);
    Ok(())
}

// TODO(tatu): Implement
fn get_cursor_position(_ctx: &Lua, _: ()) -> Result<(i32, i32)> {
    println!("GetCursorPos called");
    Ok((0, 10))
}

// FIXME(tatu): Hardcoded to create everything under 'src' for git usage. I need to fix this later,
// now I'm just focused on getting stuff to boot.
//
// In PoB installation all the files are in same level, this is completely different from the git
// layout. I think this is why they have the compiled version of PoB together with the sources as
// you cannot just run the one from git.
fn mkdir(_ctx: &Lua, dir: String) -> Result<()> {
    let full_path = Path::new("src").join(dir);
    println!("mkdir called {:?}", full_path);
    Ok(fs::create_dir_all(full_path)?)
}

// TODO: Is this really a string? The heck.
fn is_key_down(_ctx: &Lua, keycode: String) -> Result<bool> {
    println!("IsKeyDown called {}", keycode);
    Ok(false)
}

// TODO: I think I got bamboozled by the compiler and the lifetimes are not required
fn pcall<'a>(
    _lua: &'a Lua,
    (function, args): (Function<'a>, MultiValue<'a>),
) -> Result<(Option<i32>, Value<'a>)> {
    println!("PCall with arguments {:?}", args);
    // TODO: Do we need to setup any debug stuff here or does mLua already do this?
    function.call(args).map(|ret| (None, ret))
}

// TODO: implement fetching screen size
// TODO: can this actually be negative resolution?
fn get_screen_size(_ctx: &Lua, _: ()) -> Result<(i32, i32)> {
    println!("GetScreenSize called");
    Ok((1920, 1080))
}

// TODO: implement GetScriptPath
fn get_script_path(_ctx: &Lua, _: ()) -> Result<String> {
    Ok("./src".to_string())
}

// TODO: this should probably load into a texture or something?
fn new_image_handle(lua: &Lua, _: ()) -> Result<AnyUserData> {
    lua.create_userdata(ImageHandle {})
}

// TODO: implement
// XXX: This method is just named funky, it doesn't actually draw the width of the string. It
// fetches the width of the string from a glyph map AS IF IT WERE TO BE DRAWN.
fn draw_string_width(
    _lua: &Lua,
    (_size, _font_style, _label): (i32, String, String),
) -> Result<i32> {
    Ok(50)
}

// TODO: implement
fn set_draw_color(_lua: &Lua, (args): (MultiValue)) -> Result<()> {
    println!("Set draw color to {:?}", args);
    Ok(())
}

// TODO: implement
//
// Last arguments seem like they're some scaling parameters based on x and y coordinates but in
// code they're named like s1, t1, s2, t2 just to fuck you up. When actually implementing these
// please fucking name them correctly, we ain't running out of disc space due to this and these two
// letter variables ain't gonna win you the nobel price.
fn draw_image(
    lua: &Lua,
    (
        handle,
        viewport_x,
        viewport_y,
        viewport_width,
        viewport_height,
        gigabrain_s1,
        gigabrain_t1,
        gigabrain_s2,
        gigabrain_t2,
    ): (
        Value,
        f32,
        f32,
        f32,
        f32,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
    ),
) -> Result<()> {
    println!(
        "DrawnImage called ({:?},{},{})",
        handle, viewport_x, viewport_y
    );
    Ok(())
}

// TODO: implement
//
// So in lua side this can be called with nil on either value but if the first argument is nil then
// the second one must not be.
fn set_draw_layer(_lua: &Lua, (layer_id, sublayer_id): (Option<i32>, Option<i32>)) -> Result<()> {
    println!("SetDrawLayer called ({:?},{:?})", layer_id, sublayer_id);
    Ok(())
}

fn draw_image_quad(
    lua: &Lua,
    (
        handle,
        viewport_x,
        viewport_y,
        viewport_width,
        viewport_height,
        gigabrain_s1,
        gigabrain_t1,
        gigabrain_s2,
        gigabrain_t2,
    ): (
        Value,
        f32,
        f32,
        f32,
        f32,
        Option<f32>,
        Option<f32>,
        Option<f32>,
        Option<f32>,
    ),
) -> Result<()> {
    println!(
        "DrawnImageQuad called ({:?},{},{})",
        handle, viewport_x, viewport_y
    );
    Ok(())
}

fn set_viewport(
    _lua: &Lua,
    (x, y, width, height): (Option<i32>, Option<i32>, Option<i32>, Option<i32>),
) -> Result<()> {
    println!(
        "SetViewport called ({:?},{:?},{:?},{:?})",
        x, y, width, height
    );
    Ok(())
}

// TODO: implement
fn draw_string(
    _lua: &Lua,
    (x, y, _align, _height, _style, content): (i32, i32, String, i32, String, String),
) -> Result<()> {
    println!("DrawString called ({},{},{})", x, y, content);
    Ok(())
}

// TODO: This api seems bad, why do we have to query for a handle, then load the handle and
// possibly have a shitty handle? Just pass a path to the handle and abort right away if it doesn't
// work!
struct ImageHandle {
    // image: Option<Image<'a>>,
}

impl UserData for ImageHandle {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // FIXME(tatu): Again just hacking around PWD being wrong for now
        methods.add_method_mut("Load", |_, this, path: String| {
            let full_path = Path::new("src").join(path);

            // RetainedImage::from_image_bytes(&response.url, &response.bytes)
            println!("loading image in path {:?}", &full_path);

            // let image_bytes = fs::read(&full_path)?;
            //
            // println!(
            //     "final uri {}",
            //     format!("bytes://{}", full_path.to_string_lossy().into_owned())
            // );
            //
            // this.image = Some(egui::Image::from_bytes(
            //     format!("bytes://{}", full_path.to_string_lossy().into_owned()),
            //     image_bytes,
            // ));
            //
            // println!("{:?}", &this.image);

            Ok(())
        });

        // TOOD: implement
        methods.add_method_mut("ImageSize", |_, this, _: ()| {
            // let image = this.image.as_ref().expect("Calling ImageSize but no image exists! Either image is missing or you forgot to call Load!"); let size = image.size().expect("Could not calculate image size???");

            Ok((50, 50))
        });
    }
}

struct Collector(Vec<u8>, Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> result::Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }

    fn header(&mut self, data: &[u8]) -> bool {
        self.1.extend_from_slice(data);
        true
    }
}

struct EasyWrapper {
    easy2: Easy2<Collector>,
    write_callback_key: Option<RegistryKey>,
    header_callback_key: Option<RegistryKey>,
}

// TODO: This does not work at all. I think I need to use a registry or something similar here?
// Maybe it's because from this Rust perspective we cannot guarantee it's safe to store references
// as once Lua returns they might be collected or gone?
impl UserData for EasyWrapper {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // setopt_url
        methods.add_method_mut("setopt_url", |_, this, value: String| {
            println!("fetching url {}", value);
            match this.easy2.url(&value) {
                Ok(it) => it,
                Err(err) => return Err(Error::ExternalError(Arc::new(err))),
            };
            Ok(())
        });

        // setopt_useragent
        methods.add_method_mut("setopt_useragent", |_, this, value: String| {
            match this.easy2.useragent(&value) {
                Ok(it) => it,
                Err(err) => return Err(Error::ExternalError(Arc::new(err))),
            };
            Ok(())
        });

        methods.add_method_mut("setopt", |_, this, value: String| {
            match this.easy2.useragent(&value) {
                Ok(it) => it,
                Err(err) => return Err(Error::ExternalError(Arc::new(err))),
            };
            Ok(())
        });

        // setopt_writefunction
        methods.add_method_mut("setopt_writefunction", |lua, this, value: Function| {
            this.write_callback_key = Some(lua.create_registry_value(value)?);
            Ok(())
        });

        // setopt_headerfunction
        methods.add_method_mut("setopt_headerfunction", |lua, this, value: Function| {
            this.header_callback_key = Some(lua.create_registry_value(value)?);
            Ok(())
        });

        // perform
        methods.add_method_mut("perform", |lua, this, _: ()| {
            // match this.easy2.verbose(true) {
            //     Ok(it) => it,
            //     Err(err) => return Err(Error::ExternalError(Arc::new(err))),
            // };

            match this.easy2.perform() {
                Ok(it) => it,
                Err(err) => return Err(Error::ExternalError(Arc::new(err))),
            };

            if let Some(key) = &this.write_callback_key {
                lua.registry_value::<Function>(&key).and_then(|callback| {
                    let data = this.easy2.get_ref();
                    let content = String::from_utf8_lossy(&data.0).to_string();
                    callback.call(content)
                })?;
            }

            // TODO: The fuck am I returning these Ok units all the time
            Ok(())
        });

        // getinfo
        methods.add_method_mut("getinfo", |_, _, _: ()| {
            println!("IMPLEMENT getinfo");
            Ok(())
        });

        // close implemented in lua by dropping references and letting gc run
        methods.add_method_mut("close", |_, _, _: ()| {
            // Ok(this.easy2.url_encode(to_escape.as_bytes()))
            Ok(())
        });

        // escape
        methods.add_method_mut("escape", |_, this, to_escape: String| {
            Ok(this.easy2.url_encode(to_escape.as_bytes()))
        });
    }
}

fn new_curl_easy(lua: &Lua, _: ()) -> Result<AnyUserData> {
    lua.create_userdata(EasyWrapper {
        easy2: Easy2::new(Collector(Vec::new(), Vec::new())),
        write_callback_key: None,
        header_callback_key: None,
    })
}

impl PathOfBuilding {
    pub fn start(self) -> Result<()> {
        curl::init();

        // FIXME: Likely slowsdown usage due to debug
        let lua = unsafe { Lua::unsafe_new_with(StdLib::ALL, LuaOptions::default()) };

        // Set load path
        lua.load("package.path = package.path .. ';./lua/?.lua' .. ';./src/?.lua' .. ';./runtime/lua/?.lua' .. ';./runtime/lua/?/init.lua'")
            .exec()?;

        // Initialize global functions and modules used by PoB
        let globals = lua.globals();

        // Curl module hack, stolen from how nvim plugins work
        let require = globals.get::<_, Function>("require")?;
        let module = require.call::<_, Table>("lcurl.safe")?;
        let setup = module.get::<_, Function>("setup")?;
        let function_new_curl_easy = lua.create_function(new_curl_easy)?;
        setup.call(function_new_curl_easy)?;

        // Setup some globals
        globals.set("ConExecute", lua.create_function(con_execute)?)?;
        globals.set("SetWindowTitle", lua.create_function(set_window_title)?)?;
        globals.set("SetMainObject", lua.create_function(set_main_object)?)?;
        globals.set("GetTime", lua.create_function(get_time)?)?;
        globals.set("RenderInit", lua.create_function(render_init)?)?;
        globals.set("ConPrintf", lua.create_function(con_printf)?)?;
        globals.set("PLoadModule", lua.create_function(pload_module)?)?;
        globals.set("LoadModule", lua.create_function(load_module)?)?;
        globals.set("PCall", lua.create_function(pcall)?)?;
        globals.set("ShowErrMsg", lua.create_function(show_err_message)?)?;
        globals.set("GetScreenSize", lua.create_function(get_screen_size)?)?;
        globals.set("GetScriptPath", lua.create_function(get_script_path)?)?;
        globals.set("MakeDir", lua.create_function(mkdir)?)?;
        globals.set("IsKeyDown", lua.create_function(is_key_down)?)?;
        globals.set("NewImageHandle", lua.create_function(new_image_handle)?)?;
        globals.set("DrawStringWidth", lua.create_function(draw_string_width)?)?;
        globals.set("GetCursorPos", lua.create_function(get_cursor_position)?)?;
        globals.set("SetDrawColor", lua.create_function(set_draw_color)?)?;
        globals.set("DrawImage", lua.create_function(draw_image)?)?;
        globals.set("SetDrawLayer", lua.create_function(set_draw_layer)?)?;
        globals.set("DrawImageQuad", lua.create_function(draw_image_quad)?)?;
        globals.set("SetViewport", lua.create_function(set_viewport)?)?;
        globals.set("DrawString", lua.create_function(draw_string)?)?;

        // require a module located in the newly added directory
        lua.load("require('Launch')").exec()?;

        // We should now see launch in globals?
        let globals = lua.globals();

        // TODO: Call consumes launch but why can't I use Arc here?
        // TODO: Cache function lookups, this should be safe right? They don't move in memory?
        // TODO: Make it userdata and hold a reference in registry so it won't get GC'd?
        let launch = globals.get::<_, Table>("launch").unwrap();
        println!("Initializing Path Of Building");
        let on_init: Function = launch.get::<_, Function>("OnInit")?;
        on_init.call::<_, ()>(launch)?;
        println!("Succesfully initialized Path Of Building");

        let launch = globals.get::<_, Table>("launch").unwrap();
        let on_frame: Function = launch.get::<_, Function>("OnFrame")?;
        on_frame.call::<_, ()>(launch)?;

        // Common setup for creating a winit window and imgui context, not specifc
        // to this renderer at all except that glutin is used to create the window
        // since it will give us access to a GL context
        let (event_loop, window) = create_window();
        let (mut winit_platform, mut imgui_context) = imgui_init(&window);

        // OpenGL context from glow
        let gl = glow_context(&window);

        // OpenGL renderer from this crate
        let mut ig_renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui_context)
            .expect("failed to create renderer");

        let mut last_frame = Instant::now();

        // Standard winit event loop
        event_loop.run(move |event, _, control_flow| {
            match event {
                glutin::event::Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui_context
                        .io_mut()
                        .update_delta_time(now.duration_since(last_frame));
                    last_frame = now;
                }
                glutin::event::Event::MainEventsCleared => {
                    winit_platform
                        .prepare_frame(imgui_context.io_mut(), window.window())
                        .unwrap();
                    window.window().request_redraw();
                }
                glutin::event::Event::RedrawRequested(_) => {
                    // The renderer assumes you'll be clearing the buffer yourself
                    unsafe { ig_renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

                    let ui = imgui_context.frame();
                    ui.show_demo_window(&mut true);

                    winit_platform.prepare_render(ui, window.window());
                    let draw_data = imgui_context.render();

                    // This is the only extra render step to add
                    ig_renderer
                        .render(draw_data)
                        .expect("error rendering imgui");

                    window.swap_buffers().unwrap();
                }
                glutin::event::Event::WindowEvent {
                    event: glutin::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                event => {
                    winit_platform.handle_event(imgui_context.io_mut(), window.window(), &event);
                }
            }
        });
    }
}

fn create_window() -> (EventLoop<()>, Window) {
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));
    let window = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(window, &event_loop)
        .expect("could not create window");
    let window = unsafe {
        window
            .make_current()
            .expect("could not make window context current")
    };
    (event_loop, window)
}

fn glow_context(window: &Window) -> glow::Context {
    unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s).cast()) }
}

fn imgui_init(window: &Window) -> (WinitPlatform, imgui::Context) {
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);

    let mut winit_platform = WinitPlatform::init(&mut imgui_context);
    winit_platform.attach_window(
        imgui_context.io_mut(),
        window.window(),
        imgui_winit_support::HiDpiMode::Rounded,
    );

    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    imgui_context.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

    (winit_platform, imgui_context)
}
