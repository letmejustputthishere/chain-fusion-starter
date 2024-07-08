use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    storable::Storable,
    DefaultMemoryImpl, StableBTreeMap,
};
use minicbor_derive::{Decode, Encode};
use std::borrow::Cow;
use std::cell::RefCell;

const ASSETS_MEMORY_ID: MemoryId = MemoryId::new(0);

type VMem = VirtualMemory<DefaultMemoryImpl>;

pub type AssetKey = String;
type HeaderField = (String, String);
type Headers = Vec<HeaderField>;
type Bytes = Vec<u8>;

#[derive(Clone, PartialEq, Debug, Encode, Decode)]
pub struct Asset {
    #[n(0)]
    pub headers: Headers,
    #[n(1)]
    pub body: Bytes,
}

impl Storable for Asset {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        minicbor::encode(self, &mut buf).expect("asset encoding should always succeed");
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        minicbor::decode(bytes.as_ref())
            .unwrap_or_else(|e| panic!("failed to decode asset bytes {}: {e}", hex::encode(bytes)))
    }

    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    // Initialize a `StableBTreeMap`
    static ASSETS : RefCell<StableBTreeMap<AssetKey, Asset, VMem>> = MEMORY_MANAGER
        .with(|m|
            RefCell::new(
                StableBTreeMap::init(
                    m.borrow().get(ASSETS_MEMORY_ID)
                )
            )
    );
}

/// Stores the asset in the stable memory.
pub fn store_asset(path: String, asset: Asset) {
    ASSETS.with(|assets| assets.borrow_mut().insert(path, asset));
}

/// Gets an assset from stable memory.
/// Returns `None` if the asset is not found.
/// Returns `Some(asset)` if the asset is found.
pub fn get_asset(path: &String) -> Option<Asset> {
    ASSETS.with(|assets| assets.borrow().get(path))
}
