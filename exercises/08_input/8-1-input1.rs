//! 练习 input1：统一输入抽象（适配器 → 映射 → Action）
//!
//! 阶段：8（输入系统）
//! 前置：03_camera（已有相机控制的需求直觉）
//! 依赖：无（纯 Rust，可用 std::collections）
//!
//! 步骤
//! 1. 定义 `Key`（按键标识）与 `RawEvent`（平台按键按下/释放事件）。
//! 2. 实现 `InputAdapter`：消费 `RawEvent`，维护「哪些键正在按下」的集合。
//! 3. 实现 `InputMapper`：基于按键状态与时间戳，解析组合键 / 长按 / 短按等时序。
//! 4. 产出纯业务指令 `Action`，与平台无关。
//! 5. 用 `check()` 断言「事件序列 → 预期 Action」。
//!
//! 资料
//! - winit KeyboardInput: https://docs.rs/winit/latest/winit/event/struct.KeyEvent.html
//! - 组合键状态机思路: https://gamedev.stackexchange.com/questions/134377/how-to-extract-binding-details-and-action-from-input-events
//!
//! 验收
//! - InputAdapter 维护按键状态
//! - InputMapper 识别组合键 / 长按 / 短按
//! - 生成纯业务 Action
//!
//! 总结
//! 理解「平台事件 → 统一状态 → 业务意图」的分层：把易变的平台输入与稳定的业务语义解耦。

use std::collections::HashSet;

/// 平台无关的按键标识。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    W,
    A,
    S,
    D,
    Space,
    Shift,
}

/// 平台事件：一条原始输入（按下 / 释放）。
/// 抽象掉 winit / Web / 3D 鼠标等具体来源，只保留「哪个键、什么动作、何时」。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RawEvent {
    Pressed { key: Key, time_ms: u64 },
    Released { key: Key, time_ms: u64 },
}

/// 输入适配器：消费 `RawEvent`，维护「当前按下的键」集合。
#[derive(Debug, Default)]
pub struct InputAdapter {
    pressed: HashSet<Key>,
}

impl InputAdapter {
    /// 消费一条原始事件，更新内部按键状态。
    pub fn handle(&mut self, event: RawEvent) {
        // TODO: 按下则插入集合，释放则移除。
        match event {
            RawEvent::Pressed { key, .. } => {
                self.pressed.insert(key);
            }
            RawEvent::Released { key, .. } => {
                self.pressed.remove(&key);
            }
        }
    }

    /// 某个键当前是否处于按下状态。
    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }
}

/// 业务指令：与平台无关的纯语义动作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MoveForward,
    MoveBack,
    StrafeLeft,
    StrafeRight,
    Jump,
    /// 长按空格触发的高跳（较普通 Jump 更强的跳跃）。
    HighJump,
    Sprint,
}

/// 输入映射器：解析按键状态的组合与时序，生成业务指令。
#[derive(Debug, Default)]
pub struct InputMapper {
    /// 记录每个键最后一次「按下」的时刻，用于判断长按/短按。
    pressed_since: std::collections::HashMap<Key, u64>,
}

/// 长按阈值（毫秒）。空格按下超过该时长判为「长按」。
const LONG_PRESS_MS: u64 = 500;

impl InputMapper {
    /// 更新映射器状态（记录按下时刻），供下一步查询使用。
    pub fn update(&mut self, adapter: &InputAdapter, event: RawEvent) {
        match event {
            // 按下时记录时间戳（若尚未记录）。
            RawEvent::Pressed { key, time_ms } => {
                if adapter.is_pressed(key) {
                    self.pressed_since.entry(key).or_insert(time_ms);
                }
            }
            // 释放时清除记录。
            RawEvent::Released { key, .. } => {
                self.pressed_since.remove(&key);
            }
        }
    }

    /// 根据当前按键状态生成业务动作。
    /// `now_ms` 是当前时间戳，用于区分长按/短按。
    /// 规则：
    /// - W + Shift 同时按下 → Sprint（组合键）。
    /// - W → MoveForward；S → MoveBack；A → StrafeLeft；D → StrafeRight。
    /// - Space 短按 → Jump；长按（按下超过 `LONG_PRESS_MS`）→ HighJump。
    pub fn resolve(&self, adapter: &InputAdapter, now_ms: u64) -> Vec<Action> {
        let mut actions = Vec::new();

        // 组合键：W + Shift → Sprint（优先级高于单键移动）。
        if adapter.is_pressed(Key::W) && adapter.is_pressed(Key::Shift) {
            actions.push(Action::Sprint);
            return actions;
        }

        if adapter.is_pressed(Key::Space) {
            // 长按 / 短按时序判断。
            let held_ms = self
                .pressed_since
                .get(&Key::Space)
                .map(|start| now_ms.saturating_sub(*start))
                .unwrap_or(0);
            if held_ms >= LONG_PRESS_MS {
                actions.push(Action::HighJump);
            } else {
                actions.push(Action::Jump);
            }
        }

        // 单键映射。
        if adapter.is_pressed(Key::W) {
            actions.push(Action::MoveForward);
        }
        if adapter.is_pressed(Key::S) {
            actions.push(Action::MoveBack);
        }
        if adapter.is_pressed(Key::A) {
            actions.push(Action::StrafeLeft);
        }
        if adapter.is_pressed(Key::D) {
            actions.push(Action::StrafeRight);
        }

        actions
    }
}

/// 自检：用若干事件序列验证「适配器状态」与「映射出的 Action」。
fn check() {
    // 场景 1：单键按下 → MoveForward。
    let mut adapter = InputAdapter::default();
    let mut mapper = InputMapper::default();

    adapter.handle(RawEvent::Pressed {
        key: Key::W,
        time_ms: 0,
    });
    mapper.update(
        &adapter,
        RawEvent::Pressed {
            key: Key::W,
            time_ms: 0,
        },
    );
    let ok1 = mapper.resolve(&adapter, 100) == vec![Action::MoveForward];

    // 场景 2：释放 W 后状态清空。
    adapter.handle(RawEvent::Released {
        key: Key::W,
        time_ms: 10,
    });
    mapper.update(
        &adapter,
        RawEvent::Released {
            key: Key::W,
            time_ms: 10,
        },
    );
    let ok2 = !adapter.is_pressed(Key::W) && mapper.resolve(&adapter, 100).is_empty();

    // 场景 3：W + Shift 组合键 → Sprint（优先级高于 MoveForward）。
    let mut adapter3 = InputAdapter::default();
    let mut mapper3 = InputMapper::default();
    adapter3.handle(RawEvent::Pressed {
        key: Key::W,
        time_ms: 0,
    });
    adapter3.handle(RawEvent::Pressed {
        key: Key::Shift,
        time_ms: 5,
    });
    mapper3.update(
        &adapter3,
        RawEvent::Pressed {
            key: Key::W,
            time_ms: 0,
        },
    );
    mapper3.update(
        &adapter3,
        RawEvent::Pressed {
            key: Key::Shift,
            time_ms: 5,
        },
    );
    let ok3 = mapper3.resolve(&adapter3, 100) == vec![Action::Sprint];

    // 场景 4：空格短按 → Jump（按住 100ms，未达阈值）。
    let mut adapter4 = InputAdapter::default();
    let mut mapper4 = InputMapper::default();
    adapter4.handle(RawEvent::Pressed {
        key: Key::Space,
        time_ms: 0,
    });
    mapper4.update(
        &adapter4,
        RawEvent::Pressed {
            key: Key::Space,
            time_ms: 0,
        },
    );
    let short = mapper4.resolve(&adapter4, 100);
    let ok4 = short.contains(&Action::Jump) && !short.contains(&Action::HighJump);

    // 场景 5：空格长按（按住 600ms，超过阈值）→ HighJump。
    let mut adapter5 = InputAdapter::default();
    let mut mapper5 = InputMapper::default();
    adapter5.handle(RawEvent::Pressed {
        key: Key::Space,
        time_ms: 0,
    });
    mapper5.update(
        &adapter5,
        RawEvent::Pressed {
            key: Key::Space,
            time_ms: 0,
        },
    );
    let long = mapper5.resolve(&adapter5, 600);
    let ok5 = long.contains(&Action::HighJump) && !long.contains(&Action::Jump);

    if ok1 && ok2 && ok3 && ok4 && ok5 {
        println!("✅ 通过：单键、释放清空、组合键、短按/长按时序均正确。");
    } else {
        println!("❌ 未通过：单键={ok1}，释放清空={ok2}，组合键={ok3}，短按={ok4}，长按={ok5}。");
    }
}

fn main() {
    check();
}
