
use sn_rust::mobile_entity_field_2_d::MobileEntityField2D;

use crate::{ticka::Ticka, unit::Unit};

pub struct RealTimeTickaFascade {
    ticka: Ticka,

    // ticke length in seconds.
    tick_length: f64,

    // how many ticks per second should occur ?
    ticks_per_second: f64,

    last_processed_tick: f64,
    // since game loops fluctuate in their tick time,
    // we need to store the offset to the last tick.
    last_tick_adjustement: f64,
}

impl RealTimeTickaFascade {
    pub fn from_ticka(ticka: Ticka, ticks_per_second: f64) -> Self {
        let tick_length = 1.0 / ticks_per_second;
        RealTimeTickaFascade {
            ticka,
            last_processed_tick: 0.0,
            tick_length,
            ticks_per_second,
            last_tick_adjustement: 0.0,
        }
    }

    pub fn tick_if_time_has_come(&mut self, current_time: f64) {
        if self.last_processed_tick + self.tick_length < current_time {
            self.last_processed_tick = current_time;
            self.ticka.tick();
        }
    }
    pub fn units_mut(&mut self) -> &mut MobileEntityField2D<Unit> {
        self.ticka.units_mut()
    }

    pub fn units(&self) -> &MobileEntityField2D<Unit> {
        self.ticka.units()
    }

    pub fn ticka_mut(&mut self) -> &mut Ticka {
        &mut self.ticka
    }
}
