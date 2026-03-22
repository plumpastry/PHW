#![no_std]

use embedded_hal::i2c::I2c;

pub struct Drv2625<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Drv2625<I2C> where I2C: I2c<Error = E> {
    pub fn new_i2c(i2c: I2C, address: u8) -> Self {
        Drv2625 { i2c, address }
    }

    pub fn auto_calibrate(
        &mut self,
        actuator: ActuatorType,
        voltage_setting: u8,
        clamp_setting: u8,
        drive_time: u8,
    ) -> Result<(), E> {
        self.set_mode(Mode::AutoCalibration)?;
        self.set_lra_erm(actuator)?;
        // Brake factor goes here
        // Loop gain goes here
        self.set_rated_voltage(voltage_setting)?;
        self.set_od_clamp(clamp_setting)?;
        // Auto cal time goes here
        self.set_drive_time(drive_time)?;
        self.set_go_bit(1)?;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode) -> Result<(), E> {
        let mut buf = [0];
        self.i2c.write_read(self.address, &[0x07], &mut buf)?;
        let masked_data = buf[0] & 0b11111100;
        let updated_data = masked_data | mode as u8;
        self.i2c.write(self.address, &[0x07, updated_data])?;
        Ok(())
    }

    pub fn set_lra_erm(&mut self, actuator: ActuatorType) -> Result<(), E> {
        let mut buf = [0];
        self.i2c.write_read(self.address, &[0x08], &mut buf)?;
        let masked_data = buf[0] & 0b01111111;
        let updated_data = masked_data | ((actuator as u8) << 7);
        self.i2c.write(self.address, &[0x08, updated_data])?;
        Ok(())
    }

    fn set_rated_voltage(&mut self, voltage_setting: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x1F, voltage_setting])?;
        Ok(())
    }

    fn set_od_clamp(&mut self, clamp_setting: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x20, clamp_setting])?;
        Ok(())
    }

    fn set_drive_time(&mut self, drive_time: u8) -> Result<(), E> {
        let mut buf = [0];
        self.i2c.write_read(self.address, &[0x27], &mut buf)?;
        let masked_data = buf[0] & 0b11100000;
        let drive_time = (drive_time << 3) >> 3;
        let updated_data = masked_data | drive_time as u8;
        self.i2c.write(self.address, &[0x27, updated_data])?;
        Ok(())
    }

    fn set_go_bit(&mut self, go_bit: u8) -> Result<(), E> {
        let mut buf = [0];
        self.i2c.write_read(self.address, &[0x0C], &mut buf)?;
        let masked_data = buf[0] & 0b1111110;
        let go_bit = (go_bit << 7) >> 7;
        let updated_data = masked_data | go_bit as u8;
        self.i2c.write(self.address, &[0x0C, updated_data])?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode {
    RTP = 0,
    Waveform = 1,
    Diagnostics = 2,
    AutoCalibration = 3,
}

pub enum ActuatorType {
    ERM = 0,
    LRA = 1,
}