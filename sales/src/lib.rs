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
        // the promotion is applied to the cheapest items in three. so if we have 3 items, we remove the first one. but if we have 9 items, we remove the first three.
        let to_remove = temp.len() / 3;
        // remove the first element
        temp.remove(to_remove - 1);
        let mut rest = 0.0;
        for i in temp.iter() {
            // round to two decimal places and push to receipt

            self.receipt.push(((*i * 0.9551)*100.0).round() / 100.0);
            rest += (*i * (1.0 - 0.9552)*100.0).round() / 100.0 ;
        }
        // 22.06 % 23.1 * 100 = 95.51
        // 22.06 % 23.1 = 0.9549180327868852
        // 2.98 % 3.12 * 100 = 95.51
        // 2.98 % 3.12 = 0.9549180327868852
        // 

        // place the first element back
        temp.insert(0,rest);
        self.receipt.insert(0, rest);
        //self.receipt = temp;
        //println!("{:?}", self.receipt);
        self.receipt.clone()
    }
}

        