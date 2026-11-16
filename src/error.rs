use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("窗口事件循环发生致命错误: {0}")]
    EventLoop(#[from] winit::error::EventLoopError),

    #[error("无法创建系统窗口: {0}")]
    WindowCreation(#[from] winit::error::OsError),

    #[error("无法创建表面: {0}")]
    SurfaceCreation(#[from] wgpu::CreateSurfaceError),

    #[error("找不到兼容的图形适配器 (GPU)：{0}")]
    AdapterNotFound(#[from] wgpu::RequestAdapterError),

    #[error("无法请求 GPU 设备: {0}")]
    RequestDevice(#[from] wgpu::RequestDeviceError),

    #[error("Surface 能力校验失败: 不支持的格式或呈现模式")]
    SurfaceConfigError,

    #[error("文件加载/解析失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("其他引擎错误: {0}")]
    Other(String),
}

pub type EngineResult<T> = std::result::Result<T, EngineError>;
