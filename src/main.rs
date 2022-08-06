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
                        self.lookup_asset(true);
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

        let asset = Asset {
            id: Uuid::new_v4(),
            name: input
        };
        self.asset_library.dir.push(asset);
        println!("Asset Created");
        Ok(())
    } 

    fn view_assets(&self) {
        let assets = &self.asset_library.dir;
        let mut table = Table::new();
        table.add_row(row!["ID", "NAME"]);
        for asset in assets {
            table.add_row(row![asset.id, asset.name]);
        }

        table.printstd();
    }

    fn lookup_asset(&self, should_render: bool) -> Option<Asset> {
        let assets = &self.asset_library.dir;
        let input: String = Input::new()
            .with_prompt("Enter asset uuid")
            .interact_text()
            .expect("Invalid io entry.");

        match Uuid::parse_str(&input) {
            Ok(uuid) => {
                match assets.iter().position(|r| r.id == uuid) {
                    Some(index) => {
                        if should_render {  
                            let mut table = Table::new();
                            table.add_row(row!["ID", "NAME"]);
                            table.add_row(row![assets[index].id, assets[index].name]);
                            table.printstd();
                        }
                        let asset = &assets[index];
                        Some(asset)
                    },
                    None => {
                        println!("No asset with UUID {} could be located.", uuid);
                        None
                    }
                };
            },
            Err(_e)  => {
                println!("The uuid passed into search was not valid.");
            }
        };
        None
    }

    fn update_asset(&mut self) {
        match self.lookup_asset(false) {
            Some(mut asset) => {
                let input: String = Input::new()
                    .with_prompt("Enter a new name for the asset")
                    .interact_text()
                    .expect("Something went wrong during text entry.");

                asset.name = input;
                println!("Asset updated: ");
            },
            None => {
                println!("Asset not found.");
            }
        }
    }

    fn delete_asset(&mut self) {
        let assets = &mut self.asset_library.dir;
        let input: String = Input::new()
            .with_prompt("Enter the asset's UUID")
            .interact_text()
            .expect("Something went wrong during text entry.");

        match Uuid::parse_str(&input) {
            Ok(uuid) => {
                let mut index = 0;
                while index < assets.len() {
                    if assets[index].id == uuid {
                        assets.remove(index);
                    }
                    index += 1;
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