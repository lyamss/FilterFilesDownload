mod create_folder {
    pub mod folder_download_filter;
    pub mod filter_file;
}


use create_folder::folder_download_filter::folder_download_filter::{TCreateFolder, Void};
use create_folder::filter_file;
use std::thread;
use std::time::Duration;

fn main()
{
    let go = Void;
    go.display_ascii();

    loop {

        go.create_folder_filters_if_no_exist_in_folder_download();
    
    
        filter_file::filter_file::filter_all_files_in_folder_download();

        
        thread::sleep(Duration::from_secs(5));
    }

}