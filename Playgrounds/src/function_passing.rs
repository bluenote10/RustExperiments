// Fn

fn without_side_effect_1<F>(func: F)
    where F: Fn()
{
    func();
}

fn without_side_effect_2<F>(mut func: F)
    where F: Fn()
{
    func();
}

fn without_side_effect_3<F>(func: &F)
    where F: Fn()
{
    func();
}


fn without_side_effect_4<F>(func: &mut F)
    where F: Fn()
{
    func();
}

// FnMut

fn with_side_effect_1<F>(mut func: F)
    where F: FnMut()
{
    func();
}

fn with_side_effect_2<F>(func: &mut F)
    where F: FnMut()
{
    func();
}

// FnOnce

fn with_move_1<F>(func: F)
    where F: FnOnce() -> String
{
    func();
}

fn with_move_2<F>(mut func: F)
    where F: FnOnce() -> String
{
    func();
}

/*
fn with_move_3<F>(func: &F)
    where F: FnOnce() -> String
{
    func();
}
*/

/*
fn with_move_4<F>(func: &mut F)
    where F: FnOnce() -> String
{
    func();
}
*/

fn main() {
    // Fn
    without_side_effect_1(|| println!("func"));
    without_side_effect_2(|| println!("func"));
    without_side_effect_3(& || println!("func"));
    without_side_effect_4(&mut || println!("func"));

    // FnMut
    let mut x = 0;
    with_side_effect_1(|| x += 1);
    with_side_effect_2(&mut || x += 1);
    println!("{}", x);

    // FnOnce
    let s = "".to_string();
    let s_moved = with_move_1(|| s);
    let s = "".to_string();
    let s_moved = with_move_2(|| s);
    //let s = "".to_string();
    //let s_moved = with_move_3(& || s);
    //let s = "".to_string();
    //let s_moved = with_move_4(&mut || s);

}