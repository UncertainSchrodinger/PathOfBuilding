use std::{result, sync::Arc};

use curl::easy::{Easy2, Handler, WriteError};
use mlua::{
    AnyUserData, Error, Function, Lua, RegistryKey, Result, Table, UserData, UserDataFields,
    UserDataMethods,
};

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

fn get_time(_ctx: &Lua, _: ()) -> Result<()> {
    println!("Get Time called");
    Ok(())
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

fn pload_module(ctx: &Lua, module: String) -> Result<()> {
    ctx.load(&format!("require('{}')", &module)).exec()?;
    println!("pload module called {}", module);
    Ok(())
}

fn show_err_message(_ctx: &Lua, message: String) -> Result<()> {
    println!("ShowErrMsg called {}", message);
    Ok(())
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
    heeader_callback_key: Option<RegistryKey>,
}

// TODO: This does not work at all. I think I need to use a registry or something similar here?
// Maybe it's because from this Rust perspective we cannot guarantee it's safe to store references
// as once Lua returns they might be collected or gone?
impl UserData for EasyWrapper {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        // setopt_url
        methods.add_method_mut("setopt_url", |_, this, value: String| {
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

        methods.add_method_mut("setopt_useragent", |_, this, value: String| {
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
            this.heeader_callback_key = Some(lua.create_registry_value(value)?);
            Ok(())
        });

        // perform
        methods.add_method_mut("perform", |_, this, _: ()| {
            match this.easy2.perform() {
                Ok(it) => it,
                Err(err) => return Err(Error::ExternalError(Arc::new(err))),
            };
            Ok(())
        });

        // getinfo
        methods.add_method_mut("getinfo", |_, _, _: ()| {
            println!("IMPLEMENT getinfo");
            Ok(())
        });

        // close implemented in lua by dropping references and letting gc run

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
        heeader_callback_key: None,
    })
}

impl PathOfBuilding {
    pub fn start(self) -> Result<()> {
        let lua = Lua::new();

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

        // let function_new_easy_curl = lua.create_function(new_easy_curl)?;
        // globals.set("_curl_easy", function_new_easy_curl)?;

        let function_con_execute = lua.create_function(con_execute)?;
        globals.set("ConExecute", function_con_execute)?;

        let function_window_title = lua.create_function(set_window_title)?;
        globals.set("SetWindowTitle", function_window_title)?;

        let function_set_main_object = lua.create_function(set_main_object)?;
        globals.set("SetMainObject", function_set_main_object)?;

        let function_get_time = lua.create_function(get_time)?;
        globals.set("GetTime", function_get_time)?;

        let function_render_init = lua.create_function(render_init)?;
        globals.set("RenderInit", function_render_init)?;

        let function_con_printf = lua.create_function(con_printf)?;
        globals.set("ConPrintf", function_con_printf)?;

        let function_pload_module = lua.create_function(pload_module)?;
        let function_load_module = lua.create_function(pload_module)?;
        globals.set("PLoadModule", function_pload_module)?;
        globals.set("LoadModule", function_load_module)?;

        let function_show_err_msg = lua.create_function(show_err_message)?;
        globals.set("ShowErrMsg", function_show_err_msg)?;

        // require a module located in the newly added directory
        lua.load("require('Launch')").exec()?;

        // We should now see launch in globals?
        let globals = lua.globals();
        let launch = globals.get::<_, Table>("launch").unwrap();

        println!("{:?}", launch.get::<_, Function>("OnInit"));
        let on_init: Function = launch.get::<_, Function>("OnInit")?;
        on_init.call::<_, ()>(launch)?;

        Ok(())
    }
}
