use rlua::{Error, Lua, Result, Table};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
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
        let mut types_set: HashSet<String> = HashSet::new();
        for pair in spectables {
            let (key, value) = pair?;
            if value.len()? != 0 {
                let col_spec: Vec<GGPKColumn> = value
                    .sequence_values::<Table>()
                    .map(|x| x.expect("ggpkseq").into())
                    .collect();
                for col in &col_spec[..] {
                    types_set.insert(col.column_type.clone());
                }
                println!("{:?} added to specmap", &key);
                specs.insert(key, col_spec);
            }
        }
        println!("Types: {:?}", &types_set);
        let jsonwriter = fs::File::create(Path::new("ggpk_specs.json")).expect("fileopen");
        serde_json::to_writer(jsonwriter, &specs).expect("ser fail");
        Ok::<(), Error>(())
    })?;

    let jsonreader = &fs::File::open(Path::new("ggpk_specs.json")).expect("fileopen");
    let _specs: HashMap<String, Vec<GGPKColumn>> =
        serde_json::from_reader(jsonreader).expect("ser fail");

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct GGPKColumn {
    name: String,
    width: u32,
    column_type: String,
    column_is_list: bool,
    ref_to: Option<String>,
    data: Option<GGPKColumnData>,
}

#[derive(Debug, Serialize, Deserialize)]
enum GGPKColumnData {
    GGPKFloat(Vec<f64>),
    GGPKInterval(Vec<(i32, i32)>),
    GGPKKey(Vec<Vec<u64>>),
    GGPKInt(Vec<i32>),
    GGPKShortKey(Vec<u32>),
    GGPKBool(Vec<bool>),
    GGPKString(Vec<String>),
    GGPKEnum(Vec<u32>),
    GGPKUInt(Vec<u32>),
}

impl From<Table<'_>> for GGPKColumn {
    fn from(s: Table) -> GGPKColumn {
        GGPKColumn {
            name: s.get("name").expect("bad name"),
            width: s.get("width").expect("bad width"),
            column_type: s.get("type").expect("bad column_type"),
            column_is_list: s.get("list").expect("bad islist"),
            ref_to: s.get("refTo").expect("bad refTo"),
            data: None,
        }
    }
}
