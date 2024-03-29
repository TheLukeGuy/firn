use crate::{Cpu, System};
use chrono::{DateTime, Datelike, Timelike, Utc};
use firn_core::device::{Device, PortRequest, PortResponse};
use std::time;
use std::time::{Duration, SystemTime};

pub const SECONDS_REG: usize = 0x00;
pub const MINUTES_REG: usize = 0x02;
pub const HOURS_REG: usize = 0x04;
pub const DAY_OF_WEEK_REG: usize = 0x06;
pub const DAY_OF_MONTH_REG: usize = 0x07;
pub const MONTH_REG: usize = 0x08;
pub const YEAR_REG: usize = 0x09;

pub const STATUS_REG_A: usize = 0x0a;
pub const STATUS_REG_B: usize = 0x0b;

// TODO: Fully honor STATUS_REG_A and STATUS_REG_B values
pub struct Cmos {
    selected_reg: u8,
    regs: [u8; 128],

    sync_time: DateTime<Utc>,
    start_time: Option<time::Duration>,

    last_update_micros: u128,
}

impl Cmos {
    pub fn new(start_time: DateTime<Utc>) -> Self {
        Self {
            selected_reg: 0xd,
            regs: [0; 128],

            sync_time: start_time,
            start_time: None,

            last_update_micros: 0,
        }
    }

    pub fn new_current_time() -> Self {
        Self::new(Utc::now())
    }

    pub fn sync(&mut self) {
        let start_time = self
            .start_time
            .expect("cannot sync real-time clock without a start time");
        let current_time = self.current_time();

        let difference = chrono::Duration::from_std(current_time - start_time)
            .expect("time difference is too large to be synced");
        let now = self.sync_time + difference;

        self.start_updating_rtc();
        self.regs[SECONDS_REG] = now.second() as u8;
        self.regs[MINUTES_REG] = now.minute() as u8;
        self.regs[HOURS_REG] = now.hour() as u8;
        self.regs[DAY_OF_WEEK_REG] = now.weekday().number_from_sunday() as u8;
        self.regs[DAY_OF_MONTH_REG] = now.day() as u8;
        self.regs[MONTH_REG] = now.month() as u8;
        self.regs[YEAR_REG] = (now.year() % 100) as u8;
        self.stop_updating_rtc();
    }

    fn current_time(&self) -> Duration {
        SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("time went backwards")
    }

    fn start_updating_rtc(&mut self) {
        self.regs[STATUS_REG_A] |= 0x80;
        self.last_update_micros = self.current_time().as_micros();
    }

    fn stop_updating_rtc(&mut self) {
        self.regs[STATUS_REG_A] &= !0x80;
    }

    fn days_in_month(&self, month: u8, year: u8) -> u8 {
        // TODO: Properly account for leap years
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            2 if year % 4 == 0 => 29,
            2 => 28,
            _ => 30,
        }
    }
}

// TODO: Remove debug messages
impl Cmos {
    pub fn select_reg(&mut self, value: u8) -> PortResponse {
        // TODO: Implement NMI disable
        let _nmi_disable = value >> 7;
        self.selected_reg = value & !0x80;

        println!("Using CMOS register: {:#x}", self.selected_reg);

        PortResponse::Out
    }

    pub fn reg_value(&mut self) -> PortResponse {
        let value = self.regs[self.selected_reg as usize];

        println!("Read from CMOS register");

        PortResponse::In8(value)
    }

    pub fn set_reg_value(&mut self, value: u8) -> PortResponse {
        self.regs[self.selected_reg as usize] = value;

        println!("Wrote {:#x} to CMOS register", value);

        PortResponse::Out
    }
}

impl Device<Cpu> for Cmos {
    fn init(&mut self, _sys: &mut System) {
        let start_time = self.current_time();
        self.start_time = Some(start_time);

        self.sync();
    }

    fn step(&mut self, _sys: &mut System) {
        let current_time = self.current_time().as_micros();
        if current_time - self.last_update_micros < 1_000_000 {
            return;
        }

        let mut seconds = self.regs[SECONDS_REG] + 1;
        let mut minutes = self.regs[MINUTES_REG];
        let mut hours = self.regs[HOURS_REG];
        let mut day_of_week = self.regs[DAY_OF_WEEK_REG];
        let mut day_of_month = self.regs[DAY_OF_MONTH_REG];
        let mut month = self.regs[MONTH_REG];
        let mut year = self.regs[YEAR_REG];

        if seconds >= 60 {
            seconds = 0;
            minutes += 1;
        }
        if minutes >= 60 {
            minutes = 0;
            hours += 1;
        }
        if hours >= 24 {
            hours = 0;
            day_of_week += 1;
            day_of_month += 1;
        }
        if day_of_week > 7 {
            day_of_week = 1;
        }
        if day_of_month > self.days_in_month(month, year) {
            day_of_month = 0;
            month += 1;
        }
        if month > 12 {
            month = 0;
            year += 1;
        }
        if year > 99 {
            // TODO: Prepare for Y2K :flushed:
            year = 0;
        }

        self.start_updating_rtc();
        self.regs[SECONDS_REG] = seconds;
        self.regs[MINUTES_REG] = minutes;
        self.regs[HOURS_REG] = hours;
        self.regs[DAY_OF_WEEK_REG] = day_of_week;
        self.regs[DAY_OF_MONTH_REG] = day_of_month;
        self.regs[MONTH_REG] = month;
        self.regs[YEAR_REG] = year;
        self.stop_updating_rtc();
    }

    fn handle_port(&mut self, _sys: &mut System, request: PortRequest) -> Option<PortResponse> {
        match request {
            PortRequest::Out8(0x70, value) => Some(self.select_reg(value)),
            PortRequest::In8(0x71) => Some(self.reg_value()),
            PortRequest::Out8(0x71, value) => Some(self.set_reg_value(value)),
            _ => None,
        }
    }
}
