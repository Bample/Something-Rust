use std::env;
use std::fs;
use std::io;
use std::path::*;
use std::process;
use utils::{cmd7z, execa};

fn main() -> io::Result<()> {
  let cur_dir = &env::current_dir().unwrap();
  let cur_dir = cur_dir.as_path();
  let pfs = env::var("ProgramFiles").unwrap();
  let pfs = Path::new(&pfs);
  let el_path = pfs.join("Edgeless").to_path_buf();
  println!();
  println!("{}", "Edgeless 插件加载器 (Rust重构版本)");
  println!("Version: {}\n", "1.2.10");
  println!(
    "Edgeless Path 为 {}\n",
    utils::path_to_string(el_path.as_path())
  );

  parse_args(&el_path);

  let el_part = {
    let parts = utils::find_edgeless_part();
    if parts.len() < 1 {
      println!("未找到 Edgeless 盘符");
      process::exit(9009);
    };
    parts[0]
  };
  println!("使用 {} 作为 Edgeless 盘符", utils::path_to_string(el_part));

  /*
    println!("{:#?}", pfs.join("1.txt"));
    println!("{}", path_to_string(&pfs.join("1.ttt")));
  */

  let el_nes = el_part.join(r"\Edgeless\Nes_Inport.7z");
  let el_res = el_part.join(r"\Edgeless\Resource");
  println!(
    "使用 {} 作为必要组件包",
    utils::path_to_string(el_nes.as_path())
  );
  println!(
    "使用 {} 作为插件包目录",
    utils::path_to_string(el_res.as_path())
  );

  println!("正在加载必要组件包...");
  load_nes(&el_nes, &el_path).unwrap();
  println!("加载完成!\n");
  println!();

  let el_plug = utils::read_dir(el_res.as_path()).unwrap();
  let el_plug = utils::ext_filter(vec!["7z"], utils::pathbuf_to_path(&el_plug));
  for i in &el_plug {
    println!("正在加载插件包: {}...", {
      if let Some(v) = &i.to_path_buf().file_name() {
        if let Some(r) = v.to_str() {
          r
        } else {
          "NULL"
        }
      } else {
        "NULL"
      }
    });
    load_plugin(&i.to_path_buf(), &el_path).unwrap();
    clear_scripts(&el_path).unwrap();
    println!("加载完成!\n");
  }

  println!("OK!");
  Ok(())
}

fn load_nes(el_nes: &PathBuf, el_path: &PathBuf) -> Result<(), process::Output> {
  let cur_dir = &env::current_dir().unwrap();
  let cur_dir = cur_dir.as_path();
  let cmd = cmd7z::extract_package(el_nes.as_path(), el_path.as_path()).unwrap();
  if cmd.status.success() != true {
    Err(cmd)
  } else {
    let nes_ini = el_path.as_path().join(r"Nes.ini");
    let cmd = execa::run(
      "PECMD.exe",
      vec!["LOAD", utils::path_to_string(nes_ini.as_path()).as_str()],
      utils::path_to_string(cur_dir).as_str(),
    )
    .unwrap();
    Ok(())
  }
}

fn load_plugin(el_pl: &PathBuf, el_path: &PathBuf) -> Result<(), process::Output> {
  let cur_dir = &env::current_dir().unwrap();
  let cur_dir = cur_dir.as_path();
  let cmd = cmd7z::extract_package(el_pl.as_path(), el_path.as_path()).unwrap();
  if cmd.status.success() != true {
    Err(cmd)
  } else {
    let list = utils::read_dir(el_path.as_path()).unwrap();
    let list = utils::ext_filter(vec!["cmd", "wcs"], utils::pathbuf_to_path(&list));
    for i in &list {
      let i_path = utils::path_to_string(i);
      if i.extension().unwrap() == "cmd" {
        let cmd = execa::run(
          "PECMD.exe",
          vec!["EXEC", format!("=!{}", &i_path).as_str()],
          utils::path_to_string(cur_dir).as_str(),
        )
        .unwrap();
      } else if i.extension().unwrap() == "wcs" {
        let cmd = execa::run(
          "PECMD.exe",
          vec!["LOAD", &i_path.as_str()],
          utils::path_to_string(cur_dir).as_str(),
        )
        .unwrap();
      }
    }

    Ok(())
  }
}

fn clear_scripts(el_path: &PathBuf) -> io::Result<()> {
  let list = utils::read_dir(el_path.as_path()).unwrap();
  let list = utils::ext_filter(vec!["cmd", "wcs"], utils::pathbuf_to_path(&list));
  for i in &list {
    fs::remove_file(i).unwrap();
  }
  Ok(())
}

fn parse_args(el_path: &PathBuf) {
  let mut args = Vec::new();
  for i in env::args_os() {
    if let Some(v) = i.to_str() {
      args.push(v.to_string());
    }
  }
  if args.len() > 1 {
    if &args[1] == "--load-plugin" {
      let p_path = Path::new(&args[2]).to_path_buf();
      println!("正在加载插件包: {}...", {
        if let Some(v) = &p_path.to_path_buf().file_name() {
          if let Some(r) = v.to_str() {
            r
          } else {
            "NULL"
          }
        } else {
          "NULL"
        }
      });
      load_plugin(&p_path, &el_path).unwrap();
      clear_scripts(&el_path).unwrap();
      println!("加载完成!\nOK!\n");
      process::exit(0);
    }
  }
}
