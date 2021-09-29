use crate::{
    traits::{
        SpreadLayout,
        PackedLayout,
        KeyPtr,
        ExtKeyPtr,
    },
};
use ink_env::hash::{
    Blake2x256,
    CryptoHash,
    HashOutput,
};
use ink_primitives::Key;
use core::{
    marker::PhantomData,
};
use crate::traits::{
    pull_packed_root_opt,
    pull_packed_root,
    push_packed_root,
};

/// Implementation of simple hash map that has to work with storage in key-value form.
#[derive(Debug)]
pub struct SimpleHashMap<K, V, H = Blake2x256>
    where
        K: scale::Encode,
        V: PackedLayout,
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    /// The offset key for the storage mapping.
    ///
    /// This offsets the mapping for the entries stored in the contract storage
    /// so that all lazy hash map instances store equal entries at different
    /// locations of the contract storage and avoid collisions.
    key: Option<Key>,
    /// The number of items stored in the map.
    len: u32,

    key_phantom: PhantomData<K>,
    value_phantom: PhantomData<V>,

    /// The used hash builder.
    hash_builder: PhantomData<H>,
}

const PREFIX: [u8; 17] = [
    b'i', b'n', b'k', b' ', b's', b'i', b'm', b'p', b'l', b'e', b'h', b'a', b's', b'h', b'm', b'a', b'p',
];

impl<K, V, H> SimpleHashMap<K, V, H>
    where
        K: scale::Encode,
        V: PackedLayout,
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    /// Return count of elements inserted into storage.
    pub fn len(&self) -> u32 {
        self.len.clone()
    }
}

impl<K, V, H> SimpleHashMap<K, V, H>
    where
        K: scale::Encode,
        V: PackedLayout,
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    /// Creates a new empty storage hash map.
    pub fn new() -> Self {
        Self {
            key: None,
            len: 0,
            key_phantom: Default::default(),
            value_phantom: Default::default(),
            hash_builder: Default::default()
        }
    }

    /// Returns the value corresponding to the key.
    pub fn get(&self, key: &K) -> Option<V>
    {
        self
            .storage_key(key)
            .map(|key| pull_packed_root_opt::<V>(&key))
            .unwrap_or(None)
    }

    /// Inserts the value corresponding to the key.
    pub fn insert(&mut self, key: &K, value: V)
    {
        if self.get(key).is_none() {
            self.len += 1;
        }
        self
            .storage_key(key)
            .map(|key| push_packed_root::<V>(&value, &key));
    }

    /// Erases the value corresponding to the key.
    pub fn erase(&mut self, key: &K)
    {
        if self.get(key).is_some() {
            self.len -= 1;
            self
                .storage_key(key)
                .map(|key| ink_env::clear_contract_storage(&key));
        }
    }

    /// Takes the value corresponding to the key from the storage and returns it.
    pub fn take(&mut self, key: &K) -> Option<V>
    {
        let mut value = None;
        if let Some(storage_key) = self.storage_key(key) {
            value = pull_packed_root_opt::<V>(&storage_key);
            if value.is_some() {
                self.len -= 1;
            }
            ink_env::clear_contract_storage(&storage_key);
        }
        value
    }

    /// Returns a storage key for the given key.
    pub fn storage_key(&self, key: &K) -> Option<Key>
        where
            K: scale::Encode,
    {
        if self.key.is_none() {
            return None
        }
        let mut output = <H as HashOutput>::Type::default();
        ink_env::hash_encoded::<H, K>(key, &mut output);
        storage_key_common::<H>(&self.key.unwrap(), &output)
    }
}

fn storage_key_common<H>(key: &Key, value_hash: &<H as HashOutput>::Type) -> Option<Key>
    where
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    let key_pair = KeyPair {
        storage_key: &key,
        value_hash,
    };

    let mut output = <H as HashOutput>::Type::default();
    ink_env::hash_encoded::<H, KeyPair<H>>(&key_pair, &mut output);
    Some(output.into())
}

struct KeyPair<'a, H: CryptoHash> {
    storage_key: &'a Key,
    value_hash: &'a <H as HashOutput>::Type,
}

impl<'a, H: CryptoHash> scale::Encode for KeyPair<'a, H> {
    fn size_hint(&self) -> usize {
        PREFIX.size_hint() + self.storage_key.size_hint() + self.value_hash.size_hint()
    }

    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        PREFIX.encode_to(dest);
        self.storage_key.encode_to(dest);
        self.value_hash.encode_to(dest);
    }
}

impl<K, V, H> SpreadLayout for SimpleHashMap<K, V, H>
    where
        K: scale::Encode,
        V: PackedLayout,
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    const FOOTPRINT: u64 = 1;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        let mut default = Self::new();
        let key = *ExtKeyPtr::next_for::<Self>(ptr);
        default.len = pull_packed_root::<u32>(&key);
        default.key = Some(key);
        default
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        <u32 as SpreadLayout>::push_spread(&self.len, ptr);
    }

    #[inline]
    fn clear_spread(&self, _ptr: &mut KeyPtr) {}
}

/// SimpleHashMap is always packed layout
impl<K, V, H> PackedLayout for SimpleHashMap<K, V, H>
    where
        K: scale::Encode,
        V: PackedLayout,
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    fn pull_packed(&mut self, _at: &Key) {}

    fn push_packed(&self, _at: &Key) {}

    fn clear_packed(&self, _at: &Key) {}
}

impl<K, V, H> scale::Encode for SimpleHashMap<K, V, H>
    where
        K: scale::Encode,
        V: PackedLayout,
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    #[inline]
    fn size_hint(&self) -> usize {
        self.len.size_hint()
    }

    #[inline]
    fn encode_to<T: scale::Output + ?Sized>(&self, dest: &mut T) {
        self.len.encode_to(dest);
    }
}

impl<K, V, H> scale::Decode for SimpleHashMap<K, V, H>
    where
        K: scale::Encode,
        V: PackedLayout,
        H: CryptoHash,
        Key: From<<H as HashOutput>::Type>,
{
    fn decode<I: scale::Input>(input: &mut I) -> Result<Self, scale::Error> {
        let mut default = Self::new();
        default.len = <u32 as scale::Decode>::decode(input)?;
        Ok(default)
    }
}

#[cfg(feature = "std")]
const _: () = {
    use crate::traits::{
        StorageLayout,
    };
    use ink_metadata::layout::{
        CellLayout,
        Layout,
        LayoutKey,
    };

    impl<K, V, H> StorageLayout for SimpleHashMap<K, V, H>
        where
            K: scale::Encode,
            V: PackedLayout,
            H: CryptoHash,
            Key: From<<H as HashOutput>::Type>,
    {
        fn layout(key_ptr: &mut KeyPtr) -> Layout {
            Layout::Cell(CellLayout::new::<u32>(LayoutKey::from(
                key_ptr.advance_by(1),
            )))
        }
    }
};