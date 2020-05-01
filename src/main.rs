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
                let number = j * WIDTH + match (i, j) {
                    (x, y) if y % 2 == 0 => x,
                    (x, _) => WIDTH - x - 1,
                };
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
                        println!("F{}", v - 65469);
                    }
                    _ => {}
                }

                // println!("key pressed: {}", keyval);

                Inhibit(false)
            },
        );

        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
