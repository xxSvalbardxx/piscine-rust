#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    // expected public fields
    pub items: Vec::<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::<(String, f32)>::new(),
            receipt: Vec::<f32>::new(),
         }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for i in s.products.iter() {
            if i.0 == ele {
                self.items.push(i.clone());
            }
        }
        //println!("{:?}", self.items);

    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut temp = Vec::<f32>::new();
        let mut min = 0.0;
        for i in self.items.iter() {
            //println!("{:?}", i);
            //println!("{:?}", i.1);
            if i.1 < min {
                min = i.1;
            }
        }
        for i in self.items.iter() {
            if i.1 != min {
                temp.push(i.1);
            }
        }
        temp.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // remove the first element
        temp.remove(0);
        let mut rest = 0.0;
        for i in temp.iter() {
            self.receipt.push(*i * 0.95);
            rest += *i * 0.05;
        }
        // place the first element back
        temp.insert(0,rest);
        self.receipt.insert(0, rest);
        //self.receipt = temp;
        //println!("{:?}", self.receipt);
        self.receipt.clone()
    }
}

        