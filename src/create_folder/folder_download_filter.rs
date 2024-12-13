use dirs;
use std::{fs, path::PathBuf};


    pub struct Void;


    pub trait TCreateFolder {
        fn create_folder_filters_if_no_exist_in_folder_download();
        fn display_ascii(&self);
    }



    impl TCreateFolder for Void {


        fn create_folder_filters_if_no_exist_in_folder_download() {


            if let Some(download_dir) = dirs::download_dir() 
            {
                let base_dir: PathBuf = download_dir.join("FilterDownload");
                let sub_dirs: Vec<&str> = vec![
                    "Audio",
                    "Documents",
                    "Movies",
                    "Images",
                    "Other",
                ];

                if !base_dir.exists()
                {
                    if let Err(e) = fs::create_dir(&base_dir) 
                    {
                        eprintln!("Error to create 'FilterDownload =>' : {}", e);
                        return;
                    }

                    for sub_dir in sub_dirs 
                    {
                        let path: PathBuf = base_dir.join(sub_dir);
                        if let Err(e) = fs::create_dir_all(&path) 
                        {
                            eprintln!("Error creating '{}': {}", sub_dir, e);
                        }
                    }
                }
            } 
            
            else 
            {
                eprintln!("Unable to find the downloads folder");
            }

        }




        fn display_ascii(&self)
        {
            const RED: &str = "\x1b[31m";
            const RESET: &str = "\x1b[0m";

            let asciii: &str = r#"
                        ______  _____  __  __  ______
                       /_____/\ /_____/\ \/ / /_____/
                       \:::_ \ \:::_ \ \  / / \:::_ \
                        \:\ \ \ \:\ \ \ \ / /_ \:\ \ \
                         \:\_\_\ \:\_\_\ \ \/__/ \:\_\_\
                          \:\_____\/_____\/_____/  \:_____/
                           \/_____/\/_____/\/_____/   \/_____/

                          _____  _____  _____  _____
                         /_____/ /_____/ /_____/ /____/
                         \:::_ \ \:::_ \ \:::_ \ \:::_ \
                          \:\ \ \ \:\ \ \ \:\ \ \ \:\ \ \
                           \:\_\_\ \:\_\_\ \:\_\_\ \:\_\_\
                            \:_____\/_____\/_____\/_____/
                             \/_____/\/_____/\/_____/\/


💀💀🐐🐐🐐🔥🔥🔥🔥☣️☣️💀
            
            "#; 


            println!("{}{}{}", RED , asciii, RESET);
        }

    }