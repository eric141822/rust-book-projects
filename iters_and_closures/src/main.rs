use iters_and_closures::*;

fn test_store() {
    let store = Inventory::new();

    let user_pref1 = Some(ShirtColor::Red);
    let user_pref2: Option<ShirtColor> = None;

    let give_away1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} will get a {:?} shirt",
        user_pref1, give_away1
    );

    let give_away2 = store.giveaway(user_pref2);
    println!(
        "The user with no preference will get a {:?} shirt",
        give_away2
    );
}


fn main() {
    println!("Test store:");
    test_store();

    // this will fail to compile if uncommented
    // on the first call of example_closure, the compiler infers the type of x to be String
    // let example_closure = |x| x;
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
    
    println!("---------------------");
    println!("Closure borrow:");
    closure_borrow();

    println!("---------------------");
    println!("Closure mutable:");
    closure_mutable();

    println!("---------------------");
    println!("Closure thread move:");
    closure_thread_move();


}
