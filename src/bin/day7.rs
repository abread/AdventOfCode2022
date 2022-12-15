use std::{collections::HashMap, pin::Pin};

use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let exec_trace = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        //let exec_trace = include_str!("../../in")
        //    .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| parse_puzzle_input_line(&s));

    let root_dir = File::infer_tree_from_exec_trace(exec_trace);
    let (current_size, sizes) = calc_sizes(&root_dir, "", &[]);

    dbg!(
        "part1",
        sizes.values().filter(|v| **v <= 100_000).sum::<usize>()
    );

    const FS_SIZE: usize = 70_000_000;
    const REQ_FREE_SIZE: usize = 30_000_000;
    let min_dir_size = REQ_FREE_SIZE - (FS_SIZE - current_size);
    dbg!(
        "part2",
        sizes.values().filter(|dsize| **dsize >= min_dir_size).min()
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
    Directory(HashMap<String, File>),
}

#[derive(Debug)]
enum PuzzleInputLine {
    CdRoot,
    CdParent,
    CdSubdir(String),
    Ls,
    LsOutputEntry(String, File),
}

fn parse_command_invocation(input: &str) -> IResult<&str, PuzzleInputLine> {
    alt((
        map(tag("ls"), |_| PuzzleInputLine::Ls),
        preceded(
            tag("cd "),
            alt((
                map(tag("/"), |_| PuzzleInputLine::CdRoot),
                map(tag(".."), |_| PuzzleInputLine::CdParent),
                |subdir: &str| Ok(("", PuzzleInputLine::CdSubdir(subdir.to_owned()))),
            )),
        ),
        |_| panic!("nooooo"),
    ))(input)
}

fn parse_ls_output(input: &str) -> IResult<&str, PuzzleInputLine> {
    alt((
        preceded(
            tag("dir "),
            map(
                |s| Ok(("", s)),
                |name: &str| {
                    PuzzleInputLine::LsOutputEntry(name.to_owned(), File::Directory(HashMap::new()))
                },
            ),
        ),
        map(
            separated_pair(
                nom::character::complete::u64,
                tag(" "),
                map(|s| Ok(("", s)), ToOwned::to_owned),
            ),
            |(size, name)| {
                PuzzleInputLine::LsOutputEntry(
                    name,
                    File::RegularFile {
                        size: size as usize,
                    },
                )
            },
        ),
    ))(input)
}

fn parse_puzzle_input_line(input: &str) -> PuzzleInputLine {
    let parser_res: IResult<&str, PuzzleInputLine> = alt((
        preceded(tag("$ "), parse_command_invocation),
        parse_ls_output,
        |_| panic!("not command nor ls"),
    ))(input);

    let (rem, res) = parser_res.unwrap();

    if !rem.is_empty() {
        panic!("leftover input: {rem}");
    }

    res
}

impl File {
    fn infer_tree_from_exec_trace(exec_trace: impl Iterator<Item = PuzzleInputLine>) -> Self {
        struct FileTree {
            root: File,
            cwd_stack: Vec<*mut File>,
        }

        impl FileTree {
            // Safety: must pin exactly once (hiding the original binding) and call cd_root afterwards
            unsafe fn new() -> FileTree {
                FileTree {
                    root: File::Directory(HashMap::new()),
                    cwd_stack: Vec::with_capacity(1),
                }
            }

            fn cd_root(self: &mut Pin<&mut Self>) {
                self.cwd_stack.clear();
                let root_ptr = &mut self.root as *mut _;
                self.cwd_stack.push(root_ptr);
            }

            fn cd_parent(self: &mut Pin<&mut Self>) {
                self.cwd_stack.pop();
                if self.cwd_stack.is_empty() {
                    self.cd_root();
                }
            }

            fn cd_subdir(self: &mut Pin<&mut Self>, subdir_name: String) {
                let subdir = match self.cwd() {
                    File::Directory(entries) => entries
                        .entry(subdir_name)
                        .or_insert_with(|| File::Directory(HashMap::new())),
                    _ => unreachable!("file with same name already exists"),
                };

                let subdir_ptr = subdir as *mut _;
                self.cwd_stack.push(subdir_ptr);
            }

            fn cwd(self: &mut Pin<&mut Self>) -> &mut File {
                // Safety: a current working directory always exists by construction
                let cwd_ptr = unsafe { self.cwd_stack.last().copied().unwrap_unchecked() };

                // Safety: self is pinned, and no directories are removed under current working dir, so directory locations have not changed
                unsafe { &mut *cwd_ptr }
            }

            fn into_root(mut self: Pin<&mut Self>) -> File {
                std::mem::replace(&mut self.root, File::Directory(HashMap::new()))
            }
        }

        let mut tree = unsafe { FileTree::new() };
        let mut tree = unsafe { Pin::new_unchecked(&mut tree) };
        tree.cd_root();

        let mut exec_trace = exec_trace.peekable();
        while let Some(traced_cmd) = exec_trace.next() {
            match traced_cmd {
                PuzzleInputLine::CdParent => tree.cd_parent(),
                PuzzleInputLine::CdRoot => tree.cd_root(),
                PuzzleInputLine::CdSubdir(sub) => tree.cd_subdir(sub),
                PuzzleInputLine::Ls => {
                    let File::Directory(cwd_entries) = tree.cwd() else { panic!("invalid input: directory is not a directory") };
                    while let Some(PuzzleInputLine::LsOutputEntry(..)) = exec_trace.peek() {
                        let Some(PuzzleInputLine::LsOutputEntry(subname, sub)) = exec_trace.next() else { unreachable!() };

                        cwd_entries.entry(subname).or_insert(sub);
                    }
                }
                _ => panic!("invalid input"),
            }
        }

        tree.into_root()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ParseErr;
