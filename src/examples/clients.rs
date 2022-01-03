//! An example using clients and files.
//!
//! ```rust
//! use safe_index::examples::clients::{*, idx::{FileSet, ClientSet}};
//!
//! let mut data = Data::new();
//!
//! let c_1 = data.add_client("client 1");
//! let c_2 = data.add_client("client 2");
//! let c_3 = data.add_client("client 3");
//! let c_4 = data.add_client("client 4");
//!
//! let c_1_too = data.clients.index_from_usize(0);
//! assert_eq!(Some(c_1), c_1_too);
//! let c_2_too = data.clients.index_from_usize(1);
//! assert_eq!(Some(c_2), c_2_too);
//! let c_3_too = data.clients.index_from_usize(2);
//! assert_eq!(Some(c_3), c_3_too);
//! let c_4_too = data.clients.index_from_usize(3);
//! assert_eq!(Some(c_4), c_4_too);
//! let none_because_only_four_clients = data.clients.index_from_usize(4);
//! assert_eq!(None, none_because_only_four_clients);
//!
//! let f_1 = data.add_file(FileInfo::new("file 1", vec![c_1, c_2]));
//! let f_2 = data.add_file(FileInfo::new("file 2", vec![c_3]));
//! let f_3 = data.add_file(FileInfo::new("file 3", vec![c_2]));
//! let f_4 = data.add_file(FileInfo::new("file 4", vec![c_4]));
//!
//! let classes = data.client_clusters();
//! let expected: Vec<(ClientSet, FileSet)> = vec![
//!     (
//!         vec![c_1, c_2].into_iter().collect(),
//!         vec![f_1, f_3].into_iter().collect()
//!     ),
//!     (
//!         vec![c_3].into_iter().collect(),
//!         vec![f_2].into_iter().collect()
//!     ),
//!     (
//!         vec![c_4].into_iter().collect(),
//!         vec![f_4].into_iter().collect()
//!     ),
//! ];
//! assert_eq! { classes, expected }
//!
//! data.add_client_to_file(c_3, f_3);
//!
//! let classes = data.client_clusters();
//! let expected: Vec<(ClientSet, FileSet)> = vec![
//!     (
//!         vec![c_1, c_2, c_3].into_iter().collect(),
//!         vec![f_1, f_2, f_3].into_iter().collect()
//!     ),
//!     (
//!         vec![c_4].into_iter().collect(),
//!         vec![f_4].into_iter().collect()
//!     ),
//! ];
//! assert_eq! { classes, expected }
//! ```

/// Indices.
pub mod idx {
    new! {
        /// Indices of clients.
        Client,
        /// Map from clients to something.
        map: Clients,
        /// Set of clients.
        btree set: ClientSet,
    }

    new! {
        /// Indices of files.
        File,
        /// Map from files to something.
        map: Files,
        /// Set of files.
        btree set: FileSet,
    }
}

use idx::*;

/// Client information.
pub struct ClientInfo {
    /// Name of the client.
    pub name: alloc::string::String,
    /// Files associated with the client.
    pub files: FileSet,
}
/// File information.
pub struct FileInfo {
    /// Name of the file.
    pub name: alloc::string::String,
    /// Clients concerned by the file.
    pub clients: ClientSet,
}
impl FileInfo {
    /// Constructor.
    pub fn new<S, I>(name: S, clients: I) -> Self
    where
        S: Into<alloc::string::String>,
        I: core::iter::IntoIterator<Item = Client>,
    {
        FileInfo {
            name: name.into(),
            clients: clients.into_iter().collect(),
        }
    }
}

impl core::ops::Index<Client> for Data {
    type Output = ClientInfo;
    fn index(&self, client: Client) -> &ClientInfo {
        &self.clients[client]
    }
}

/// Aggregates client and file info.
pub struct Data {
    /// Map from client indexes to client information.
    pub clients: Clients<ClientInfo>,
    /// Map from file indexes to file information.
    pub files: Files<FileInfo>,
}
impl Data {
    /// Constructor.
    pub fn new() -> Data {
        Data {
            clients: Clients::with_capacity(103),
            files: Files::with_capacity(103),
        }
    }

    /// Adds a client.
    ///
    /// Does not add the client again if it's already there (by name).
    pub fn add_client<S: Into<alloc::string::String>>(&mut self, name: S) -> Client {
        let name = name.into();
        for (client, info) in self.clients.index_iter() {
            if info.name == name {
                return client;
            }
        }
        self.clients.push(ClientInfo {
            name,
            files: FileSet::new(),
        })
    }

    /// Adds a file, updates the clients concerned.
    pub fn add_file(&mut self, file: FileInfo) -> File {
        let idx = self.files.push(file);
        let file = &self.files[idx];
        for client in &file.clients {
            let is_new = self.clients[*client].files.insert(idx);
            debug_assert! { is_new }
        }
        idx
    }

    /// Retrieves information about a file.
    pub fn get_file(&mut self, file: File) -> &FileInfo {
        &self.files[file]
    }

    /// Adds a client to a file.
    pub fn add_client_to_file(&mut self, client: Client, file: File) {
        let is_new = self.files[file].clients.insert(client);
        debug_assert! { is_new }
        let is_new = self.clients[client].files.insert(file);
        debug_assert! { is_new }
    }

    /// Returns the client equivalence classes.
    ///
    /// Two clients are in the same equivalence class if they are associated to the same file,
    /// transitively.
    pub fn client_clusters(&self) -> alloc::vec::Vec<(ClientSet, FileSet)> {
        let mut res: alloc::vec::Vec<(ClientSet, FileSet)> = alloc::vec![];
        macro_rules! is_known {
            ($file:expr) => {
                res.iter().any(|(_, files)| files.contains(&$file))
            };
        }

        'all_files: for (file, file_info) in self.files.index_iter() {
            if is_known!(file) {
                continue 'all_files;
            }

            let (mut clients, mut files) = (ClientSet::new(), FileSet::new());
            files.insert(file);

            let mut to_dos = alloc::vec![&file_info.clients];

            while let Some(to_do) = to_dos.pop() {
                for client in to_do {
                    let is_new = clients.insert(*client);
                    if is_new {
                        for file in &self.clients[*client].files {
                            let is_new = files.insert(*file);
                            if is_new {
                                to_dos.push(&self.files[*file].clients)
                            }
                        }
                    }
                }
            }

            res.push((clients, files))
        }
        res
    }
}

#[test]
fn run() {
    let mut data = Data::new();

    let c_1 = data.add_client("client 1");
    let c_2 = data.add_client("client 2");
    let c_3 = data.add_client("client 3");
    let c_4 = data.add_client("client 4");

    let f_1 = data.add_file(FileInfo::new("file 1", alloc::vec![c_1, c_2]));
    let f_2 = data.add_file(FileInfo::new("file 2", alloc::vec![c_3]));
    let f_3 = data.add_file(FileInfo::new("file 3", alloc::vec![c_2]));
    let f_4 = data.add_file(FileInfo::new("file 4", alloc::vec![c_4]));

    let classes = data.client_clusters();
    let expected: alloc::vec::Vec<(ClientSet, FileSet)> = alloc::vec![
        (
            alloc::vec![c_1, c_2].into_iter().collect(),
            alloc::vec![f_1, f_3].into_iter().collect(),
        ),
        (
            alloc::vec![c_3].into_iter().collect(),
            alloc::vec![f_2].into_iter().collect(),
        ),
        (
            alloc::vec![c_4].into_iter().collect(),
            alloc::vec![f_4].into_iter().collect(),
        ),
    ];
    assert_eq! { classes, expected }

    data.add_client_to_file(c_3, f_3);

    let classes = data.client_clusters();
    let expected: alloc::vec::Vec<(ClientSet, FileSet)> = alloc::vec![
        (
            alloc::vec![c_1, c_2, c_3].into_iter().collect(),
            alloc::vec![f_1, f_2, f_3].into_iter().collect(),
        ),
        (
            alloc::vec![c_4].into_iter().collect(),
            alloc::vec![f_4].into_iter().collect(),
        ),
    ];
    assert_eq! { classes, expected }
}
