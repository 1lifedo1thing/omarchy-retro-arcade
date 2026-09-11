use eframe::egui::{self, Event, PointerButton, Pos2, Rect, Vec2};
use omarchy_chess::{
    engine::Answer,
    game::{Game, Mode},
    ui::{square_at, square_rect, ChessApp},
};
use shakmaty::Square;
fn frame(app: &mut ChessApp, ctx: &egui::Context, events: Vec<Event>) -> egui::FullOutput {
    ctx.run(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(1060., 780.))),
            events,
            ..Default::default()
        },
        |ctx| app.draw(ctx),
    )
}
fn click(app: &mut ChessApp, ctx: &egui::Context, pos: Pos2) {
    frame(
        app,
        ctx,
        vec![
            Event::PointerMoved(pos),
            Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            },
        ],
    );
    frame(
        app,
        ctx,
        vec![Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        }],
    );
}
#[test]
fn orientation_mapping() {
    let rect = Rect::from_min_size(Pos2::new(20., 20.), Vec2::splat(480.));
    for flipped in [false, true] {
        for square in Square::ALL {
            assert_eq!(
                square_at(rect, square_rect(rect, square, flipped).center(), flipped),
                Some(square)
            );
        }
    }
    assert!(square_at(rect, Pos2::ZERO, false).is_none());
}
#[test]
fn native_widget_click_move_and_history_protection() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = ChessApp::new(dir.path().into());
    app.game.mode = Mode::Local;
    let ctx = egui::Context::default();
    egui_extras::install_image_loaders(&ctx);
    frame(&mut app, &ctx, vec![]);
    frame(&mut app, &ctx, vec![]);
    let board = app.board_rect.unwrap();
    click(
        &mut app,
        &ctx,
        square_rect(board, Square::E2, false).center(),
    );
    assert_eq!(app.selected, Some(Square::E2));
    click(
        &mut app,
        &ctx,
        square_rect(board, Square::E4, false).center(),
    );
    assert_eq!(app.game.moves.len(), 1);
    app.preview = Some(0);
    let before = app.game.fen();
    let m = app.game.parse_move("e5").unwrap();
    app.accept_move(m);
    assert_eq!(before, app.game.fen());
}
#[test]
fn stale_engine_result_ignored() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = ChessApp::new(dir.path().into());
    let before = app.game.fen();
    app.accept_answer(Answer {
        revision: 99,
        hint: false,
        result: Ok("e2e4".into()),
    });
    assert_eq!(app.game.fen(), before);
}
#[test]
fn replacing_game_archives_previous_game() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = ChessApp::new(dir.path().into());
    app.game.mode = Mode::Local;
    let m = app.game.parse_move("e4").unwrap();
    app.accept_move(m);
    app.replace_game(Game {
        mode: Mode::Local,
        ..Game::default()
    })
    .unwrap();
    assert_eq!(
        std::fs::read_dir(dir.path().join("archive"))
            .unwrap()
            .count(),
        1
    );
    assert!(app.game.moves.is_empty());
}
#[test]
fn compact_native_layout_produces_board() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = ChessApp::new(dir.path().into());
    app.game.mode = Mode::Local;
    let ctx = egui::Context::default();
    egui_extras::install_image_loaders(&ctx);
    let output = ctx.run(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(740., 560.))),
            ..Default::default()
        },
        |ctx| app.draw(ctx),
    );
    assert!(app.board_rect.unwrap().width() >= 300.);
    assert!(!output.shapes.is_empty());
}

#[test]
fn drag_uses_pressed_square_even_when_first_motion_crosses_square() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = ChessApp::new(dir.path().into());
    app.game.mode = Mode::Local;
    let ctx = egui::Context::default();
    egui_extras::install_image_loaders(&ctx);
    frame(&mut app, &ctx, vec![]);
    frame(&mut app, &ctx, vec![]);
    let board = app.board_rect.unwrap();
    let from = square_rect(board, Square::E2, false).center();
    let to = square_rect(board, Square::E4, false).center();
    frame(
        &mut app,
        &ctx,
        vec![
            Event::PointerMoved(from),
            Event::PointerButton {
                pos: from,
                button: PointerButton::Primary,
                pressed: true,
                modifiers: Default::default(),
            },
        ],
    );
    frame(&mut app, &ctx, vec![Event::PointerMoved(to)]);
    frame(
        &mut app,
        &ctx,
        vec![Event::PointerButton {
            pos: to,
            button: PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        }],
    );
    assert_eq!(app.game.notation, vec!["1.  e4"]);
}

#[test]
fn focused_board_keyboard_can_complete_move() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = ChessApp::new(dir.path().into());
    app.game.mode = Mode::Local;
    let ctx = egui::Context::default();
    egui_extras::install_image_loaders(&ctx);
    frame(&mut app, &ctx, vec![]);
    frame(&mut app, &ctx, vec![]);
    let board = app.board_rect.unwrap();
    click(
        &mut app,
        &ctx,
        square_rect(board, Square::E2, false).center(),
    );
    frame(&mut app, &ctx, vec![]);
    for key in [egui::Key::ArrowUp, egui::Key::ArrowUp, egui::Key::Enter] {
        frame(
            &mut app,
            &ctx,
            vec![Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: Default::default(),
            }],
        );
        frame(
            &mut app,
            &ctx,
            vec![Event::Key {
                key,
                physical_key: None,
                pressed: false,
                repeat: false,
                modifiers: Default::default(),
            }],
        );
    }
    assert_eq!(app.game.notation, vec!["1.  e4"]);
}
