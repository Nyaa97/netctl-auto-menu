#![feature(exit_status_error)]
use std::{process::{Command, Stdio}, str, io::{Write, BufReader, BufRead}};
use regex::Regex;

fn main() -> Result<(), String> {
  let cmdarg: Vec<String> = std::env::args().collect();

  let result = Command::new("netctl-auto")
    .arg("list")
    .output()
    .map_err(|_|{
      "netctl not found".to_string()
    })?;

  result
    .status.exit_ok()
    .map_err(|e| {
      format!("netctl error code: {}", e.code().unwrap())
    })?;

  let re = Regex::new(r"(?m)^.{2}(.+)$(?-m)").unwrap();
  let text = String::from_utf8_lossy(&result.stdout).to_string();
  let mwifi: Vec<&str> = re
    .captures_iter(&text)
    .map(|cap| {
      cap
        .get(1)
        .unwrap()
        .as_str()
    })
    .collect();
  
  let result = Command::new("xprop")
    .args(["-name", &cmdarg[1]])
    .output()
    .map_err(|_|{
      "xprop not found".to_string()
    })?;

  let re = Regex::new(r"(\d+)\sby\s(\d+)").unwrap();
  let text = String::from_utf8_lossy(&result.stdout).to_string();
  let cap = re
    .captures(&text)
    .unwrap();

  let tmp: &str = &format!(
    "{}",
    cap.get(1).unwrap().as_str().parse::<i32>().unwrap()
    - &cmdarg[3].parse::<i32>().map_err(|_| {
      "Bad width value".to_string()
    })?
  );
  let tmp2: &str = &format!("{}", mwifi.len());
  
  let args: Vec<&str> = vec![
    "-x",
    tmp,
    "-y",
    cap.get(2).unwrap().as_str(),
    "-w",
    &cmdarg[3],
    "-h",
    cap.get(2).unwrap().as_str(),
    "-l",
    tmp2,
    "-fn",
    &cmdarg[2],
    "-e",
    "onstart=uncollapse;button3=exit:retval;button4=scrollup:1;button5=scrolldown:1"
  ];

  #[allow(unused_mut)]
  let mut dzen = Command::new("dzen2")
    .args(&args)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .map_err(|_| {
      "dzen2 not found".to_string()
    })?;
  
  let mut writer = dzen.stdin.unwrap();
  match writer.write("WIFI LIST\n".as_bytes()) {
    _ => ()
  }
  for wifi in mwifi {
    match writer.write(format!("^ca(1,echo {}){}^ca()\n", wifi, wifi).as_bytes()) {
      _ => ()
    }
  }
  let mut buf = Default::default();
  if let Some(stdout) = dzen.stdout {
    match BufReader::new(stdout).read_line(&mut buf) {
      _ => ()
    }
    buf.pop();
    if buf.len() == 0 {
      return Err("Network not selected".to_string());
    }
  }

  Command::new("netctl-auto")
    .args(["switch-to", &buf])
    .spawn()
    .map_err(|_|{
      "netctl not found".to_string()
    })?;
  Ok(())
}
