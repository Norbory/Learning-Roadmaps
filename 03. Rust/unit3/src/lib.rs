pub fn showdifference(x: f32,y: f32) {
    let diff = ((x.powi(2)-y.powi(2)).abs()).sqrt();
    println!("La diferencia es {}", diff);
}

pub fn print_array(x:[f32;2]) {
    for (index,number) in x.into_iter().enumerate() {
        println!("Esta es el itemn N° {} cuyo valor es {}",index,number);
    }
}

pub fn ding(x:[i8;7]) {
    for (index,number) in x.into_iter().enumerate() {
        if number==13 {
            println!("Ding, you found it in item N°{}!",index+1);
        }
    }
}

pub fn on_off(x:bool) {
    if x==true {
        println!("Lights are on!");
    }
}

