use esp_idf_sys as _; // For binstart

use esp_idf_hal::rom::crc;

fn main() {
    esp_idf_sys::link_patches();

    let data = "123456789";

    loop {
        let crc_hdlc = crc::crc32_le(!0xffffffff, data.as_ref());
        let crc_bzip2 = crc::crc32_be(!0xffffffff, data.as_ref());
        let crc_mpeg2 = !crc::crc32_be(!0xffffffff, data.as_ref());
        let crc_cksum = crc::crc32_be(!0, data.as_ref());
        let crc_kermit = !crc::crc16_le(!0, data.as_ref());
        let crc_genibus = crc::crc16_be(!0xffff, data.as_ref());
        let crc_rohc = !crc::crc8_le(!0xff, data.as_ref());
        let crc_smbus = !crc::crc8_be(!0, data.as_ref());

        assert_eq!(crc_hdlc, 0xcbf43926);
        assert_eq!(crc_bzip2, 0xfc891918);
        assert_eq!(crc_mpeg2, 0x0376e6e7);
        assert_eq!(crc_cksum, 0x765e7680);
        assert_eq!(crc_kermit, 0x2189);
        assert_eq!(crc_genibus, 0xd64e);
        assert_eq!(crc_rohc, 0xd0);
        assert_eq!(crc_smbus, 0xf4);

        println!("{:08x} {:08x} {:08x} {:08x} {:04x} {:04x} {:02x} {:02x}",
            crc_hdlc, crc_bzip2, crc_mpeg2, crc_cksum, crc_kermit, crc_genibus, crc_rohc, crc_smbus);

        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}
