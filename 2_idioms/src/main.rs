

#[derive(Debug, PartialEq, Clone)]
pub enum Coin {
    One = 1,
    Two = 2,
    Five = 5,
    Ten = 10,
    Twenty = 20,
    Fifty = 50,
}

#[derive(PartialEq, Debug, Clone)]
pub enum VendingMachineErrors {
    GlobalCapacityOverflow,
    CapacityOverflow,
    NoSuchSlot,
    NotEnoughMoney,
}

mod product {
    #[derive(Clone, Debug, PartialEq)]
    pub struct price(pub usize);

    #[derive(Clone, Debug, PartialEq)]
    pub struct name(pub &'static str);
}

#[derive(PartialEq, Debug, Clone)]
struct Product {
    name: product::name,
    price: product::price,
}

mod vending_machine {
    #[derive(Clone, Debug, PartialEq)]
    pub struct capacity(pub usize);

    #[derive(Clone, Debug, PartialEq)]
    pub struct global_capacity(pub usize);

    #[derive(Clone, Debug, PartialEq)]
    pub struct count(pub usize);
}

struct Slot {
    product: Product,
    capacity: vending_machine::capacity,
    count: vending_machine::count,
}

struct VendingMachine {
    slots: Vec<Slot>,
    global_capacity: vending_machine::global_capacity,
}

impl VendingMachine {
    fn new(c: vending_machine::global_capacity) -> Self {
        Self {
            slots: Vec::with_capacity(c.0),
            global_capacity: c,
        }
    }

    fn add_slot(&mut self, slot: Slot) -> Result<(), VendingMachineErrors>{
        if slot.count.0 <= slot.capacity.0{
            if self.global_capacity.0 > self.slots.len() {
                self.slots.push(slot);
                return Ok(());
            } else {
                return Err(VendingMachineErrors::GlobalCapacityOverflow);
            }
        } else {
            return Err(VendingMachineErrors::CapacityOverflow);
        }
    }

    fn show_slots(&self) {
        for i in &self.slots {
            println!("name: {}, price: {}, count: {}", i.product.name.0, i.product.price.0, i.count.0);
        }
    }

    fn delete_slot(&mut self, prod: &Product) -> Result<(), VendingMachineErrors> {
        let index = self.slots.iter().position(|slot| slot.product.name.0 == prod.name.0);

        match index {
            None => {
                return Err(VendingMachineErrors::NoSuchSlot);
            },
            Some(index) => {
                self.slots.swap_remove(index);
                return Ok(());
            },
        }
    }

    fn give_change(mut change: usize) -> Vec<Coin>{
        let mut change_vec: Vec<Coin> = Vec::new();
        while change != 0usize {
            if change >= 50 {
                change -= 50;
                change_vec.push(Coin::Fifty);
                continue;
            } else if change >= 20 {
                change -= 20;
                change_vec.push(Coin::Twenty);
                continue;
            } else if change >= 10 {
                change -= 10;
                change_vec.push(Coin::Ten);
                continue;
            } else if change >= 5 {
                change -= 5;
                change_vec.push(Coin::Five);
                continue;
            } else if change >= 2 {
                change -= 2;
                change_vec.push(Coin::Two);
                continue;
            } else if change >= 1 {
                change -= 1;
                change_vec.push(Coin::One);
                continue;
            }
        }
        return change_vec;
    }

    fn buy(&mut self, prod: Product, money: Vec<Coin>) -> Result<(Product, Vec<Coin>), VendingMachineErrors> {
        let mut money_summ: usize = 0;
        for i in money {
            money_summ += i as usize;
        }

        let p = self.slots.iter_mut().find(|slot| slot.product.name.0 == prod.name.0 && slot.product.price.0 == prod.price.0 && slot.count.0 >= 1);

        match p {
            None => {
                return Err(VendingMachineErrors::NoSuchSlot);
            },
            Some(slot) => {
                if money_summ >= slot.product.price.0 {
                    slot.count.0 -= 1;
                    return Ok((prod, VendingMachine::give_change(money_summ - slot.product.price.0)));
                } else {
                    return Err(VendingMachineErrors::NotEnoughMoney);
                }
            }
        }
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_product(name: &'static str, price: usize) -> Product {
        Product {
            name: product::name(name),
            price: product::price(price),
        }
    }

    fn create_slot(name: &'static str, price: usize, count: usize, capacity: usize) -> Slot {
        Slot {
            product: create_product(name, price),
            capacity: vending_machine::capacity(capacity),
            count: vending_machine::count(count),
        }
    }

    #[test]
    fn test_add_slot_success() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(2));
        let slot = create_slot("Water", 10, 3, 5);

        assert!(machine.add_slot(slot).is_ok());
        assert_eq!(machine.slots.len(), 1);
    }

    #[test]
    fn test_add_slot_capacity_overflow() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(2));
        let slot = create_slot("Water", 10, 6, 5);

        let result = machine.add_slot(slot);
        assert_eq!(result, Err(VendingMachineErrors::CapacityOverflow));
    }

    #[test]
    fn test_add_slot_global_capacity_overflow() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(1));
        let slot1 = create_slot("Water", 10, 3, 5);
        let slot2 = create_slot("Cola", 15, 2, 5);

        assert!(machine.add_slot(slot1).is_ok());
        let result = machine.add_slot(slot2);
        assert_eq!(result, Err(VendingMachineErrors::GlobalCapacityOverflow));
    }

    #[test]
    fn test_delete_slot_success() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(2));
        let prod = create_product("Water", 10);
        let slot = create_slot("Water", 10, 3, 5);

        machine.add_slot(slot).unwrap();
        assert!(machine.delete_slot(&prod).is_ok());
        assert_eq!(machine.slots.len(), 0);
    }

    #[test]
    fn test_delete_slot_not_found() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(2));
        let prod = create_product("Water", 10);

        let result = machine.delete_slot(&prod);
        assert_eq!(result, Err(VendingMachineErrors::NoSuchSlot));
    }

    #[test]
    fn test_give_change() {
        let change = VendingMachine::give_change(68);
        let expected = vec![
            Coin::Fifty,
            Coin::Ten,
            Coin::Five,
            Coin::Two,
            Coin::One,
        ];
        assert_eq!(change, expected);
    }

    #[test]
    fn test_buy_product_success_and_count_decreases() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(2));
        let slot = create_slot("Soda", 15, 2, 5);
        machine.add_slot(slot).unwrap();

        let target_prod = create_product("Soda", 15);
        let money = vec![Coin::Twenty];

        let result = machine.buy(target_prod.clone(), money);
        assert!(result.is_ok());

        let (bought_prod, change) = result.unwrap();
        assert_eq!(bought_prod, target_prod);
        assert_eq!(change, vec![Coin::Five]);
        assert_eq!(machine.slots[0].count.0, 1);
    }

    #[test]
    fn test_buy_product_not_enough_money() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(2));
        let slot = create_slot("Chips", 25, 2, 5);
        machine.add_slot(slot).unwrap();

        let target_prod = create_product("Chips", 25);
        let money = vec![Coin::Ten];

        let result = machine.buy(target_prod, money);
        assert_eq!(result, Err(VendingMachineErrors::NotEnoughMoney));
        assert_eq!(machine.slots[0].count.0, 2);
    }

    #[test]
    fn test_buy_product_out_of_stock() {
        let mut machine = VendingMachine::new(vending_machine::global_capacity(2));
        let slot = create_slot("Juice", 20, 0, 5);
        machine.add_slot(slot).unwrap();

        let target_prod = create_product("Juice", 20);
        let money = vec![Coin::Twenty];

        let result = machine.buy(target_prod, money);
        assert_eq!(result, Err(VendingMachineErrors::NoSuchSlot));
    }
}
