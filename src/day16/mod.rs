/*
Day 16: Packet Decoder

You receive a tranmission in hex format with a specific encoding.
Bits represent a version number and a type. The type determines what the next bits are.
Operator types can have multiple sub packets.

Part 1: Add together all the version numbers for every packet and sub packet

Part 2: Using rules for each operator type (sum, product, etc.), calculate the packet value.
*/

use std::fs;
use std::collections::HashMap;

// Packet represented by a struct
// Value is optional and only in type_id 4
// sub_packets are only present in type_id != 4, empty otherwise
// Avoid rust borrow issues by having the Packet own the sub packets
pub struct Packet {
    version: i32,
    type_id: i32,
    value: Option<i64>,
    sub_packets: Vec<Packet>
}

impl Packet {
    // Part 1: Count all the packet version numbers by recursively calling into sub packets
    pub fn count_version(&self) -> i32 {
        self.version + self.sub_packets.iter().map(|p| p.count_version()).sum::<i32>()
    }

    // Part 2: Calculate operations depend on the type_id
    // The tree like nature of the Packet struct makes this pretty straightforward
    pub fn calculate(&self) -> i64 {
        return match self.type_id {
            4 => self.value.unwrap(),
            0 => self.sub_packets.iter().map(|p| p.calculate()).sum(),
            1 => self.sub_packets.iter().map(|p| p.calculate()).product(),
            2 => self.sub_packets.iter().map(|p| p.calculate()).min().unwrap(),
            3 => self.sub_packets.iter().map(|p| p.calculate()).max().unwrap(),
            5 => if self.sub_packets[0].calculate() > self.sub_packets[1].calculate() { 1 } else { 0 },
            6 => if self.sub_packets[0].calculate() < self.sub_packets[1].calculate() { 1 } else { 0 },
            7 => if self.sub_packets[0].calculate() == self.sub_packets[1].calculate() { 1 } else { 0 },
            _ => panic!("unknown type")

        };
    }
}

// Converts our hex string into an array of chars that are either '0' or '1'
// Maybe it would be better to do bytes and bitwise operations, but I'm not super familiar with that in Rust
fn parse_hex_packet(hex_string: &str) -> Packet {
    let hex_map: HashMap<char, &str> = ('0'..='9').chain('A'..='F')
        .zip(vec!["0000","0001","0010","0011","0100","0101","0110","0111","1000","1001","1010","1011","1100","1101","1110","1111"])
        .collect();

    let binary: Vec<_> = hex_string.chars().map(|c| hex_map[&c]).collect();
    let binary: Vec<char> = binary.join("").chars().collect();
    parse_packet(&binary[..]).0
}

// Recursive method to parse the binary bit array into packets and sub packets
// Returns the packet and the number of bits it took to create the packet
fn parse_packet(binary: &[char]) -> (Packet, usize) {
    //Version and type_id are common to all packets
    let version: String = binary[..3].iter().collect();
    let version = i32::from_str_radix(&version, 2).unwrap();
    let type_id: String = binary[3..6].iter().collect();
    let type_id = i32::from_str_radix(&type_id, 2).unwrap();

    // Value type packet
    if type_id == 4 {
        let mut idx = 6;
        let mut chunks: Vec<char> = Vec::new();
        let mut next = &binary[idx..idx+5];
        // Loop through 5 bit chunks until the first bit is 0
        loop {
            // grab the last 4 bits, discarding the first one
            chunks.extend_from_slice(&next[1..]);
            idx += 5;
            if next[0] == '0' {
                break;
            }
            next = &binary[idx..idx+5];
        }
        let value: String = chunks.iter().collect();
        let value =  i64::from_str_radix(&value, 2).unwrap();
        return (Packet { version, type_id, value: Some(value), sub_packets: vec![] }, idx); 

    }// Operator type packet
    else {
        let length_id = binary[6];
        let length: usize = match length_id {
            '0' => 15,
            _ => 11
        };
        let mut sub_start = 7 + length;
        let length: String = binary[7..sub_start].iter().collect();

        // Length calculations will depend on length_id
        // but either way, loop until we have all sub packets
        let mut length = i32::from_str_radix(&length, 2).unwrap();
        let mut sub_packets: Vec<Packet> = Vec::new();
        while length > 0 {
            // pass down the bits not used yet to get the next sub packet
            let (p, bits) = parse_packet(&binary[sub_start..]);
            sub_packets.push(p);
            // the next sub packet will index after the end of the previous one
            sub_start += bits;
            if length_id == '0' {
                // For length_id 0, length represents the total bits in the sub packets
                length -= bits as i32;
            } else {
                // for length_id 1, length represents the number of sub packets
                length -= 1;
            }
        }
        (Packet { version, type_id, value: None, sub_packets }, sub_start)
    }
}

pub fn read_packet() -> Packet {
    let input = fs::read_to_string("src/day16/packets.txt").expect("missing packet.txt");
    parse_hex_packet(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        let packet = parse_hex_packet("D2FE28");
        assert_eq!(6, packet.version);
        assert_eq!(4, packet.type_id);
        assert_eq!(2021, packet.value.unwrap());

        let packet = parse_hex_packet("38006F45291200");
        assert_eq!(1, packet.version);
        assert_eq!(6, packet.type_id);
        assert_eq!(2, packet.sub_packets.len());
        assert_eq!(10, packet.sub_packets[0].value.unwrap());
        assert_eq!(20, packet.sub_packets[1].value.unwrap());

        let packet = parse_hex_packet("EE00D40C823060");
        assert_eq!(7, packet.version);
        assert_eq!(3, packet.type_id);
        assert_eq!(3, packet.sub_packets.len());
        assert_eq!(1, packet.sub_packets[0].value.unwrap());
        assert_eq!(2, packet.sub_packets[1].value.unwrap());
        assert_eq!(3, packet.sub_packets[2].value.unwrap());
    }

    #[test]
    fn test_count_packet_version() {
        let packet = parse_hex_packet("8A004A801A8002F478");
        assert_eq!(16, packet.count_version());

        let packet = parse_hex_packet("620080001611562C8802118E34");
        assert_eq!(12, packet.count_version());

        let packet = parse_hex_packet("C0015000016115A2E0802F182340");
        assert_eq!(23, packet.count_version());

        let packet = parse_hex_packet("A0016C880162017C3686B18A3D4780");
        assert_eq!(31, packet.count_version());
    }

    #[test]
    fn test_packet_calculation() {
        let packet = parse_hex_packet("C200B40A82");
        assert_eq!(3, packet.calculate());

        let packet = parse_hex_packet("04005AC33890");
        assert_eq!(54, packet.calculate());

        let packet = parse_hex_packet("880086C3E88112");
        assert_eq!(7, packet.calculate());

        let packet = parse_hex_packet("CE00C43D881120");
        assert_eq!(9, packet.calculate());

        let packet = parse_hex_packet("D8005AC2A8F0");
        assert_eq!(1, packet.calculate());

        let packet = parse_hex_packet("F600BC2D8F");
        assert_eq!(0, packet.calculate());

        let packet = parse_hex_packet("9C005AC2F8F0");
        assert_eq!(0, packet.calculate());

        let packet = parse_hex_packet("9C0141080250320F1802104A08");
        assert_eq!(1, packet.calculate());
    }
}

