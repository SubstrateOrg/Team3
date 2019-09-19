use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result, ensure, decl_event};
use system::ensure_signed;
use parity_codec::{Encode, Decode};

/*
    一.设计加密码猫模块
       1. 数据结构
       2. 存储定义
       3. 可调用函数
       4. 算法伪代码
    二. 需求
       1. 链上存储加密猫数据
       2. 遍历所有加密猫
       3. 每只猫都有自己的dna，为128bit的数据
       4. 设计如何生成dna (伪代码算法）
       5. 每个用户可以拥有零到多只猫
       6. 每只猫只有一个主人
       7. 遍历用户拥有的所有猫
*/

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash, // 区分每只不同的kitty
    price: Balance, // 将猫进行买卖的时候的价格信息
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where <T as system::Trait>::AccountId, <T as system::Trait>::Hash {
        Created(AccountId, Hash),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // kitty 实例
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        // 拥有kitty的用户
        KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;
        // 多只 kitty
        KittiesArray get(kitty_by_index): map u64 => T::Hash;
        // 猫的数量
        KittiesCount get(all_kitties_count): u64;
        // 绑定拥有者与 kitty 列表
        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        // 绑定拥有者与 kitty 数量
        OwnedKittiesCount get(owned_kitty_count): map T::AccountId => u64;

    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_kitty(origin) -> Result {
//            创建dna 并判断是否存在（防止重复）
//              dna = new dna code
//              while result = judge_exist(dna) until result = False
//            创建新的 kitty 并设置dna、价格（默认给定值），然后存储
//              new_kitty = Kitty{id, dna, price}
//              add new_kitty to Kitties
//            将新 kitty 与拥有者进行绑定，并更新拥有者 kitty 的数量信息
//              update count and return OK
        }
    }
}