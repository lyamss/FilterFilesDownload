use dirs;
use std::fs::{self, ReadDir};
use std::path::PathBuf;


    pub fn filter_all_files_in_folder_download()
    {


        if let Some(download_dir) = dirs::download_dir()
        {

            let base_dir: PathBuf = download_dir.join("FilterDownload");


            if base_dir.exists()
            {

                let directory: ReadDir = fs::read_dir(download_dir).unwrap();


                for file in directory
                {

                    let path: &PathBuf = &file.unwrap().path();


                    if path.is_file() || path.is_dir()
                    {

                        if let Some(extension) = path.extension() 
                        {

                            
                            if let Some(ext_str) = extension.to_str() 
                            {
                                
                                    let ext_lower: String = ext_str.to_lowercase();


                                    if let Some(file_name) = path.file_name()
                                    {
                                        let sub_dirs: Vec<(&str, Vec<&str>)> = vec![
                                            ("Audio", vec!["mp3", "wav", "flac", "aac", "aiff"]),
                                            ("Documents", vec!["pdf", "txt", "csv", "md"]),
                                            ("Movies", vec!["mp4", "mov", "mpeg"]),
                                            ("Images", vec!["gif", "png", "jpg", "jpeg", "svg"]),
                                            ("Other", vec![]),
                                        ];


                                        if let Some((dest_folder, _)) = sub_dirs.iter().find(|&&(_, ref allowed_exts)| allowed_exts.contains(&ext_lower.as_str())) 
                                        {
                                            let destination_path:PathBuf = base_dir.join(dest_folder).join(file_name);
                                            if let Err(e) = fs::rename(&path, &destination_path) 
                                            {
                                                eprintln!("Error moving file '{}': {}", path.display(), e);
                                            }
                                        } 

                                        else 
                                        {
                                            let destination_path:PathBuf = base_dir.join("Other").join(file_name);
                                            if let Err(e) = fs::rename(&path, &destination_path) 
                                            {
                                                eprintln!("Error moving file '{}': {}", path.display(), e);
                                            }
                                        }

                                    }
                            }
                        }

                        else 
                        {
                            if let Some(p) = path.file_name()
                            {
                                let destination_path:PathBuf = base_dir.join("Other").join(p);
                                if let Err(e) = fs::rename(&path, &destination_path) 
                                {
                                    drop(e);
                                    // eprintln!("Error moving file '{}': {}", path.display(), e);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        else 
        {
            println!("Unable to find the downloads folder");
        }
    }