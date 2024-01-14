use nigiri_sys::*;
use std::{ffi::{CString, c_void}, collections::HashMap};
use chrono;
use serde::{Serialize, Deserialize};

unsafe fn str_from_ptr<'a>(ptr: *const ::std::os::raw::c_char, len: u32) -> &'a str {
    let slice = std::slice::from_raw_parts(ptr as *const u8, len.try_into().unwrap());
    std::str::from_utf8_unchecked(slice)
}

extern "C" fn nigiri_callback<F>(evt: nigiri_event_change, context: *mut c_void) where F: FnMut(EventChange) {
    unsafe {
        let closure = &mut *(context as *mut F);
        closure(EventChange {
            transport_idx: evt.transport_idx.try_into().unwrap(),
            day_idx: evt.day_idx,
            stop_idx: evt.stop_idx.try_into().unwrap(),
            is_departure: evt.is_departure,
            delay: evt.delay,
            cancelled: evt.cancelled,
        });
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

    pub fn update_with_rt<F: FnMut(EventChange)>(&self, path: &str, mut callback: F) {
        unsafe {
            let path = CString::new(path).unwrap();
            nigiri_update_with_rt(self.t, path.as_ptr(), Some(nigiri_callback::<F>), &mut callback as *mut _ as *mut c_void);
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
                n_transports: nigiri_get_transport_count(self.t).try_into().unwrap()
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
            route: None,
            day_idx: 0,
            stop_idx: 0,
            id: 0,
            transport_at_day_to_connection_id: HashMap::new()
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
                lon: (*raw_location).lon as f32,
                transfer_time: (*raw_location).transfer_time,
                footpaths: std::slice::from_raw_parts((*raw_location).footpaths, (*raw_location).n_footpaths.try_into().unwrap()).iter().map(|f| Footpath{
                    target_location_idx: f.target_location_idx().try_into().unwrap(),
                    duration: f.duration()
                }).collect()       
            }
        }
    }

    pub fn get_transport(&self, transport_idx: usize) -> Transport {
        unsafe {
            let transport = nigiri_get_transport(self.t, transport_idx.try_into().unwrap());        
            Transport {
                ptr: transport,
                route_idx: (*transport).route_idx,
                event_mams: std::slice::from_raw_parts((*transport).event_mams, usize::try_from((*transport).n_event_mams).unwrap()),
                name: str_from_ptr((*transport).name, (*transport).name_len).to_string()
            }
        }
    }

    pub fn is_transport_active(&self, transport_idx: usize, day_idx: u16) -> bool {
        unsafe {
            nigiri_is_transport_active(self.t, transport_idx.try_into().unwrap(), day_idx)
        }
    }

    pub fn get_start_day_ts(&self) -> i64 {
        unsafe {
            nigiri_get_start_day_ts(self.t)
        }
    }

    pub fn get_journeys(&self, start_location_idx: usize, destination_location_idx: usize, time: i32, backward_search: bool) -> ParetoSet {
        unsafe {
            let time_ref = self.get_start_day_ts();
            let ptr = nigiri_get_journeys(self.t, start_location_idx.try_into().unwrap(), destination_location_idx.try_into().unwrap(), time as i64*60+time_ref, backward_search);
            let journeys = std::slice::from_raw_parts((*ptr).journeys, (*ptr).n_journeys.try_into().unwrap());
            println!("lnsdajfh{}", journeys.len());
            ParetoSet {
                ptr: ptr,
                journeys: journeys.iter().map(|j| Journey{
                    start_time: (((*j).start_time-time_ref)/60) as i32,
                    dest_time: (((*j).dest_time-time_ref)/60) as i32,
                    legs: std::slice::from_raw_parts((*j).legs, (*j).n_legs.try_into().unwrap()).iter().map(|l| Leg {
                        is_footpath: (*l).is_footpath,
                        transport_idx: (*l).transport_idx.try_into().unwrap(),
                        day_idx: (*l).day_idx,
                        from_stop_idx: (*l).from_stop_idx,
                        from_location_idx: (*l).from_location_idx.try_into().unwrap(),
                        to_stop_idx: (*l).to_stop_idx,
                        to_location_idx: (*l).to_location_idx.try_into().unwrap(),
                        duration: (*l).duration
                    }).collect()
                }).collect()
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Footpath {
    pub target_location_idx: usize,
    pub duration: u32 
}

#[derive(Debug)]
pub struct Location<'a> {
    ptr: *const nigiri_location_t,
    pub id: &'a str,
    pub name: &'a str,
    pub lat: f32,
    pub lon: f32,
    pub transfer_time: u16,
    pub footpaths: Vec<Footpath>
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Transport<'a> {
    ptr: *const nigiri_transport_t,
    pub route_idx: u32,
    pub event_mams: &'a [i16],
    pub name: String,
}

impl<'a> Drop for Transport<'a> {
    fn drop(&mut self) {
        unsafe {
            nigiri_destroy_transport(self.ptr);
        }
    }
}

pub struct Transports<'a> {
    t: &'a Timetable,
    i: usize,
    n_transports: usize
}

impl<'a> Iterator for Transports<'a> {
    type Item = Transport<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.n_transports {
            return None
        }
        let result = self.t.get_transport(self.i);  
        self.i += 1;
        Some(result)
    }
}

#[derive(Debug)]
pub struct Connection {
    pub id: usize,
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
    transport: Option<Transport<'a>>,
    route: Option<Route>,
    day_idx: u16,
    stop_idx: usize,
    id: usize,
    pub transport_at_day_to_connection_id: HashMap<(usize, u16), usize>
}

impl<'a> Connections<'a> {
    fn get_minutes_after_base(day_idx: u16, mam: i16) -> i32 {
        day_idx as i32*1440+mam as i32
    }

    fn next_active_day_idx(&mut self) {
        while self.day_idx < self.n_days && !self.t.is_transport_active(self.transport_idx, self.day_idx) {
            self.day_idx += 1;
        }
        self.transport_at_day_to_connection_id.insert((self.transport_idx, self.day_idx), self.id);
    }

    pub fn get_connection_idx(&self, transport_idx: usize, day_idx: u16, stop_idx: usize) -> usize {
        self.transport_at_day_to_connection_id[&(transport_idx, day_idx)] + stop_idx
    }
}

impl<'a> Into<HashMap<(usize, u16), usize>> for Connections<'a> {
    fn into(self) -> HashMap<(usize, u16), usize> {
        self.transport_at_day_to_connection_id
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
            self.route = Some(self.t.get_route(self.transport.as_ref().unwrap().route_idx));
            self.next_active_day_idx();
        }
        if self.stop_idx >= self.route.as_ref().unwrap().stops.len()-1 {
            self.stop_idx = 0;
            self.day_idx += 1;
            self.next_active_day_idx();
        }

        if self.day_idx >= self.n_days { 
            self.transport = match self.transports.next() {
                Some(t) => {
                    self.stop_idx = 0;
                    self.day_idx = 0;
                    self.transport_idx += 1;
                    if self.transport.as_ref().unwrap().route_idx != t.route_idx {
                        self.route = Some(self.t.get_route(t.route_idx));
                    }
                    self.next_active_day_idx();
                    Some(t)
                },
                None => return None
            };
        }
        let transport = self.transport.as_ref().unwrap();
        //TODO route
        let c = Connection {
            id: self.id,
            route_idx: transport.route_idx,
            trip_id: self.transports.i.try_into().unwrap(),
            from_idx: self.route.as_ref().unwrap().stops[self.stop_idx].try_into().unwrap(),
            to_idx: self.route.as_ref().unwrap().stops[self.stop_idx+1].try_into().unwrap(),
            departure: Connections::get_minutes_after_base(self.day_idx, transport.event_mams[self.stop_idx*2]),
            arrival: Connections::get_minutes_after_base(self.day_idx, transport.event_mams[self.stop_idx*2+1])
        };
        assert_eq!(self.get_connection_idx(self.transport_idx, self.day_idx, self.stop_idx), self.id);
        self.stop_idx += 1;
        self.id += 1;
        Some(c)
    }
}

#[derive(Debug)]
pub struct EventChange {
    pub transport_idx: usize,
    pub day_idx: u16,
    pub stop_idx: u16,
    pub is_departure: bool,
    pub delay: i16,
    pub cancelled: bool,
}

#[derive(Debug)]
pub struct ParetoSet {
    ptr: *const nigiri_pareto_set_t,
    pub journeys: Vec<Journey>
}

impl<'a> Drop for ParetoSet {
    fn drop(&mut self) {
        unsafe {
            nigiri_destroy_journeys(self.ptr);
        }
    }
}

#[derive(Debug)]
pub struct Journey {
    pub legs: Vec<Leg>,
    pub start_time: i32,
    pub dest_time: i32
}

#[derive(Debug)]
pub struct Leg {
    pub is_footpath: bool,
    pub transport_idx: usize,
    pub day_idx: u16,
    pub from_stop_idx: u16,
    pub from_location_idx: usize,
    pub to_stop_idx: u16,
    pub to_location_idx: usize,
    pub duration: u32
}

