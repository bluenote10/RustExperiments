use std::io::Result;
use std::io::Write;

///
/// General trait to make a type serializable.
///
/// In contrast to serde this supports context aware serialization by default.
/// The generic type `C` is in general user provided and is irrelevant for
/// fundamental types. Container like types forward the context into the serialization
/// of its elements. This allows to a user defined parametrization of the serialization.
///
/// As an alternative, I considered using the work-around mentioned here:
/// - https://users.rust-lang.org/t/serde-question-access-to-a-shared-context-data-within-serialize-and-deserialize/39546
/// - https://users.rust-lang.org/t/how-to-implement-display-that-requires-dynamic-context/8268/2
///
/// But this work-around has a few flaws that I couldn't resolve easily:
/// - Forwarding the context in containers doesn't work out-of-the box. In particular if a user defined type
///   T is only serializable for `(T, Context)` it becomes impossible to serialize e.g. a Vec<T> directly,
///   because serde will requires `T` to be serializable on its own. It looks like this would require to
///   duplicate the implementation of the sequence serialization for `[(T, Context)]`, but I was running into
///   lifetime issues trying to do so.
/// - In general the syntax to forward the context is more clumsy. First one needs to split `self` into
///   `(this, context)` and then there are lots of `(this.field, context).serialize_into()` calls. Since
///   passing around the context isn't done automatically everywhere, it fragments the types into those
///   some are directly serializable and others that are only serializable via (T, context). The "syntactical
///   hurdle" to switch from one to the other is higher then it needs to be.
///
/// A possible downside of this approach is that Rust only allows to implement a trait
/// for one template specialization:
///
/// https://stackoverflow.com/questions/24771274/how-do-you-implement-specific-types-on-generic-traits-in-rust
///
pub trait Serialize<C> {
    fn serialize_into<W>(&self, wr: &mut W, context: &C) -> Result<()>
    where
        W: Write;
}

#[allow(dead_code)]
pub fn serialize_into_vec<T>(val: &T) -> Result<Vec<u8>>
where
    T: Serialize<()> + ?Sized,
{
    let mut wr = Vec::with_capacity(128);
    let context = ();
    val.serialize_into(&mut wr, &context)?;
    Ok(wr)
}
