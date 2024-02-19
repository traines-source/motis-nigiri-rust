use std::collections::HashMap;

use motis_nigiri::*;
use chrono;

#[test]
pub fn it_works() {
    let t = Timetable::load("./nigiri-sys/tests/fixtures/gtfs_minimal_swiss/", chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), chrono::NaiveDate::from_ymd_opt(2024, 1, 10).unwrap());
    
    let mut i = 0;
    let mut connections = t.get_connections();
    let locations: Vec<Location> = t.get_locations().collect();
    for c in &mut connections {
        if i == 0 {
            assert_eq!(c.departure, 1440*5+0);
            assert_eq!(c.arrival, 1440*5+2);
            let route = t.get_route(c.route_idx);
            assert_eq!(route.clasz, 8);
            let stop = t.get_location(c.from_idx);
            assert_eq!(stop.name, "Zürich HB");
            assert_eq!(locations[c.from_idx].name, "Zürich HB");
        }
        if i == 1 {
            assert_eq!(c.departure, 1440*5+2);
            assert_eq!(c.arrival, 1440*5+5);
            let route = t.get_route(c.route_idx);
            assert_eq!(route.clasz, 8);
            let stop = t.get_location(c.to_idx);
            assert_eq!(stop.name, "Zürich Altstetten");
            assert_eq!(locations[c.to_idx].name, "Zürich Altstetten");
            assert_eq!(c.out_allowed, false);
        }
        if i == 2 {
            assert_eq!(c.in_allowed, true);
        }
        if i == 119 {
            assert_eq!(c.departure, 1440*5+7);
            assert_eq!(c.arrival, 1440*5+11);
            let route = t.get_route(c.route_idx);
            assert_eq!(route.clasz, 8);
            let stop = t.get_location(c.from_idx);
            assert_eq!(stop.id, "8502113:0:1");
            assert_eq!(stop.name, "Aarau");
        }
        i += 1;
    }
    assert_eq!(i, 313);
    assert_eq!(t.get_locations().count(), 90);
    for s in t.get_locations() {
        assert!(s.id.len() > 0);
    }

    t.update_with_rt("./nigiri-sys/tests/fixtures/2024-01-02T01_48_02+01_00.gtfsrt", |evt| println!("{:?}", evt));

    let mapping: HashMap<(usize, u16), usize> = connections.into();
    let p = t.get_journeys(11, 69, 7000, false);
    assert_eq!(p.journeys.len(), 1);
    let j1 = &p.journeys[0];
    assert_eq!(j1.start_time, 7000);
    assert_eq!(j1.dest_time, 7528);
    for l in &p.journeys[0].legs {
        println!("{:?} {:?}", l, t.get_location(l.from_location_idx));
    }
    assert_eq!(j1.legs[0].from_location_idx, 11);
    assert_eq!(mapping[&(j1.legs[0].transport_idx, j1.legs[0].day_idx)]+j1.legs[0].to_stop_idx as usize, 17);
    
}