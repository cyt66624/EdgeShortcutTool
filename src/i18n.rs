//! Translation tables for the user interface.
//!
//! Every user visible string lives here so that a language switch only has to
//! re-apply the active table to the existing controls. The tables are plain
//! `&'static str` values, which keeps them cheap to share and lets the UI code
//! borrow them without any allocation or lifetime juggling.

use windows::Win32::Globalization::GetUserDefaultUILanguage;

/// Languages offered by the language menu, in menu order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Language {
    English,
    SimplifiedChinese,
    TraditionalChinese,
    Japanese,
    German,
    French,
    Spanish,
    Korean
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

impl Language {
    pub const ALL: [Language; 8] = [
        Language::English,
        Language::SimplifiedChinese,
        Language::TraditionalChinese,
        Language::Japanese,
        Language::German,
        Language::French,
        Language::Spanish,
        Language::Korean
    ];

    /// Stable identifier persisted in the registry.
    pub const fn code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::SimplifiedChinese => "zh-CN",
            Language::TraditionalChinese => "zh-TW",
            Language::Japanese => "ja",
            Language::German => "de",
            Language::French => "fr",
            Language::Spanish => "es",
            Language::Korean => "ko"
        }
    }

    /// Name of the language in the language itself, as shown in the menu.
    pub const fn native_name(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::SimplifiedChinese => "简体中文",
            Language::TraditionalChinese => "繁體中文",
            Language::Japanese => "日本語",
            Language::German => "Deutsch",
            Language::French => "Français",
            Language::Spanish => "Español",
            Language::Korean => "한국어"
        }
    }

    /// Compact label for the small top-right language button.
    pub const fn short_label(self) -> &'static str {
        match self {
            Language::English => "EN",
            Language::SimplifiedChinese => "简",
            Language::TraditionalChinese => "繁",
            Language::Japanese => "日",
            Language::German => "DE",
            Language::French => "FR",
            Language::Spanish => "ES",
            Language::Korean => "한"
        }
    }

    /// Accepts both the persisted codes and a few common aliases.
    pub fn from_code(value: &str) -> Option<Language> {
        let normalized = value.trim().to_ascii_lowercase().replace('_', "-");

        match normalized.as_str() {
            "en" | "en-us" | "en-gb" => Some(Language::English),
            "zh" | "zh-cn" | "zh-hans" | "zh-sg" => Some(Language::SimplifiedChinese),
            "zh-tw" | "zh-hk" | "zh-mo" | "zh-hant" => Some(Language::TraditionalChinese),
            "ja" | "ja-jp" => Some(Language::Japanese),
            "de" | "de-de" => Some(Language::German),
            "fr" | "fr-fr" => Some(Language::French),
            "es" | "es-es" => Some(Language::Spanish),
            "ko" | "ko-kr" => Some(Language::Korean),
            _ => None
        }
    }

    /// Best guess for a first start without a stored preference.
    pub fn detect_system() -> Language {
        // SAFETY: GetUserDefaultUILanguage only reads the current user language
        // id and has no pointer or lifetime requirements.
        let langid = unsafe { GetUserDefaultUILanguage() };
        Language::from_windows_langid(langid)
    }

    fn from_windows_langid(langid: u16) -> Language {
        const SUBLANG_CHINESE_TRADITIONAL: u16 = 0x01;
        const SUBLANG_CHINESE_SIMPLIFIED: u16 = 0x02;
        const SUBLANG_CHINESE_HONG_KONG: u16 = 0x03;
        const SUBLANG_CHINESE_SINGAPORE: u16 = 0x04;
        const SUBLANG_CHINESE_MACAU: u16 = 0x05;

        let primary = langid & 0x03ff;
        let sublang = langid >> 10;

        match primary {
            0x04 => match sublang {
                SUBLANG_CHINESE_SIMPLIFIED | SUBLANG_CHINESE_SINGAPORE => Language::SimplifiedChinese,
                SUBLANG_CHINESE_TRADITIONAL | SUBLANG_CHINESE_HONG_KONG | SUBLANG_CHINESE_MACAU => {
                    Language::TraditionalChinese
                }
                // Unspecified Chinese sub-language: simplified is the safer default.
                _ => Language::SimplifiedChinese
            },
            0x11 => Language::Japanese,
            0x07 => Language::German,
            0x0c => Language::French,
            0x0a => Language::Spanish,
            0x12 => Language::Korean,
            _ => Language::English
        }
    }
}

/// One block of the Help window.
pub struct HelpSection {
    pub title: &'static str,
    pub body: &'static [&'static str],
    /// Reminder blocks are rendered in bold red.
    pub reminder: bool
}

const NORMAL_SECTION: bool = false;
const REMINDER_SECTION: bool = true;

pub struct Strings {
    // Titles
    pub app_title: &'static str,
    pub advanced_window_title: &'static str,
    pub custom_window_title: &'static str,
    pub help_window_title: &'static str,
    pub info_window_title: &'static str,

    // Main window
    pub preset_title: &'static str,
    pub old_workaround: &'static str,
    pub new_workaround: &'static str,
    pub restore_default: &'static str,
    pub hide_sign_in: &'static str,
    pub restore_sidebar: &'static str,
    pub disable_extensions: &'static str,
    pub exit: &'static str,
    pub status_ready: &'static str,
    pub status_applying: &'static str,
    pub status_completed_with_errors: &'static str,
    /// `{}` is replaced with the localized name of the applied preset.
    pub status_applied: &'static str,
    pub status_enter_custom_flags: &'static str,

    // Advanced window
    pub shortcut_targets_title: &'static str,
    pub shortcut_targets_description: &'static str,
    pub external_links_title: &'static str,
    pub external_links_description: &'static str,
    pub apply_to_external_links: &'static str,
    pub ok: &'static str,
    pub cancel: &'static str,

    // Custom window
    pub standalone: &'static str,
    pub enable_features: &'static str,
    pub disable_features: &'static str,
    pub apply: &'static str,
    pub close: &'static str,

    // Info window
    pub info_executables: &'static str,
    pub info_shortcuts: &'static str,

    // Tooltips
    pub tip_advanced: &'static str,
    pub tip_custom: &'static str,
    pub tip_help: &'static str,
    pub tip_info: &'static str,
    pub tip_restore_default: &'static str,
    pub tip_external_links: &'static str,
    /// `{}` is replaced with an example command line.
    pub tip_example: &'static str,
    /// `{}` is replaced with the native name of the active language.
    pub tip_language: &'static str,

    // Info window reports
    /// `{}` is replaced with the selected shortcut names.
    pub report_selected: &'static str,
    pub report_selected_none: &'static str,
    pub report_updated: &'static str,
    pub report_failed: &'static str,
    pub report_missing: &'static str,
    pub report_found: &'static str,
    pub report_registry: &'static str,
    pub report_external_links: &'static str,
    pub report_none_applied: &'static str,
    pub report_selected_header: &'static str,
    pub report_candidates: &'static str,
    pub report_not_found: &'static str,
    pub report_exists: &'static str,
    pub report_status_missing: &'static str,
    pub report_env_missing: &'static str,
    pub report_reference_only: &'static str,
    pub report_env_not_set: &'static str,
    /// Four `{}` placeholders: found, updated, missing, failed.
    pub report_latest_summary: &'static str,
    pub report_no_operation: &'static str,

    // Shortcut location labels
    pub location_system_start_menu: &'static str,
    pub location_public_desktop: &'static str,
    pub location_pinned_taskbar: &'static str,
    pub location_pinned_start_menu: &'static str,
    pub location_implicit_pinned: &'static str,
    pub location_quick_launch: &'static str,
    pub location_user_start_menu: &'static str,
    pub location_user_desktop: &'static str,
    pub location_generic: &'static str,

    // Startup error dialogs
    pub error_font: &'static str,
    pub error_com: &'static str,
    pub error_ui: &'static str,

    pub help_sections: &'static [HelpSection]
}

static EN: Strings = Strings {
    app_title: "Edge Shortcut Tool",
    advanced_window_title: "Advanced",
    custom_window_title: "Custom",
    help_window_title: "Help",
    info_window_title: "Info",

    preset_title: "Fix rounded corners",
    old_workaround: "Old workaround",
    new_workaround: "New workaround",
    restore_default: "Restore default",
    hide_sign_in: "Hide sign-in red dot",
    restore_sidebar: "Restore sidebar",
    disable_extensions: "Disable extensions",
    exit: "Exit",
    status_ready: "Ready",
    status_applying: "Applying...",
    status_completed_with_errors: "Completed with errors",
    status_applied: "Applied: {}",
    status_enter_custom_flags: "Enter custom flags",

    shortcut_targets_title: "Shortcut targets",
    shortcut_targets_description: "Choose which shortcut names are updated.",
    external_links_title: "External links",
    external_links_description: "Use the same flags when Edge is opened by another app.\r\nSuch as Discord, Teams, Terminal.",
    apply_to_external_links: "Apply to external links",
    ok: "OK",
    cancel: "Cancel",

    standalone: "Standalone",
    enable_features: "Enable features:",
    disable_features: "Disable features:",
    apply: "Apply",
    close: "Close",

    info_executables: "EXECUTABLES",
    info_shortcuts: "SHORTCUTS",

    tip_advanced: "Advanced",
    tip_custom: "Custom",
    tip_help: "Help",
    tip_info: "Info",
    tip_restore_default: "Restore normal Edge shortcut settings",
    tip_external_links: "Apply flags when Edge is opened by other apps",
    tip_example: "Example: {}",
    tip_language: "Language: {}",

    report_selected: "Selected: {}",
    report_selected_none: "Selected: None",
    report_updated: "Updated:",
    report_failed: "Failed:",
    report_missing: "Missing:",
    report_found: "Found:",
    report_registry: "Registry:",
    report_external_links: "External links:",
    report_none_applied: "None applied",
    report_selected_header: "Selected:",
    report_candidates: "Candidates:",
    report_not_found: "Not found in common install locations",
    report_exists: "Exists",
    report_status_missing: "Missing",
    report_env_missing: "Environment missing",
    report_reference_only: "For reference and diagnostic purposes only.",
    report_env_not_set: "(environment variable not set)",
    report_latest_summary: "Latest summary: {} found, {} updated, {} missing, {} failed",
    report_no_operation: "No operation has run during this session.",

    location_system_start_menu: "System Start Menu",
    location_public_desktop: "Public Desktop",
    location_pinned_taskbar: "Pinned Taskbar",
    location_pinned_start_menu: "Pinned Start Menu",
    location_implicit_pinned: "Implicit pinned shortcut",
    location_quick_launch: "Quick Launch",
    location_user_start_menu: "Current user Start Menu",
    location_user_desktop: "Current user Desktop",
    location_generic: "Shortcut",

    error_font: "Failed to initialize Segoe UI font.",
    error_com: "Failed to initialize COM.",
    error_ui: "Failed to build the user interface.",

    help_sections: &[
        HelpSection {
            title: "Old workaround",
            body: &[
                "Uses msFeatureGroupNewLookAndFeelHoldout.",
                "This is the older/broader method for removing rounded corner look."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "New workaround",
            body: &[
                "Uses msForceNoRoundedCornerAndMargin.",
                "This is the newer/more direct method for removing rounded corners and margins."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Restore default",
            body: &["Clears all command-line flags managed by this tool and restores normal launch behavior."],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Hide sign-in red dot",
            body: &[
                "When checked, this also disables msShowSignInIndicator.",
                "This hides the red sign-in reminder dot on profile icon."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Restore sidebar",
            body: &[
                "When checked, this also disables msHubAppsSidebarRetirement.",
                "This attempts to restore the classic sidebar app list."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Disable extensions",
            body: &[
                "When checked, this also adds --disable-extensions.",
                "This starts Edge without loading browser extensions."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Advanced",
            body: &[
                "Choose which shortcut names are updated.",
                "You can also apply flags when Edge is opened by another app such as Discord or Teams."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Custom",
            body: &[
                "Standalone example: --disable-extensions --force-dark-mode --mute-audio",
                "Enable example: msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "Disable example: msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Info",
            body: &[
                "Displays executable candidates and selected shortcut locations.",
                "For reference and diagnostic purposes only."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Reminder",
            body: &[
                "Disable Startup Boost in Edge first. Copy and paste this into Edge address bar: ",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

static ZH_CN: Strings = Strings {
    app_title: "Edge 快捷方式工具",
    advanced_window_title: "高级",
    custom_window_title: "自定义",
    help_window_title: "帮助",
    info_window_title: "信息",

    preset_title: "修复圆角",
    old_workaround: "旧方案",
    new_workaround: "新方案",
    restore_default: "恢复默认",
    hide_sign_in: "隐藏登录红点",
    restore_sidebar: "恢复侧边栏",
    disable_extensions: "禁用扩展",
    exit: "退出",
    status_ready: "就绪",
    status_applying: "正在应用…",
    status_completed_with_errors: "完成，但有错误",
    status_applied: "已应用：{}",
    status_enter_custom_flags: "请输入自定义参数",

    shortcut_targets_title: "快捷方式目标",
    shortcut_targets_description: "选择要更新的快捷方式名称。",
    external_links_title: "外部链接",
    external_links_description: "由其他应用打开 Edge 时使用相同参数。\r\n例如 Discord、Teams、终端。",
    apply_to_external_links: "应用到外部链接",
    ok: "确定",
    cancel: "取消",

    standalone: "独立参数",
    enable_features: "启用功能：",
    disable_features: "禁用功能：",
    apply: "应用",
    close: "关闭",

    info_executables: "可执行文件",
    info_shortcuts: "快捷方式",

    tip_advanced: "高级",
    tip_custom: "自定义",
    tip_help: "帮助",
    tip_info: "信息",
    tip_restore_default: "恢复 Edge 快捷方式的默认设置",
    tip_external_links: "由其他应用打开 Edge 时同样应用参数",
    tip_example: "示例：{}",
    tip_language: "语言：{}",

    report_selected: "已选择：{}",
    report_selected_none: "已选择：无",
    report_updated: "已更新：",
    report_failed: "失败：",
    report_missing: "缺失：",
    report_found: "已找到：",
    report_registry: "注册表：",
    report_external_links: "外部链接：",
    report_none_applied: "未应用",
    report_selected_header: "已选择：",
    report_candidates: "候选：",
    report_not_found: "未在常见安装位置找到",
    report_exists: "存在",
    report_status_missing: "缺失",
    report_env_missing: "环境变量缺失",
    report_reference_only: "仅供参考和诊断使用。",
    report_env_not_set: "（环境变量未设置）",
    report_latest_summary: "最新汇总：找到 {} 个，更新 {} 个，缺失 {} 个，失败 {} 个",
    report_no_operation: "本次会话尚未执行任何操作。",

    location_system_start_menu: "系统开始菜单",
    location_public_desktop: "公共桌面",
    location_pinned_taskbar: "任务栏固定项",
    location_pinned_start_menu: "开始菜单固定项",
    location_implicit_pinned: "隐式固定快捷方式",
    location_quick_launch: "快速启动",
    location_user_start_menu: "当前用户开始菜单",
    location_user_desktop: "当前用户桌面",
    location_generic: "快捷方式",

    error_font: "无法初始化 Segoe UI 字体。",
    error_com: "无法初始化 COM。",
    error_ui: "无法创建用户界面。",

    help_sections: &[
        HelpSection {
            title: "旧方案",
            body: &[
                "使用 msFeatureGroupNewLookAndFeelHoldout。",
                "这是移除圆角外观的旧版、更宽泛的方法。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "新方案",
            body: &[
                "使用 msForceNoRoundedCornerAndMargin。",
                "这是移除圆角和边距的较新、更直接的方法。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "恢复默认",
            body: &["清除本工具管理的所有命令行参数，并恢复正常启动行为。"],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "隐藏登录红点",
            body: &[
                "勾选后会同时禁用 msShowSignInIndicator。",
                "用于隐藏配置图标上的红色登录提醒点。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "恢复侧边栏",
            body: &[
                "勾选后会同时禁用 msHubAppsSidebarRetirement。",
                "尝试恢复经典侧边栏应用列表。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "禁用扩展",
            body: &[
                "勾选后会同时添加 --disable-extensions。",
                "以不加载浏览器扩展的方式启动 Edge。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "高级",
            body: &[
                "选择要更新的快捷方式名称。",
                "也可以在 Discord、Teams 等其他应用打开 Edge 时应用这些参数。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "自定义",
            body: &[
                "独立参数示例：--disable-extensions --force-dark-mode --mute-audio",
                "启用功能示例：msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "禁用功能示例：msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "信息",
            body: &[
                "显示可执行文件候选与所选快捷方式位置。",
                "仅供参考和诊断使用。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "提醒",
            body: &[
                "请先在 Edge 中关闭“启动加速”。将下面的地址复制到 Edge 地址栏：",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

static ZH_TW: Strings = Strings {
    app_title: "Edge 捷徑工具",
    advanced_window_title: "進階",
    custom_window_title: "自訂",
    help_window_title: "說明",
    info_window_title: "資訊",

    preset_title: "修正圓角",
    old_workaround: "舊方案",
    new_workaround: "新方案",
    restore_default: "恢復預設",
    hide_sign_in: "隱藏登入紅點",
    restore_sidebar: "恢復側邊欄",
    disable_extensions: "停用擴充功能",
    exit: "結束",
    status_ready: "就緒",
    status_applying: "正在套用…",
    status_completed_with_errors: "完成，但有錯誤",
    status_applied: "已套用：{}",
    status_enter_custom_flags: "請輸入自訂參數",

    shortcut_targets_title: "捷徑目標",
    shortcut_targets_description: "選擇要更新的捷徑名稱。",
    external_links_title: "外部連結",
    external_links_description: "由其他應用程式開啟 Edge 時使用相同參數。\r\n例如 Discord、Teams、終端機。",
    apply_to_external_links: "套用到外部連結",
    ok: "確定",
    cancel: "取消",

    standalone: "獨立參數",
    enable_features: "啟用功能：",
    disable_features: "停用功能：",
    apply: "套用",
    close: "關閉",

    info_executables: "可執行檔",
    info_shortcuts: "捷徑",

    tip_advanced: "進階",
    tip_custom: "自訂",
    tip_help: "說明",
    tip_info: "資訊",
    tip_restore_default: "恢復 Edge 捷徑的預設設定",
    tip_external_links: "由其他應用程式開啟 Edge 時同樣套用參數",
    tip_example: "範例：{}",
    tip_language: "語言：{}",

    report_selected: "已選擇：{}",
    report_selected_none: "已選擇：無",
    report_updated: "已更新：",
    report_failed: "失敗：",
    report_missing: "缺少：",
    report_found: "已找到：",
    report_registry: "登錄檔：",
    report_external_links: "外部連結：",
    report_none_applied: "未套用",
    report_selected_header: "已選擇：",
    report_candidates: "候選：",
    report_not_found: "未在常見安裝位置找到",
    report_exists: "存在",
    report_status_missing: "缺少",
    report_env_missing: "環境變數缺少",
    report_reference_only: "僅供參考與診斷使用。",
    report_env_not_set: "（環境變數未設定）",
    report_latest_summary: "最新摘要：找到 {} 個，更新 {} 個，缺少 {} 個，失敗 {} 個",
    report_no_operation: "本次工作階段尚未執行任何操作。",

    location_system_start_menu: "系統開始功能表",
    location_public_desktop: "公用桌面",
    location_pinned_taskbar: "工作列釘選項目",
    location_pinned_start_menu: "開始功能表釘選項目",
    location_implicit_pinned: "隱含釘選捷徑",
    location_quick_launch: "快速啟動",
    location_user_start_menu: "目前使用者開始功能表",
    location_user_desktop: "目前使用者桌面",
    location_generic: "捷徑",

    error_font: "無法初始化 Segoe UI 字型。",
    error_com: "無法初始化 COM。",
    error_ui: "無法建立使用者介面。",

    help_sections: &[
        HelpSection {
            title: "舊方案",
            body: &[
                "使用 msFeatureGroupNewLookAndFeelHoldout。",
                "這是移除圓角外觀的舊版、較廣泛的方法。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "新方案",
            body: &[
                "使用 msForceNoRoundedCornerAndMargin。",
                "這是移除圓角與邊距的較新、更直接的方法。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "恢復預設",
            body: &["清除本工具管理的所有命令列參數，並恢復正常啟動行為。"],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "隱藏登入紅點",
            body: &[
                "勾選後也會停用 msShowSignInIndicator。",
                "用於隱藏設定檔圖示上的紅色登入提醒點。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "恢復側邊欄",
            body: &[
                "勾選後也會停用 msHubAppsSidebarRetirement。",
                "嘗試恢復經典側邊欄應用程式清單。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "停用擴充功能",
            body: &[
                "勾選後也會加入 --disable-extensions。",
                "在不載入瀏覽器擴充功能的情況下啟動 Edge。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "進階",
            body: &[
                "選擇要更新的捷徑名稱。",
                "也可以在 Discord、Teams 等其他應用程式開啟 Edge 時套用這些參數。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "自訂",
            body: &[
                "獨立參數範例：--disable-extensions --force-dark-mode --mute-audio",
                "啟用功能範例：msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "停用功能範例：msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "資訊",
            body: &[
                "顯示可執行檔候選與所選捷徑位置。",
                "僅供參考與診斷使用。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "提醒",
            body: &[
                "請先在 Edge 關閉「啟動加速」。將下列網址複製到 Edge 網址列：",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

static JA: Strings = Strings {
    app_title: "Edge ショートカット ツール",
    advanced_window_title: "詳細設定",
    custom_window_title: "カスタム",
    help_window_title: "ヘルプ",
    info_window_title: "情報",

    preset_title: "角丸を修正",
    old_workaround: "旧方式",
    new_workaround: "新方式",
    restore_default: "既定に戻す",
    hide_sign_in: "サインインの赤い印を隠す",
    restore_sidebar: "サイドバーを復元",
    disable_extensions: "拡張機能を無効化",
    exit: "終了",
    status_ready: "準備完了",
    status_applying: "適用しています…",
    status_completed_with_errors: "完了（エラーあり）",
    status_applied: "適用しました：{}",
    status_enter_custom_flags: "カスタム フラグを入力してください",

    shortcut_targets_title: "ショートカットの対象",
    shortcut_targets_description: "更新するショートカット名を選択します。",
    external_links_title: "外部リンク",
    external_links_description: "他のアプリから Edge を開くときも同じフラグを使います。\r\n例：Discord、Teams、ターミナル。",
    apply_to_external_links: "外部リンクにも適用",
    ok: "OK",
    cancel: "キャンセル",

    standalone: "単独フラグ",
    enable_features: "有効にする機能：",
    disable_features: "無効にする機能：",
    apply: "適用",
    close: "閉じる",

    info_executables: "実行ファイル",
    info_shortcuts: "ショートカット",

    tip_advanced: "詳細設定",
    tip_custom: "カスタム",
    tip_help: "ヘルプ",
    tip_info: "情報",
    tip_restore_default: "Edge ショートカットの標準設定に戻す",
    tip_external_links: "他のアプリから Edge を開いたときにもフラグを適用",
    tip_example: "例：{}",
    tip_language: "言語：{}",

    report_selected: "選択中：{}",
    report_selected_none: "選択中：なし",
    report_updated: "更新済み：",
    report_failed: "失敗：",
    report_missing: "見つかりません：",
    report_found: "検出：",
    report_registry: "レジストリ：",
    report_external_links: "外部リンク：",
    report_none_applied: "適用なし",
    report_selected_header: "選択中：",
    report_candidates: "候補：",
    report_not_found: "一般的なインストール先に見つかりません",
    report_exists: "存在",
    report_status_missing: "なし",
    report_env_missing: "環境変数なし",
    report_reference_only: "参照および診断用です。",
    report_env_not_set: "（環境変数が設定されていません）",
    report_latest_summary: "最新の結果：検出 {} 件、更新 {} 件、なし {} 件、失敗 {} 件",
    report_no_operation: "このセッションではまだ何も実行されていません。",

    location_system_start_menu: "システムのスタート メニュー",
    location_public_desktop: "パブリック デスクトップ",
    location_pinned_taskbar: "タスクバーにピン留め",
    location_pinned_start_menu: "スタート メニューにピン留め",
    location_implicit_pinned: "暗黙のピン留めショートカット",
    location_quick_launch: "クイック起動",
    location_user_start_menu: "現在のユーザーのスタート メニュー",
    location_user_desktop: "現在のユーザーのデスクトップ",
    location_generic: "ショートカット",

    error_font: "Segoe UI フォントを初期化できませんでした。",
    error_com: "COM を初期化できませんでした。",
    error_ui: "ユーザー インターフェイスを構築できませんでした。",

    help_sections: &[
        HelpSection {
            title: "旧方式",
            body: &[
                "msFeatureGroupNewLookAndFeelHoldout を使用します。",
                "角丸表示を取り除く従来の広範な方法です。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "新方式",
            body: &[
                "msForceNoRoundedCornerAndMargin を使用します。",
                "角丸と余白を取り除く、より新しい直接的な方法です。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "既定に戻す",
            body: &["このツールが管理するコマンド ライン フラグをすべて消去し、通常の起動動作に戻します。"],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "サインインの赤い印を隠す",
            body: &[
                "チェックすると msShowSignInIndicator も無効になります。",
                "プロフィール アイコンの赤いサインイン通知の印を隠します。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "サイドバーを復元",
            body: &[
                "チェックすると msHubAppsSidebarRetirement も無効になります。",
                "従来のサイドバー アプリ一覧の復元を試みます。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "拡張機能を無効化",
            body: &[
                "チェックすると --disable-extensions も追加されます。",
                "拡張機能を読み込まずに Edge を起動します。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "詳細設定",
            body: &[
                "更新するショートカット名を選択します。",
                "Discord や Teams など他のアプリから Edge を開いたときにもフラグを適用できます。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "カスタム",
            body: &[
                "単独フラグの例：--disable-extensions --force-dark-mode --mute-audio",
                "有効にする機能の例：msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "無効にする機能の例：msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "情報",
            body: &[
                "実行ファイルの候補と、選択したショートカットの場所を表示します。",
                "参照および診断用です。"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "注意",
            body: &[
                "先に Edge のスタートアップ ブーストをオフにしてください。次の URL を Edge のアドレス バーに貼り付けます：",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

static DE: Strings = Strings {
    app_title: "Edge Verknüpfungs-Tool",
    advanced_window_title: "Erweitert",
    custom_window_title: "Eigene Flags",
    help_window_title: "Hilfe",
    info_window_title: "Info",

    preset_title: "Ecken korrigieren",
    old_workaround: "Alte Methode",
    new_workaround: "Neue Methode",
    restore_default: "Standard wiederherstellen",
    hide_sign_in: "Anmeldepunkt ausblenden",
    restore_sidebar: "Seitenleiste zurückholen",
    disable_extensions: "Erweiterungen abschalten",
    exit: "Beenden",
    status_ready: "Bereit",
    status_applying: "Wird angewendet …",
    status_completed_with_errors: "Mit Fehlern beendet",
    status_applied: "Angewendet: {}",
    status_enter_custom_flags: "Eigene Flags eingeben",

    shortcut_targets_title: "Verknüpfungen",
    shortcut_targets_description: "Wählen Sie, welche Verknüpfungen aktualisiert werden.",
    external_links_title: "Externe Links",
    external_links_description: "Gleiche Flags, wenn andere Apps Edge öffnen.\r\nZ. B. Discord, Teams, Terminal.",
    apply_to_external_links: "Auf externe Links anwenden",
    ok: "OK",
    cancel: "Abbrechen",

    standalone: "Einzelne Flags",
    enable_features: "Funktionen aktivieren:",
    disable_features: "Funktionen deaktivieren:",
    apply: "Anwenden",
    close: "Schließen",

    info_executables: "PROGRAMME",
    info_shortcuts: "VERKNÜPFUNGEN",

    tip_advanced: "Erweitert",
    tip_custom: "Eigene Flags",
    tip_help: "Hilfe",
    tip_info: "Info",
    tip_restore_default: "Normale Edge-Verknüpfungseinstellungen wiederherstellen",
    tip_external_links: "Flags auch setzen, wenn andere Apps Edge öffnen",
    tip_example: "Beispiel: {}",
    tip_language: "Sprache: {}",

    report_selected: "Ausgewählt: {}",
    report_selected_none: "Ausgewählt: Keine",
    report_updated: "Aktualisiert:",
    report_failed: "Fehlgeschlagen:",
    report_missing: "Fehlt:",
    report_found: "Gefunden:",
    report_registry: "Registry:",
    report_external_links: "Externe Links:",
    report_none_applied: "Nicht angewendet",
    report_selected_header: "Ausgewählt:",
    report_candidates: "Kandidaten:",
    report_not_found: "In den üblichen Installationsorten nicht gefunden",
    report_exists: "Vorhanden",
    report_status_missing: "Fehlt",
    report_env_missing: "Umgebungsvariable fehlt",
    report_reference_only: "Nur zur Information und Diagnose.",
    report_env_not_set: "(Umgebungsvariable nicht gesetzt)",
    report_latest_summary: "Letzte Zusammenfassung: {} gefunden, {} aktualisiert, {} fehlend, {} fehlgeschlagen",
    report_no_operation: "In dieser Sitzung wurde noch nichts ausgeführt.",

    location_system_start_menu: "Systemstartmenü",
    location_public_desktop: "Öffentlicher Desktop",
    location_pinned_taskbar: "An Taskleiste angeheftet",
    location_pinned_start_menu: "An Startmenü angeheftet",
    location_implicit_pinned: "Implizit angeheftete Verknüpfung",
    location_quick_launch: "Schnellstart",
    location_user_start_menu: "Startmenü des Benutzers",
    location_user_desktop: "Desktop des Benutzers",
    location_generic: "Verknüpfung",

    error_font: "Schriftart Segoe UI konnte nicht initialisiert werden.",
    error_com: "COM konnte nicht initialisiert werden.",
    error_ui: "Die Benutzeroberfläche konnte nicht erstellt werden.",

    help_sections: &[
        HelpSection {
            title: "Alte Methode",
            body: &[
                "Verwendet msFeatureGroupNewLookAndFeelHoldout.",
                "Das ist die ältere, breitere Methode gegen abgerundete Ecken."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Neue Methode",
            body: &[
                "Verwendet msForceNoRoundedCornerAndMargin.",
                "Das ist die neuere, direktere Methode gegen abgerundete Ecken und Ränder."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Standard wiederherstellen",
            body: &["Entfernt alle von diesem Tool gesetzten Befehlszeilen-Flags und stellt das normale Startverhalten wieder her."],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Anmeldepunkt ausblenden",
            body: &[
                "Wenn aktiviert, wird zusätzlich msShowSignInIndicator deaktiviert.",
                "Blendet den roten Anmelde-Hinweispunkt am Profilsymbol aus."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Seitenleiste zurückholen",
            body: &[
                "Wenn aktiviert, wird zusätzlich msHubAppsSidebarRetirement deaktiviert.",
                "Versucht, die klassische Seitenleisten-App-Liste wiederherzustellen."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Erweiterungen abschalten",
            body: &[
                "Wenn aktiviert, wird zusätzlich --disable-extensions gesetzt.",
                "Startet Edge, ohne Browser-Erweiterungen zu laden."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Erweitert",
            body: &[
                "Wählen Sie, welche Verknüpfungen aktualisiert werden.",
                "Flags lassen sich auch setzen, wenn Edge von einer anderen App wie Discord oder Teams geöffnet wird."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Eigene Flags",
            body: &[
                "Einzelne Flags, Beispiel: --disable-extensions --force-dark-mode --mute-audio",
                "Aktivieren, Beispiel: msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "Deaktivieren, Beispiel: msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Info",
            body: &[
                "Zeigt Programm-Kandidaten und die gewählten Verknüpfungsorte an.",
                "Nur zur Information und Diagnose."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Hinweis",
            body: &[
                "Deaktivieren Sie zuerst den Startboost in Edge. Fügen Sie dies in die Adressleiste von Edge ein: ",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

static FR: Strings = Strings {
    app_title: "Outil de raccourcis Edge",
    advanced_window_title: "Avancé",
    custom_window_title: "Personnalisé",
    help_window_title: "Aide",
    info_window_title: "Infos",

    preset_title: "Corriger les coins",
    old_workaround: "Ancienne méthode",
    new_workaround: "Nouvelle méthode",
    restore_default: "Rétablir par défaut",
    hide_sign_in: "Masquer le point de connexion",
    restore_sidebar: "Restaurer la barre latérale",
    disable_extensions: "Désactiver les extensions",
    exit: "Quitter",
    status_ready: "Prêt",
    status_applying: "Application…",
    status_completed_with_errors: "Terminé avec des erreurs",
    status_applied: "Appliqué : {}",
    status_enter_custom_flags: "Saisissez les options",

    shortcut_targets_title: "Raccourcis",
    shortcut_targets_description: "Choisissez les noms de raccourcis à mettre à jour.",
    external_links_title: "Liens externes",
    external_links_description: "Mêmes options quand Edge est ouvert par une autre app.\r\nEx. : Discord, Teams, Terminal.",
    apply_to_external_links: "Appliquer aux liens externes",
    ok: "OK",
    cancel: "Annuler",

    standalone: "Options seules",
    enable_features: "Activer les fonctions :",
    disable_features: "Désactiver les fonctions :",
    apply: "Appliquer",
    close: "Fermer",

    info_executables: "EXÉCUTABLES",
    info_shortcuts: "RACCOURCIS",

    tip_advanced: "Avancé",
    tip_custom: "Personnalisé",
    tip_help: "Aide",
    tip_info: "Infos",
    tip_restore_default: "Rétablir les réglages normaux des raccourcis Edge",
    tip_external_links: "Appliquer les options quand d'autres apps ouvrent Edge",
    tip_example: "Exemple : {}",
    tip_language: "Langue : {}",

    report_selected: "Sélection : {}",
    report_selected_none: "Sélection : aucune",
    report_updated: "Mis à jour :",
    report_failed: "Échec :",
    report_missing: "Manquant :",
    report_found: "Trouvé :",
    report_registry: "Registre :",
    report_external_links: "Liens externes :",
    report_none_applied: "Aucune",
    report_selected_header: "Sélection :",
    report_candidates: "Candidats :",
    report_not_found: "Introuvable dans les emplacements habituels",
    report_exists: "Présent",
    report_status_missing: "Absent",
    report_env_missing: "Variable d'environnement absente",
    report_reference_only: "À titre indicatif et diagnostique uniquement.",
    report_env_not_set: "(variable d'environnement non définie)",
    report_latest_summary: "Dernier bilan : {} trouvés, {} mis à jour, {} manquants, {} échecs",
    report_no_operation: "Aucune opération n'a été exécutée dans cette session.",

    location_system_start_menu: "Menu Démarrer système",
    location_public_desktop: "Bureau public",
    location_pinned_taskbar: "Épinglé à la barre des tâches",
    location_pinned_start_menu: "Épinglé au menu Démarrer",
    location_implicit_pinned: "Raccourci épinglé implicite",
    location_quick_launch: "Lancement rapide",
    location_user_start_menu: "Menu Démarrer de l'utilisateur",
    location_user_desktop: "Bureau de l'utilisateur",
    location_generic: "Raccourci",

    error_font: "Impossible d'initialiser la police Segoe UI.",
    error_com: "Impossible d'initialiser COM.",
    error_ui: "Impossible de créer l'interface.",

    help_sections: &[
        HelpSection {
            title: "Ancienne méthode",
            body: &[
                "Utilise msFeatureGroupNewLookAndFeelHoldout.",
                "C'est l'ancienne méthode, plus générale, pour supprimer les coins arrondis."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Nouvelle méthode",
            body: &[
                "Utilise msForceNoRoundedCornerAndMargin.",
                "C'est la méthode plus récente et plus directe pour supprimer coins arrondis et marges."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Rétablir par défaut",
            body: &["Supprime toutes les options de ligne de commande gérées par cet outil et rétablit un lancement normal."],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Masquer le point de connexion",
            body: &[
                "Si coché, désactive aussi msShowSignInIndicator.",
                "Masque le point rouge de rappel de connexion sur l'icône de profil."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Restaurer la barre latérale",
            body: &[
                "Si coché, désactive aussi msHubAppsSidebarRetirement.",
                "Tente de restaurer l'ancienne liste d'applications de la barre latérale."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Désactiver les extensions",
            body: &[
                "Si coché, ajoute aussi --disable-extensions.",
                "Démarre Edge sans charger les extensions du navigateur."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Avancé",
            body: &[
                "Choisissez les noms de raccourcis à mettre à jour.",
                "Vous pouvez aussi appliquer des options quand Edge est ouvert par une autre app, comme Discord ou Teams."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Personnalisé",
            body: &[
                "Options seules, ex. : --disable-extensions --force-dark-mode --mute-audio",
                "Activer, ex. : msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "Désactiver, ex. : msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Infos",
            body: &[
                "Affiche les exécutables candidats et les emplacements des raccourcis choisis.",
                "À titre indicatif et diagnostique uniquement."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Rappel",
            body: &[
                "Désactivez d'abord l'amélioration du démarrage dans Edge. Copiez ceci dans la barre d'adresse : ",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

static ES: Strings = Strings {
    app_title: "Herramienta de accesos de Edge",
    advanced_window_title: "Avanzado",
    custom_window_title: "Personalizado",
    help_window_title: "Ayuda",
    info_window_title: "Información",

    preset_title: "Corregir esquinas",
    old_workaround: "Método antiguo",
    new_workaround: "Método nuevo",
    restore_default: "Restaurar valores",
    hide_sign_in: "Ocultar punto de sesión",
    restore_sidebar: "Restaurar barra lateral",
    disable_extensions: "Desactivar extensiones",
    exit: "Salir",
    status_ready: "Listo",
    status_applying: "Aplicando…",
    status_completed_with_errors: "Completado con errores",
    status_applied: "Aplicado: {}",
    status_enter_custom_flags: "Introduce los parámetros",

    shortcut_targets_title: "Accesos directos",
    shortcut_targets_description: "Elige qué nombres de acceso se actualizan.",
    external_links_title: "Vínculos externos",
    external_links_description: "Los mismos parámetros cuando otra aplicación abre Edge.\r\nPor ejemplo Discord, Teams o Terminal.",
    apply_to_external_links: "Aplicar a vínculos externos",
    ok: "OK",
    cancel: "Cancelar",

    standalone: "Parámetros sueltos",
    enable_features: "Activar funciones:",
    disable_features: "Desactivar funciones:",
    apply: "Aplicar",
    close: "Cerrar",

    info_executables: "EJECUTABLES",
    info_shortcuts: "ACCESOS",

    tip_advanced: "Avanzado",
    tip_custom: "Personalizado",
    tip_help: "Ayuda",
    tip_info: "Información",
    tip_restore_default: "Restaurar los ajustes normales del acceso de Edge",
    tip_external_links: "Aplicar parámetros cuando otras apps abren Edge",
    tip_example: "Ejemplo: {}",
    tip_language: "Idioma: {}",

    report_selected: "Seleccionado: {}",
    report_selected_none: "Seleccionado: ninguno",
    report_updated: "Actualizado:",
    report_failed: "Fallido:",
    report_missing: "Faltante:",
    report_found: "Encontrado:",
    report_registry: "Registro:",
    report_external_links: "Vínculos externos:",
    report_none_applied: "Ninguno aplicado",
    report_selected_header: "Seleccionado:",
    report_candidates: "Candidatos:",
    report_not_found: "No encontrado en las ubicaciones habituales",
    report_exists: "Existe",
    report_status_missing: "No existe",
    report_env_missing: "Falta la variable de entorno",
    report_reference_only: "Solo como referencia y diagnóstico.",
    report_env_not_set: "(variable de entorno no definida)",
    report_latest_summary: "Último resumen: {} encontrados, {} actualizados, {} faltantes, {} fallidos",
    report_no_operation: "No se ha ejecutado ninguna operación en esta sesión.",

    location_system_start_menu: "Menú Inicio del sistema",
    location_public_desktop: "Escritorio público",
    location_pinned_taskbar: "Anclado a la barra de tareas",
    location_pinned_start_menu: "Anclado al menú Inicio",
    location_implicit_pinned: "Acceso anclado implícito",
    location_quick_launch: "Inicio rápido",
    location_user_start_menu: "Menú Inicio del usuario",
    location_user_desktop: "Escritorio del usuario",
    location_generic: "Acceso directo",

    error_font: "No se pudo inicializar la fuente Segoe UI.",
    error_com: "No se pudo inicializar COM.",
    error_ui: "No se pudo crear la interfaz.",

    help_sections: &[
        HelpSection {
            title: "Método antiguo",
            body: &[
                "Usa msFeatureGroupNewLookAndFeelHoldout.",
                "Es el método antiguo y más general para quitar las esquinas redondeadas."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Método nuevo",
            body: &[
                "Usa msForceNoRoundedCornerAndMargin.",
                "Es el método más reciente y directo para quitar esquinas redondeadas y márgenes."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Restaurar valores",
            body: &["Borra todos los parámetros de línea de comandos gestionados por esta herramienta y restaura el inicio normal."],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Ocultar punto de sesión",
            body: &[
                "Si se marca, también desactiva msShowSignInIndicator.",
                "Oculta el punto rojo de recordatorio de sesión del icono de perfil."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Restaurar barra lateral",
            body: &[
                "Si se marca, también desactiva msHubAppsSidebarRetirement.",
                "Intenta restaurar la lista clásica de aplicaciones de la barra lateral."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Desactivar extensiones",
            body: &[
                "Si se marca, también añade --disable-extensions.",
                "Inicia Edge sin cargar las extensiones del navegador."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Avanzado",
            body: &[
                "Elige qué nombres de acceso directo se actualizan.",
                "También puedes aplicar parámetros cuando otra aplicación abre Edge, como Discord o Teams."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Personalizado",
            body: &[
                "Parámetros sueltos, ejemplo: --disable-extensions --force-dark-mode --mute-audio",
                "Activar, ejemplo: msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "Desactivar, ejemplo: msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Información",
            body: &[
                "Muestra los ejecutables candidatos y las ubicaciones de los accesos elegidos.",
                "Solo como referencia y diagnóstico."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "Recordatorio",
            body: &[
                "Desactiva primero el inicio rápido en Edge. Copia esto en la barra de direcciones de Edge: ",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

static KO: Strings = Strings {
    app_title: "Edge 바로 가기 도구",
    advanced_window_title: "고급",
    custom_window_title: "사용자 지정",
    help_window_title: "도움말",
    info_window_title: "정보",

    preset_title: "모서리 수정",
    old_workaround: "이전 방식",
    new_workaround: "새 방식",
    restore_default: "기본값 복원",
    hide_sign_in: "로그인 빨간 점 숨기기",
    restore_sidebar: "사이드바 복원",
    disable_extensions: "확장 프로그램 끄기",
    exit: "종료",
    status_ready: "준비됨",
    status_applying: "적용하는 중…",
    status_completed_with_errors: "오류와 함께 완료",
    status_applied: "적용됨: {}",
    status_enter_custom_flags: "사용자 지정 플래그를 입력하세요",

    shortcut_targets_title: "바로 가기 대상",
    shortcut_targets_description: "업데이트할 바로 가기 이름을 선택하세요.",
    external_links_title: "외부 링크",
    external_links_description: "다른 앱에서 Edge를 열 때도 같은 플래그를 사용합니다.\r\n예: Discord, Teams, 터미널.",
    apply_to_external_links: "외부 링크에도 적용",
    ok: "확인",
    cancel: "취소",

    standalone: "단독 플래그",
    enable_features: "기능 사용:",
    disable_features: "기능 사용 안 함:",
    apply: "적용",
    close: "닫기",

    info_executables: "실행 파일",
    info_shortcuts: "바로 가기",

    tip_advanced: "고급",
    tip_custom: "사용자 지정",
    tip_help: "도움말",
    tip_info: "정보",
    tip_restore_default: "Edge 바로 가기의 기본 설정으로 복원",
    tip_external_links: "다른 앱에서 Edge를 열 때도 플래그 적용",
    tip_example: "예: {}",
    tip_language: "언어: {}",

    report_selected: "선택됨: {}",
    report_selected_none: "선택됨: 없음",
    report_updated: "업데이트됨:",
    report_failed: "실패:",
    report_missing: "누락:",
    report_found: "찾음:",
    report_registry: "레지스트리:",
    report_external_links: "외부 링크:",
    report_none_applied: "적용 안 됨",
    report_selected_header: "선택됨:",
    report_candidates: "후보:",
    report_not_found: "일반적인 설치 위치에서 찾을 수 없음",
    report_exists: "있음",
    report_status_missing: "없음",
    report_env_missing: "환경 변수 없음",
    report_reference_only: "참고 및 진단용입니다.",
    report_env_not_set: "(환경 변수가 설정되지 않음)",
    report_latest_summary: "최근 결과: 찾음 {}개, 업데이트 {}개, 누락 {}개, 실패 {}개",
    report_no_operation: "이 세션에서는 아직 아무 작업도 실행되지 않았습니다.",

    location_system_start_menu: "시스템 시작 메뉴",
    location_public_desktop: "공용 바탕 화면",
    location_pinned_taskbar: "작업 표시줄 고정",
    location_pinned_start_menu: "시작 메뉴 고정",
    location_implicit_pinned: "암시적 고정 바로 가기",
    location_quick_launch: "빠른 실행",
    location_user_start_menu: "현재 사용자 시작 메뉴",
    location_user_desktop: "현재 사용자 바탕 화면",
    location_generic: "바로 가기",

    error_font: "Segoe UI 글꼴을 초기화하지 못했습니다.",
    error_com: "COM을 초기화하지 못했습니다.",
    error_ui: "사용자 인터페이스를 만들지 못했습니다.",

    help_sections: &[
        HelpSection {
            title: "이전 방식",
            body: &[
                "msFeatureGroupNewLookAndFeelHoldout을 사용합니다.",
                "둥근 모서리 표시를 없애는 예전의 폭넓은 방법입니다."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "새 방식",
            body: &[
                "msForceNoRoundedCornerAndMargin을 사용합니다.",
                "둥근 모서리와 여백을 없애는 더 새롭고 직접적인 방법입니다."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "기본값 복원",
            body: &["이 도구가 관리하는 모든 명령줄 플래그를 지우고 정상 시작 동작으로 되돌립니다."],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "로그인 빨간 점 숨기기",
            body: &[
                "선택하면 msShowSignInIndicator도 사용 안 함으로 설정합니다.",
                "프로필 아이콘의 빨간 로그인 알림 점을 숨깁니다."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "사이드바 복원",
            body: &[
                "선택하면 msHubAppsSidebarRetirement도 사용 안 함으로 설정합니다.",
                "예전 사이드바 앱 목록 복원을 시도합니다."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "확장 프로그램 끄기",
            body: &[
                "선택하면 --disable-extensions도 추가합니다.",
                "브라우저 확장 프로그램을 불러오지 않고 Edge를 시작합니다."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "고급",
            body: &[
                "업데이트할 바로 가기 이름을 선택합니다.",
                "Discord나 Teams 같은 다른 앱에서 Edge를 열 때도 플래그를 적용할 수 있습니다."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "사용자 지정",
            body: &[
                "단독 플래그 예: --disable-extensions --force-dark-mode --mute-audio",
                "기능 사용 예: msForceNoRoundedCornerAndMargin,msUndersideButton,ParallelDownloading",
                "기능 사용 안 함 예: msShowSignInIndicator,msHubAppsSidebarRetirement,MediaRouter"
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "정보",
            body: &[
                "실행 파일 후보와 선택한 바로 가기 위치를 표시합니다.",
                "참고 및 진단용입니다."
            ],
            reminder: NORMAL_SECTION
        },
        HelpSection {
            title: "알림",
            body: &[
                "먼저 Edge에서 시작 부스트를 끄세요. 다음 주소를 Edge 주소 표시줄에 붙여 넣으세요: ",
                "edge://settings/system/manageSystem"
            ],
            reminder: REMINDER_SECTION
        }
    ]
};

pub fn strings(language: Language) -> &'static Strings {
    match language {
        Language::English => &EN,
        Language::SimplifiedChinese => &ZH_CN,
        Language::TraditionalChinese => &ZH_TW,
        Language::Japanese => &JA,
        Language::German => &DE,
        Language::French => &FR,
        Language::Spanish => &ES,
        Language::Korean => &KO
    }
}

/// Replaces the `{}` placeholders in a template, left to right.
pub fn fill(template: &str, values: &[&str]) -> String {
    let mut result = String::with_capacity(template.len() + 32);
    let mut rest = template;

    for value in values {
        match rest.find("{}") {
            Some(index) => {
                result.push_str(&rest[..index]);
                result.push_str(value);
                rest = &rest[index + 2..];
            }
            None => break
        }
    }

    result.push_str(rest);
    result
}

pub struct HelpTitleSpan {
    pub start: usize,
    pub end: usize,
    pub reminder: bool
}

pub struct RenderedHelp {
    pub text: String,
    pub titles: Vec<HelpTitleSpan>
}

impl RenderedHelp {
    fn push_section(&mut self, section: &HelpSection) {
        // Blank line between blocks, matching the original help layout.
        if !self.text.is_empty() {
            self.text.push_str("\r\n");
        }

        let start = self.text.len();
        self.text.push_str(section.title);
        self.titles.push(HelpTitleSpan {
            start,
            end: self.text.len(),
            reminder: section.reminder
        });
        self.text.push_str("\r\n");

        for line in section.body {
            self.text.push_str(line);
            self.text.push_str("\r\n");
        }
    }
}

/// Builds the Help window text together with the byte ranges that have to be
/// rendered as headings, so formatting never relies on searching for words that
/// could also appear in the body of another block.
pub fn render_help(language: Language) -> RenderedHelp {
    let table = strings(language);
    let mut rendered = RenderedHelp {
        text: String::new(),
        titles: Vec::new()
    };

    for section in table.help_sections {
        rendered.push_section(section);
    }

    // The original text ends on the last body line, without a trailing break.
    while rendered.text.ends_with("\r\n") {
        rendered.text.truncate(rendered.text.len() - 2);
    }

    rendered
}
