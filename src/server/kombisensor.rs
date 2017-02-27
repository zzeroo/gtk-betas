use server::Sensor;
use errors::*;

pub struct Kombisensor {
    modbus_slave_id: u32,
    sensors: Vec<Sensor>,
}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            modbus_slave_id: 247,
            sensors: vec![],
        }
    }

    pub fn update(&mut self) {
        for mut sensor in self.get_sensors_mut() {
            sensor.set_adc_value(1);
        }
    }

    pub fn get_modbus_slave_id(&self) -> u32 {
        self.modbus_slave_id
    }

    pub fn get_sensors(&self) -> &Vec<Sensor> {
        &self.sensors
    }

    pub fn get_sensors_mut(&mut self) -> &mut Vec<Sensor> {
        &mut self.sensors
    }


    pub fn set_modbus_slave_id(&mut self, modbus_slave_id: u32) -> Result<()> {
        if modbus_slave_id > 0 && modbus_slave_id < 247 {
            self.modbus_slave_id = modbus_slave_id;
        } else {
            bail!("Modbus Slave ID muss zwischen 1 und 247 liegen.")
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use server::Kombisensor;

    #[test]
    fn test_modbus_slave_id_valid_input() {
        let mut kombisensor = Kombisensor::new();
        assert!(kombisensor.set_modbus_slave_id(1).is_ok());
    }

    #[test]
    fn test_modbus_slave_id_invalid_input() {
        let mut kombisensor = Kombisensor::new();
        assert!(kombisensor.set_modbus_slave_id(0).is_err());
        assert!(kombisensor.set_modbus_slave_id(255).is_err());
    }

}
