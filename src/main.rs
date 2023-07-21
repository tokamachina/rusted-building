use rlua::{Lua, Result, Table};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let pob_lua = Lua::new();
    pob_lua.context(|lua_ctx| {
        let _globals = lua_ctx.globals();
        let spectables = lua_ctx
            .load(&fs::read(Path::new(".\\data\\ggpk\\spec.lua")).expect("error loading lua"))
            .set_name("PoB data spec definition")?
            .into_function()?
            .call::<_, Table>(())?
            .pairs::<String, Table>();
        let mut specs: HashMap<String, Vec<GGPKColumn>> = HashMap::new();
        for pair in spectables {
            let (key, value) = pair?;
            if value.len()? != 0 {
                let col_spec: Vec<GGPKColumn> = value
                    .sequence_values::<Table>()
                    .map(|x| x.expect("ggpk table sequence").into())
                    .collect();
                specs.insert(key.to_string(), col_spec);
                println!("{:?} added to specmap", key);
            }
        }
        Ok(())
    })
}

struct GGPKColumn {
    name: String,
    width: u32,
    column_type: String,
    ref_to: Option<String>,
}

impl From<Table<'_>> for GGPKColumn {
    fn from(s: Table) -> GGPKColumn {
        GGPKColumn {
            name: s.get("name").expect("bad name"),
            width: s.get("width").expect("bad width"),
            column_type: s.get("type").expect("bad column_type"),
            ref_to: s.get("refTo").expect("bad refTo"),
        }
    }
}
