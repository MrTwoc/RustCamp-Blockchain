use chrono::Utc;
use sha3::{Sha3_256, Digest};

// 定义区块结构体
struct Block {
    block_index: u64,
    // 区块时间戳: 用于POH共识机制
    timestamp: String,
    block_hash: String,
    // 领导节点
    leader: String,
    // 区块奖励
    _reward: u64,
    // 区块中的交易数据
    transactions: Vec<String>,
    previous_hash: String,
}

impl Block {
    // 计算区块的哈希值
    fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{:?}{}", self.block_index, self.timestamp, self.leader, self.transactions, self.previous_hash);
        let mut hasher = Sha3_256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // 创建新的区块
    fn new(index: u64, transactions: Vec<String>, previous_hash: String) -> Block {
        let timestamp = Utc::now().to_rfc3339();
        let mut block = Block {
            block_index: index,
            timestamp: timestamp,
            block_hash: "".to_string(),
            leader: "".to_string(),
            _reward: 0,
            transactions: transactions,
            previous_hash: previous_hash,
        };
        block.block_hash = block.calculate_hash();
        block
    }
}

// 定义区块链结构体
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    // 创建新的区块链
    fn new() -> Blockchain {
        let mut genesis_block = Vec::new();
        genesis_block.push("原始区块".to_string());
        let genesis_block = Block::new(0, genesis_block, "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    // 获取最后一个区块
    fn get_latest_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    // 添加新的区块
    fn add_block(&mut self, transactions: Vec<String>) {
        let previous_block = self.get_latest_block();
        let new_block = Block::new(previous_block.block_index + 1, transactions, previous_block.block_hash.clone());
        self.chain.push(new_block);
    }
}

// 定义 POH 结构体
struct POH {
    counter: u64,
}

impl POH {
    // 创建新的 POH 实例
    fn new() -> POH {
        POH {
            counter: 0,
        }
    }

    // 记录一个事件
    fn record_event(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }
}

// 定义 DPoS 结构体
struct DPoS {
    validators: Vec<String>,
    current_validator_index: usize,
}

impl DPoS {
    // 创建新的 DPoS 实例
    fn new(validators: Vec<String>) -> DPoS {
        DPoS {
            validators,
            current_validator_index: 0,
        }
    }

    // 获取当前验证者
    fn get_current_validator(&mut self) -> &str {
        let validator = &self.validators[self.current_validator_index];
        self.current_validator_index = (self.current_validator_index + 1) % self.validators.len();
        validator
    }
}

fn main() {
    // 创建区块链
    let mut blockchain = Blockchain::new();
    // 创建 POH 实例
    let mut poh = POH::new();
    // 创建 DPoS 实例
    let mut dpos = DPoS::new(vec!["验证者1".to_string(), "验证者2".to_string()]);

    // 添加一些区块
    for i in 0..5 {
        let event_time = poh.record_event();
        let validator = dpos.get_current_validator();
        let mut transactions: Vec<String> = vec![];
        for j in 0..5 {
            let data = format!("交易 {} 由 {} 在时间 {}", j * i, validator, event_time);
            transactions.push(data);
        }
        blockchain.add_block(transactions);
    }

    // 打印区块链
    for block in blockchain.chain {
        println!("索引: {}, 哈希值: {}, 交易数据: {:#?}，\n前哈希：{}", block.block_index, block.block_hash, block.transactions, block.previous_hash);
        println!("");
    }
}