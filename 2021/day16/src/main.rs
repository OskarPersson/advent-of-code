fn parse_input(contents: &str) -> String {
    contents
        .trim()
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => {
                unreachable!()
            }
        })
        .collect()
}

#[derive(Clone)]
struct Packet {
    pub bits: String,
    sub_packets: Vec<Packet>,
    type_id: i32,
}

impl Packet {
    fn new(s: String, sub_packets: Vec<Packet>) -> Self {
        Packet {
            bits: s.clone(),
            sub_packets,
            type_id: get_type_id(&s),
        }
    }

    fn len(&self) -> usize {
        self.bits.len()
    }

    fn literal_value(&self) -> i64 {
        let chars = self.bits.chars().skip(6).collect::<Vec<char>>();
        let s = chars
            .chunks(5)
            .map(|grp| grp.iter().skip(1).collect::<String>())
            .collect::<String>();
        i64::from_str_radix(&s, 2).unwrap()
    }

    fn value(&self) -> i64 {
        match self.type_id {
            0 => self.sub_packets.iter().map(|p| p.value()).sum(),
            1 => self.sub_packets.iter().map(|p| p.value()).product(),
            2 => self.sub_packets.iter().map(|p| p.value()).min().unwrap(),
            3 => self.sub_packets.iter().map(|p| p.value()).max().unwrap(),
            4 => self.literal_value(),
            5 => {
                if self.sub_packets[0].value() > self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.sub_packets[0].value() < self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.sub_packets[0].value() == self.sub_packets[1].value() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }
}

fn binary_str_to_int(s: &str) -> i32 {
    i32::from_str_radix(s, 2).unwrap()
}

fn get_version(s: &str) -> i32 {
    binary_str_to_int(&s.chars().take(3).collect::<String>())
}

fn get_type_id(s: &str) -> i32 {
    binary_str_to_int(&s.chars().skip(3).take(3).collect::<String>())
}

fn get_length_type_id(s: &str) -> i32 {
    binary_str_to_int(&s.chars().skip(6).take(1).collect::<String>())
}

fn exctract_packet(s: &str) -> Packet {
    if get_type_id(s) == 4 {
        let mut pointer = 6;
        loop {
            let group = s.chars().skip(pointer).take(5).collect::<String>();
            pointer += 5;
            if group.starts_with('0') {
                return Packet::new(s.chars().take(pointer).collect(), vec![]);
            }
        }
    }

    let mut pointer = 0;
    let mut packets = vec![];
    let length_type_id = get_length_type_id(s);
    pointer += 7;

    if length_type_id == 0 {
        let sub_packets_length =
            usize::from_str_radix(&s.chars().skip(3 + 3 + 1).take(15).collect::<String>(), 2)
                .unwrap();

        pointer += 15;
        let mut pointer2 = 0;

        while pointer2 < sub_packets_length {
            let sub_str = s.chars().skip(pointer + pointer2).collect::<String>();
            let sub_packet = exctract_packet(&sub_str);
            let sub_packet_len = sub_packet.clone().len();
            packets.push(sub_packet.clone());
            pointer2 += sub_packet_len;
        }
        pointer += pointer2;
    } else {
        let number_of_sub_packets =
            usize::from_str_radix(&s.chars().skip(3 + 3 + 1).take(11).collect::<String>(), 2)
                .unwrap();

        pointer += 11;
        while packets.len() < number_of_sub_packets {
            let sub_str = s.chars().skip(pointer).collect::<String>();
            let sub_packet = exctract_packet(&sub_str);
            packets.push(sub_packet.clone());
            pointer += sub_packet.len();
        }
    }
    Packet::new(s.chars().take(pointer).collect(), packets)
}

fn get_packet_version_sum(s: &str) -> i32 {
    let version = get_version(s);
    let type_id = get_type_id(s);

    match type_id {
        4 => version,
        _ => {
            let sub_packets = exctract_packet(s).sub_packets;
            version
                + sub_packets
                    .iter()
                    .map(|p| get_packet_version_sum(&p.bits))
                    .sum::<i32>()
        }
    }
}

fn part1(contents: &str) -> i32 {
    let s = parse_input(contents);
    get_packet_version_sum(&s)
}

fn part2(contents: &str) -> i64 {
    let s = parse_input(contents);
    let packet = exctract_packet(&s);
    packet.value()
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);

    let part2 = part2(contents);
    println!("part2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let contents = "D2FE28";
        assert_eq!(parse_input(contents), "110100101111111000101000");
    }

    #[test]
    fn test_part_1_version() {
        assert_eq!(get_version("110100101111111000101000"), 6);
    }

    #[test]
    fn test_part_1_type_id() {
        assert_eq!(get_type_id("110100101111111000101000"), 4);
    }

    #[test]
    fn test_part_1_operator_packet_length_type_id_0() {
        let contents = "38006F45291200";
        assert_eq!(part1(contents), 9);
    }

    #[test]
    fn test_part_1_operator_packet_length_type_id_1() {
        let contents = "EE00D40C823060";
        assert_eq!(part1(contents), 14);
    }

    #[test]
    fn test_part_1_example_1() {
        let contents = "8A004A801A8002F478";
        assert_eq!(part1(contents), 16);
    }

    #[test]
    fn test_part_1_example_2() {
        let contents = "620080001611562C8802118E34";
        assert_eq!(part1(contents), 12);
    }

    #[test]
    fn test_part_1_example_3() {
        let contents = "C0015000016115A2E0802F182340";
        assert_eq!(part1(contents), 23);
    }

    #[test]
    fn test_part_1_example_4() {
        let contents = "A0016C880162017C3686B18A3D4780";
        assert_eq!(part1(contents), 31);
    }

    #[test]
    fn test_part_2_example_1() {
        let contents = "C200B40A82";
        assert_eq!(part2(contents), 3);
    }

    #[test]
    fn test_part_2_example_2() {
        let contents = "04005AC33890";
        assert_eq!(part2(contents), 54);
    }

    #[test]
    fn test_part_2_example_3() {
        let contents = "880086C3E88112";
        assert_eq!(part2(contents), 7);
    }

    #[test]
    fn test_part_2_example_4() {
        let contents = "CE00C43D881120";
        assert_eq!(part2(contents), 9);
    }

    #[test]
    fn test_part_2_example_5() {
        let contents = "D8005AC2A8F0";
        assert_eq!(part2(contents), 1);
    }

    #[test]
    fn test_part_2_example_6() {
        let contents = "F600BC2D8F";
        assert_eq!(part2(contents), 0);
    }

    #[test]
    fn test_part_2_example_7() {
        let contents = "9C005AC2F8F0";
        assert_eq!(part2(contents), 0);
    }

    #[test]
    fn test_part_2_example_8() {
        let contents = "9C0141080250320F1802104A08";
        assert_eq!(part2(contents), 1);
    }
}
