// ignore-tidy-linelength

//@ is "$.index[*][?(@.name=='hello')].inner.function.sig.output.impl_trait[1].use[0].Lifetime" \"\'a\"
//@ is "$.index[*][?(@.name=='hello')].inner.function.sig.output.impl_trait[1].use[1].Param" \"T\"
//@ is "$.index[*][?(@.name=='hello')].inner.function.sig.output.impl_trait[1].use[2].Param" \"N\"
pub fn hello<'a, T, const N: usize>() -> impl Sized + use<'a, T, N> {}
