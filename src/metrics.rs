use core::{fmt, ops::Sub};

#[derive(Debug, Clone)]
pub struct Metrics {
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
}

impl Metrics {
    pub fn get_instant() -> Self {
        Metrics {
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
        }
    }

    pub fn csv(&self) -> MetricsCSV {
        MetricsCSV(self)
    }
}

impl Sub for Metrics {
    type Output = Metrics;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
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
        }
    }
}

pub fn get_metrics<R, F: FnOnce() -> R>(f: F) -> (Metrics, R) {
    let before = Metrics::get_instant();
    let r = f();
    let after = Metrics::get_instant();
    (after - before, r)
}

pub struct MetricsCSV<'a>(&'a Metrics);

impl MetricsCSV<'_> {
    pub const HEADER: &'static str =
        "thread_id,cycle,instret,executor,load,store,amo,lr,sc,sc_fail,error,pext";
}

impl<'a> fmt::Display for MetricsCSV<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{}",
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
        )
    }
}
