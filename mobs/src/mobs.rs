
pub mod boss;
pub mod member;

pub use boss::*;
pub use member::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}
impl Mob {
    pub fn recruit(&mut self, name: &str, age: u8) {
        self.members.push(Member::new(name ,Role::Associate , age))
    }

    pub fn attack(&mut self,mut mob: Mob) {
        let mut atk_score: u32 = 0;
        let mut def_score: u32 = 0;

        for member in &mut self.members {
            match member.role {
                Role::Underboss => atk_score += 4,
                Role::Caporegime => atk_score += 3,
                Role::Soldier => atk_score += 2,
                Role::Associate => atk_score += 1,
            }
        }
        for member in &mut mob.members {
            match member.role {
                Role::Underboss => def_score += 4,
                Role::Caporegime => def_score += 3,
                Role::Soldier => def_score += 2,
                Role::Associate => def_score += 1,
            }
        }
        if atk_score > def_score {
            mob.members.pop();
            if mob.members.len() == 0 {
                self.wealth += mob.wealth;
                mob.wealth = 0;
                self.cities.append(&mut mob.cities)
            }
        } else {
            self.members.pop();
            if self.members.len() == 0 {
                mob.wealth += self.wealth;
                self.wealth = 0;
                mob.cities.append(&mut self.cities)
            }
        } 
    }

    pub fn steal(&mut self, mob:&mut Mob, mut value: u32) {
        if mob.wealth <= value {
            value = mob.wealth;
        }
        mob.wealth -= value;
        self.wealth += value;
    }

    pub fn conquer_city(&mut self, mobs: Vec<Mob>, city_name: String, value: u8) {
        let mut is_taked = false;
        for mob in mobs {
            for city in mob.cities {
                if city.0 == city_name {
                    is_taked = true;
                }
            }
        }
        if is_taked == false {
            self.cities.push((city_name , value ));
        }
    }
}
