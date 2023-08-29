use std::collections::HashMap;
use std::io::{self, Write};

// This is a interactive program that allows the user to create a business, add a menu to the business, add an order to the business, and show orders for the business.
fn main() {
    let mut restaurateur = Restaurateur::new();

    loop {
        println!("1. Create a new business");
        println!("2. Add a menu to an existing business");
        println!("3. Add an order to an existing business");
        println!("4. Show orders for an existing business");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse::<u32>() {
            Ok(1) => {
                create_business(&mut restaurateur);
            }
            Ok(2) => {
                create_menu(&mut restaurateur);
            }
            Ok(3) => {
                add_order(&mut restaurateur);
            }
            Ok(4) => {
                show_orders(&mut restaurateur);
            }
            Ok(5) => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice!");
            }
        }
    }
}


#[derive(Debug, Clone)]
pub enum FoodType {
    Veg,
    NonVeg,
}

#[derive(Debug, Clone)]
pub enum FoodCategory {
    Appetizer,
    MainCourse,
    Dessert,
}

#[derive(Debug, Clone)]
pub struct Food {
    pub(crate) name: String,
    pub(crate) food_type: FoodType,
    pub(crate) food_category: FoodCategory,
    pub(crate) price: f32,
}

#[derive(Debug, Clone)]
pub enum PaymentMode {
    Cash,
    Card,
    UPI,
    Wallet,
}

#[derive(Debug, Clone)]
pub struct FoodOrder {
    food: Food,
    quantity: u8,
    price: f32,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub(crate) foods: Vec<FoodOrder>,
    pub(crate) customer: Customer,
    pub(crate) date: String,
    pub(crate) payment_mode: PaymentMode,
    pub(crate) price: f32,
}

#[derive(Debug, Clone)]
pub struct Customer {
    pub(crate) name: String,
    pub(crate) age: u8,
    pub(crate) address: String,
    pub(crate) phone: String,
}

#[derive(Debug, Clone)]
pub struct Cuisine {
    pub(crate) name: String,
    pub(crate) foods: Vec<Food>,
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub(crate) name: String,
    pub(crate) price: f32,
    pub(crate) is_veg: bool,
    pub(crate) cuisines: HashMap<String, Cuisine>,
}

#[derive(Debug, Clone)]
pub struct Business {
    pub(crate) name: String,
    pub(crate) address: String,
    pub(crate) phone: String,
    pub(crate) menu: Option<Menu>,
    pub(crate) orders: Vec<Order>,
}

pub trait Restaurant {
    fn add_business(&mut self, business: Business);
    fn remove_business(&mut self, name: String);
    fn get_business(&self, name: String) -> Option<&Business>;
    fn add_menu(&mut self, name: String, menu: Menu);
    fn add_order(&mut self, name: String, order: Order);
    fn remove_order(&mut self, name: String, date: String);
    fn show_orders(&self, name: String) -> Vec<Order>;
}

#[derive(Debug, Clone)]
pub struct Restaurateur {
    cache: HashMap<String, Business>,
}

impl Restaurateur {
    fn new() -> Restaurateur {
        Restaurateur {
            cache: HashMap::new(),
        }
    }
}

impl Restaurant for Restaurateur {
    fn add_business(&mut self, business: Business) {
        self.cache.insert(business.name.clone(), business);
    }

    fn remove_business(&mut self, name: String) {
        self.cache.remove(&name);
    }

    fn get_business(&self, name: String) -> Option<&Business> {
        self.cache.get(&name)
    }

    fn add_menu(&mut self, name: String, menu: Menu) {
        if let Some(business) = self.cache.get_mut(&name) {
            business.menu = Some(menu);
        }
    }

    fn add_order(&mut self, name: String, order: Order) {
        if let Some(business) = self.cache.get_mut(&name) {
            business.orders.push(order);
        }
    }

    fn remove_order(&mut self, name: String, date: String) {
        if let Some(business) = self.cache.get_mut(&name) {
            business.orders.retain(|order| order.date != date);
        }
    }

    fn show_orders(&self, name: String) -> Vec<Order> {
        if let Some(business) = self.cache.get(&name) {
            business.orders.clone()
        } else {
            vec![]
        }
    }
}



fn show_orders(restaurateur: &mut Restaurateur) {
    print!("Enter business name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    if let Some(business) = restaurateur.get_business(name.trim().to_string()) {
        let orders = restaurateur.show_orders(name.trim().to_string());

        if orders.is_empty() {
            println!("No orders found for business!");
        } else {
            println!("Orders for business '{}':", business.name);

            for order in orders {
                println!("Date: {}", order.date);
                println!("Customer: {}", order.customer.name);
                println!("Payment mode: {:?}", order.payment_mode);
                println!("Price: {}", order.price);
                println!("Foods:");

                for food in order.foods {
                    println!("- {} ({} x {}): {}", food.food.name, food.quantity, food.food.price, food.price);
                }

                println!();
            }
        }
    } else {
        println!("Business not found!");
    }
}

fn add_order(restaurateur: &mut Restaurateur) {
    print!("Enter business name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    if let Some(business) = restaurateur.get_business(name.trim().to_string()) {
        let mut foods = vec![];

        loop {
            print!("Enter food name (or 'done' to finish): ");
            io::stdout().flush().unwrap();
            let mut food_name = String::new();
            io::stdin().read_line(&mut food_name).unwrap();

            if food_name.trim().to_lowercase() == "done" {
                break;
            }

            let food = match business.menu {
                Some(ref menu) => {
                    let mut cuisine_name = String::new();

                    loop {
                        print!("Enter cuisine name: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut cuisine_name).unwrap();

                        if let Some(cuisine) = menu.cuisines.get(&cuisine_name.trim().to_string()) {
                            let mut food_found = false;

                            for f in &cuisine.foods {
                                if f.name == food_name.trim().to_string() {
                                    foods.push(FoodOrder {
                                        food: f.clone(),
                                        quantity: 1,
                                        price: f.price,
                                    });
                                    food_found = true;
                                    break;
                                }
                            }

                            if food_found {
                                break;
                            } else {
                                println!("Food not found in cuisine!");
                            }
                        } else {
                            println!("Cuisine not found in menu!");
                        }
                    }
                }
                None => {
                    println!("Menu not found for business!");
                    continue;
                }
            };
        }

        let mut customer_name = String::new();
        print!("Enter customer name: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut customer_name).unwrap();

        let mut customer_age = String::new();
        print!("Enter customer age: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut customer_age).unwrap();

        let mut customer_address = String::new();
        print!("Enter customer address: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut customer_address).unwrap();

        let mut customer_phone = String::new();
        print!("Enter customer phone: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut customer_phone).unwrap();

        let order = Order {
            customer: Customer {
                name: customer_name.trim().to_string(),
                age: match customer_age.trim().parse::<u8>() {
                    Ok(age) => age,
                    Err(_) => {
                        println!("Invalid age, defaulting to 0");
                        0
                    }
                },
                address: customer_address.trim().to_string(),
                phone: customer_phone.trim().to_string(),
            },
            date: "2021-10-01".to_string(),
            payment_mode: PaymentMode::Card,
            price: *&foods.iter().map(|f| f.price).sum::<f32>(),
            foods
        };

        restaurateur.add_order(name.trim().to_string(), order);
        println!("Order added successfully!");
    } else {
        println!("Business not found!");
    }
}

fn create_menu(restaurateur: &mut Restaurateur) {
    print!("Enter business name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    if let Some(business) = restaurateur.get_business(name.trim().to_string()) {
        print!("Enter menu name: ");
        io::stdout().flush().unwrap();
        let mut menu_name = String::new();
        io::stdin().read_line(&mut menu_name).unwrap();

        print!("Enter menu price: ");
        io::stdout().flush().unwrap();
        let mut menu_price = String::new();
        io::stdin().read_line(&mut menu_price).unwrap();

        print!("Is the menu veg? (y/n): ");
        io::stdout().flush().unwrap();
        let mut menu_is_veg = String::new();
        io::stdin().read_line(&mut menu_is_veg).unwrap();

        let mut cuisines = HashMap::new();

        loop {
            print!("Enter cuisine name (or 'done' to finish): ");
            io::stdout().flush().unwrap();
            let mut cuisine_name = String::new();
            io::stdin().read_line(&mut cuisine_name).unwrap();

            if cuisine_name.trim().to_lowercase() == "done" {
                break;
            }

            let mut foods = vec![];

            loop {
                print!("Enter food name (or 'done' to finish): ");
                io::stdout().flush().unwrap();
                let mut food_name = String::new();
                io::stdin().read_line(&mut food_name).unwrap();

                if food_name.trim().to_lowercase() == "done" {
                    break;
                }

                print!("Enter food type (veg/nonveg): ");
                io::stdout().flush().unwrap();
                let mut food_type = String::new();
                io::stdin().read_line(&mut food_type).unwrap();

                let food_type = match food_type.trim().to_lowercase().as_str() {
                    "veg" => FoodType::Veg,
                    "nonveg" => FoodType::NonVeg,
                    _ => {
                        println!("Invalid food type, defaulting to veg");
                        FoodType::Veg
                    }
                };

                print!("Enter food category (appetizer/maincourse/dessert): ");
                io::stdout().flush().unwrap();
                let mut food_category = String::new();
                io::stdin().read_line(&mut food_category).unwrap();

                let food_category = match food_category.trim().to_lowercase().as_str() {
                    "appetizer" => FoodCategory::Appetizer,
                    "maincourse" => FoodCategory::MainCourse,
                    "dessert" => FoodCategory::Dessert,
                    _ => {
                        println!("Invalid food category, defaulting to appetizer");
                        FoodCategory::Appetizer
                    }
                };

                print!("Enter food price: ");
                io::stdout().flush().unwrap();
                let mut food_price = String::new();
                io::stdin().read_line(&mut food_price).unwrap();

                let food_price = match food_price.trim().parse::<f32>() {
                    Ok(price) => price,
                    Err(_) => {
                        println!("Invalid food price, defaulting to 0");
                        0.0
                    }
                };

                let food = Food {
                    name: food_name.trim().to_string(),
                    food_type,
                    food_category,
                    price: food_price,
                };

                foods.push(food);
            }

            let cuisine = Cuisine {
                name: cuisine_name.trim().to_string(),
                foods,
            };

            cuisines.insert(cuisine_name.trim().to_string(), cuisine);
        }

        let menu = Menu {
            name: menu_name.trim().to_string(),
            price: match menu_price.trim().parse::<f32>() {
                Ok(price) => price,
                Err(_) => {
                    println!("Invalid menu price, defaulting to 0");
                    0.0
                }
            },
            is_veg: match menu_is_veg.trim().to_lowercase().as_str() {
                "y" => true,
                "n" => false,
                _ => {
                    println!("Invalid input, defaulting to veg");
                    true
                }
            },
            cuisines,
        };

        restaurateur.add_menu(name.trim().to_string(), menu);
        println!("Menu added successfully!");
    } else {
        println!("Business not found!");
    }
}

fn create_business(restaurateur: &mut Restaurateur) {
    print!("Enter business name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    print!("Enter business address: ");
    io::stdout().flush().unwrap();
    let mut address = String::new();
    io::stdin().read_line(&mut address).unwrap();

    print!("Enter business phone: ");
    io::stdout().flush().unwrap();
    let mut phone = String::new();
    io::stdin().read_line(&mut phone).unwrap();

    let business = Business {
        name: name.trim().to_string(),
        address: address.trim().to_string(),
        phone: phone.trim().to_string(),
        menu: None,
        orders: vec![],
    };

    restaurateur.add_business(business);
    println!("Business created successfully!");
}