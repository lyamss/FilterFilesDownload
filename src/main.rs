mod CreateFolder {
    pub mod FolderDownloadFilter;
}


use CreateFolder::FolderDownloadFilter::FolderDownloadFilter::{TCreateFolder, Void};

fn main()
{

    let go = Void;
    go.DisplayASCIII();
    go.getPathFolderDownloadAndCreateFolderFiltersIfNoExist();

}