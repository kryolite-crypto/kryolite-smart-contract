use std::io::Write;

pub fn write_json(json: &String) {
  let _ = std::fs::create_dir("pkg").is_ok();
  let mut file = std::fs::File::create("pkg/manifest.json").expect("create failed");
  file.write_all(json.as_bytes()).expect("write failed");
}
