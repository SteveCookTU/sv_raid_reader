use crate::raid_fixed_reward_item_generated::RaidRewardItemSubjectType;
use crate::{raid_fixed_reward_item_generated, raid_lottery_reward_item_generated};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct ItemTable(pub HashMap<u64, Vec<Item>>);

const FIXED_RAW: &[u8] = include_bytes!("../resources/raid_fixed_reward_item.bin");
const LOTTERY_RAW: &[u8] = include_bytes!("../resources/raid_lottery_reward_item.bin");

lazy_static! {
    pub static ref FIXED_ITEMS: ItemTable =
        raid_fixed_reward_item_generated::root_as_raid_fixed_reward_item_array(FIXED_RAW)
            .unwrap()
            .into();
    pub static ref LOTTERY_ITEMS: ItemTable =
        raid_lottery_reward_item_generated::root_as_raid_lottery_reward_item_array(LOTTERY_RAW)
            .unwrap()
            .into();
}

#[derive(Copy, Clone, Default)]
pub struct Item {
    pub id: u16,
    pub amount: u8,
    pub probability: f64,
    pub subject: ItemSubject,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum ItemSubject {
    #[default]
    All,
    Host,
    Guest,
    Once,
}

impl From<crate::raid_fixed_reward_item_generated::RaidRewardItemSubjectType> for ItemSubject {
    fn from(value: RaidRewardItemSubjectType) -> Self {
        match value.0 {
            1 => ItemSubject::Host,
            2 => ItemSubject::Guest,
            3 => ItemSubject::Once,
            _ => ItemSubject::All,
        }
    }
}

impl From<crate::raid_fixed_reward_item_generated::RaidFixedRewardItemArray<'_>> for ItemTable {
    fn from(items: crate::raid_fixed_reward_item_generated::RaidFixedRewardItemArray<'_>) -> Self {
        let mut table = HashMap::new();
        for value in items.values().iter() {
            let hash = value.table_name();
            let mut items = Vec::new();
            if value.reward_item_00().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_00().itemID().0 as u16,
                    amount: value.reward_item_00().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_00().subject_type().into(),
                });
            } else if value.reward_item_00().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_00().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_00().subject_type().into(),
                });
            } else if value.reward_item_00().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_00().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_00().subject_type().into(),
                });
            }
            if value.reward_item_01().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_01().itemID().0 as u16,
                    amount: value.reward_item_01().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_01().subject_type().into(),
                });
            } else if value.reward_item_01().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_01().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_01().subject_type().into(),
                });
            } else if value.reward_item_01().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_01().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_01().subject_type().into(),
                });
            }
            if value.reward_item_02().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_02().itemID().0 as u16,
                    amount: value.reward_item_02().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_02().subject_type().into(),
                });
            } else if value.reward_item_02().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_02().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_02().subject_type().into(),
                });
            } else if value.reward_item_02().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_02().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_02().subject_type().into(),
                });
            }
            if value.reward_item_03().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_03().itemID().0 as u16,
                    amount: value.reward_item_03().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_03().subject_type().into(),
                });
            } else if value.reward_item_03().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_03().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_03().subject_type().into(),
                });
            } else if value.reward_item_03().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_03().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_03().subject_type().into(),
                });
            }
            if value.reward_item_04().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_04().itemID().0 as u16,
                    amount: value.reward_item_04().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_04().subject_type().into(),
                });
            } else if value.reward_item_04().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_04().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_04().subject_type().into(),
                });
            } else if value.reward_item_04().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_04().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_04().subject_type().into(),
                });
            }
            if value.reward_item_05().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_05().itemID().0 as u16,
                    amount: value.reward_item_05().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_05().subject_type().into(),
                });
            } else if value.reward_item_05().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_05().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_05().subject_type().into(),
                });
            } else if value.reward_item_05().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_05().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_05().subject_type().into(),
                });
            }
            if value.reward_item_06().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_06().itemID().0 as u16,
                    amount: value.reward_item_06().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_06().subject_type().into(),
                });
            } else if value.reward_item_06().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_06().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_06().subject_type().into(),
                });
            } else if value.reward_item_06().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_06().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_06().subject_type().into(),
                });
            }
            if value.reward_item_07().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_07().itemID().0 as u16,
                    amount: value.reward_item_07().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_07().subject_type().into(),
                });
            } else if value.reward_item_07().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_07().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_07().subject_type().into(),
                });
            } else if value.reward_item_07().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_07().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_07().subject_type().into(),
                });
            }
            if value.reward_item_08().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_08().itemID().0 as u16,
                    amount: value.reward_item_08().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_08().subject_type().into(),
                });
            } else if value.reward_item_08().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_08().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_08().subject_type().into(),
                });
            } else if value.reward_item_08().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_08().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_08().subject_type().into(),
                });
            }
            if value.reward_item_09().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_09().itemID().0 as u16,
                    amount: value.reward_item_09().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_09().subject_type().into(),
                });
            } else if value.reward_item_09().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_09().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_09().subject_type().into(),
                });
            } else if value.reward_item_09().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_09().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_09().subject_type().into(),
                });
            }
            if value.reward_item_10().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_10().itemID().0 as u16,
                    amount: value.reward_item_10().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_10().subject_type().into(),
                });
            } else if value.reward_item_10().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_10().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_10().subject_type().into(),
                });
            } else if value.reward_item_10().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_10().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_10().subject_type().into(),
                });
            }
            if value.reward_item_11().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_11().itemID().0 as u16,
                    amount: value.reward_item_11().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_11().subject_type().into(),
                });
            } else if value.reward_item_11().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_11().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_11().subject_type().into(),
                });
            } else if value.reward_item_11().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_11().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_11().subject_type().into(),
                });
            }
            if value.reward_item_12().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_12().itemID().0 as u16,
                    amount: value.reward_item_12().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_12().subject_type().into(),
                });
            } else if value.reward_item_12().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_12().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_12().subject_type().into(),
                });
            } else if value.reward_item_12().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_12().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_12().subject_type().into(),
                });
            }
            if value.reward_item_13().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_13().itemID().0 as u16,
                    amount: value.reward_item_13().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_13().subject_type().into(),
                });
            } else if value.reward_item_13().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_13().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_13().subject_type().into(),
                });
            } else if value.reward_item_13().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_13().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_13().subject_type().into(),
                });
            }
            if value.reward_item_14().itemID()
                != crate::raid_fixed_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_14().itemID().0 as u16,
                    amount: value.reward_item_14().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_14().subject_type().into(),
                });
            } else if value.reward_item_14().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_14().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_14().subject_type().into(),
                });
            } else if value.reward_item_14().category()
                == crate::raid_fixed_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_14().num() as u8,
                    probability: 100.0,
                    subject: value.reward_item_14().subject_type().into(),
                });
            }
            table.insert(hash, items);
        }
        Self(table)
    }
}

impl From<crate::raid_lottery_reward_item_generated::RaidLotteryRewardItemArray<'_>> for ItemTable {
    fn from(
        items: crate::raid_lottery_reward_item_generated::RaidLotteryRewardItemArray<'_>,
    ) -> Self {
        let mut table = HashMap::new();
        for value in items.values().iter() {
            let hash = value.table_name();
            let mut items = Vec::new();
            let mut rate = 0;
            rate += value.reward_item_00().rate() as u32;
            rate += value.reward_item_01().rate() as u32;
            rate += value.reward_item_02().rate() as u32;
            rate += value.reward_item_03().rate() as u32;
            rate += value.reward_item_04().rate() as u32;
            rate += value.reward_item_05().rate() as u32;
            rate += value.reward_item_06().rate() as u32;
            rate += value.reward_item_07().rate() as u32;
            rate += value.reward_item_08().rate() as u32;
            rate += value.reward_item_09().rate() as u32;
            rate += value.reward_item_10().rate() as u32;
            rate += value.reward_item_11().rate() as u32;
            rate += value.reward_item_12().rate() as u32;
            rate += value.reward_item_13().rate() as u32;
            rate += value.reward_item_14().rate() as u32;
            rate += value.reward_item_15().rate() as u32;
            rate += value.reward_item_16().rate() as u32;
            rate += value.reward_item_17().rate() as u32;
            rate += value.reward_item_18().rate() as u32;
            rate += value.reward_item_19().rate() as u32;
            rate += value.reward_item_20().rate() as u32;
            rate += value.reward_item_21().rate() as u32;
            rate += value.reward_item_22().rate() as u32;
            rate += value.reward_item_23().rate() as u32;
            rate += value.reward_item_24().rate() as u32;
            rate += value.reward_item_25().rate() as u32;
            rate += value.reward_item_26().rate() as u32;
            rate += value.reward_item_27().rate() as u32;
            rate += value.reward_item_28().rate() as u32;
            rate += value.reward_item_29().rate() as u32;
            let rate = f64::from(rate);
            if value.reward_item_00().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_00().itemID().0 as u16,
                    amount: value.reward_item_00().num() as u8,
                    probability: f64::from(value.reward_item_00().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_00().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_00().num() as u8,
                    probability: f64::from(value.reward_item_00().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_00().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_00().num() as u8,
                    probability: f64::from(value.reward_item_00().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_01().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_01().itemID().0 as u16,
                    amount: value.reward_item_01().num() as u8,
                    probability: f64::from(value.reward_item_01().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_01().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_01().num() as u8,
                    probability: f64::from(value.reward_item_01().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_01().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_01().num() as u8,
                    probability: f64::from(value.reward_item_01().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_02().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_02().itemID().0 as u16,
                    amount: value.reward_item_02().num() as u8,
                    probability: f64::from(value.reward_item_02().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_02().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_02().num() as u8,
                    probability: f64::from(value.reward_item_02().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_02().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_02().num() as u8,
                    probability: f64::from(value.reward_item_02().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_03().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_03().itemID().0 as u16,
                    amount: value.reward_item_03().num() as u8,
                    probability: f64::from(value.reward_item_03().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_03().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_03().num() as u8,
                    probability: f64::from(value.reward_item_03().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_03().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_03().num() as u8,
                    probability: f64::from(value.reward_item_03().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_04().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_04().itemID().0 as u16,
                    amount: value.reward_item_04().num() as u8,
                    probability: f64::from(value.reward_item_04().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_04().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_04().num() as u8,
                    probability: f64::from(value.reward_item_04().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_04().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_04().num() as u8,
                    probability: f64::from(value.reward_item_04().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_05().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_05().itemID().0 as u16,
                    amount: value.reward_item_05().num() as u8,
                    probability: f64::from(value.reward_item_05().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_05().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_05().num() as u8,
                    probability: f64::from(value.reward_item_05().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_05().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_05().num() as u8,
                    probability: f64::from(value.reward_item_05().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_06().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_06().itemID().0 as u16,
                    amount: value.reward_item_06().num() as u8,
                    probability: f64::from(value.reward_item_06().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_06().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_06().num() as u8,
                    probability: f64::from(value.reward_item_06().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_06().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_06().num() as u8,
                    probability: f64::from(value.reward_item_06().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_07().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_07().itemID().0 as u16,
                    amount: value.reward_item_07().num() as u8,
                    probability: f64::from(value.reward_item_07().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_07().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_07().num() as u8,
                    probability: f64::from(value.reward_item_07().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_07().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_07().num() as u8,
                    probability: f64::from(value.reward_item_07().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_08().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_08().itemID().0 as u16,
                    amount: value.reward_item_08().num() as u8,
                    probability: f64::from(value.reward_item_08().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_08().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_08().num() as u8,
                    probability: f64::from(value.reward_item_08().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_08().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_08().num() as u8,
                    probability: f64::from(value.reward_item_08().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_09().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_09().itemID().0 as u16,
                    amount: value.reward_item_09().num() as u8,
                    probability: f64::from(value.reward_item_09().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_09().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_09().num() as u8,
                    probability: f64::from(value.reward_item_09().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_09().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_09().num() as u8,
                    probability: f64::from(value.reward_item_09().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_10().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_10().itemID().0 as u16,
                    amount: value.reward_item_10().num() as u8,
                    probability: f64::from(value.reward_item_10().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_10().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_10().num() as u8,
                    probability: f64::from(value.reward_item_10().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_10().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_10().num() as u8,
                    probability: f64::from(value.reward_item_10().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_11().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_11().itemID().0 as u16,
                    amount: value.reward_item_11().num() as u8,
                    probability: f64::from(value.reward_item_11().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_11().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_11().num() as u8,
                    probability: f64::from(value.reward_item_11().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_11().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_11().num() as u8,
                    probability: f64::from(value.reward_item_11().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_12().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_12().itemID().0 as u16,
                    amount: value.reward_item_12().num() as u8,
                    probability: f64::from(value.reward_item_12().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_12().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_12().num() as u8,
                    probability: f64::from(value.reward_item_12().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_12().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_12().num() as u8,
                    probability: f64::from(value.reward_item_12().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_13().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_13().itemID().0 as u16,
                    amount: value.reward_item_13().num() as u8,
                    probability: f64::from(value.reward_item_13().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_13().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_13().num() as u8,
                    probability: f64::from(value.reward_item_13().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_13().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_13().num() as u8,
                    probability: f64::from(value.reward_item_13().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_14().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_14().itemID().0 as u16,
                    amount: value.reward_item_14().num() as u8,
                    probability: f64::from(value.reward_item_14().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_14().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_14().num() as u8,
                    probability: f64::from(value.reward_item_14().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_14().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_14().num() as u8,
                    probability: f64::from(value.reward_item_14().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_15().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_15().itemID().0 as u16,
                    amount: value.reward_item_15().num() as u8,
                    probability: f64::from(value.reward_item_15().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_15().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_15().num() as u8,
                    probability: f64::from(value.reward_item_15().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_15().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_15().num() as u8,
                    probability: f64::from(value.reward_item_15().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_16().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_16().itemID().0 as u16,
                    amount: value.reward_item_16().num() as u8,
                    probability: f64::from(value.reward_item_16().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_16().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_16().num() as u8,
                    probability: f64::from(value.reward_item_16().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_16().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_16().num() as u8,
                    probability: f64::from(value.reward_item_16().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_17().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_17().itemID().0 as u16,
                    amount: value.reward_item_17().num() as u8,
                    probability: f64::from(value.reward_item_17().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_17().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_17().num() as u8,
                    probability: f64::from(value.reward_item_17().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_17().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_17().num() as u8,
                    probability: f64::from(value.reward_item_17().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_18().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_18().itemID().0 as u16,
                    amount: value.reward_item_18().num() as u8,
                    probability: f64::from(value.reward_item_18().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_18().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_18().num() as u8,
                    probability: f64::from(value.reward_item_18().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_18().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_18().num() as u8,
                    probability: f64::from(value.reward_item_18().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_19().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_19().itemID().0 as u16,
                    amount: value.reward_item_19().num() as u8,
                    probability: f64::from(value.reward_item_19().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_19().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_19().num() as u8,
                    probability: f64::from(value.reward_item_19().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_19().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_19().num() as u8,
                    probability: f64::from(value.reward_item_19().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_20().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_20().itemID().0 as u16,
                    amount: value.reward_item_20().num() as u8,
                    probability: f64::from(value.reward_item_20().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_20().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_20().num() as u8,
                    probability: f64::from(value.reward_item_20().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_20().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_20().num() as u8,
                    probability: f64::from(value.reward_item_20().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_21().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_21().itemID().0 as u16,
                    amount: value.reward_item_21().num() as u8,
                    probability: f64::from(value.reward_item_21().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_21().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_21().num() as u8,
                    probability: f64::from(value.reward_item_21().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_21().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_21().num() as u8,
                    probability: f64::from(value.reward_item_21().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_22().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_22().itemID().0 as u16,
                    amount: value.reward_item_22().num() as u8,
                    probability: f64::from(value.reward_item_22().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_22().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_22().num() as u8,
                    probability: f64::from(value.reward_item_22().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_22().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_22().num() as u8,
                    probability: f64::from(value.reward_item_22().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_23().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_23().itemID().0 as u16,
                    amount: value.reward_item_23().num() as u8,
                    probability: f64::from(value.reward_item_23().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_23().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_23().num() as u8,
                    probability: f64::from(value.reward_item_23().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_23().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_23().num() as u8,
                    probability: f64::from(value.reward_item_23().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_24().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_24().itemID().0 as u16,
                    amount: value.reward_item_24().num() as u8,
                    probability: f64::from(value.reward_item_24().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_24().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_24().num() as u8,
                    probability: f64::from(value.reward_item_24().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_24().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_24().num() as u8,
                    probability: f64::from(value.reward_item_24().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_25().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_25().itemID().0 as u16,
                    amount: value.reward_item_25().num() as u8,
                    probability: f64::from(value.reward_item_25().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_25().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_25().num() as u8,
                    probability: f64::from(value.reward_item_25().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_25().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_25().num() as u8,
                    probability: f64::from(value.reward_item_25().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_26().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_26().itemID().0 as u16,
                    amount: value.reward_item_26().num() as u8,
                    probability: f64::from(value.reward_item_26().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_26().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_26().num() as u8,
                    probability: f64::from(value.reward_item_26().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_26().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_26().num() as u8,
                    probability: f64::from(value.reward_item_26().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_27().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_27().itemID().0 as u16,
                    amount: value.reward_item_27().num() as u8,
                    probability: f64::from(value.reward_item_27().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_27().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_27().num() as u8,
                    probability: f64::from(value.reward_item_27().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_27().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_27().num() as u8,
                    probability: f64::from(value.reward_item_27().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_28().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_28().itemID().0 as u16,
                    amount: value.reward_item_28().num() as u8,
                    probability: f64::from(value.reward_item_28().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_28().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_28().num() as u8,
                    probability: f64::from(value.reward_item_28().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_28().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_28().num() as u8,
                    probability: f64::from(value.reward_item_28().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            if value.reward_item_29().itemID()
                != crate::raid_lottery_reward_item_generated::ItemID::ITEMID_NONE
            {
                items.push(Item {
                    id: value.reward_item_29().itemID().0 as u16,
                    amount: value.reward_item_29().num() as u8,
                    probability: f64::from(value.reward_item_29().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_29().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::GEM
            {
                items.push(Item {
                    id: u16::MAX - 1,
                    amount: value.reward_item_29().num() as u8,
                    probability: f64::from(value.reward_item_29().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            } else if value.reward_item_29().category()
                == crate::raid_lottery_reward_item_generated::RaidRewardItemCategoryType::POKE
            {
                items.push(Item {
                    id: u16::MAX,
                    amount: value.reward_item_29().num() as u8,
                    probability: f64::from(value.reward_item_29().rate()) / rate * 100.0,
                    subject: ItemSubject::All,
                });
            }
            table.insert(hash, items);
        }
        Self(table)
    }
}
