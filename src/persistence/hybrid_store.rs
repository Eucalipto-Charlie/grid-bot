use crate::persistence::trade_store::TradeStore;
use crate::persistence::memory_store::MemoryTradeStore;
use crate::persistence::file_store::FileTradeStore;
use crate::types::intent::{TradeIntent, TradeResult};

// 同时维护 MemoryTradeStore 和 FileTradeStore
pub struct HybridTradeStore {
    mem: MemoryTradeStore,
    file: FileTradeStore,
}

impl HybridTradeStore {
    //创建 HybridTradeStore
    pub fn new(base_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            mem: MemoryTradeStore::new(),
            file: FileTradeStore::new(base_dir),
        }
    }

    pub fn dump(&self) {
        self.mem.dump();
    }

    pub fn dump_table(&self) {
        self.mem.dump_table();
    }
}

impl TradeStore for HybridTradeStore {
    fn record_intent(&mut self, intent: &TradeIntent) {
        self.mem.record_intent(intent);
        self.file.record_intent(intent);
    }

    fn record_result(&mut self, result: &TradeResult) {
        self.mem.record_result(result);
        self.file.record_result(result);
    }

    fn list_intents(&self) -> Vec<TradeIntent> {
        self.mem.list_intents()
    }

    fn list_results(&self) -> Vec<TradeResult> {
        self.mem.list_results()
    }
}
