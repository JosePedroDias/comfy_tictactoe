use std::process::exit;

use comfy::*;

mod board;
use board::{Marker, new_board, get_other_player, who_won, is_full};

const TH : f32 = 0.08;

struct State {
    pub board: board::Board,
    pub is_game_finished: bool,
    pub winner: Marker,
}

static STATE: Lazy<AtomicRefCell<State>> = Lazy::new(|| {
    AtomicRefCell::new(State {
        board: new_board(),
        is_game_finished: false,
        winner: Marker::NA,
    })
});

simple_game!("tictactoe", setup, update);

fn setup(c: &mut EngineContext) {
    c.load_fonts_from_bytes(&[(
        "uni",
        include_bytes!("../assets/univers-light-normal.ttf"),
    )]);
    
    let mut cam = main_camera_mut();
    cam.zoom /= 4.0;
}

fn draw_o(ctr: &Vec2) {
    draw_circle_outline(*ctr, 0.4, TH, RED, 0);
}

fn draw_x(ctr: &Vec2) {
    draw_line(
        vec2(-0.4 + ctr.x, -0.4 + ctr.y),
        vec2(0.4 + ctr.x, 0.4 + ctr.y),
        TH,
        GREEN,
        0,
    );
    draw_line(
        vec2(0.4 + ctr.x, -0.4 + ctr.y),
        vec2(-0.4 + ctr.x, 0.4 + ctr.y),
        TH,
        GREEN,
        0,
    );
}

fn update(_c: &mut EngineContext) {
    if is_key_down(KeyCode::Escape) {
        exit(0); // TODO
    }
    
    let mut state = STATE.borrow_mut();
    
    if !state.is_game_finished {
        let down: bool = is_mouse_button_pressed(MouseButton::Left);
        if down {
            let pos = mouse_world();
            let x: i32 = (pos.x + 1.5).floor() as i32;
            let y: i32 = (pos.y + 1.5).floor() as i32;
            if x < 0 || x > 2 || y < 0 || y > 2 {return; }
            // println!("mouse click: {pos} {x}{y}");
            
            let i: usize = (x + y * 3) as usize;
            if state.board.spaces[i] == Marker::NA {
                // lets add another marker to the board...
                state.board.spaces[i] = state.board.current_player;
                // println!("{}", state.board);
                
                // has anyone won?
                if let Some(winner) = who_won(&state.board) {
                    state.winner = winner;
                    state.is_game_finished = true;
                    println!("{} won!", state.winner);
                } else if is_full(&state.board) {
                    state.is_game_finished = true;
                    println!("it's a draw!");
                }
                
                state.board.current_player = get_other_player(state.board.current_player);
            }
        }
    }
    
    // draw hash grid
    for i in 0..2 {
        let f = if i == 0 { 0.0 } else { 1.0 };
        draw_line(vec2(-0.5+f, -1.5), vec2(-0.5+f, 1.5), TH, BLUE, 1);
        draw_line(vec2(-1.5, -0.5+f), vec2(1.5, -0.5+f), TH, BLUE, 1);
    }
    
    // draw markers
    for y in 0..3 {
        for x in 0..3 {
            let i: usize = (x + y * 3) as usize;
            let center = vec2(
                x as f32 - 1.0,
                y as f32 - 1.0,
            );
            let marker = state.board.spaces[i];
            if marker == Marker::X {
                draw_x(&center);
            } else if marker == Marker::O {
                draw_o(&center);
            }
        }
    }
    
    // draw text overlay
    if state.is_game_finished {
        let mut label = String::from("");
        if state.winner == Marker::NA {
            label.push_str("it's a draw!");
        } else {
            label = format!("{} won!", state.winner);
        };
        
        //draw_text(label.as_str(), vec2(0.0, 0.0), WHITE, TextAlign::Center);
        draw_text_ex(
            label.as_str(),
            vec2(0.0, 0.0),
            TextAlign::Center,
            TextParams {
                color: WHITE,
                font: egui::FontId::new(
                    64.0,
                    egui::FontFamily::Name("uni".into()),
                ),
            ..Default::default()
        });
    }
}
