fn main() {
    let mut v = vec![1, 2, 3, 4, 5] ;
    let r1 = &v ; // несколько shared ок
    let r2 = &v ; // несколько shared ок

    println!("r1: {r1:?}, r2: {r2:?}") ;    // Out: r1: [1, 2, 3, 4, 5], r2: [1, 2, 3, 4, 5]

    // r1 и r2 уже не используются благодаря NLL (Non Lexical Lifetime)
    let r3 = &mut v ;
    r3.push(6); 
    println!("r3: {r3:?}") ;    // Out: r3: [1, 2, 3, 4, 5, 6]
}
