pub mod FolderDownloadFilter {
    use dirs;
    use std::fs;
    use std::path::PathBuf;



    impl TCreateFolder for Void {

        fn getPathFolderDownloadAndCreateFolderFiltersIfNoExist(&self) {

            if let Some(download_dir) = dirs::download_dir() 
            {
                let new_dir = download_dir.join("FilterDownload");

                if !new_dir.exists()
                {
                    if let Err(e) = fs::create_dir(&new_dir) 
                    {
                        println!("Error to create 'FilterDownload =>' : {}", e);
                    }
                }
            } 
            
            else 
            {
                println!("Unable to find the downloads folder");
            }

        }




        fn DisplayASCIII(&self)
        {
            const RED: &str = "\x1b[31m";
            const RESET: &str = "\x1b[0m";

            let ASCiii: &str = r#"
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


            println!("{}{}{}", RED , ASCiii, RESET);
        }

    }




    pub struct Void;



    
    pub trait TCreateFolder {
        fn getPathFolderDownloadAndCreateFolderFiltersIfNoExist(&self);
        fn DisplayASCIII(&self);
    }
}