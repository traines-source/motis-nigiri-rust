use std::ffi::CString;

use nigiri_sys::*;
use chrono;

extern "C" fn my_callback(evt: nigiri_event_change) {
    println!("{:?}", evt);
}

unsafe fn str_from_ptr<'a>(ptr: *const ::std::os::raw::c_char, len: u32) -> &'a str {
    let slice = std::slice::from_raw_parts(ptr as *const u8, len.try_into().unwrap());
    std::str::from_utf8_unchecked(slice)
}

#[test]
pub fn it_works() {
    unsafe {
        let gtfs_path = CString::new("./tests/fixtures/gtfs_minimal_swiss/").unwrap();
        let t = nigiri_load(gtfs_path.as_ptr(), chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00-00:00").unwrap().timestamp(), chrono::DateTime::parse_from_rfc3339("2024-12-09T00:00:00-00:00").unwrap().timestamp());
        let count = nigiri_get_transport_count(t);
        assert_eq!(count, 12);
        let stop_count = nigiri_get_location_count(t);
        assert_eq!(stop_count, 90);
        let s0 = nigiri_get_location(t, 0);
        assert_eq!(str_from_ptr((*s0).name, (*s0).name_len), "START");
        let s0 = nigiri_get_location(t, 9);
        assert_eq!(str_from_ptr((*s0).name, (*s0).name_len), "Zürich HB");
        let s0 = nigiri_get_location(t, stop_count-2);
        assert_eq!(str_from_ptr((*s0).name, (*s0).name_len), "Sion");

        assert_eq!(nigiri_get_start_day_ts(t), chrono::DateTime::parse_from_rfc3339("2023-12-27T00:00:00-00:00").unwrap().timestamp());
        assert_eq!(nigiri_get_day_count(t), 349);

        for i in 0..count {
            let transport = nigiri_get_transport(t, i);
            let transport_name = str_from_ptr((*transport).name, (*transport).name_len);
            let event_mams = std::slice::from_raw_parts((*transport).event_mams, (*transport).n_event_mams.try_into().unwrap());
            let route = nigiri_get_route(t, (*transport).route_idx);
            let stops = std::slice::from_raw_parts((*route).stops, (*route).n_stops.try_into().unwrap());
            let stop = nigiri_get_location(t, stops[0].location_idx());
            let stop_name = str_from_ptr((*stop).name, (*stop).name_len);

            assert_eq!(event_mams.len(), (stops.len()-1)*2);
            
            if i == 0 {
                assert_eq!(stops[0].in_allowed(), true);
                assert_eq!(stops[0].out_allowed(), false);            
                assert_eq!(transport_name, "1 13710");
                assert_eq!((*route).clasz, 8);
                assert_eq!(stop_name, "Zürich HB");
                assert_eq!(event_mams[0], 0);
                assert_eq!(event_mams[1], 2);
                assert_eq!(event_mams[2], 2);
                assert_eq!(nigiri_is_transport_active(t, i, 4), false);
                assert_eq!(nigiri_is_transport_active(t, i, 5), true);
                assert_eq!(nigiri_is_transport_active(t, i, 6), true);
                assert_eq!(nigiri_is_transport_active(t, i, 7), false);
                assert_eq!(nigiri_is_transport_active(t, i, 8), true);
                assert_eq!(nigiri_is_transport_active(t, i, 9), true);
                assert_eq!(nigiri_is_transport_active(t, i, 10), true);
                assert_eq!(nigiri_is_transport_active(t, i, 11), false);
            }

            println!("{} clasz: {} from: {}, stops: {} evts: {} {:?} {:?} {:?}", transport_name, (*route).clasz, stop_name, stops.len(), event_mams.len(), event_mams, stops[0].in_allowed(), stops[0].out_allowed());
            nigiri_destroy_location(stop);
            nigiri_destroy_route(route);
            nigiri_destroy_transport(transport);
        }
        let gtfsrt_path = CString::new("./tests/fixtures/2024-01-02T01_48_02+01_00.gtfsrt").unwrap();
        nigiri_update_with_rt(t, gtfsrt_path.as_ptr(), Some(my_callback));
        nigiri_update_with_rt(t, gtfsrt_path.as_ptr(), Some(my_callback));
        nigiri_destroy(t);
    }
}