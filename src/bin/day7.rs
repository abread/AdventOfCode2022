use std::{collections::HashMap, io::Read};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::map,
    multi::{many0, many_m_n},
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

fn main() {
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let root_dir = File::parse_root(&input).unwrap();

    dbg!(
        "part1",
        calc_sizes(&root_dir, "", &[])
            .1
            .into_values()
            .filter(|v| *v <= 100_000)
            .sum::<usize>()
    );
}

fn calc_sizes<'f, 'pr>(
    root: &'f File,
    name: &'f str,
    path_prefix: &'pr [&'f str],
) -> (usize, HashMap<Vec<&'f str>, usize>) {
    let mut sizes = HashMap::new();

    match root {
        File::RegularFile { size } => (*size, sizes),
        File::Directory(entries) => {
            let mut sum = 0usize;
            let path = {
                let mut p = path_prefix.to_owned();
                p.push(name);
                p
            };

            for (name, file) in entries {
                let (sz, sub_sz) = calc_sizes(file, name, &path);
                sum += sz;
                sizes.extend(sub_sz);
            }

            sizes.insert(path, sum);
            (sum, sizes)
        }
    }
}

#[derive(Debug)]
enum File {
    RegularFile { size: usize },
    Directory(Vec<(String, File)>),
}

impl File {
    fn parse_root(input: &str) -> Result<Self, ParseErr> {
        let (_rem, (name, file)) = File::_parser(0)(input).map_err(|_| ParseErr)?;

        if name == "/" && matches!(file, File::Directory(_)) {
            Ok(file)
        } else {
            Err(ParseErr)
        }
    }

    fn _parser(level: usize) -> impl Fn(&str) -> IResult<&str, (String, Self)> {
        move |input| {
            let (mut rem, (file_name, mut file_node)) = delimited(
                preceded(many_m_n(level, level, tag("  ")), tag("- ")),
                separated_pair(
                    map(take_while1(|c| c != ' '), ToOwned::to_owned),
                    tag(" "),
                    delimited(tag("("), File::_parse_file_metadata, tag(")")),
                ),
                tag("\n"),
            )(input)?;

            if let File::Directory(entries) = &mut file_node {
                let (new_rem, parsed_entries) = many0(File::_parser(level + 1))(rem)?;
                *entries = parsed_entries;
                rem = new_rem;
            }

            Ok((rem, (file_name, file_node)))
        }
    }

    fn _parse_file_metadata(input: &str) -> IResult<&str, Self> {
        alt((
            map(tag("dir"), |_| File::Directory(Vec::new())),
            map(
                preceded(tag("file, size="), nom::character::complete::u64),
                |size| File::RegularFile {
                    size: size as usize,
                },
            ),
        ))(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParseErr;
