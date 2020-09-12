
#[derive(Debug)]
pub struct Fnel {
    pub domain: i32,
    pub field: i32
}

impl Fnel {
    pub fn new (domain: i32, field: i32 ) -> Self {
        Self{domain, field}
    }
}

// is domain unique
// isduiq(fnset: vec) -> bool {
//     for 
// }

pub fn getdupdoms(fnset: &Vec<Fnel>) -> Vec<i32> {
    //all domains
    //duplicate domains sets
    let mut adoms = vec![];
    let mut dupdoms = vec![];
   
    for set in fnset {
        if !adoms.is_empty() {
            for a in adoms.to_owned() {
                
                if a == set.domain {
                    &adoms.push(set.domain);
                    dupdoms.push(set.domain);
                }
            } 
        }
        adoms.push(set.domain);


    }
    dupdoms

}
