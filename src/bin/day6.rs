use std::io::Read;

fn main() {
    let input: Vec<_> = std::io::stdin()
        .lock()
        .bytes()
        .map(Result::unwrap)
        .collect();

    dbg!("part1", find_packet_start(input.iter().copied()));
    dbg!("part2", find_message_start(input.into_iter()));
}

fn find_packet_start(input: impl Iterator<Item = u8>) -> usize {
    find_start_n_different_chars::<4>(input)
}

fn find_message_start(input: impl Iterator<Item = u8>) -> usize {
    find_start_n_different_chars::<14>(input)
}

fn find_start_n_different_chars<const N: usize>(input: impl Iterator<Item = u8>) -> usize {
    let mut ring = [0u8; N];
    let mut idx = 0;
    let mut st = 0;

    for b in input {
        ring[idx % N] = b;

        let mut new_st = st + 1;
        for i in 1..=st {
            if ring[(idx - i) % N] == b {
                new_st = i;
                break;
            }
        }
        st = new_st;

        idx += 1;

        if st == N {
            return idx;
        }
    }

    usize::MAX
}
