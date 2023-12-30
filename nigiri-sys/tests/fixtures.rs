use std::ffi::CStr;
use std::ffi::CString;

use nigiri_sys::*;
use chrono;

#[test]
pub fn it_works() {
    unsafe {
        let path = CString::new("./tests/fixtures/gtfs_minimal_swiss/").unwrap();
        let t = nigiri_load(path.as_ptr(), chrono::DateTime::parse_from_rfc3339("2018-12-09T00:00:00-00:00").unwrap().timestamp(), chrono::DateTime::parse_from_rfc3339("2019-12-09T00:00:00-00:00").unwrap().timestamp());
        let count = nigiri_get_transport_count(t);
        assert_eq!(count, 12);
        let stop_count = nigiri_get_stop_count(t);
        assert_eq!(stop_count, 90);
        let s0 = nigiri_get_stop(t, 0);
        assert_eq!(CStr::from_ptr((*s0).name).to_str().unwrap(), "START");
        let s0 = nigiri_get_stop(t, 9);
        assert_eq!(CStr::from_ptr((*s0).name).to_str().unwrap(), "Zürich HB");
        let s0 = nigiri_get_stop(t, stop_count-2);
        assert_eq!(CStr::from_ptr((*s0).name).to_str().unwrap(), "Sion");

        for i in 0..count {
            let transport = nigiri_get_transport(t, i);
            let transport_name = CStr::from_ptr((*transport).name).to_str().unwrap();
            let event_mams = std::slice::from_raw_parts((*transport).event_mams, (*transport).n_event_mams.try_into().unwrap());
            let route = nigiri_get_route(t, (*transport).route_idx);
            let stops = std::slice::from_raw_parts((*route).stops, (*route).n_stops.try_into().unwrap());
            let stop = nigiri_get_stop(t, stops[0]);
            let stop_name = CStr::from_ptr((*stop).name).to_str().unwrap();

            assert_eq!(event_mams.len(), (stops.len()-1)*2);
            
            if i == 0 {
                assert_eq!(transport_name, "1 13710");
                assert_eq!((*route).clasz, 8);
                assert_eq!(stop_name, "Zürich HB");
                assert_eq!(event_mams[0], 0);
                assert_eq!(event_mams[1], 2);
                assert_eq!(event_mams[2], 2);
            }

            println!("{} clasz: {} from: {}, stops: {} evts: {} {:?}", transport_name, (*route).clasz, stop_name, stops.len(), event_mams.len(), event_mams);
            nigiri_destroy_stop(stop);
            nigiri_destroy_route(route);
            nigiri_destroy_transport(transport);
        }
        nigiri_destroy(t);
    }
}