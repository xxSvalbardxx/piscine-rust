
mod mobs;
pub use mobs::*;
pub use mobs::member::*;
pub use mobs::boss::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    name: String,
    boss: Boss,
    members: Vec<Member>,
    cities: Vec<(String, u8)>,
    wealth: u32,
}

impl Mob {
    pub fn recruit(&mut self, name: String, age: u32) {
        self.members.push(Member::new(name, age));
    }

    pub fn attack(&mut self,mut mob: Mob) {
        let atk_score: u32;
        let def_score: u32;

        for member in self.members.iter_mut() {
            if member.role == Role::Underboss {
                atk_score += 4;
            } else if member.role == Role::Caporegime {
                atk_score += 3;
            } else if member.role == Role::Soldier {
                atk_score += 2;
            } else if member.role == Role::Associate {
                atk_score += 1;
            }
        }
        for member in mob.members.iter_mut() {
            if member.role == Role::Underboss {
                def_score += 4;
            } else if member.role == Role::Caporegime {
                def_score += 3;
            } else if member.role == Role::Soldier {
                def_score += 2;
            } else if member.role == Role::Associate {
                def_score += 1;
            }
        }
        if atk_score > def_score {
            mob.members.pop();
            if mob.members.len() == 0 {
                self.wealth += mob.wealth;
                for city in mob.cities.iter_mut() {
                    self.cities.push(city);
                }
                mob.wealth = 0;
                mob.cities = Vec::new();
            }
        } else {
            self.members.pop();
            if self.members.len() == 0 {
                mob.wealth += self.wealth;
                for city in self.cities.iter_mut() {
                    mob.cities.push(city);
                }
                self.wealth = 0;
                self.cities = Vec::new();
            }
        } 
    }

    pub fn steal(&mut self,mut mob: Mob, mut value: u32) {
        if mob.wealth <= value {
            value = mob.wealth;
        }
        mob.wealth -= value;
        self.wealth += value;
    }

    pub fn conquer_city(&mut self, mut mobs: Vec<Mob>, city: String, value: u8) {
        for mob in mobs.iter_mut() {
            if mob.cities.contains(&city, &value) {
                mob.cities.remove(city , value);
                self.cities.push(city , value);
                
            }
        }
    }
}
