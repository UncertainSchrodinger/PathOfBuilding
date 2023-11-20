use mlua::{Function, Lua, Result, Table};

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

impl PathOfBuilding {
    pub fn start(self) -> Result<()> {
        let lua = Lua::new();

        // Initialize global functions used by PoB
        let globals = lua.globals();

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

        // add `some_directory` to the package path
        lua.load("package.path = package.path .. ';./src/?.lua' .. ';./runtime/lua/?.lua'")
            .exec()?;

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
