//! WASM 执行器 - 技能安全沙箱

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, error};
use wasmtime::*;
use wasmtime_wasi::*;

/// WASM 执行器
pub struct WasmExecutor {
    engine: Engine,
    wasi_ctx: Arc<Mutex<WasiCtx>>,
}

impl WasmExecutor {
    /// 创建新的 WASM 执行器
    pub fn new() -> Result<Self> {
        let config = Config::new();
        let engine = Engine::new(&config)?;
        
        let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();
        
        Ok(Self {
            engine,
            wasi_ctx: Arc::new(Mutex::new(wasi_ctx)),
        })
    }
    
    /// 执行 WASM 模块
    pub async fn execute(&mut self, wasm_bytes: &[u8], input: &str) -> Result<String> {
        info!("执行 WASM 模块，大小：{} 字节", wasm_bytes.len());
        
        // 创建 Store
        let mut store = Store::new(
            &self.engine,
            self.wasi_ctx.lock().await.clone(),
        );
        
        // 加载模块
        let module = Module::from_binary(&self.engine, wasm_bytes)?;
        
        // 创建链接器
        let mut linker = Linker::new(&self.engine);
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        
        // 实例化
        let instance = linker.instantiate(&mut store, &module)?;
        
        // 获取 main 函数（如果存在）
        if let Some(main_func) = instance.get_func(&mut store, "main") {
            // 调用 main 函数
            main_func.call(&mut store, &[], &mut [])?;
            Ok("WASM 执行成功".to_string())
        } else {
            // 没有 main 函数，返回模块信息
            let exports: Vec<&str> = instance.exports(&store)
                .filter_map(|export| export.name())
                .collect();
            
            Ok(format!("WASM 模块已加载，导出函数：{:?}", exports))
        }
    }
    
    /// 执行 WASM 文件
    pub async fn execute_file(&mut self, path: &str, input: &str) -> Result<String> {
        use tokio::fs;
        
        let wasm_bytes = fs::read(path).await?;
        self.execute(&wasm_bytes, input).await
    }
}
