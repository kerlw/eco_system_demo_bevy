//! 用户界面系统模块
//!
//! 负责游戏界面的渲染和交互

mod animal_state_ui;
pub mod cards;
mod error_tips;
pub mod hud;
mod progress_bar;
mod progress_bar_material;

pub use animal_state_ui::AnimalStateUIPanel;
pub use cards::*;
pub use error_tips::ErrorTipsPlugin;
pub use error_tips::show_error_tips;
pub use progress_bar::*;
pub use progress_bar_material::ProgressBarMaterial;
