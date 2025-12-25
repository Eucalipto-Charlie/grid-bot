use crate::{
    execution::executor::Executor,
    types::{intent::TradeIntent, event::Event},
};

pub struct SolanaExecutor {
    // rpc_client
    // keypair
    // pending_signatures
}

impl SolanaExecutor {
    pub fn new(/* rpc_url, keypair */) -> Self {
        Self {
            // 初始化 RPC / signer
        }
    }
}

impl Executor for SolanaExecutor {
    fn submit(&mut self, intent: TradeIntent) {
        // 1. intent → DEX order
        // 2. 构造 Solana Transaction
        // 3. send_transaction
        // 4. 记录 signature
    }

    fn poll_events(&mut self) -> Vec<Event> {
        // 1. 查询 signature 状态
        // 2. 成功 → TxConfirmed
        // 3. 失败 → TxFailed
        vec![]
    }
}
