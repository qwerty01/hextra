use config::HexConfig;
use decode::printable::BasicDecoder;

pub mod config;
pub mod decode;

pub struct HexDump<'a, D> {
    data: &'a [u8],
    config: HexConfig<D>,
}

pub trait HexDumpExt {
    fn hex_dump<D>(&self) -> HexDump<BasicDecoder> {
        self.hex_dump_cfg(HexConfig::default())
    }
    fn hex_dump_cfg<D>(&self, config: HexConfig<D>) -> HexDump<D>;
}
impl<T: AsRef<[u8]>> HexDumpExt for T {
    fn hex_dump_cfg<D>(&self, config: HexConfig<D>) -> HexDump<D> {
        HexDump {
            data: self.as_ref(),
            config,
        }
    }
}
