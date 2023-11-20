use rlua::{Lua, Result};

pub struct PathOfBuilding {}

impl PathOfBuilding {
    pub fn start(self) -> Result<()> {
        let lua = Lua::new();
        lua.context(|lua_ctx| {
            // add `some_directory` to the package path
            lua_ctx
                .load("package.path = package.path .. ';./lua/?.lua'")
                .exec()?;

            // require a module located in the newly added directory
            lua_ctx.load("require'main'").exec()?;

            Ok(())
        })?;

        Ok(())
    }
}
