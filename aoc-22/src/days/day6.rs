use crate::day_exec::DayExecutor;
pub struct Day6;

impl DayExecutor for Day6 {
    fn exec_part1(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(solve_part1(&input))
    }

    fn exec_part2(&self, input: String) -> Box<dyn std::fmt::Display> {
        Box::new(solve_part2(&input))
    }
}

fn solve_part1(input: &str) -> String {
    format!(
        "Start-of-packet marker at offset {}",
        find_start_of_packet_offset(input)
    )
}

fn solve_part2(input: &str) -> String {
    format!(
        "Start-of-message marker at offset {}",
        find_start_of_message_offset(input)
    )
}

fn find_start_of_packet_offset(input: &str) -> usize {
    let data = input.chars().collect::<Vec<_>>();
    const PACKET_SIZE: usize = 4;
    find_unique_offset(data.as_slice(), PACKET_SIZE)
}

fn find_start_of_message_offset(input: &str) -> usize {
    let data = input.chars().collect::<Vec<_>>();
    const MESSAGE_SIZE: usize = 14;
    find_unique_offset(data.as_slice(), MESSAGE_SIZE)
}

fn find_unique_offset(data: &[char], size: usize) -> usize {
    data.windows(size)
        .enumerate()
        .skip_while(|(_, packet)| !is_unique(packet))
        .next()
        .map(|(i, _)| size + i)
        .expect("Could not find unique packet in message")
}

fn is_unique(packet: &[char]) -> bool {
    for (i, c) in packet.iter().take(packet.len() - 1).enumerate() {
        if packet[i + 1..].contains(c) {
            return false;
        }
    }
    true
}
