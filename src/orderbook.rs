enum OrderType {
    GoodTillCancel,
    FillAndKill,
}

struct levelInfo {
    price: u32,
    quantity: u32,
}

type levelInfos = Vec<levelInfo>;

struct orderBookLevelInfos {
    bids: levelInfos,
    asks: levelInfos,
}

impl orderBookLevelInfos {
    fn new(bids: levelInfos, asks: levelInfos) -> Self {
        Self { bids, asks }
    }
}
