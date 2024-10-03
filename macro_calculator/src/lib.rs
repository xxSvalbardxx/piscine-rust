#[derive(Debug)]
pub struct Food {
    //expected public fields
    pub name: String,
    pub calories: [String;2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut macro_json = json::JsonValue::new_object();
    let mut calories = 0.0;
    let mut fats = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;

    for food in foods.iter() {
        calories += food.calories[1].replace("kcal", "").parse::<f64>().unwrap() * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
    }
    calories = (calories * 100.0).round() / 100.0; 
    fats = (fats * 100.0).round() / 100.0;
    carbs = (carbs * 100.0).round() / 100.0;
    proteins = (proteins * 100.0).round() / 100.0;
    
    macro_json["cals"] = json::JsonValue::Number(calories.into());
    macro_json["carbs"] = json::JsonValue::Number(carbs.into());
    macro_json["proteins"] = json::JsonValue::Number(proteins.into());
    macro_json["fats"] = json::JsonValue::Number(fats.into());

    macro_json
}
