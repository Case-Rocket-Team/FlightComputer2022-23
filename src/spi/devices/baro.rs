use crate::{spi::spi_manager::{SPIInterface, SPIDevice}, spi_transfer, util::Any};

const BMP388_READ_BIT: u8 = 0x80;
const BMP388_ADDRESS: u8 = 0x77;

pub enum BaroRegister {
    CHIP_ID = 0x00,
    ERR_REG = 0x02,
    STATUS = 0x03,
    PRESSURE_DATA = 0x04,
    TEMP_DATA = 0x07,
    INT_CTRL = 0x11,
    INT_STATUS = 0x12,
    FIFO_LENGTH = 0x14,
    FIFO_DATA = 0x15,
}

enum Error {
    NO_READ
}

pub struct Baro<TInterface: SPIInterface> {
    interface: TInterface
}

impl<TInterface: SPIInterface> Baro<TInterface> {
    pub fn new(interface: TInterface) -> Baro<TInterface> {
        Baro {
            interface
        }
    }

    pub fn read_register(&mut self, register: BaroRegister) -> u8 {
        let mut buffer = [0b1000_0000, 0, 0, 0, 0];
        spi_transfer!(self.interface, &mut buffer);
        buffer[2]
    }

}
