use server::Kombisensor;


pub struct Server<'a> {
    serial_interface: &'a str,
    kombisensors: Vec<Kombisensor>,
}

impl<'a> Server<'a> {
    pub fn new() -> Self {
        Server {
            serial_interface: "/dev/ttyUSB0",
            kombisensors: vec![],
        }
    }

    /// Update Kombisensors via Modbus
    pub fn update(&mut self) {
        for mut kombisensor in self.kombisensors.iter_mut() {
            kombisensor.update();
        }
    }

    /// Simulation 1
    /// FIXME: Doku fehlt
    pub fn simulation(&mut self) {
        extern crate rand;
        use rand::Rng;

        for mut kombisensor in self.kombisensors.iter_mut() {
            for mut sensor in kombisensor.get_sensors_mut().iter_mut() {
                sensor.set_adc_value(rand::thread_rng().gen_range(0, 1023));
            }
        }
    }
}
