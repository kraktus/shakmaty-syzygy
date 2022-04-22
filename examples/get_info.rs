use anyhow::Result;
use shakmaty::{
    fen::Fen, san::SanPlus, CastlingMode, Chess, Color, EnPassantMode, Outcome, Position,
};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::{error::Error, path::PathBuf};
use std::collections::HashMap;
use serde_json::ser::Serializer;
use serde::ser::Serialize;

use shakmaty_syzygy::{
    table::{open_table_file, Table, WdlTag, InfoTable},
    Material, MaybeRounded
};

fn main() -> Result<()> {
    let paths = fs::read_dir("./partial_dl")?;
    let mut o = File::create("encoding.json")?;
    let mut r: HashMap<String, InfoTable> = HashMap::new();
    for path_opt in paths {
        if let Ok(path) = path_opt {
            let path_ = path.path();
            let file_name = path_.file_name().unwrap().to_str().unwrap().to_string();
            let (material_str, _) = file_name.split_once(".").unwrap();
            let material = Material::from_str(material_str).unwrap();
            let infos = Table::<WdlTag, Chess, _>::get_info(open_table_file(path_)?, &material)?;
            r.insert(material_str.to_string(), infos);
        }
    }
    let mut formatter = Serializer::pretty(o);
    let pretty_json = r.serialize(&mut formatter)?;
    Ok(())
}
