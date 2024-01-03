use nigiri_sys::*;
use std::ffi::CString;
use chrono;

unsafe fn str_from_ptr<'a>(ptr: *const ::std::os::raw::c_char, len: u32) -> &'a str {
    let slice = std::slice::from_raw_parts(ptr as *const u8, len.try_into().unwrap());
    std::str::from_utf8_unchecked(slice)
}

static mut TIMETABLE_UPDATING: Option<fn(evt: EventChange)> = None;

extern "C" fn nigiri_callback(evt: nigiri_event_change) {
    unsafe {
        if TIMETABLE_UPDATING.is_some() {
            let c = TIMETABLE_UPDATING.unwrap();
            c(EventChange {
                    transport_idx: evt.transport_idx,
                    day: evt.day_idx,
                    stop_idx: evt.stop_idx,
                    is_departure: evt.is_departure,
                    delay: evt.delay,
                    cancelled: evt.cancelled,
            });
        }
    } 
}

pub struct Timetable {
    t: *const nigiri_timetable_t
}

impl Timetable {
    pub fn load(path: &str, start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Timetable {
        unsafe {
            let path = CString::new(path).unwrap();
            let t = nigiri_load(path.as_ptr(), start_date.and_hms_opt(0, 0, 0).unwrap().timestamp(), end_date.and_hms_opt(0, 0, 0).unwrap().timestamp());
            Timetable {
                t: t
            }
        }
    }

    pub fn update_with_rt(&self, path: &str, callback: fn(evt: EventChange)) -> Result<(), &'static str> {
        unsafe {
            if TIMETABLE_UPDATING.is_some() {
                return Err("only one timetable can be updated at a time");
            }
            TIMETABLE_UPDATING = Some(callback);
            let path = CString::new(path).unwrap();
            nigiri_update_with_rt(self.t, path.as_ptr(), Some(nigiri_callback));
            TIMETABLE_UPDATING = None;
            Ok(())
        }   
    }

    pub fn get_locations(&self) -> Locations {
        unsafe {
            Locations {
                t: self,
                i: 0,
                n_locations: nigiri_get_location_count(self.t).try_into().unwrap()
            }
        }
    }

    pub fn get_transports(&self) -> Transports {
        unsafe {
            Transports {
                t: self,
                i: 0,
                n_transports: nigiri_get_transport_count(self.t)
            }
        }
    }

    pub fn get_connections(&self) -> Connections {
        let transports = self.get_transports();
        let n_days;
        unsafe {
            n_days = nigiri_get_day_count(self.t);
        }
        Connections {
            t: self,
            n_days: n_days,
            transport_idx: 0,
            transports: transports,
            transport: None,
            i: 0,
            day_idx: 0
        }
    }

    pub fn get_route(&self, route_idx: u32) -> Route {
        unsafe {
            let raw_route = nigiri_get_route(self.t, route_idx);
            let stops = std::slice::from_raw_parts((*raw_route).stops, (*raw_route).n_stops.try_into().unwrap());
            Route {
                ptr: raw_route,
                route_idx: route_idx,
                stops: stops.iter().map(|s| s.location_idx()).collect(),
                clasz: (*raw_route).clasz
            }
        }
    }

    pub fn get_location(&self, location_idx: usize) -> Location {
        unsafe {
            let raw_location = nigiri_get_location(self.t, location_idx.try_into().unwrap());
            Location {
                ptr: raw_location,
                id: str_from_ptr((*raw_location).id, (*raw_location).id_len),
                name: str_from_ptr((*raw_location).name, (*raw_location).name_len),
                lat: (*raw_location).lat as f32,
                lon: (*raw_location).lon as f32
            }
        }
    }

    pub fn is_transport_active(&self, transport_idx: usize, day_idx: u16) -> bool {
        unsafe {
            nigiri_is_transport_active(self.t, transport_idx.try_into().unwrap(), day_idx)
        }
    }
}

impl<'a> Drop for Timetable {
    fn drop(&mut self) {
        unsafe {
            nigiri_destroy(self.t);
        }
    }
}

pub struct Location<'a> {
    ptr: *const nigiri_location_t,
    pub id: &'a str,
    pub name: &'a str,
    pub lat: f32,
    pub lon: f32
}

impl<'a> Drop for Location<'a> {
    fn drop(&mut self) {
        unsafe {
            nigiri_destroy_location(self.ptr);
        }
    }
}

pub struct Locations<'a> {
    t: &'a Timetable,
    i: usize,
    n_locations: usize
}

impl<'a> Iterator for Locations<'a> {
    type Item = Location<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n_locations {
            return None
        }
        let location = self.t.get_location(self.i);
        self.i += 1;
        Some(location)
    }
}

pub struct Route {
    ptr: *const nigiri_route_t,
    pub route_idx: u32,
    pub stops: Vec<u32>,
    pub clasz: u16,
}

impl Drop for Route {
    fn drop(&mut self) {
        unsafe {
            nigiri_destroy_route(self.ptr);
        }
    }
}

pub struct Transport {
    ptr: *const nigiri_transport_t,
    pub route_idx: u32,
    pub event_mams: Vec<i16>,
    pub name: String,
}

impl Drop for Transport {
    fn drop(&mut self) {
        unsafe {
            nigiri_destroy_transport(self.ptr);
        }
    }
}

pub struct Transports<'a> {
    t: &'a Timetable,
    i: u32,
    n_transports: u32
}

impl<'a> Iterator for Transports<'a> {
    type Item = Transport;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.i >= self.n_transports {
                return None
            }
            let transport = nigiri_get_transport(self.t.t, self.i);        
            self.i += 1;
            let result = Transport {
                ptr: transport,
                route_idx: (*transport).route_idx,
                event_mams: std::slice::from_raw_parts((*transport).event_mams, usize::try_from((*transport).n_event_mams).unwrap()).to_vec(), // TODO no clone
                name: str_from_ptr((*transport).name, (*transport).name_len).to_string()
            };
            Some(result)
        }
    }
}

pub struct Connection {
	pub route_idx: u32,
	pub trip_id: u32,
	pub from_idx: usize,
	pub to_idx: usize,
	pub departure: i32,
	pub arrival: i32,
}

pub struct Connections<'a> {
    t: &'a Timetable,
    n_days: u16,
    transport_idx: usize,
    transports: Transports<'a>,
    transport: Option<Transport>,
    day_idx: u16,
    i: usize
}

impl<'a> Connections<'a> {
    fn get_minutes_after_base(day_idx: u16, mam: i16) -> i32 {
        day_idx as i32*1440+mam as i32
    }

    fn next_active_day_idx(&mut self) {
        while self.day_idx < self.n_days && !self.t.is_transport_active(self.transport_idx, self.day_idx) {
            println!("weird {} {}", self.n_days, self.day_idx);
            self.day_idx += 1;
        }
    }
}

impl<'a> Iterator for Connections<'a> {
    type Item = Connection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.transport.is_none() {
            self.transport = self.transports.next();
            if self.transport.is_none() {
                return None
            }
            self.next_active_day_idx();
        }
        let route = self.t.get_route(self.transport.as_ref().unwrap().route_idx);
        if self.i >= route.stops.len()-1 {
            self.i = 0;
            self.day_idx += 1;
            self.next_active_day_idx();
        }

        if self.day_idx >= self.n_days { 
            self.transport = match self.transports.next() {
                Some(t) => {
                    self.i = 0;
                    self.day_idx = 0;
                    self.next_active_day_idx();
                    self.transport_idx += 1;
                    Some(t)
                },
                None => return None
            }
        }
        let transport = self.transport.as_ref().unwrap();
        //TODO route
        let c = Connection {
            route_idx: transport.route_idx,
            trip_id: self.transports.i,
            from_idx: route.stops[self.i].try_into().unwrap(),
            to_idx: route.stops[self.i+1].try_into().unwrap(),
            departure: Connections::get_minutes_after_base(self.day_idx, transport.event_mams[self.i*2]),
            arrival: Connections::get_minutes_after_base(self.day_idx, transport.event_mams[self.i*2+1])
        };
        self.i += 1;
        Some(c)
    }
}

#[derive(Debug)]
pub struct EventChange {
    pub transport_idx: u32,
    pub day: u16,
    pub stop_idx: u32,
    pub is_departure: bool,
    pub delay: i16,
    pub cancelled: bool,
}