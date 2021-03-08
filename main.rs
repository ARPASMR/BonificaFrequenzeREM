use std::{env::args, io::prelude::*};
use itertools::*;

fn main() {
    let filename = args().nth(1).unwrap();
    let changes_file = format!("changes-{}", filename);
    let plottable_file = format!("plottable-{}", filename);

    let mut f = std::fs::File::open(filename.clone()).unwrap();
    let mut s = String::new();
   
    if f.read_to_string(&mut s).is_ok() {
        let timestamps = 
            s.lines()
            .filter_map(|line| 
                 chrono::NaiveDateTime::parse_from_str(&*line, "%Y-%m-%d %H:%M:00.000").ok()) // lettura marche temporali (elimina eventuali errori di parsing)
            .tuple_windows() // prende le coppie consecutive
            .map(|(time1, time2)| 
                 {(time1.clone(),  // prima marca temporale
                   time2.signed_duration_since(time1).num_seconds())}) // differenza in secondi
            .filter(|(_, duration)| 
                    vec![60, 300, 600, 900, 1800, 3600, 7200, 10800, 14400, 21600, 43200, 86400].contains(duration)) // solo differenze previste
            .group_by(|(x, _)| x.date()).into_iter() // raggruppa per giorno
            .map(|(_, group)| 
                 group.into_iter()
                 .min_by(|(_, diff1), (_, diff2)|  // valore minimo giornaliero per la differenza
                         match diff1 > diff2 { 
                             true => std::cmp::Ordering::Greater, 
                             false => std::cmp::Ordering::Less
                         })
                 .unwrap()) 
            .group_by(|(_, time)| *time).into_iter().map(|(_, group)| group.into_iter().collect::<Vec<_>>()) // ragguppa periodi con la stessa frequenza
            .filter(|vec| vec.len() > 1).collect::<Vec<_>>(); // toglie periodi che durano un solo giorno
        
        std::fs::remove_file(changes_file.clone()).unwrap();
        let mut out1 = std::fs::File::create(changes_file).unwrap();

        timestamps.iter()
            .map(|vec| vec[0]) // stampa solo una misura per periodo
            .for_each(|value| out1.write_all(format!("{} {}\n", value.0, value.1).as_bytes()).unwrap());
        
        std::fs::remove_file(plottable_file.clone()).unwrap();
        let mut out2 = std::fs::File::create(plottable_file).unwrap();

        timestamps.iter().flatten() // stampa tutte le misure
            .for_each(|value| out2.write_all(format!("{} {}\n", value.0, value.1).as_bytes()).unwrap());
    }
}
