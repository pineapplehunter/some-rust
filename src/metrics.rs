use core::{fmt, ops::Sub};

#[derive(Debug, Clone)]
pub struct MetricsInstant {
    thread_id: usize,
    cycle: usize,
    instret: usize,
    executor: usize,
    load: usize,
    store: usize,
    amo: usize,
    lr: usize,
    sc: usize,
    sc_fail: usize,
    error: usize,
    pext: usize,
    reorder_buffer_full: usize,
    load_store_full: usize,
    atomic_full: usize,
    reservation_station_full: usize,
}

#[derive(Debug, Clone)]
pub struct Metrics(MetricsInstant);

impl Metrics {
    pub fn csv(&self) -> MetricsCSV {
        MetricsCSV(&self.0)
    }
}

impl MetricsInstant {
    pub fn get_instant() -> Self {
        MetricsInstant {
            thread_id: riscv::register::mhartid::read(),
            cycle: riscv::register::cycle::read(),
            instret: riscv::register::instret::read(),
            executor: riscv::register::hpmcounter3::read(),
            load: riscv::register::hpmcounter4::read(),
            store: riscv::register::hpmcounter5::read(),
            amo: riscv::register::hpmcounter6::read(),
            lr: riscv::register::hpmcounter7::read(),
            sc: riscv::register::hpmcounter8::read(),
            sc_fail: riscv::register::hpmcounter9::read(),
            error: riscv::register::hpmcounter10::read(),
            pext: riscv::register::hpmcounter11::read(),
            reorder_buffer_full: riscv::register::hpmcounter12::read(),
            load_store_full: riscv::register::hpmcounter13::read(),
            atomic_full: riscv::register::hpmcounter14::read(),
            reservation_station_full: riscv::register::hpmcounter15::read(),
        }
    }

    pub fn csv(&self) -> MetricsCSV {
        MetricsCSV(self)
    }
}

impl Sub for MetricsInstant {
    type Output = Metrics;

    fn sub(self, rhs: Self) -> Self::Output {
        Metrics(Self {
            thread_id: self.thread_id,
            cycle: self.cycle - rhs.cycle,
            instret: self.instret - rhs.instret,
            executor: self.executor - rhs.executor,
            load: self.load - rhs.load,
            store: self.store - rhs.store,
            amo: self.amo - rhs.amo,
            lr: self.lr - rhs.lr,
            sc: self.sc - rhs.sc,
            sc_fail: self.sc_fail - rhs.sc_fail,
            error: self.error - rhs.error,
            pext: self.pext - rhs.pext,
            reorder_buffer_full: self.reorder_buffer_full - rhs.reorder_buffer_full,
            load_store_full: self.load_store_full - rhs.load_store_full,
            atomic_full: self.atomic_full - rhs.atomic_full,
            reservation_station_full: self.reservation_station_full - rhs.reservation_station_full,
        })
    }
}

pub fn get_metrics<R, F: FnOnce() -> R>(f: F) -> (Metrics, R) {
    let before = MetricsInstant::get_instant();
    let r = f();
    let after = MetricsInstant::get_instant();
    (after - before, r)
}

pub struct MetricsCSV<'a>(&'a MetricsInstant);

impl MetricsCSV<'_> {
    pub const HEADER: &'static str = "thread_id,cycle,instret,executor,load,store,amo,lr,sc,sc_fail,error,pext,reorderbuffer_full,load_store_full,atomic_full,reservation_station_full";
}

impl<'a> fmt::Display for MetricsCSV<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            self.0.thread_id,
            self.0.cycle,
            self.0.instret,
            self.0.executor,
            self.0.load,
            self.0.store,
            self.0.amo,
            self.0.lr,
            self.0.sc,
            self.0.sc_fail,
            self.0.error,
            self.0.pext,
            self.0.reorder_buffer_full,
            self.0.load_store_full,
            self.0.atomic_full,
            self.0.reservation_station_full
        )
    }
}
