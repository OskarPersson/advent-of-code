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
                println!("'{}'", c);
                unreachable!()
            }
        })
        .collect()
}

fn binary_str_to_int(s: &str) -> i32 {
    i32::from_str_radix(&s, 2).unwrap()
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

fn get_literal_value(s: &str) -> i32 {
    s.chars().skip(6);
    todo!()
}

fn get_sub_packets(s: &str) -> Vec<String> {
    println!("get sub packets for: {}, version: {}", s, get_version(&s));
    let mut packets = vec![];

    let length_type_id = get_length_type_id(s);
    match length_type_id {
        0 => {
            // 3 bits for version
            // 3 bits for type ID
            // 1 bit for length type ID
            // 15 bits for total sub packet length
            let sub_packets_length =
                usize::from_str_radix(&s.chars().skip(3 + 3 + 1).take(15).collect::<String>(), 2)
                    .unwrap();

            let mut sub_packet = String::from("");
            let mut s_index = 3 + 3 + 1 + 15;
            for (idx, c) in s
                .chars()
                .skip(3 + 3 + 1 + 15)
                .take(sub_packets_length)
                .enumerate()
            {
                sub_packet.push(c);
                s_index += 1;
                if sub_packet.len() < 7 {
                    continue;
                }

                let sub_type_id = get_type_id(&sub_packet);

                match sub_type_id {
                    4 => {
                        // find groups of five until one group begins with 0
                        if (sub_packet.len() - 6) % 5 == 0
                            && sub_packet.chars().nth(sub_packet.len() - 5).unwrap() == '0'
                        {
                            packets.push(sub_packet.clone());
                            sub_packet.clear();
                        }
                    }
                    _ => {
                        let sub_type_length_id = get_length_type_id(&sub_packet);

                        match sub_type_length_id {
                            0 => {
                                if sub_packet.len() < 3 + 3 + 1 + 15 {
                                    continue;
                                }
                                let sub_sub_packets_length = usize::from_str_radix(
                                    &sub_packet
                                        .chars()
                                        .skip(3 + 3 + 1)
                                        .take(15)
                                        .collect::<String>(),
                                    2,
                                )
                                .unwrap();
                                if sub_packet.len() == 3 + 3 + 1 + 15 + sub_sub_packets_length {
                                    packets.push(sub_packet.clone());
                                    sub_packet.clear();
                                }
                            }
                            1 => {
                                if sub_packet.len() == 7 {
                                    let mut subs = get_sub_packets(
                                        &s.chars().skip(s_index - 7).collect::<String>(),
                                    );
                                    packets.push(s.chars().skip(s_index - 7).collect::<String>())
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        1 => {
            // 3 bits for version
            // 3 bits for type ID
            // 1 bit for length type ID
            // 11 bits for number of sub packets
            let number_of_sub_packets =
                usize::from_str_radix(&s.chars().skip(3 + 3 + 1).take(11).collect::<String>(), 2)
                    .unwrap();

            let mut sub_packets = vec![];
            let mut sub_packet = String::from("");
            let mut s_index = 3 + 3 + 1 + 11;
            for c in s.chars().skip(3 + 3 + 1 + 11) {
                if sub_packets.len() == number_of_sub_packets {
                    // do same changes above??????
                    packets.push(s.chars().skip(s_index - 7).collect::<String>());
                    break;
                }

                sub_packet.push(c);
                s_index += 1;

                if sub_packet.len() < 7 {
                    continue;
                }

                let sub_type_id = get_type_id(&sub_packet);

                match sub_type_id {
                    4 => {
                        // find groups of five until one group begins with 0
                        if (sub_packet.len() - 6) % 5 == 0
                            && sub_packet.chars().nth(sub_packet.len() - 5).unwrap() == '0'
                        {
                            packets.push(sub_packet.clone());
                            sub_packet.clear();
                        }
                    }
                    _ => {
                        let sub_type_length_id = get_length_type_id(&sub_packet);

                        match sub_type_length_id {
                            0 => {
                                if sub_packet.len() < 3 + 3 + 1 + 15 {
                                    continue;
                                }
                                let sub_sub_packets_length = usize::from_str_radix(
                                    &sub_packet
                                        .chars()
                                        .skip(3 + 3 + 1)
                                        .take(15)
                                        .collect::<String>(),
                                    2,
                                )
                                .unwrap();
                                if sub_packet.len() == 3 + 3 + 1 + 15 + sub_sub_packets_length {
                                    packets.push(sub_packet.clone());
                                    sub_packet.clear();
                                }
                            }
                            1 => {
                                if sub_packet.len() == 7 {
                                    let mut subs = get_sub_packets(
                                        &s.chars().skip(s_index - 7).collect::<String>(),
                                    );
                                    sub_packets.append(&mut subs);
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }
    packets
}

fn get_packet_version_sum(s: &str) -> i32 {
    let version = get_version(&s);
    let type_id = get_type_id(&s);
    let d = s.clone();

    match type_id {
        4 => {
            println!("\n================================\n");

            println!("{:?}", s);
            println!("version: {:?}", version);
            println!("type_id: {:?}", type_id);
            version
        }
        _ => {
            println!("\n================================\n");

            println!("{:?}", d);
            println!("version: {:?}", version);
            println!("type_id: {:?}", type_id);
            println!("length type_id: {:?}", get_length_type_id(&s));
            let sub_packets = get_sub_packets(&s);
            println!("sub packets: {:?}", sub_packets);
            version
                + sub_packets
                    .iter()
                    .map(|p| get_packet_version_sum(p))
                    .sum::<i32>()
        }
    }
}

fn part1(contents: &str) -> i32 {
    let s = parse_input(contents);
    get_packet_version_sum(&s)
}

fn main() {
    let contents = include_str!("../input.txt");

    let part1 = part1(contents);
    println!("part1: {}", part1);
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
    fn test_part_1_literal_value() {
        assert_eq!(get_literal_value("110100101111111000101000"), 2021);
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
}
