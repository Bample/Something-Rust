use std::path::Path;
use std::path::PathBuf;
pub fn read_dir(_path: &Path) -> std::io::Result<Vec<PathBuf>> {
  let mut dirs = Vec::new();
  for el in std::fs::read_dir(_path)? {
    let el = el?;
    dirs.push(el.path());
  }

  let mut out = Vec::new();
  for ll in dirs {
    let el = ll;
    out.push(el);
  }

  Ok(out)
}

pub fn pathbuf_to_path<'a>(pb: &'a Vec<PathBuf>) -> Vec<&'a Path> {
  let mut out = Vec::new();
  for ll in pb {
    let el = ll.as_path();
    out.push(el);
  }
  out
}

pub fn pathlist_to_stringlist(_path: Vec<&Path>) -> Vec<String> {
  let mut out = Vec::new();
  for ll in &_path {
    if let Some(v) = ll.to_str() {
      out.push(v.to_string());
    }
  }
  out
}

pub fn path_to_string(p: &Path) -> String {
  match p.to_str() {
    Some(v) => v.to_string(),
    _ => String::new(),
  }
}

pub fn find_edgeless_part() -> Vec<&'static Path> {
  let dlist: Vec<&str> = vec![
    "Z:\\", "Y:\\", "X:\\", "W:\\", "V:\\", "U:\\", "T:\\", "S:\\", "R:\\", "Q:\\", "P:\\", "O:\\",
    "N:\\", "M:\\", "L:\\", "K:\\", "J:\\", "I:\\", "H:\\", "G:\\", "F:\\", "E:\\", "D:\\", "C:\\",
    "B:\\", "A:\\",
  ];
  let mut elist: Vec<&Path> = Vec::new();
  for i in dlist {
    if &Path::new(format!("{}{}", i, "Edgeless\\version.txt").as_str()).exists() == &true {
      elist.push(Path::new(i));
    }
  }
  elist
}

pub fn read_line() -> String {
  let mut rlmut = String::new();
  std::io::stdin().read_line(&mut rlmut).unwrap();
  rlmut
}

pub fn ext_filter<'a>(ext_list: Vec<&str>, list: Vec<&'a Path>) -> Vec<&'a Path> {
  let mut out: Vec<&Path> = Vec::new();

  for i in &list {
    let ext_name = {
      if let Some(v) = i.extension() {
        if let Some(e) = v.to_str() {
          e
        } else {
          ""
        }
      } else {
        ""
      }
    };
    for l in &ext_list {
      if &ext_name == l {
        out.push(i)
      }
    }
  }
  out
}

pub mod cmd7z {
  use crate::execa;
  use std::env;
  use std::io;
  use std::path::Path;
  use std::process;
  pub fn extract_package(pkg: &Path, elpath: &Path) -> io::Result<process::Output> {
    let cur_dir = &env::current_dir().unwrap();
    let cur_dir = cur_dir.as_path();
    execa::run(
      "lib7z.dll",
      vec![
        "x",
        format!("{}", pkg.to_str().unwrap()).as_str(),
        "-y",
        format!("-o{}", elpath.to_str().unwrap()).as_str(),
      ],
      cur_dir.to_str().unwrap(),
    )
  }
}
pub mod execa {
  use std::io;
  use std::process;
  use std::process::Command;
  pub fn run(cmd: &str, args: Vec<&str>, path: &str) -> io::Result<process::Output> {
    let cmd = Command::new(cmd)
      .args(args)
      .current_dir(path)
      .output()
      .unwrap();
    Ok(cmd)
  }
  pub fn spawn(cmd: &str, args: Vec<&str>, path: &str) -> io::Result<process::Child> {
    let cmd = Command::new(cmd)
      .args(args)
      .current_dir(path)
      .spawn()
      .unwrap();
    Ok(cmd)
  }
}
