struct  Clolor(u8,u8,u8);
fn main() {
    let white=Clolor(255,255,255);
    let red=white.0;
    let green=white.1;
    let blue=white.2;
    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}", blue);
    
    let orange= Clolor(255,165,0);
    //直接解构字段
    let Clolor(r,g,b)=orange;
    println!("R:{},G:{},B:{},(orange)",r,g,b);
    //解构时忽略字段
    let Clolor(r,_,b)=orange; 
    
}
