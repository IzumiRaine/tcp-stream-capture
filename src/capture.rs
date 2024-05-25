use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::capture::ffi::IpAddressVersion;

use self::ffi::IpAddress;

#[cxx::bridge(namespace = "tcp_stream_capture")]
pub(crate) mod ffi {
    pub(crate) struct LiveDevice {
        m_device: *mut PcapLiveDevice,
    }

    #[derive(Debug)]
    pub struct MacAddress {
        pub bytes: [u8; 6],
    }

    #[derive(Debug)]
    pub(crate) struct OptionMacAddress {
        value: MacAddress,
        valid: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Ipv4Address {
        pub(crate) bytes: [u8; 4],
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Ipv6Address {
        pub(crate) bytes: [u8; 16],
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) enum IpAddressVersion {
        V4,
        V6,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct IpAddress {
        bytes: [u8; 16],
        version: IpAddressVersion,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct TcpConnection {
        src_addr: IpAddress,
        dst_addr: IpAddress,
        src_port: u16,
        dst_port: u16,
        flow_key: u32,
        start_time_s: i64,
        start_time_us: i64,
        end_time_s: i64,
        end_time_us: i64,
    }


    unsafe extern "C++" {
        include!("tcp_stream_capture/src/capture.h");

        #[namespace = "pcpp"]
        type PcapLiveDevice;

        fn get_live_devices() -> Vec<LiveDevice>;

        /// Returns NULL if no such device exists.
        fn find_by_name(name: &str) -> LiveDevice;
        fn find_by_ip(ip: &str) -> LiveDevice;
        fn find_by_ip_or_name(ip_or_name: &str) -> LiveDevice;

        fn name(self: &LiveDevice) -> Result<String>;
        fn mac_address(self: &LiveDevice) -> OptionMacAddress;
        fn ipv4_address(self: &LiveDevice) -> Ipv4Address;
        fn ipv6_address(self: &LiveDevice) -> Ipv6Address;
        fn ip_addresses(self: &LiveDevice) -> Vec<IpAddress>;

        /*
        type LiveDeviceList;
        fn new_live_device_list() -> UniquePtr<LiveDeviceList>;
        #[cxx_name = "size"]
        fn len(self: &LiveDeviceList) -> usize;
        fn get(self: &LiveDeviceList, i: usize) -> LiveDevice;
        */
    }
}

impl ffi::LiveDevice {
    pub(crate) fn is_null(&self) -> bool
    {
        self.m_device.is_null()
    }
}

impl Display for ffi::MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.bytes[0],
            self.bytes[1],
            self.bytes[2],
            self.bytes[3],
            self.bytes[4],
            self.bytes[5],
        )
    }
}

impl ffi::OptionMacAddress {
    pub(crate) fn as_option(self) -> Option<ffi::MacAddress>
    {
        if self.valid {
            Some(self.value)
        } else {
            None
        }
    }
}

impl From<IpAddress> for IpAddr {
    fn from(value: IpAddress) -> Self {
        match value.version {
            IpAddressVersion::V4 => {
                let bytes = <[u8; 4]>::try_from(&value.bytes[0..4]).unwrap();
                Ipv4Addr::from(bytes).into()
            }
            IpAddressVersion::V6 => Ipv6Addr::from(value.bytes).into(),
            _ => panic!("unexpected IpAddressVersion"),
        }
    }
}
