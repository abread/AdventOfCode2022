use std::io::Read;

fn main() {
    let input = std::io::stdin().lock().bytes().map(Result::unwrap);

    dbg!("part1", find_packet_start(input));
}

fn find_packet_start(input: impl Iterator<Item = u8>) -> usize {
    let mut ring = [0u8; 4];
    let mut idx = 0;
    let mut st = 0;

    for b in input {
        ring[idx % 4] = b;

        let mut new_st = st + 1;
        for i in 1..=st {
            if ring[(idx - i) % 4] == b {
                new_st = i;
                break;
            }
        }
        st = new_st;

        idx += 1;

        if st == 4 {
            return idx;
        }
    }

    usize::MAX
}
