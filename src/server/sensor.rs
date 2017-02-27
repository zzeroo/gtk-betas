

pub struct Sensor {
    adc_value: u32,
    average: u32,
}

impl Sensor {
    pub fn new() -> Self {
        Sensor {
            adc_value: 0,
            average: 0,
        }
    }

    pub fn get_adc_value(&self) -> u32 {
        self.adc_value
    }

    pub fn get_average(&self) -> u32 {
        self.average
    }

    pub fn set_adc_value(&mut self, adc_value: u32) {
        if adc_value < 1024 {
            self.adc_value = adc_value;
        } else {
            self.adc_value = 1023;
        }
    }
}
