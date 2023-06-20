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
    pub items: Vec<(String, f32)>,
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
        let mut res = Vec::<f32>::new();
        for i in &self.items {
            res.push(i.1);
        }
        res.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let third = res.len() / 3;
        let res_sum: f32 = res.iter().sum(); // somme des elements
        let mut res_sum2: f32 = 0.0; // somme des elements du tiers
        for i in 0..third {
            res_sum2 += res[i]; // somme des elements du tiers
        }
        for i in 0..res.len() {
            res[i] = two_decimals(res[i] - res[i] / (res_sum) * res_sum2) // here is an exemple: 1.23 - 1.23 / 27.55 * 7.65
                .parse::<f32>()
                .unwrap();
        }
        for i in &res {
            self.receipt.push(*i);
        }
        res
    }
}

fn two_decimals(fl: f32) -> String {
    format!("{:.02}", fl)
}
