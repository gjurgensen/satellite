// TODO: implement monadic crate?
pub fn fold_option<A, B, F>(iter: impl Iterator<Item = A>, init: B, mut f: F) -> Option<B>
where
    F: FnMut(B, A) -> Option<B>
{
    iter.fold(Some(init), |o, a| o.map(|x| f(x, a)).flatten())
}