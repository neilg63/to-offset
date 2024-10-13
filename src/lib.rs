/// Trait to allow all main signed and unsigne integer 
/// where negative values are treated as offsets from the end, defined by length
/// -1 == length -1
/// 0 == start
/// 1 == second position
/// should values fall outside the range
pub trait ToOffset where Self:PartialOrd {
    fn to_offset(self, length: usize) -> usize;
}

  
fn offset_wihin_range(offset: usize, length: usize) -> usize {
if offset > length {
    length
} else {
    offset
}
}

/// Macro to implement the above for signed types
macro_rules! impl_indexable_signed_to_offset {
    ($t:ty) => {
        impl ToOffset for $t {
        fn to_offset(self, length: usize) -> usize {
            let offset = if self < 0 {
                length.saturating_sub(self.abs() as usize)
            } else {
                self as usize
            };
            offset_wihin_range(offset, length)
        }
        }
    };
}

/// Macro to implement the above unsigned types
macro_rules! impl_indexable_unsigned_to_offset {
    ($t:ty) => {
        impl ToOffset for $t {
        fn to_offset(self, length: usize) -> usize {
            offset_wihin_range(self as usize, length)
        }
        }
    };
}

// Implement for i32
impl_indexable_signed_to_offset!(i32);

// Implement for i64
impl_indexable_signed_to_offset!(i64);

// Implement for u8
impl_indexable_unsigned_to_offset!(u8);

// Implement for u16
impl_indexable_unsigned_to_offset!(u16);

// Implement for u32
impl_indexable_unsigned_to_offset!(u32);

// Implement for u64
impl_indexable_unsigned_to_offset!(u64);

// Implement for usize
impl_indexable_unsigned_to_offset!(usize);
 

/// Provide from_offset convenience method that allows negative offsets 
/// from the end of an array/vector and never extends beyond the array/vector bounds
/// It returns Option<&T> in keeping with the default get() method
/// and because the array or vector may still be empty
pub trait FromOffset<T> where T:Sized {
    fn from_offset<U: ToOffset>(&self, relative_index: U) -> Option<&T>;
}

impl<T> FromOffset<T> for [T] {
    fn from_offset<U: ToOffset>(&self, relative_index: U) -> Option<&T> {
        let length = self.len();
        if length < 1 {
            return None;
        }
        let target_index = relative_index.to_offset(length);
        let index = if target_index < length {
            target_index
        } else {
            target_index - 1
        };
        self.get(index)
    }
}

 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_offset() {
        let integer_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        let penultimate_index = (-2).to_offset(integer_array.len());

        assert_eq!(penultimate_index, 10);

        let penultimate_value = integer_array[penultimate_index];

        assert_eq!(penultimate_value, 11);

        let third_from_last = integer_array.from_offset(-3);

        assert_eq!(*third_from_last.unwrap(), 10);
    }

    #[test]
    fn test_array_overflow() {
        let integer_array = [1, 2, 3, 4, 5, 6];

        let distant_element_index = 1_000_000.to_offset(integer_array.len());

        // should be the last element
        assert_eq!(distant_element_index, 6); 

        let twentieth_from_the_end = integer_array.from_offset(-20);

        // will the first [0] element as it cannot extend before the start
        assert_eq!(twentieth_from_the_end.unwrap().to_owned(), 1); 

    }
}