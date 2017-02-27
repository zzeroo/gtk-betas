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
}
