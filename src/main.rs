//! prbasins displays Puerto Rico water basin levels and status.
//
// Rust version of Edwood Ocasio's https://github.com/eocasio/embalses
// Inspired by http://mate.uprh.edu/embalsespr/
// Copyright 2022 Jose E. Colon <jec.rod@gmail.com> (a.k.a. Dude the Builder)
//
// Data provided by the CienciaDatosPR group of the University of Puerto Rico
// Humacao Campus Math Department. Data is subject to revision by the USGS.

use async_std::io;
use async_std::io::prelude::*;
use async_std::stream::StreamExt;
use async_std::sync::Arc;
use chrono::{Duration, Utc};
use std::convert::TryInto;
use surf::{Client, Config};

struct Basin {
    name: String,
    id: u64,
    overflow: f64,
    secure: f64,
    observe: f64,
    adjust: f64,
    control: f64,
    capacity: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let basins: [Basin; 11] = [
        Basin {
            name: "Carite".to_string(),
            id: 50039995,
            overflow: 544.,
            secure: 542.,
            observe: 539.,
            adjust: 537.,
            control: 536.,
            capacity: 8320.,
        },
        Basin {
            name: "Carraizo".to_string(),
            id: 50059000,
            overflow: 41.14,
            secure: 39.5,
            observe: 38.5,
            adjust: 36.5,
            control: 30.,
            capacity: 12000.,
        },
        Basin {
            name: "La Plata".to_string(),
            id: 50045000,
            overflow: 51.,
            secure: 43.,
            observe: 39.,
            adjust: 38.,
            control: 31.,
            capacity: 26516.,
        },
        Basin {
            name: "Cidra".to_string(),
            id: 50047550,
            overflow: 403.05,
            secure: 401.05,
            observe: 400.05,
            adjust: 399.05,
            control: 398.05,
            capacity: 4480.,
        },
        Basin {
            name: "Patillas".to_string(),
            id: 50093045,
            overflow: 67.07,
            secure: 66.16,
            observe: 64.33,
            adjust: 60.52,
            control: 59.45,
            capacity: 9890.,
        },
        Basin {
            name: "Toa Vaca".to_string(),
            id: 50111210,
            overflow: 161.,
            secure: 152.,
            observe: 145.,
            adjust: 139.,
            control: 133.,
            capacity: 50650.,
        },
        Basin {
            name: "Rio Blanco".to_string(),
            id: 50076800,
            overflow: 28.75,
            secure: 26.5,
            observe: 24.25,
            adjust: 22.5,
            control: 18.,
            capacity: 3795.,
        },
        Basin {
            name: "Caonillas".to_string(),
            id: 50026140,
            overflow: 252.,
            secure: 248.,
            observe: 244.,
            adjust: 242.,
            control: 235.,
            capacity: 31730.,
        },
        Basin {
            name: "Fajardo".to_string(),
            id: 50071225,
            overflow: 52.5,
            secure: 48.3,
            observe: 43.4,
            adjust: 37.5,
            control: 26.,
            capacity: 4430.,
        },
        Basin {
            name: "Guajataca".to_string(),
            id: 50010800,
            overflow: 196.,
            secure: 194.,
            observe: 190.,
            adjust: 186.,
            control: 184.,
            capacity: 33340.,
        },
        Basin {
            name: "Cerrillos".to_string(),
            id: 50113950,
            overflow: 173.4,
            secure: 160.,
            observe: 155.5,
            adjust: 149.4,
            control: 137.2,
            capacity: 42600.,
        },
    ];

    println!(
        "{:10} {:>8} {:8} {:16} {:10}  {:>8} {:>9} {:>8} {:>8} {:>8} {:>8}",
        "Basin",
        "Level",
        "Change",
        "Date",
        "Status",
        "Overflow",
        "Secure",
        "Observe",
        "Adjust",
        "Control",
        "Capacity",
    );

    let today = Utc::now();
    let yesterday = today.checked_sub_signed(Duration::days(1)).unwrap();
    let today = Arc::new(format!("{}", today.format("%Y.%m.%d")));
    let yesterday = Arc::new(format!("{}", yesterday.format("%Y.%m.%d")));
    let mut handles = vec![];

    let client: Arc<Client> = Arc::new(
        Config::new()
            .set_timeout(Some(std::time::Duration::from_secs(60)))
            .try_into()?,
    );

    for basin in basins.into_iter() {
        let c_clone = client.clone();
        let t_clone = today.clone();
        let y_clone = yesterday.clone();

        handles.push(async_std::task::spawn(async move {
            let mut res = c_clone.get(format!("https://nwis.waterdata.usgs.gov/pr/nwis/uv/?cb_62616=on&format=rdb&site_no={}&begin_date={}&end_date={}",
                    basin.id,
                    y_clone,
                    t_clone,
                )).send().await?;
            let body = res.body_string().await?;
            let mut lines = io::Cursor::new(body).lines();
            let mut line_count = 0;

            let mut previous = 0.;
            let mut latest = 0.;
            let mut date = String::new();
            let mut status = "UNKNOWN";

            while let Some(line_result) = lines.next().await {
                let line = line_result?;

                if !line.starts_with("USGS") {
                    continue;
                }

                line_count += 1;
                if line_count < 96 {
                    continue;
                }

                for (i, column) in line.split("\t").enumerate() {
                    match i {
                        2 => date = column.to_string(),
                        4 => latest = column.parse()?,
                        _ => {},
                    }
                }

                if line_count == 96 {
                    previous = latest;
                }

                if latest >= basin.overflow {
                    status = "OVERFLOW";
                } else if latest >= basin.secure {
                    status = "SECURE";
                } else if latest >= basin.observe {
                    status = "OBSERVE";
                } else if latest >= basin.adjust {
                    status = "ADJUST";
                } else if latest < basin.adjust {
                    status = "CONTROL";
                }
            }

            println!(
                "{:10} {:8.2} [{:5.2}m] {:16} {:10}  {:8.2} {:9.2} {:8.2} {:8.2} {:8.2} {:8}",
                basin.name,
                latest,
                latest - previous,
                date,
                status,
                basin.overflow,
                basin.secure,
                basin.observe,
                basin.adjust,
                basin.control,
                basin.capacity,
            );

            Ok::<(), Box<dyn std::error::Error + Send + Sync + 'static>>(())
        }));
    }

    for handle in handles {
        async_std::task::block_on(handle)?;
    }

    Ok(())
}
