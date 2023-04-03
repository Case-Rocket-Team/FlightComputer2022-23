use crate::{spi::{devices::flash::W25Q64}, spi_devices};

spi_devices! {
    P1 flash Flash: W25Q64::<SPIInterfaceActiveLow<FlashCS>>
}