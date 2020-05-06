
use nix::sys::termios;

use std::io::{Read, Write};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Instant, Duration};
use std::borrow::Borrow;

mod board;
mod block;
mod blocks;
mod generator;

use board::{Board, BlockPosition};
use block::Block;
use generator::BlockGenerator;

const TICK_DURATION: u64 = 500;

#[derive(Debug)]
enum Update {
    Down,
    Left,
    Right,
    RotateLeft,
    RotateRight,
    Tick,
}

struct GameState {
    board: Board,
    block: Box<dyn Block>,
    block_position: BlockPosition,
    block_color: u8,
    remaining_input_time: Duration,
    generator: BlockGenerator,
    end: bool,
}

fn redrawn(state: GameState) -> GameState {
    std::io::stdout().write("\x1b[A".repeat(state.board.height() + 2).as_bytes()).unwrap();
    drawn(state)
}

fn drawn(state: GameState) -> GameState {
    let merged = state.board.with_block(
        state.block.borrow(),
        state.block_position,
        state.block_color
    );
    std::io::stdout().write(merged.render().as_bytes()).unwrap();
    state
}

fn make_update_receiver() -> std::sync::mpsc::Receiver<Update> {
    let (input_tx, input_rx) = channel::<Update>();

    thread::spawn(move || {
        let _orig_term = termios::tcgetattr(0).unwrap();
        let mut term = termios::tcgetattr(0).unwrap();

        // Unset canonical mode, so we get characters immediately
        term.local_flags.remove(termios::LocalFlags::ICANON);

        // Disable local echo
        term.local_flags.remove(termios::LocalFlags::ECHO);

        termios::tcsetattr(0, termios::SetArg::TCSADRAIN, &term).unwrap();

        loop {
            for byte_result in std::io::stdin().bytes() {
                if let Ok(byte) = byte_result {
                    match byte as char {
                        'a' => { input_tx.send(Update::Left).unwrap(); },
                        'd' => { input_tx.send(Update::Right).unwrap(); },
                        's' => { input_tx.send(Update::Down).unwrap(); },
                        'q' => { input_tx.send(Update::RotateLeft).unwrap(); },
                        'e' => { input_tx.send(Update::RotateRight).unwrap(); },
                        _ => {},
                    }
                }
            }
            thread::sleep(Duration::from_millis(20));
        }
    });

    input_rx
}

fn remaining_time(total: Duration, passed: Duration) -> Duration {
    total.checked_sub(passed).unwrap_or(Duration::from_millis(0))
}

fn main() {
    let update_receiver = make_update_receiver();

    let initial_board = Board::new(10, 20);
    let initial_block = BlockGenerator::new().next();
    let initial_block_position = BlockPosition {
        x: (initial_board.width() as i32 - initial_block.block.width() as i32) / 2,
        y: 0,
    };

    let mut state = drawn(GameState {
        board: initial_board,
        block: initial_block.block,
        block_color: initial_block.block_color,
        block_position: initial_block_position,
        remaining_input_time: Duration::from_millis(TICK_DURATION),
        generator: initial_block.generator,
        end: false,
    });

    while !state.end {
        let time = Instant::now();
        let update = if state.remaining_input_time > Duration::from_millis(0) {
            update_receiver.recv_timeout(state.remaining_input_time).unwrap_or(Update::Tick)
        } else {
            Update::Tick
        };

        state = redrawn(match update {
            Update::Left => {
                let tested_position = BlockPosition {
                    x: state.block_position.x - 1,
                    ..state.block_position
                };
                let can_move_left = !state.board.block_collides(state.block.borrow(), tested_position);

                GameState {
                    block_position: if can_move_left { tested_position } else { state.block_position },
                    remaining_input_time: remaining_time(state.remaining_input_time, time.elapsed()),
                    ..state
                }
            }
            Update::Right => {
                let tested_position = BlockPosition {
                    x: state.block_position.x + 1,
                    ..state.block_position
                };
                let can_move_right = !state.board.block_collides(state.block.borrow(), tested_position);

                GameState {
                    block_position: if can_move_right { tested_position } else { state.block_position },
                    remaining_input_time: remaining_time(state.remaining_input_time, time.elapsed()),
                    ..state
                }
            }
            Update::Down => {
                let tested_position = BlockPosition {
                    y: state.block_position.y + 1,
                    ..state.block_position
                };
                let can_move_down = !state.board.block_collides(state.block.borrow(), tested_position);

                if can_move_down {
                    GameState {
                        block_position: tested_position,
                        remaining_input_time: Duration::from_millis(TICK_DURATION),
                        ..state
                    }
                } else {
                    GameState {
                        remaining_input_time: remaining_time(state.remaining_input_time, time.elapsed()),
                        ..state
                    }
                }
            }
            Update::RotateLeft => {
                let rotated_block = state.block.rotate_left();
                let can_rotate_left = !state.board.block_collides(rotated_block.borrow(), state.block_position);

                GameState {
                    block: if can_rotate_left { rotated_block } else { state.block },
                    remaining_input_time: remaining_time(state.remaining_input_time, time.elapsed()),
                    ..state
                }
            }
            Update::RotateRight => {
                let rotated_block = state.block.rotate_right();
                let can_rotate_right = !state.board.block_collides(rotated_block.borrow(), state.block_position);

                GameState {
                    block: if can_rotate_right { rotated_block } else { state.block },
                    remaining_input_time: remaining_time(state.remaining_input_time, time.elapsed()),
                    ..state
                }
            }
            Update::Tick => {
                let tested_position = BlockPosition {
                    y: state.block_position.y + 1,
                    ..state.block_position
                };
                let can_be_moved_down = !state.board.block_collides(state.block.borrow(), tested_position);

                if can_be_moved_down {
                    GameState {
                        block_position: tested_position,
                        remaining_input_time: Duration::from_millis(TICK_DURATION),
                        ..state
                    }
                } else {
                    let merged_board = state.board.with_block(
                        state.block.borrow(),
                        state.block_position,
                        state.block_color,
                    );
                    let compressed_board = merged_board.compress();
                    let next_block = state.generator.next();
                    let new_position = BlockPosition {
                        x: (state.board.width() as i32 - next_block.block.width() as i32) / 2,
                        y: 0,
                    };

                    GameState {
                        end: compressed_board.block_collides(next_block.block.borrow(), new_position),
                        board: compressed_board,
                        block: next_block.block,
                        block_color: next_block.block_color,
                        block_position: new_position,
                        remaining_input_time: Duration::from_millis(TICK_DURATION),
                        generator: next_block.generator,
                    }
                }
            }
        });
    }
}
