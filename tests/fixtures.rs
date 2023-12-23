use motis_nigiri::*;
use chrono;

#[test]
pub fn it_works() {
    let t = Timetable::load("./nigiri-sys/tests/fixtures/gtfs_minimal_swiss/", chrono::NaiveDate::from_ymd_opt(2018, 12, 9).unwrap(), chrono::NaiveDate::from_ymd_opt(2019, 12, 9).unwrap());
    
    let mut i = 0;
    let connections = t.get_connections();
    for c in connections {
        if i == 0 {
            assert_eq!(c.departure, 0);
            assert_eq!(c.arrival, 2);
            let route = t.get_route(c.route_idx);
            assert_eq!(route.clasz, 8);
            let stop = t.get_stop(c.from_idx);
            assert_eq!(stop.name, "Zürich HB");
        }
        if i == 1 {
            assert_eq!(c.departure, 2);
            assert_eq!(c.arrival, 5);
            let route = t.get_route(c.route_idx);
            assert_eq!(route.clasz, 8);
            let stop = t.get_stop(c.to_idx);
            assert_eq!(stop.name, "Zürich Altstetten");
        }
        i += 1;
    }
    assert_eq!(i, 86);
}