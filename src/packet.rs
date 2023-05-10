use std::net::IpAddr;

pub struct IPPacket<'a>(&'a [u8]);

impl<'a> IPPacket<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        IPPacket(data)
    }

    pub fn get_size(&self) -> usize {
        (self.0[3] as usize) | ((self.0[2] as usize) << 8)
    }

    // IPver returns 4 or 6 for IPv4 or IPv6
    pub fn ip_ver(&self) -> u8 {
        let version = self.0[0] >> 4;
        match version {
            4 | 6 => version,
            _ => 0,
        }
    }

    // Dst returns [4]u8 for destination of packet
    pub fn dst(&self) -> [u8; 4] {
        [self.0[16], self.0[17], self.0[18], self.0[19]]
    }

    // DstV4 returns std::net::IpAddr for destination of packet
    pub fn dst_v4(&self) -> IpAddr {
        IpAddr::V4(std::net::Ipv4Addr::new(self.0[16], self.0[17], self.0[18], self.0[19]))
    }

    // Src returns [4]u8 for source address of packet
    pub fn src(&self) -> [u8; 4] {
        [self.0[12], self.0[13], self.0[14], self.0[15]]
    }

    // IsMulticast returns if IP destination looks like multicast
    pub fn is_multicast(&self) -> bool {
        (self.0[16] > 223) && (self.0[16] < 240)
    }
}

