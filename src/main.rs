use std::thread;
use std::time::Duration;
mod create_folder;
use create_folder::filter_file;
use create_folder::folder_download_filter::{Void, TCreateFolder};

fn main()
{
    let go = Void;
    go.display_ascii();

    loop {

        Void::create_folder_filters_if_no_exist_in_folder_download();
    
        filter_file::filter_all_files_in_folder_download();

        
        thread::sleep(Duration::from_secs(5));
    }

}