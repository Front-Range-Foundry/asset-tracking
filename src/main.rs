use uuid::{Uuid}; 
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    Input
};
#[macro_use] extern crate prettytable;
use prettytable::{Table };

struct Asset { 
    id: Uuid,
    name: String
}

struct AssetLibrary {
    dir: Vec<Asset>
}

struct Session {
    is_started: bool,
    asset_library: AssetLibrary,
}

impl AssetLibrary {
    fn get_index(&self, id: Uuid) -> Option<usize> {
        let assets = &self.dir;
        match assets.iter().position(|r| r.id == id) {
            Some(index) => {
                Some(index)
            },
            None => {
                None
            }
        }
    }
    fn get_one(&self, id: Uuid) -> Option<&Asset> {
       match self.get_index(id) {
            Some(index) => {
                Some(&self.dir[index])
            },
            None => {
                None
            }
       }
    }
    fn get_all(&self) -> &Vec<Asset> {
        &self.dir
    }
    fn update_one(&mut self, id: Uuid, name: String) -> Option<&Asset> {
        match self.get_index(id) {
            Some(index) => {
                self.dir[index].name = name;
                Some(&self.dir[index])
            },
            None => {
                None
            }
        }
    }
    fn delete_one(&mut self, id: Uuid) -> Option<Asset> {
        match self.get_index(id) {
            Some(index) => {
                let removed_asset = self.dir.remove(index);
                Some(removed_asset)
            },
            None => { None }
        }
    }
    fn create_one(&mut self, name: String) -> usize {
        let new_asset = Asset {
            id: Uuid::new_v4(),
            name
        };

        self.dir.push(new_asset);
        self.dir.len()
    }
}

impl Session {
    fn start(&mut self) -> std::io::Result<()> {
        println!("Welcome to Asset Handling.");
        if !self.is_started { 
            self.is_started = true;
            println!("The session has commenced.");

            loop {
                let items = vec![
                    "Create an Asset",
                    "Asset Lookup",
                    "Modify an Asset",
                    "Delete an Asset",
                    "View All Assets",
                    "Exit Program",
                ];

                let chosen: usize = Select::with_theme(&ColorfulTheme::default())
                    .items(&items)
                    .default(0)
                    .interact()?;

                match chosen {
                    0 => {
                        self.create_asset()?;
                    },
                    1 => {
                        self.lookup_asset();
                    },
                    2 => {
                        self.update_asset();
                    },
                    3 => {
                        self.delete_asset();
                    },
                    4 => {
                        self.view_assets();
                    },
                    5 => {
                        break;
                    },
                    _ => {
                        println!("You managed to select in invalid option! Ridiculous!");
                    }
                }
            }
            Ok(())
        } else {
            Ok(())
        } 
    }
    fn create_asset(&mut self) -> std::io::Result<()> {
        let input: String = Input::new()
            .with_prompt("Enter a name for the asset")
            .interact_text()?;
        self.asset_library.create_one(input);
        println!("Asset Created");
        Ok(())
    } 
    fn view_assets(&self) {
        let assets = self.asset_library.get_all();
        let mut table = Table::new();
        table.add_row(row!["ID", "NAME"]);
        for asset in assets {
            table.add_row(row![asset.id, asset.name]);
        }
        table.printstd();
    }
    fn lookup_asset(&self) -> Option<Asset> {
        let input: String = Input::new()
            .with_prompt("Enter asset uuid")
            .interact_text()
            .expect("Invalid io entry.");

        match Uuid::parse_str(&input) {
            Ok(uuid) => {
                match self.asset_library.get_one(uuid) {
                    Some(asset) => {
                        let mut table = Table::new();
                        table.add_row(row!["ID", "NAME"]);
                        table.add_row(row![asset.id, asset.name]);
                        table.printstd();
                    },
                    None => {
                        println!("No asset found for UUID {}", input);
                    }
                }
            },
            Err(_e)  => {
                println!("The uuid passed into search was not valid.");
            }
        };
        None
    }
    fn update_asset(&mut self) {
        let input: String = Input::new()
            .with_prompt("Enter the asset's UUID")
            .interact_text()
            .expect("Something went wrong during text entry.");

        match Uuid::parse_str(&input) {
            Ok(uuid) => {
                let input: String = Input::new()
                    .with_prompt("Enter a new name for the asset")
                    .interact_text()
                    .expect("Invalid entry.");

                match self.asset_library.update_one(uuid, input) {
                    Some(asset) => {
                        println!("Asset {} now has {} for a name.", asset.id, asset.name);    
                    }
                    None => {
                        println!("No Asset with UUID {} could be found.", uuid);
                    }
                }
            }, 
            Err(_e) => {
                println!("Invalid  uuid.");
            }
        }
    }
    fn delete_asset(&mut self) {
        let input: String = Input::new()
            .with_prompt("Enter the asset's UUID")
            .interact_text()
            .expect("Something went wrong during text entry.");

        match Uuid::parse_str(&input) {
            Ok(uuid) => {
                match self.asset_library.delete_one(uuid) {
                    Some(asset) => {
                        println!("Asset {} was deleted.", asset.id);
                    }, 
                    None => {
                        println!("Asset {} could not be found.", uuid);
                    }
                }
            }, 
            Err(_e) => {
                println!("Invalid  uuid.");
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let dir: Vec<Asset> = Vec::new();
    let asset_library: AssetLibrary = AssetLibrary {
        dir
    };

    let mut console_session = Session {
        is_started: false,
        asset_library
    };
    console_session.start()?;
    Ok(())
}