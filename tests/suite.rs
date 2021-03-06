use shakmaty::fen::Fen;
use shakmaty::variants::{Atomic, Chess, Giveaway};
use shakmaty::{Position, FromSetup};
use shakmaty_syzygy::{Syzygy, Tablebase};

fn test_csv<S>(path: &str)
where
    S: Position + FromSetup + Syzygy + Clone
{
    let mut tables = Tablebase::new();
    tables.add_directory("tables/regular").expect("read directory");
    tables.add_directory("tables/atomic").expect("read directory");
    tables.add_directory("tables/giveaway").expect("read directory");

    let mut reader = csv::Reader::from_path(path).expect("reader");

    for line in reader.records() {
        let record = line.expect("record");

        let fen: Fen = record
            .get(0).expect("fen field")
            .parse().expect("valid fen");

        let expected_wdl: i8 = record
            .get(1).expect("wdl field")
            .parse().expect("valid wdl");

        let expected_dtz: i32 = record
            .get(2).expect("dtz field")
            .parse().expect("valid dtz");

        let pos: S = fen.position().expect("legal");

        println!("{} | wdl: {} | dtz: {}", fen, expected_wdl, expected_dtz);

        match tables.probe_wdl(&pos) {
            Ok(wdl) => assert_eq!(i8::from(wdl), expected_wdl),
            Err(err) => panic!("probe wdl: {}", err),
        }

        match tables.probe_dtz(&pos) {
            Ok(dtz) => assert_eq!(i32::from(dtz), expected_dtz),
            Err(err) => panic!("probe dtz: {}", err),
        }
    }
}

#[test]
fn test_regular() {
    test_csv::<Chess>("tests/regular.csv");
}

#[test]
fn test_atomic() {
    test_csv::<Atomic>("tests/atomic.csv");
}

#[test]
fn test_giveaway() {
    test_csv::<Giveaway>("tests/giveaway.csv");
}
