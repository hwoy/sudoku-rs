#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub extern crate sudoku_sys;
use std::os::raw::{c_int, c_uint};

pub trait GLIBCRNG_impl {
    fn new() -> Self;
}

impl GLIBCRNG_impl for sudoku_sys::GLIBCRNG {
    fn new() -> Self {
        [0; 31]
    }
}

pub trait sgs_unit_impl {
    fn new() -> Self;
}

impl sgs_unit_impl for sudoku_sys::sgs_unit {
    fn new() -> Self {
        Self {
            idx: 0,
            idy: 0,
            idz: 0,
            value: 0,
            valuep: 0,
        }
    }
}

pub trait sgs_board_impl {
    fn new() -> Self;
}

impl sgs_board_impl for sudoku_sys::sgs_board {
    fn new() -> Self {
        Self {
            unit: [[sudoku_sys::sgs_unit::new(); 9]; 9],
        }
    }
}

pub trait sgs_game_impl {
    fn new(bid: sudoku_sys::sgt_bid, numblank: c_uint) -> Self;
    fn setvalue(&mut self, value: sudoku_sys::sgt_set, x: c_uint, y: c_uint);
    fn getvalue(&self, x: c_uint, y: c_uint) -> sudoku_sys::sgt_set;

    fn setbid(&mut self, bid: c_uint);
    fn getvalue_x(&self, y: c_uint) -> sudoku_sys::sgt_set;
    fn getvalue_y(&self, x: c_uint) -> sudoku_sys::sgt_set;

    fn getvalue_z(&self, x: c_uint, y: c_uint) -> sudoku_sys::sgt_set;
    fn getvalue_p(&mut self, x: c_uint, y: c_uint) -> sudoku_sys::sgt_set;

    fn countvalue(&self, x: c_uint, y: c_uint) -> c_uint;

    fn seed(&mut self, seed: sudoku_sys::URND32);
    fn rand(&mut self) -> c_uint;
    fn random(&mut self, min: c_uint, max: c_uint) -> c_uint;

    fn findvalueone(&mut self, x: c_uint, y: c_uint) -> c_uint;
    fn findvalueunique(&mut self, x: c_uint, y: c_uint) -> c_uint;

    fn getobstruct(&mut self) -> c_uint;

    fn genboard(&mut self) -> c_int;

    fn completeboard(&self) -> c_int;

    fn resetboard(&mut self);

    fn findboard(&mut self) -> sudoku_sys::sgt_bid;

    fn getbid(&self) -> sudoku_sys::sgt_bid;

    fn createsudoku(&mut self);

    fn createsudoku_rnd(&mut self, sd: c_uint);

    fn setnblank(&mut self, numblank: c_uint);

    fn getnblank(&self) -> c_uint;
}

impl sgs_game_impl for sudoku_sys::sgs_game {
    fn new(bid: sudoku_sys::sgt_bid, numblank: c_uint) -> Self {
        Self {
            rng: sudoku_sys::GLIBCRNG::new(),
            bid: bid,
            numblank: numblank,
            board: sudoku_sys::sgs_board::new(),
            majorver: sudoku_sys::SUDOKU_ENGINE_MAJOR_VERSION,
            minorver: sudoku_sys::SUDOKU_ENGINE_MINOR_VERSION,
            sminorver: sudoku_sys::SUDOKU_ENGINE_SMINOR_VERSION,
        }
    }

    fn setvalue(&mut self, value: sudoku_sys::sgt_set, x: c_uint, y: c_uint) {
        unsafe { sudoku_sys::sgf_setvalue(value, self, x, y) }
    }

    fn getvalue(&self, x: c_uint, y: c_uint) -> sudoku_sys::sgt_set {
        unsafe { sudoku_sys::sgf_getvalue(self, x, y) }
    }

    fn setbid(&mut self, bid: c_uint) {
        unsafe { sudoku_sys::sgf_setbid(self, bid) }
    }

    fn getvalue_x(&self, y: c_uint) -> sudoku_sys::sgt_set {
        unsafe { sudoku_sys::sgf_getvalue_x(self, y) }
    }

    fn getvalue_y(&self, x: c_uint) -> sudoku_sys::sgt_set {
        unsafe { sudoku_sys::sgf_getvalue_y(self, x) }
    }

    fn getvalue_z(&self, x: c_uint, y: c_uint) -> sudoku_sys::sgt_set {
        unsafe { sudoku_sys::sgf_getvalue_z(self, x, y) }
    }

    fn getvalue_p(&mut self, x: c_uint, y: c_uint) -> sudoku_sys::sgt_set {
        unsafe { sudoku_sys::sgf_getvalue_p(self, x, y) }
    }

    fn countvalue(&self, x: c_uint, y: c_uint) -> c_uint {
        unsafe { sudoku_sys::sgf_countvalue(self, x, y) }
    }

    fn seed(&mut self, seed: sudoku_sys::URND32) {
        unsafe { sudoku_sys::sgf_seed(self, seed) }
    }

    fn rand(&mut self) -> c_uint {
        unsafe { sudoku_sys::sgf_rand(self) }
    }

    fn random(&mut self, min: c_uint, max: c_uint) -> c_uint {
        unsafe { sudoku_sys::sgf_random(self, min, max) }
    }

    fn findvalueone(&mut self, x: c_uint, y: c_uint) -> c_uint {
        unsafe { sudoku_sys::sgf_findvalueone(self, x, y) }
    }

    fn findvalueunique(&mut self, x: c_uint, y: c_uint) -> c_uint {
        unsafe { sudoku_sys::sgf_findvalueunique(self, x, y) }
    }

    fn getobstruct(&mut self) -> c_uint {
        unsafe { sudoku_sys::sgf_getobstruct(self) }
    }

    fn genboard(&mut self) -> c_int {
        unsafe { sudoku_sys::sgf_genboard(self) }
    }

    fn completeboard(&self) -> c_int {
        unsafe { sudoku_sys::sgf_completeboard(self) }
    }

    fn resetboard(&mut self) {
        unsafe { sudoku_sys::sgf_resetboard(self) }
    }

    fn findboard(&mut self) -> sudoku_sys::sgt_bid {
        unsafe { sudoku_sys::sgf_findboard(self) }
    }

    fn getbid(&self) -> sudoku_sys::sgt_bid {
        unsafe { sudoku_sys::sgf_getbid(self) }
    }

    fn createsudoku(&mut self) {
        unsafe { sudoku_sys::sgf_createsudoku(self) }
    }

    fn createsudoku_rnd(&mut self, sd: c_uint) {
        unsafe { sudoku_sys::sgf_createsudoku_rnd(self, sd) }
    }

    fn setnblank(&mut self, numblank: c_uint) {
        unsafe { sudoku_sys::sgf_setnblank(self, numblank) }
    }

    fn getnblank(&self) -> c_uint {
        unsafe { sudoku_sys::sgf_getnblank(self) }
    }
}

pub fn seed_from_entropy() -> sudoku_sys::URND32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as sudoku_sys::URND32
}

pub mod prelude {
    pub extern crate sudoku_sys;
    pub use crate::sgs_board_impl;
    pub use crate::sgs_game_impl;
    pub use crate::sgs_unit_impl;
    pub use crate::GLIBCRNG_impl;
}
