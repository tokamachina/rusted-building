use rlua::{Error, Lua, Result, Table};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
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
    let mut specs: HashMap<String, Vec<GGPKColumn>> =
        serde_json::from_reader(jsonreader).expect("ser fail");
    let mut ggpk: HashMap<String, Vec<GGPKColumn>> = HashMap::new();
    for (data_name, data_set) in specs.drain() {
        for column in data_set {}
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct GGPKColumn {
    name: Option<String>,
    width: u32,
    column_type: String,
    column_is_list: bool,
    ref_to: Option<String>,
    data: Option<GGPKColumnData>,
    offset: u32,
}

struct GGPKData {
    name: String,
    columns: Vec<GGPKColumn>,
    total_width: u32,
    row_count: u32,
    col_count: u32,
}

fn new_ggpkdata(mut name: String, mut newcols: Vec<GGPKColumn>) -> GGPKData {
    let dat64 = &fs::read(String::from(".\\data\\ggpk\\") + &name + &String::from(".dat64"))
        .expect("datread");
    let row_count = as_u32_be(&dat64[0..4]);
    let col_count = u32::try_from(newcols.len()).expect("column count");
    let mut total_width: u32 = 0;
    for mut ggpkcol in newcols {
        ggpkcol.offset = total_width.clone();
        total_width += ggpkcol.width;
        ggpkcol.data = match (ggpkcol.column_type.as_str(), ggpkcol.column_is_list) {
            ("Key", false) => Some(GGPKKey(todo!())),
            ("Enum", false) => Some(GGPKEnum(todo!())),
            ("Interval", false) => Some(GGPKInterval(todo!())),
            ("Bool", false) => Some(GGPKBool(todo!())),
            ("String", false) => Some(GGPKString(todo!())),
            ("UInt", false) => Some(GGPKUInt(todo!())),
            ("Float", false) => Some(GGPKFloat(todo!())),
            ("Int", false) => Some(GGPKInt(todo!())),
            ("Key", true) => Some(ListGGPKKey(todo!())),
            ("Enum", true) => Some(ListGGPKEnum(todo!())),
            ("Interval", true) => Some(ListGGPKInterval(todo!())),
            ("Bool", true) => Some(ListGGPKBool(todo!())),
            ("String", true) => Some(ListGGPKString(todo!())),
            ("UInt", true) => Some(ListGGPKUInt(todo!())),
            ("Float", true) => Some(ListGGPKFloat(todo!())),
            ("Int", true) => Some(ListGGPKInt(todo!())),
            _ => None,
        };
    }
    GGPKData {
        name,
        col_count,
        row_count,
        columns: todo!(),
        total_width,
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum GGPKColumnData {
    GGPKFloat(Vec<f64>),
    GGPKInterval(Vec<(i32, i32)>),
    GGPKKey(Vec<u64>),
    GGPKInt(Vec<i32>),
    GGPKShortKey(Vec<u32>),
    GGPKBool(Vec<bool>),
    GGPKString(Vec<String>),
    GGPKEnum(Vec<u32>),
    GGPKUInt(Vec<u32>),
    ListGGPKFloat(Vec<Vec<f64>>),
    ListGGPKInterval(Vec<Vec<(i32, i32)>>),
    ListGGPKKey(Vec<Vec<u64>>),
    ListGGPKInt(Vec<Vec<i32>>),
    ListGGPKShortKey(Vec<Vec<u32>>),
    ListGGPKBool(Vec<Vec<bool>>),
    ListGGPKString(Vec<Vec<String>>),
    ListGGPKEnum(Vec<Vec<u32>>),
    ListGGPKUInt(Vec<Vec<u32>>),
}

// let self.data = match self.column_is_list {
//     true => match self.column_type.expect("list coltype") {
//         "Key" => todo!(),
//         "Enum" => todo!(),
//         "Interval" => todo!(),
//         "Bool" => todo!(),
//         "String" => todo!(),
//         "UInt" => todo!(),
//         "Float" => todo!(),
//         "Int" => todo!(),
//         "ShortKey" => todo!(),
//     },
//     false => match self.column_type.expect("nonlist coltype") {
//         "Key" => todo!(),
//         "Enum" => todo!(),
//         "Interval" => todo!(),
//         "Bool" => todo!(),
//         "String" => todo!(),
//         "UInt" => todo!(),
//         "Float" => todo!(),
//         "Int" => todo!(),
//         "ShortKey" => todo!(),
//     }
// };

impl From<Table<'_>> for GGPKColumn {
    fn from(s: Table) -> GGPKColumn {
        GGPKColumn {
            name: match s.get::<&str, String>("name").expect("bad name").as_str() {
                "" => None,
                x => Some(x.to_string()),
            },
            width: s.get("width").expect("bad width"),
            column_type: s.get("type").expect("bad column_type"),
            column_is_list: s.get("list").expect("bad islist"),
            ref_to: match s.get::<&str, String>("refTo").expect("bad refTo").as_str() {
                "" => None,
                x => Some(x.to_string()),
            },
            data: None,
            offset: None,
        }
    }
}

fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}
