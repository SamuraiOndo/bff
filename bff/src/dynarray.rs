use std::fmt::Debug;
use std::marker::PhantomData;

use binrw::{binrw, BinRead, BinWrite};
use derive_more::Deref;
use serde::{Deserialize, Serialize};

#[binrw]
#[derive(Debug, Serialize, Deref, Deserialize)]
#[serde(transparent)]
#[br(import_raw(inner: <InnerType as BinRead>::Args<'_>))]
pub struct DynArray<InnerType: BinRead + BinWrite + 'static, SizeType: BinRead + BinWrite = u32>
where
    for<'a> <InnerType as BinRead>::Args<'a>: Clone + Default,
    for<'a> <SizeType as BinRead>::Args<'a>: Default,
    SizeType: TryInto<usize>,
    <SizeType as TryInto<usize>>::Error: Debug,

    // BinWrite derive doesnt support generics well because it assumes the Args type is always the unit type. We can work around this by forcing the Args type to be the unit type.
    for<'a> InnerType: BinWrite<Args<'a> = ()>,
    for<'a> SizeType: BinWrite<Args<'a> = ()>,
    usize: TryInto<SizeType>,
    <usize as TryInto<SizeType>>::Error: Debug,
{
    #[br(temp)]
    #[bw(calc = inner.len().try_into().unwrap())]
    count: SizeType,
    #[deref]
    #[br(args { count: count.try_into().unwrap(), inner })]
    pub inner: Vec<InnerType>,
    #[serde(skip)]
    _phantom: PhantomData<SizeType>,
}
