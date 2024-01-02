use nigiri_sys::*;
use std::ffi::CStr;
use std::ffi::CString;
use chrono;

static mut TIMETABLE_UPDATING: Option<fn(evt: EventChange)> = None;

extern "C" fn nigiri_callback(evt: nigiri_event_change) {
    unsafe {
        if TIMETABLE_UPDATING.is_some() {
            let c = TIMETABLE_UPDATING.unwrap();
            c(EventChange {
                    transport_idx: evt.transport_idx,
                    day: evt.day,
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

    pub fn get_stops(&self) -> Stops {
        unsafe {
            Stops {
                t: self,
                i: 0,
                n_stops: nigiri_get_stop_count(self.t).try_into().unwrap()
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
        Connections {
            t: self,
            transports: transports,
            transport: None,
            i: 0
        }
    }

    pub fn get_route(&self, route_idx: u32) -> Route {
        unsafe {
            let raw_route = nigiri_get_route(self.t, route_idx);
            let stops = std::slice::from_raw_parts((*raw_route).stops, (*raw_route).n_stops.try_into().unwrap());
            Route {
                ptr: raw_route,
                route_idx: route_idx,
                stops: stops.to_vec(), // TODO no clone?
                clasz: (*raw_route).clasz
            }
        }
    }

    pub fn get_stop(&self, stop_idx: usize) -> Stop {
        unsafe {
            let raw_stop = nigiri_get_stop(self.t, stop_idx.try_into().unwrap());
            Stop {
                ptr: raw_stop,
                id: CStr::from_ptr((*raw_stop).id).to_str().unwrap(),
                name: CStr::from_ptr((*raw_stop).name).to_str().unwrap(),
                lat: (*raw_stop).lat as f32,
                lon: (*raw_stop).lon as f32
            }
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

pub struct Stop<'a> {
    ptr: *const nigiri_stop_t,
    pub id: &'a str,
    pub name: &'a str,
    pub lat: f32,
    pub lon: f32
}

impl<'a> Drop for Stop<'a> {
    fn drop(&mut self) {
        unsafe {
            nigiri_destroy_stop(self.ptr);
        }
    }
}

pub struct Stops<'a> {
    t: &'a Timetable,
    i: usize,
    n_stops: usize
}

impl<'a> Iterator for Stops<'a> {
    type Item = Stop<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n_stops {
            return None
        }
        let stop = self.t.get_stop(self.i);
        self.i += 1;
        Some(stop)
    }
}

pub struct Route {
    ptr: *const nigiri_route_t,
    pub route_idx: u32,
    pub stops: Vec<u32>,
    pub clasz: u32,
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
                name: CStr::from_ptr((*transport).name).to_str().unwrap().to_string() //TODO no clone
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
    transports: Transports<'a>,
    transport: Option<Transport>,
    i: usize
}

impl<'a> Iterator for Connections<'a> {
    type Item = Connection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.transport.is_none() {
            self.transport = self.transports.next();
            if self.transport.is_none() {
                return None
            }
        }
        let route = self.t.get_route(self.transport.as_ref().unwrap().route_idx);
        if self.i >= route.stops.len()-1 {
            self.transport = match self.transports.next() {
                Some(t) => {
                    self.i = 0;
                    Some(t)
                },
                None => return None
            }
        }
        let transport = self.transport.as_ref().unwrap();
        //TODO bitfield, mam, route
        let c = Connection {
            route_idx: transport.route_idx,
            trip_id: self.transports.i,
            from_idx: route.stops[self.i].try_into().unwrap(),
            to_idx: route.stops[self.i+1].try_into().unwrap(),
            departure: transport.event_mams[self.i*2] as i32,
            arrival: transport.event_mams[self.i*2+1] as i32
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