use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34056));
}

pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_v(max)
}

struct RandGen {
    current: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandGen {
    pub fn new(current: usize) -> Self {
        Self {
            current,
            mul: 57489346,
            inc: 374589567,
            modulo: 26343359860,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.current = (self.current * self.mul + self.inc) % self.modulo;
        self.current % max
    }
}
