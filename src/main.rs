#[allow(unused_braces)]
impl<T, A: Allocator, const CO_ALLOC_PREF: CoAllocPref> Vec<T, A, CO_ALLOC_PREF>
where
    [(); { crate::meta_num_slots!(A, CO_ALLOC_PREF) }]:,
{
    #[stable(feature = "dedup_by", since = "1.16.0")]
    pub fn dedup_by<F>(&mut self, mut same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        let len = self.len();
        if len <= 1 {
            return;
        }

        /* INVARIANT: vec.len() > read >= write > write-1 >= 0 */
        #[allow(unused_braces)]
        struct FillGapOnDrop<'a, T, A: core::alloc::Allocator, const CO_ALLOC_PREF: CoAllocPref>
        where
            [(); { crate::meta_num_slots!(A, CO_ALLOC_PREF) }]:,
        {
            _field_is_needed: (),
        }
    }
}

fn main() {}
