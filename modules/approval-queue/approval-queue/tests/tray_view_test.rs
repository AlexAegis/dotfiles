use approval_queue::{
    alert_settings::{AlertSettings, CadencePreset},
    tray_view::{EntryKind, TrayView},
};

#[test]
fn should_take_only_the_question_tool_as_a_question() {
    assert_eq!(EntryKind::of_tool("AskUserQuestion"), EntryKind::Question);
    assert_eq!(EntryKind::of_tool("Bash"), EntryKind::Permission);
    assert_eq!(EntryKind::of_tool(""), EntryKind::Permission);
}

#[test]
fn should_show_the_empty_icon_while_nothing_waits() {
    assert_eq!(TrayView::default().icon_name(), "approval-queue-empty");
}

#[test]
fn should_show_the_question_icon_while_only_questions_wait() {
    let view = TrayView {
        questions: 2,
        ..TrayView::default()
    };
    assert_eq!(view.icon_name(), "approval-queue-question");
}

#[test]
fn should_show_the_permission_icon_before_the_question_icon() {
    let view = TrayView {
        permissions: 1,
        questions: 2,
        ..TrayView::default()
    };
    assert_eq!(view.icon_name(), "approval-queue-permission");
}

#[test]
fn should_show_the_flash_icon_before_each_other_icon() {
    let view = TrayView {
        permissions: 1,
        is_flashing: true,
        ..TrayView::default()
    };
    assert_eq!(view.icon_name(), "approval-queue-flash");
}

#[test]
fn should_need_attention_only_while_a_prompt_waits_or_a_ping_shows() {
    assert!(!TrayView::default().needs_attention());
    let views = [
        TrayView {
            permissions: 1,
            ..TrayView::default()
        },
        TrayView {
            questions: 1,
            ..TrayView::default()
        },
        TrayView {
            is_flashing: true,
            ..TrayView::default()
        },
    ];
    assert!(views.iter().all(TrayView::needs_attention));
    let idle = TrayView {
        idle_sessions: 1,
        ..TrayView::default()
    };
    assert!(!idle.needs_attention());
    assert_eq!(idle.icon_name(), "approval-queue-idle");
}

#[test]
fn should_show_the_idle_icon_only_while_no_prompt_waits() {
    let idle = TrayView {
        idle_sessions: 1,
        ..TrayView::default()
    };
    assert_eq!(idle.icon_name(), "approval-queue-idle");
    assert_eq!(idle.summary(), "idle sessions: 1");
    let with_question = TrayView {
        questions: 1,
        ..idle
    };
    assert_eq!(with_question.icon_name(), "approval-queue-question");
}

#[test]
fn should_name_what_waits_the_mute_and_the_high_alert_in_the_summary() {
    assert_eq!(TrayView::default().summary(), "nothing waits");
    let view = TrayView {
        permissions: 3,
        questions: 1,
        idle_sessions: 2,
        is_flashing: false,
        muted_minutes: Some(7),
        settings: AlertSettings {
            high_alert: true,
            cadence: CadencePreset::Normal,
        },
    };
    assert_eq!(
        view.summary(),
        "permission prompts: 3, questions: 1, idle sessions: 2, pings muted for 7 min, high alert"
    );
}
