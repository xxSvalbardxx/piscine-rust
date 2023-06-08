
pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut result : usize;
    if source.len() > target.len() {
        result = source.len() - target.len();
    } else {
        result = target.len() - source.len();
    }
    if target.len() >= 10 {
        result += source.len()/2;
    }
    let mut index_source : usize = 0;
    let mut index_target : usize = 0;
    for _i in source.chars() {
        let mut finded : bool = false;
        if source.chars().nth(index_source) != target.chars().nth(index_target) {
            for (_,v) in target.chars().enumerate(){
                if let Some(source_char) = source.chars().nth(index_source) {
                    if v == source_char {
                        finded = true;
                    }
                } 
            }
            if !finded {
                index_target +=1;
                result +=1;
                
            }
        }
        index_source += 1;
        index_target += 1;
    }
    result
}