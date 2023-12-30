use motis_nigiri::*;
use chrono;

#[test]
pub fn it_works() {
    let t = Timetable::load("./nigiri-sys/tests/fixtures/gtfs_minimal_swiss/", chrono::NaiveDate::from_ymd_opt(2018, 12, 9).unwrap(), chrono::NaiveDate::from_ymd_opt(2019, 12, 9).unwrap());
    
    let mut i = 0;
    let connections = t.get_connections();
    let stops: Vec<Stop> = t.get_stops().collect();
    for c in connections {
        if i == 0 {
            assert_eq!(c.departure, 0);
            assert_eq!(c.arrival, 2);
            let route = t.get_route(c.route_idx);
            assert_eq!(route.clasz, 8);
            let stop = t.get_stop(c.from_idx);
            assert_eq!(stop.name, "Zürich HB");
            assert_eq!(stops[c.from_idx].name, "Zürich HB");
        }
        if i == 1 {
            assert_eq!(c.departure, 2);
            assert_eq!(c.arrival, 5);
            let route = t.get_route(c.route_idx);
            assert_eq!(route.clasz, 8);
            let stop = t.get_stop(c.to_idx);
            assert_eq!(stop.name, "Zürich Altstetten");
            assert_eq!(stops[c.to_idx].name, "Zürich Altstetten");
        }
        i += 1;
    }
    assert_eq!(i, 86);
    assert_eq!(t.get_stops().count(), 90);
    for s in t.get_stops() {
        assert!(s.id.len() > 0);
    }
}