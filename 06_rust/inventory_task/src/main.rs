use std::collections::HashMap;
use std::fmt::{self, Debug};

#[derive(Debug)]
enum InventoryError {
    DuplicateId,
    InvalidId,
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InventoryError::DuplicateId => {
                write!(f, "Duplicate ID. Try giving a new ID.")
            }
            InventoryError::InvalidId => {
                write!(f, "Given ID does not exist.")
            }
        }
    }
}

trait DisplayItem {
    fn display(&self) -> String;
}

#[derive(Clone, Debug)]
struct Item {
    data: HashMap<String, i32>,
}

impl DisplayItem for Item {
    fn display(&self) -> String {
        format!("{:?}", self.data)
    }
}

#[derive(Clone, Debug)]
struct Inventory<T>
where
    T: DisplayItem + Clone,
{
    items: HashMap<String, T>,
}

impl<T> Inventory<T>
where
    T: DisplayItem + Clone + Debug,
{
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    fn add_item(&mut self, id: String, item: T) -> Result<(), InventoryError> {
        if self.items.contains_key(&id) {
            return Err(InventoryError::DuplicateId);
        }

        self.items.insert(id, item);
        Ok(())
    }

	fn get_item(&self,id_param:&str) -> Result<(), InventoryError>{
		match self.items.get(id_param) {
			Some(item) => {
				println!("Item with id : {id_param} is {:?}",item); Ok(())},
			None => Err(InventoryError::InvalidId),
		}
	}

    fn display(&self) {
        for (id, item) in &self.items {
            println!("ID: {id}, Item: {}", item.display());
        }
    }
}

fn main() {
    let mut laptop_data = HashMap::new();
    laptop_data.insert("Laptop".to_string(), 50_000);

    let laptop = Item { data: laptop_data };

    let mut inventory = Inventory::new();

    inventory
        .add_item("1".to_string(), laptop)
        .expect("Failed to add item");

	let item = inventory.get_item("7");

	match item {
		Ok(()) => println!("Item fetched successfully"),
		Err(e) => println!("Error: {}", e),
	}

    inventory.display();
}