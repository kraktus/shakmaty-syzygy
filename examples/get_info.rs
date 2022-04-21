use anyhow::Result;
use shakmaty::{
    fen::Fen, san::SanPlus, CastlingMode, Chess, Color, EnPassantMode, Outcome, Position,
};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::{error::Error, path::PathBuf};

use shakmaty_syzygy::{
    table::{open_table_file, Table, WdlTag},
    Material, MaybeRounded,
};

fn main() -> Result<()> {
    let paths = fs::read_dir("./partial_dl")?;
    let mut o = File::create("encoding.rs")?;
    write!(
        o,
        r#"// GENERATED BY https://github.com/kraktus/shakmaty-syzygy/blob/helpmate_tb/examples/get_info.rs

use shakmaty::{{Piece, Color::*, Role::*}};
use crate::{{Material, Pieces}};
use arrayvec::ArrayVec;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GroupDataInfo {{
    pub pieces: Pieces,
    pub order: [u8; 2],
}}

type InfoTable = ArrayVec<ArrayVec<GroupDataInfo, 2>, 4>

static ENCODING: HashMap<Material, InfoTable> = [
"#);

    for path_opt in paths {
        if let Ok(path) = path_opt {
            let path_ = path.path();
            let file_name = path_.file_name().unwrap().to_str().unwrap().to_string();
            let (material_str, _) = file_name.split_once(".").unwrap();
            let material = Material::from_str(material_str).unwrap();
            let infos = Table::<WdlTag, Chess, _>::get_info(open_table_file(path_)?, &material)?;
            let len = infos.len();
            let material_count = material.count();
            let nb_side = infos[0].len();
            write!(
                o,
                "     (Material::from_str({material_str}).unwrap(), ["
            );
            for info in infos {
                write!(o, "
                     [");
                for side in info {
                    write!(
                        o,
                        "GroupDataInfo {{
        pieces: {:?}.into_iter().collect(),
        order: {:?}
    }},",
                        side.pieces, side.order
                    );
                }
                write!(o, "
                     ].into_iter().map(|x| ArrayVec::from_iter(x.into_iter())),");
            }
            write!(
                o,
                "]),

"
            );
            break;
        }
    }
    write!(o, r"].into_iter().collect();");
    Ok(())
}
