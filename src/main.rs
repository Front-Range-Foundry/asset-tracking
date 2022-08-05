use uuid::Uuid; 
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    Input
};
#[macro_use] extern crate prettytable;
use prettytable::{Table};

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

                println!("You have decided to choose: {}", chosen);
                match chosen {
                    0 => {
                        self.create_asset()?;
                    },
                    1 => {
                    },
                    2 => {
                    },
                    3 => {
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
            .with_prompt("Enter a name for the asset:")
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