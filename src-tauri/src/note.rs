use std::fs;
use std::path::Path;
use dirs_next;

pub fn get_notes(search: String) -> Result<Vec<String>, String> {
  ensure_notes_folder();
  let lower_search = search.to_lowercase();
  match fs::read_dir(get_notes_folder()) {
    Ok(dir) => {
      let mut notes = dir
        .filter(|entry| {
          return entry.is_ok();
        })
        .map(|entry| {
          return String::from(entry.unwrap().file_name().to_str().unwrap());
        })
        .filter(|file| file.ends_with(".txt"))
        .map(get_name_without_extension)
        .filter(|name| {
          let lower_name = name.to_lowercase();
          return lower_name.contains(&lower_search);
        })
        .collect::<Vec<String>>();
      notes.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
      return Ok(notes);
    },
    Err(err) => {
      let err_msg = err.to_string();
      return Err(err_msg);
    }
  };
}

fn get_name_without_extension(file_name: String) -> String {
  let count = file_name.len();
  let chars = file_name.chars();
  let extension_length = 4;
  return chars.take(count - extension_length).collect::<String>();
}

pub fn read_note(name: &str) -> Result<String, String> {
  let note_path = wrap_absolute_path(name);
  return match fs::read_to_string(note_path) {
    Ok(note_content) => Ok(note_content),
    Err(err) => {
      let err_msg = err.to_string();
      return Err(format!("Can't read note. {err_msg}"));
    },
  }
}

pub fn create_note(name: &str) -> Result<(), String> {
  ensure_notes_folder();
  let path = wrap_absolute_path(name);
  let result = fs::write(path, name);
  if result.is_err() {
    let err = result.err().unwrap().to_string();
    return Err(err);
  }
  return Ok(());
}

pub fn save_note(name_old: &str, name_new: &str, content: &str) -> Result<(), String> {
  ensure_notes_folder();

  let path_old = wrap_absolute_path(name_old);
  let path_new = wrap_absolute_path(name_new);

  println!("{}", format!("save_note[old={path_old},new={path_new}"));

  if path_old != path_new {
    if Path::new(&path_new).exists() {
      println!("File exists");
      return Err(format!("Can't rename note: note with the same name already exists"));
    }

    println!("{}", format!("renaming[old={path_old},new={path_new}"));

    let result = fs::rename(&path_old, &path_new);
    if result.is_err() {
      let err = result.err().unwrap().to_string();
      return Err(format!("Can't rename note. {err}"));
    }
  }

  let result = fs::write(path_new, content);
  if result.is_err() {
    let err = result.err().unwrap().to_string();
    return Err(format!("Can't rename note. {err}"));
  }
  return Ok(());
}

fn get_notes_folder() -> String {
  let documents_dir = dirs_next::document_dir().unwrap().to_str().unwrap().to_string();
  return format!("{documents_dir}/focus")
}

fn wrap_absolute_path(note_name: &str) -> String {
  let notes_folder = get_notes_folder();
  return format!("{notes_folder}/{note_name}.txt");
}

fn ensure_notes_folder() {
  let notes_folder = &get_notes_folder();
  if !Path::new(notes_folder).exists() {
    let _ = fs::create_dir(notes_folder);
  }
}
