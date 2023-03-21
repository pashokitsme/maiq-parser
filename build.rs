use std::{env, fs, path::Path, process::Command};

fn get_build_profile_name() -> String {
  env::var("OUT_DIR")
    .unwrap()
    .split(std::path::MAIN_SEPARATOR)
    .nth_back(3)
    .unwrap_or("unknown")
    .to_string()
}

fn rcopy(from: &Path, to: &Path) {
  if fs::metadata(to).is_err() {
    fs::create_dir(to).unwrap();
  }

  let paths = fs::read_dir(from).unwrap();
  for path in paths.map(|p| p.unwrap()).filter(|p| p.metadata().unwrap().is_file()) {
    fs::copy(path.path(), to.join(path.file_name())).unwrap();
  }
}

fn main() {
  println!("cargo:rerun-if-changed=defaults/");
  if get_build_profile_name() != "cli" {
    println!("Profile is not cli");
    return;
  }

  let manifest_dir_str = env::var("CARGO_MANIFEST_DIR").unwrap();
  let default = Path::new(&manifest_dir_str).join("default");
  let output = Path::new(&manifest_dir_str).join("target\\cli").join("default");
  rcopy(&default, &output);
}
