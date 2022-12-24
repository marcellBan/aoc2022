use std::cell::RefCell;
use std::io;
use std::rc::Rc;

use crate::input_reader;

#[derive(Debug)]
enum FSEntry {
    File {
        name: String,
        size: u32,
    },
    Directory {
        name: String,
        contents: Vec<Rc<RefCell<FSEntry>>>,
        parent: Rc<RefCell<FSEntry>>,
        size: Option<u32>,
    },
    Nil,
}

fn calc_size(node: Rc<RefCell<FSEntry>>) -> u32 {
    let mut size = 0;
    if let FSEntry::Directory {
        name: _,
        contents: cont,
        parent: _,
        size: dir_size,
    } = &mut *node.borrow_mut()
    {
        if let Some(s) = dir_size {
            size = *s;
        } else {
            for entry in cont {
                let is_dir = match &*(**entry).borrow() {
                    FSEntry::Directory {
                        name: _,
                        contents: _,
                        parent: _,
                        size: _,
                    } => true,
                    FSEntry::File { name: _, size: siz } => {
                        size += siz;
                        false
                    }
                    _ => false,
                };
                if is_dir {
                    size += calc_size(entry.to_owned());
                }
            }
            *dir_size = Some(size);
        }
    }
    size
}

fn collect_sizes(root: Rc<RefCell<FSEntry>>, sizes: &mut Vec<u32>) -> () {
    let is_dir = match &*(*root).borrow() {
        FSEntry::Directory {
            name: _,
            contents: _,
            parent: _,
            size: _,
        } => true,
        _ => false,
    };
    if is_dir {
        sizes.push(calc_size(root.to_owned()));
        match &*(*root).borrow() {
            FSEntry::Directory {
                name: _,
                contents: cont,
                parent: _,
                size: _,
            } => {
                for entry in cont {
                    collect_sizes(entry.to_owned(), sizes);
                }
            }
            _ => (),
        }
    }
}

fn calculate_sizes(root: Rc<RefCell<FSEntry>>) -> Vec<u32> {
    let mut sizes = Vec::new();
    collect_sizes(root.to_owned(), &mut sizes);
    sizes
}

pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day7.in")?;

    let root = Rc::new(RefCell::new(FSEntry::Directory {
        name: "/".to_string(),
        contents: Vec::new(),
        parent: Rc::new(RefCell::new(FSEntry::Nil)),
        size: None,
    }));
    let mut pwd = root.to_owned();

    for line in lines {
        let parts = line.split(" ").collect::<Vec<_>>();
        match parts[0] {
            "$" => match parts[1] {
                "cd" => match parts[2] {
                    "/" => pwd = root.to_owned(),
                    ".." => {
                        let new_pwd = if let FSEntry::Directory {
                            name: _,
                            contents: _,
                            parent: par,
                            size: _,
                        } = &*(*pwd).borrow()
                        {
                            par.to_owned()
                        } else {
                            panic!("Pwd was not directory: {:?}", pwd)
                        };
                        pwd = new_pwd.to_owned();
                    }
                    dir => {
                        let new_pwd = if let FSEntry::Directory {
                            name: _,
                            contents: cont,
                            parent: _,
                            size: _,
                        } = &*(*pwd).borrow()
                        {
                            cont.iter()
                                .filter(|x| {
                                    if let FSEntry::Directory {
                                        name: nam,
                                        contents: _,
                                        parent: _,
                                        size: _,
                                    } = &*(***x).borrow()
                                    {
                                        return nam.as_str() == dir;
                                    }
                                    false
                                })
                                .collect::<Vec<_>>()
                                .first()
                                .unwrap()
                                .to_owned()
                                .to_owned()
                        } else {
                            panic!("No dir found with name {}", dir);
                        };
                        pwd = new_pwd.to_owned();
                    }
                },
                "ls" => {}
                unk => panic!("Unknown command: {}", unk),
            },
            "dir" => {
                if let FSEntry::Directory {
                    name: _,
                    contents: cont,
                    parent: _,
                    size: _,
                } = &mut *pwd.borrow_mut()
                {
                    cont.push(Rc::new(RefCell::new(FSEntry::Directory {
                        name: parts[1].to_string(),
                        contents: Vec::new(),
                        parent: pwd.to_owned(),
                        size: None,
                    })));
                } else {
                    panic!("Pwd was not directory: {:?}", pwd)
                }
            }
            siz => {
                let parsed_size = siz.parse::<u32>().unwrap();
                if let FSEntry::Directory {
                    name: _,
                    contents: cont,
                    parent: _,
                    size: _,
                } = &mut *pwd.borrow_mut()
                {
                    cont.push(Rc::new(RefCell::new(FSEntry::File {
                        name: parts[1].to_string(),
                        size: parsed_size,
                    })));
                } else {
                    panic!("Pwd was not directory: {:?}", pwd)
                }
            }
        }
    }

    let sizes = calculate_sizes(root.to_owned());

    let size_sum = sizes.iter().filter(|x| x <= &&100000).sum::<u32>();
    println!("Size sum under 100 000: {}", size_sum);

    let space_needed = match &*(*root).borrow() {
        FSEntry::Directory {
            name: _,
            contents: _,
            parent: _,
            size: Some(siz),
        } => 30000000 - (70000000 - (*siz as i64)),
        _ => panic!("Root must be a directory and already should have a size!"),
    };
    let size_to_delete = sizes
        .iter()
        .filter(|x| (**x as i64) >= space_needed)
        .min()
        .unwrap();
    println!("Size of directory to delete: {}", size_to_delete);

    Ok(())
}
