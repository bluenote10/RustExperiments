use dashu_ratio::Relaxed;
use fraction::{DynaFraction, Fraction};
use num_rational::Ratio;

fn demo_fraction() {
    // https://github.com/dnsl48/fraction
    // Note that this fails to work for e.g. 1e20 and 1e-20, probably
    // because the initial fraction cannot really be constructed.
    let a = DynaFraction::<u64>::from_fraction(Fraction::from(1e10));
    let b = DynaFraction::<u64>::from_fraction(Fraction::from(1e-10));
    let c = a + b;
    println!("{} {:#.20}", c, c);
}

fn demo_num_rational() {
    // https://docs.rs/num-rational/latest/num_rational/struct.Ratio.html
    let a = Ratio::from_float(1e10).unwrap();
    let b = Ratio::from_float(1e-10).unwrap();
    let c = a + b;
    println!("{}", c);
}

fn demo_dashu() {
    // https://docs.rs/dashu-ratio/latest/dashu_ratio/
    let a = Relaxed::try_from(1e10).unwrap();
    let b = Relaxed::try_from(1e-10).unwrap();
    let c = a + b;
    println!("{}", c);
}

fn main() {
    demo_fraction();
    demo_num_rational();
    demo_dashu();
}
