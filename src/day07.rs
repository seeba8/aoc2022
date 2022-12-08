use crate::util::read_input;
const FILE_SYSTEM_SIZE: usize = 70_000_000;
const REQUIRED_FREE_SPACE: usize = 30_000_000;

pub fn solve() -> color_eyre::Result<()> {
    let input_raw = read_input("day07.txt");
    println!("Day 07 part 1: {}", part1(&input_raw)?);
    println!("Day 07 part 2: {}", part2(&input_raw)?);
    Ok(())
}

fn part1(input_raw: &str) -> color_eyre::Result<usize> {
    let mut root = Folder::new("/");
    let input: Vec<&str> = input_raw.lines().collect();
    root.parse(&input, &mut 0)?;
    Ok(root.get_sizes_of_folders_smaller_than(100_000))
}

fn part2(input_raw: &str) -> color_eyre::Result<usize> {
    let mut root = Folder::new("/");
    let input: Vec<&str> = input_raw.lines().collect();
    root.parse(&input, &mut 0)?;
    root.get_folders_larger_than(REQUIRED_FREE_SPACE - (FILE_SYSTEM_SIZE - root.size())).iter().min().ok_or_else(|| color_eyre::eyre::eyre!("no folder found")).copied()
}

pub trait FileOrFolder {
    fn size(&self) -> usize;
    fn name(&self) -> &str;
    fn parse(&mut self, input: &[&str], index: &mut usize) -> color_eyre::Result<()>;
    fn is_folder(&self) -> bool;
    fn get_sizes_of_folders_smaller_than(&self, max: usize) -> usize;
    fn get_folders_larger_than(&self, min: usize) -> Vec<usize>;
}

#[derive(Default, Clone, Debug)]
pub struct File {
    size: usize,
    name: String,
}
impl FileOrFolder for File {
    fn size(&self) -> usize {
        self.size
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn parse(&mut self, _input: &[&str], _index: &mut usize) -> color_eyre::Result<()> {
        unreachable!()
    }

    fn is_folder(&self) -> bool {
        false
    }

    fn get_sizes_of_folders_smaller_than(&self, _max: usize) -> usize {
        0
    }

    fn get_folders_larger_than(&self, _min: usize) -> Vec<usize> {
        vec![]
    }
}

#[derive(Default)]
pub struct Folder {
    children: Vec<Box<dyn FileOrFolder>>,
    name: String,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

impl FileOrFolder for Folder {
    fn size(&self) -> usize {
        self.children.iter().map(|c| c.size()).sum()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn parse(&mut self, input: &[&str], index: &mut usize) -> color_eyre::Result<()> {
        while *index < input.len() {
            let cmd = if input[*index].starts_with("$ ") {
                &input[*index][2..]
            } else {
                input[*index]
            };
            let (command, arg) = cmd.split_once(' ').unwrap_or((cmd, ""));
            match command {
                "cd" => match arg {
                    ".." => {
                        *index += 1;
                        return Ok(());
                    }
                    "/" => {
                        if self.name == "/" {
                            *index += 1;
                        } else {
                            return Ok(());
                        }
                    }
                    foldername => match self.children.iter_mut().find(|c| c.name() == foldername) {
                        Some(folder) => {
                            *index += 1;
                            folder.parse(input, index)?;
                        }
                        None => unreachable!(),
                    },
                },
                "ls" => {
                    *index += 1;
                    while *index < input.len() && !input[*index].starts_with("$ ") {
                        match input[*index].split_once(' ') {
                            Some(("dir", dirname)) => {
                                self.children.push(Box::new(Self::new(dirname)));
                            }
                            Some((filesize, filename)) => {
                                self.children.push(Box::new(File {
                                    name: filename.to_string(),
                                    size: filesize.parse()?,
                                }));
                            }
                            None => {
                                return Err(color_eyre::eyre::eyre!(
                                    "Invalid ls return: {}",
                                    input[*index]
                                ))
                            }
                        }
                        *index += 1;
                    }
                }
                _ => {
                    return Err(color_eyre::eyre::eyre!("Invalid command: {command}"));
                }
            }
        }
        Ok(())
    }

    fn is_folder(&self) -> bool {
        true
    }

    fn get_sizes_of_folders_smaller_than(&self, max: usize) -> usize {
        let mut res = 0;
        if self.size() < max {
            res += self.size();
        }
        for child in &self.children {
            res += child.get_sizes_of_folders_smaller_than(max);
        }
        res
    }

    fn get_folders_larger_than(&self, min: usize) -> Vec<usize> {
        let mut folders = vec![];
        if self.size() > min {
            folders.push(self.size());
        }
        for child in &self.children {
            folders.append(&mut child.get_folders_larger_than(min));
        }
        folders
    }
}

#[cfg(test)]
pub mod tests {
    use crate::util::read_example;

    use super::*;
    #[test]
    fn it_parses_example() -> color_eyre::Result<()> {
        let mut root = Folder::new("/");
        let input_raw = read_example("day07.txt");
        let input: Vec<&str> = input_raw.lines().collect();
        root.parse(&input, &mut 0)?;
        assert_eq!(4, root.children.len());
        Ok(())
    }

    #[test]
    fn it_gets_sizes() -> color_eyre::Result<()> {
        let mut root = Folder::new("/");
        let input_raw = read_example("day07.txt");
        let input: Vec<&str> = input_raw.lines().collect();
        root.parse(&input, &mut 0)?;
        assert_eq!(48_381_165, root.size());
        Ok(())
    }

    #[test]
    fn it_gets_sizes_smaller_than_x() -> color_eyre::Result<()> {
        let mut root = Folder::new("/");
        let input_raw = read_example("day07.txt");
        let input: Vec<&str> = input_raw.lines().collect();
        root.parse(&input, &mut 0)?;
        assert_eq!(95437, root.get_sizes_of_folders_smaller_than(100_000));
        Ok(())
    }

    #[test]
    fn it_gets_folders_larger_than() -> color_eyre::Result<()> {
        let mut root = Folder::new("/");
        let input_raw = read_example("day07.txt");
        let input: Vec<&str> = input_raw.lines().collect();
        root.parse(&input, &mut 0)?;
        assert_eq!(24933642, *root.get_folders_larger_than(REQUIRED_FREE_SPACE - (FILE_SYSTEM_SIZE - root.size())).iter().min().unwrap());
        Ok(())
    }


}
