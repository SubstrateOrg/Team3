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
//         字典类型，k为用户标识，v为kitty的列表（vec表示）
//         示例
//        {
//            "accountId_1": ["catX", "cat(X+1)",...],
//            "accountId_2": ["catY", "cat(Y+1)",...],
//              ...
//        }
        OwnerRelationWithKitties get(owner_relation_with_kitties): map (T::AccountId, u64) => Vec<Kitty>;
        KittiesCount get(kitties_count): u64;
    }
}

decl_module! {
    // 以下代码为思路，不可直接运行
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

//        创建kitty
        fn create_kitty(origin) -> Result {

            let sender = ensure_signed(origin)?;
            let random_hash = String::from("此处应随机生成唯一序列用户kitty的唯一性区分");
            let new_kitty = Kitty {
                id: random_hash,
                dna: random_hash,
                price: <T::Balance as As<u64>>::sa(0),
            };

            let kitties: Vec<Kitty> = OwnerRelationWithKitties::get(sender.clone);

            if kitties {
                // 存在则进行更新
                // 在原有kitty列表中添加新kitty
                kitties.push(new_kitty);
                // 更新原有用户中的kitty列表
                OwnerRelationWithKitties::update(sender.clone, kitties);
            } else {
                // 创建kitty列表
                let firstKitty = vec![new_kitty];
                // 将kitty列表与用户首次绑定
                OwnerRelationWithKitties::insert(sender.clone, firstKitty);
            }

            // 更新kitty总数
            let kitties_count = Self::kitties_count();
            let new_kitties_count = kitties_count.checked_add(1).ok_or("error message")?;
            KittiesCount::put(new_kitties_count);
        }

        // 获取kitty数量
        fn get_kitty_count(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let kitties: Vec<Kitty> = OwnerRelationWithKitties::get(sender.clone);
            let kitties_count = Self::kitties_count();

            print("kitty 总数" + kitties_count)
            print("用户" + sender + "拥有" + kitties.len() + " kitty");

            // 以下用于输出所有用户kitty数量
            for (k, v) in owner_relation_with_kitties.iter() {
                print("用户" + k + "拥有" + item.len() + " kitty");
            }
        }
    }
}