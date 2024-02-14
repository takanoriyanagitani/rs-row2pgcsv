#![forbid(clippy::unwrap_used)]

use std::io::Write;
use std::process::ExitCode;

use serde::{Serialize, Serializer};

struct Arr4i(Vec<i16>);

impl Serialize for Arr4i {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut buf: Vec<u8> = vec![];
        let vals: &[i16] = &self.0;
        row2pgcsv::arr::to_writer(&mut buf, &vals).map_err(|e| {
            serde::ser::Error::custom(format!("unable to serialize arr values: {e}"))
        })?;
        let s: String = String::from_utf8(buf)
            .map_err(|e| serde::ser::Error::custom(format!("unexpected error: {e}")))?;
        ser.serialize_str(s.as_str())
    }
}

impl From<Vec<i16>> for Arr4i {
    fn from(v: Vec<i16>) -> Self {
        Self(v)
    }
}

#[derive(serde::Serialize)]
struct Row {
    i: i32,
    d: f64,
    f: f32,
    s: String,
    b: bool,
    a: Arr4i,
}

fn sub() -> Result<(), String> {
    let rows = vec![
        Row {
            i: 634,
            d: 42.195,
            f: 3.776,
            s: "mount".into(),
            b: true,
            a: vec![2, 3, 5, 7, 11].into(),
        },
        Row {
            i: 333,
            d: 2.99792458,
            f: 0.599,
            s: "takao".into(),
            b: false,
            a: vec![1, 3, 7, 15, 31].into(),
        },
    ];

    let o = std::io::stdout();
    let mut l = o.lock();
    {
        let bw = std::io::BufWriter::new(&mut l);
        let mut cwtr = csv::WriterBuilder::new().from_writer(bw);
        let cnt: u64 = rows.into_iter().try_fold(0, |tot, next| {
            cwtr.serialize(next)
                .map(|_| tot + 1)
                .map_err(|e| format!("unable to serialize a row: {e}"))
        })?;
        eprintln!("rows count: {cnt}");
    }
    l.flush().map_err(|e| format!("unable to flush: {e}"))?;

    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
