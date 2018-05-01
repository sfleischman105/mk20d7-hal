use mk20d7::{self, osc::RegisterBlock};

pub struct Oscillator<'a> {
    osc: &'a RegisterBlock,
}

impl<'a> Oscillator<'a> {
    pub fn new (osc: &'a RegisterBlock) -> Oscillator<'a> {
        Oscillator { osc }
    }

    pub fn is_enabled(&self) -> bool {
        match self.osc.cr.read().erclken() {
            // External reference clock is inactive.
            mk20d7::osc::cr::ERCLKENR::_0 => false,

            // External reference clock is enabled.
            mk20d7::osc::cr::ERCLKENR::_1 => true,
        }
    }

    pub fn enable(&self) {
        self.osc.cr.write(
            |w| {
                w.erclken().set_bit()
            }
        );
    }

    pub fn disable(&self) {
        self.osc.cr.write(
            |w| {
                w.erclken().clear_bit()
            }
        );
    }

    pub fn set_capacitance(&self, capacitance: u8) {
        if capacitance % 2 == 1 || capacitance > 30 {
            panic!("Invalid crystal capacitance value: {}", capacitance)
        }

        let mut binary = [false; 5];
        for i in 0..5 {
            binary[i] = ((capacitance >> i) & 1) == 1;
        }

        self.osc.cr.write(
            |w| {
                if binary[1] { w.sc2p().set_bit(); } else { w.sc2p().clear_bit(); }
                if binary[2] { w.sc4p().set_bit(); } else { w.sc4p().clear_bit(); }
                if binary[3] { w.sc8p().set_bit(); } else { w.sc8p().clear_bit(); }
                if binary[4] { w.sc16p().set_bit(); } else { w.sc16p().clear_bit(); }
                w
            }
        );
    }
}