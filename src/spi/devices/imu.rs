use crate::{spi::spi_manager::{SPIInterface, SPIDevice}, spi_transfer, util::Any};

const ICM20948_READ_BIT: u8 = 0x80;

enum Register {
    ACCEL_ID = 0x00,
    MAG_ID = 0x01,
    USER_CTRL = 0x03,
    LP_CONFIG = 0x05,
    PWR_1 = 0x06,
    PWR_2 = 0x07,
    ACCEL_XOUT_H = 0x2d,
    ACCEL_XOUT_L = 0x2e,
    ACCEL_YOUT_H = 0x2f,
    ACCEL_YOUT_L = 0x30,
    ACCEL_ZOUT_H = 0x31,
    ACCEL_ZOUT_L = 0x32,
    GYRO_XOUT_H = 0x33,
    GYRO_XOUT_L = 0x34,
    GYRO_YOUT_H = 0x35,
    GYRO_YOUT_L = 0x36,
    GYRO_ZOUT_H = 0x37,
    GYRO_ZOUT_L = 0x38,
    MAG_XOUT_L = 0x11,
    MAG_XOUT_H = 0x12,
    MAG_YOUT_L = 0x13,
    MAG_YOUT_H = 0x14,
    MAG_ZOUT_L = 0x15,
    MAG_ZOUT_H = 0x16,
    MAG_STAT_1 = 0x10,
    MAG_STAT_2 = 0x18,
    MAG_CTRL_1 = 0x31,
    MAG_RESET = 0x32,
    TEMP_OUT = 0x3a
}

enum SensorType {
    ACCEL,
    GYRO,
    MAG
}

enum Axis {
    X_AXIS,
    Y_AXIS,
    Z_AXIS
}

enum Error {
    NO_READ
}

pub struct IMU<TInterface: SPIInterface> {
    interface: TInterface
}

impl<TInterface: SPIInterface> IMU<TInterface> {
    pub fn new(interface: TInterface) -> IMU<TInterface> {
        let mut icm = IMU{
            interface
        };
        let id = IMU::<TInterface>::read_register(&mut icm, Register::ACCEL_ID);
        if id != 0xEA {
            log::error!("no IMU!");
            panic!("ahh");
        }
        icm
    }
    fn read_register(&mut self, register: Register) -> u8 {
        let mut buffer = [0u8; 2];
        buffer[0] = register as u8 | ICM20948_READ_BIT;
        spi_transfer!(self.interface, &mut buffer);
        buffer[1]
    }
    fn write_register(&mut self, register: Register, data: u8) {
        let mut buffer = [0u8; 2];
        buffer[0] = register as u8;
        buffer[1] = data;
        spi_transfer!(self.interface, &mut buffer);
    }
    fn get_sensor_value(sensor: SensorType, a: Axis) -> i16 {
        match sensor {
            ACCEL=>
                match a {
                    X_AXIS=>read_register(&mut self, Register::ACCEL_XOUT_H) << 8 | read_register(&mut self, Register::ACCEL_XOUT_L),
                    Y_AXIS=>read_register(&mut self, Register::ACCEL_YOUT_H) << 8 | read_register(&mut self, Register::ACCEL_YOUT_L),
                    Z_AXIS=>read_register(&mut self, Register::ACCEL_ZOUT_H) << 8 | read_register(&mut self, Register::ACCEL_ZOUT_L)
                },
            GYRO=>
                match a {
                    X_AXIS=>read_register(&mut self, Register::GYRO_XOUT_H) << 8 | read_register(&mut self, Register::GYRO_XOUT_L),
                    Y_AXIS=>read_register(&mut self, Register::GYRO_YOUT_H) << 8 | read_register(&mut self, Register::GYRO_YOUT_L),
                    Z_AXIS=>read_register(&mut self, Register::GYRO_ZOUT_H) << 8 | read_register(&mut self, Register::GYRO_ZOUT_L)
                },
            MAG=>
                match a {
                    X_AXIS=>read_register(&mut self, Register::MAG_XOUT_H) << 8 | read_register(&mut self, Register::MAG_XOUT_L),
                    Y_AXIS=>read_register(&mut self, Register::MAG_YOUT_H) << 8 | read_register(&mut self, Register::MAG_YOUT_L),
                    Z_AXIS=>read_register(&mut self, Register::MAG_ZOUT_H) << 8 | read_register(&mut self, Register::MAG_ZOUT_L)
                }
        }
    }

}

