pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut v = Vec::new();
    for i in &names{
        let mut s = String::new();
        //v.push(String::from(*i));
        for j in i.chars(){
            if j.is_uppercase(){
                s.push_str(&j.to_string());
                s.push_str(". ");
            }else {
                continue
            }
        }
        s.pop();
        v.push(s.to_string());
    }
    v
}
