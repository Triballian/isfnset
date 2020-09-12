//use std::vec::Vec;
// use isfn::{
//     Fnel,
//     getdupdoms,

// };
mod isfn;


fn main() {
    let fnelone: isfn::isfno::Fnel = isfn::isfno::Fnel::new(1 , 2);
    let fneltwo: isfn::isfno::Fnel = isfn::isfno::Fnel::new(6, 4);
    let fnelthree: isfn::isfno::Fnel = isfn::isfno::Fnel::new(1, 4);
    let mut fnset = vec![];


    fnset.push(fnelone);
    fnset.push(fneltwo);
    fnset.push(fnelthree);


    let dupdoms = isfn::isfno::getdupdoms(&fnset);

    println!("Fnset: {:?}", fnset[1].domain);
    println!("dupdoms: {:?}", dupdoms)
    //if !getdupdoms.is_empty() {
    //    println!("not a fuction")
    //}
}
