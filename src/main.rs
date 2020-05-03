extern crate gtk;
extern crate gio;
extern crate gdk;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;

use std::env;

const SQUARE_SIZE: i32 = 100;
const WIDTH: i32 = 4;
const HEIGHT: i32 = 4;

static WRAP_VALUE: u32 = 1000;

// the inspiration of this came from someone called neophob on GitHub. here is the link to his
// older arduino code: https://github.com/neophob/ExpeditInvaders
//
// futher steps:
//
// add buffer for 3-tuple rgb values
// map buttons to select between of any of these 3-tuple
//
// add different animations based on selecting a value representing a linear combination of any
// 2-tuple of the selected 3-tuple. one can look imagine this like the graphic below.
// this image represents 3 points on a color circle. the lines represent the range different
// linear combinations of any 2-tuple.
//
//       *
//      / \
//     /   \
//    /     \
//   *-------*
//
// map buttons to select any of the animations
//
// map buttons for selecting the speed of the animation.

macro_rules! pos_from_array_idx {
    ($x:expr, $y:expr) => {
        $y * WIDTH + match ($x, $y) {
            (x, y) if y % 2 == 0 => x,
            (x, _) => WIDTH - x - 1,
        }
    }
}

macro_rules! mix_color {
    ($a:expr, $b:expr, $offset:expr) => {
        ($b * $offset as f64 + $a * (85 - $offset) as f64) / 85.0
    }
}

macro_rules! convert_hex {
    ($v:expr) => {
        $v as f64 / 255.0
    }
}

macro_rules! RGB {
    ($r:expr, $g:expr, $b:expr) => {
        gdk::RGBA { red: convert_hex!($r), green: convert_hex!($g), blue: convert_hex!($b), alpha: 1.0 }
    }
}

static mut mode: u32 = 1;
static mut color_mode: usize = 0;

static colors: [[gdk::RGBA; 3]; 9] = [
    [
        RGB!(0xdc, 0x32, 0x3c),
        RGB!(0xf0, 0xcb, 0x58),
        RGB!(0x3c, 0x82, 0x5e),
    ],
    [
        RGB!(0xd3, 0x51, 0x7d),
        RGB!(0x15, 0xa0, 0xbf),
        RGB!(0xff, 0xc0, 0x62),
    ],
    [
        RGB!(0x00, 0x8c, 0x53),
        RGB!(0x2e, 0x00, 0xe4),
        RGB!(0xdf, 0xea, 0x00),
    ],
    [
        RGB!(0x58, 0x8F, 0x27),
        RGB!(0x04, 0xBF, 0xBF),
        RGB!(0xF7, 0xE9, 0x67),
    ],
    [
        RGB!(0x9f, 0x45, 0x6b),
        RGB!(0x47, 0x7a, 0x9a),
        RGB!(0xe6, 0xc8, 0x4c),
    ],
    [
        RGB!(0x32, 0x32, 0x28),
        RGB!(0x71, 0x71, 0x55),
        RGB!(0xb4, 0xdc, 0x00),
    ],
    [
        RGB!(0x00, 0x00, 0x00),
        RGB!(0x0d, 0x9a, 0x0d),
        RGB!(0xff, 0xff, 0xff),
    ],
    [
        RGB!(0x00, 0x00, 0xff),
        RGB!(0x00, 0xff, 0x00),
        RGB!(0xff, 0xff, 0xff),
    ],
    [
        RGB!(0x3e, 0x3e, 0x3e),
        RGB!(0xd4, 0xb6, 0x00),
        RGB!(0xff, 0xff, 0xff),
    ],
];

fn tick(grid: &gtk::Grid) -> gtk::prelude::Continue {
    static mut cntr: u32 = 0;

    unsafe {
        if cntr >= WRAP_VALUE {
            cntr = 0;
        }
    };

    if unsafe { cntr % 60 } == 0 {
        println!("tick {}", unsafe { cntr });
    }

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let pos = pos_from_array_idx!(i, j) as u8;

            let offset: u8 = (match unsafe { mode } {
                // random buffer. TODO: this should be created where mode is switched.
                // for now this is not implemented
                1 => { 0 }
                // ordered color animation
                2 => { pos }
                // solid color
                3 => { 0 }
                // checkerboard
                4 => { 2^7 * (pos % 2) }
                5 => { 2^5 * (pos % 4) }
                6 => { 2^4 * (pos % 8) }
                _ => { 0 }
            } as u16 + (unsafe { cntr as f64 } / WRAP_VALUE as f64 * 255.0) as u16) as u8;

            let buffer_offset = (offset - offset % 85) / 85;
            let wheel_offset = offset % 85;

            // get the linear combination of the color at offsets offset%arrayCount and (offset + 1)%arrayCount
            let a = colors[unsafe { color_mode }][(buffer_offset%3) as usize];
            let b = colors[unsafe { color_mode }][((buffer_offset+1)%3) as usize];
            
            grid.get_child_at(i, j).unwrap()
                .override_background_color(gtk::StateFlags::NORMAL,
                                           Some(&gdk::RGBA { red: mix_color!(a.red, b.red, wheel_offset),
                                                             green: mix_color!(a.green, b.green, wheel_offset),
                                                             blue: mix_color!(a.blue, b.blue, wheel_offset),
                                                             alpha: 1.0 }));
        }
    }

    // test if the upper left thing will fade with green
    // grid.get_child_at(0, 0).unwrap()
    //     .override_background_color(gtk::StateFlags::NORMAL,
    //                                Some(&gdk::RGBA { red: 0.0, green: 1.0, blue: 0.0, alpha: unsafe { cntr as f64 } / 1000.0 }));

    unsafe { cntr += 1 };

    Continue(true)
}


fn main() {
    let uiapp = gtk::Application::new(Some("men.arkom.ikea-shelf-light-animator"),
    gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);

        win.set_default_size(WIDTH * SQUARE_SIZE, HEIGHT * SQUARE_SIZE);
        win.set_title("ikea-shelf-light-animator");

        let grid = gtk::Grid::new();

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                let number = pos_from_array_idx!(i, j);
                let square = gtk::BoxBuilder::new().build();
                square.set_property_width_request(SQUARE_SIZE);
                square.set_property_height_request(SQUARE_SIZE);
                square.override_background_color(gtk::StateFlags::NORMAL,
                                                 Some(&gdk::RGBA { red: 1.0, green: 0.0, blue: 0.0, alpha: number as f64 / (WIDTH * HEIGHT) as f64 }));
                grid.attach(&square, i, j, 1, 1);
            }
        }

        win.add(&grid);

        win.connect_key_press_event(
            move |_, key| {
                let keyval = key.get_keyval();

                match keyval {
                    v if v >= 49 && v <= 57 => {
                        unsafe { mode = v - 48 };
                        println!("N{}", v - 48);
                    }
                    43 => {
                        println!("faster");
                    }
                    45 => {
                        println!("slower");
                    }
                    32 => {
                        println!("pause");
                    }
                    v if v >= 65470 && v <= 65481 => {
                        unsafe { color_mode = (v - 65470) as usize };
                        println!("F{}", v - 65469);
                    }
                    _ => {}
                }

                // println!("key pressed: {}", keyval);

                Inhibit(false)
            },
        );

        win.show_all();

        gtk::timeout_add((1000.0 / 60.0) as _, move || {
            tick(&grid)
        });
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
