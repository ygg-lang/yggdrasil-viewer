#![allow(dead_code)]

extern crate pancurses;
extern crate reingold_tilford;

pub fn display<L, N, T>(
    tree: &T,
    root: N,
    layout: &std::collections::HashMap<T::Key, reingold_tilford::Coordinate>,
    labeller: L,
) where
    L: std::panic::RefUnwindSafe + Fn(&T, N) -> String,
    N: Copy + std::panic::RefUnwindSafe,
    T: std::panic::RefUnwindSafe + reingold_tilford::NodeInfo<N>,
    T::Key: std::panic::RefUnwindSafe,
{
    let window = pancurses::initscr();

    eprintln!(
        "{:?}",
        std::panic::catch_unwind(|| {
            let mut queue = std::collections::VecDeque::from(vec![root]);
            while let Some(node) = queue.pop_front() {
                let children = tree.children(node).into_iter().collect::<Vec<_>>();

                let coord = layout.get(&tree.key(node)).unwrap();
                let (x, y) = (coord.x as i32, coord.y as i32);

                let dimensions = tree.dimensions(node);
                let border = tree.border(node);

                assert_eq!(dimensions.top, 0.5);
                assert_eq!(dimensions.bottom, 0.5);

                for i in 0..border.top as i32 {
                    window.mv(y - 1 - i, x);
                    window.addch('|' /* '\u{2502}' */);
                }

                window.mv(y, x - dimensions.left as i32);
                window.addstr(labeller(tree, node));

                if children.len() == 1 {
                    // + 1 for the row of horizontal connectors.
                    for i in 0..border.bottom as i32 + 1 {
                        window.mv(y + 1 + i, x);
                        window.addch('|' /* '\u{2502}' */);
                    }
                }
                else if children.len() > 1 {
                    for i in 0..border.bottom as i32 {
                        window.mv(y + 1 + i, x);
                        window.addch('|' /* '\u{2502}' */);
                    }

                    let mut current_x = layout.get(&tree.key(children[0])).unwrap().x as i32;

                    for child in &children {
                        let this_x = layout.get(&tree.key(*child)).unwrap().x as i32;

                        for i in (current_x + 1)..this_x {
                            window.mv(y + 1 + border.bottom as i32, i);
                            window.addch('-' /* '\u{2500}' */);
                        }

                        window.mv(y + 1 + border.bottom as i32, this_x);
                        window.addch('+' /* if this_x == x { '\u{253C}' } else { '\u{252C}' } */);

                        current_x = this_x;
                    }

                    let final_x = layout.get(&tree.key(*children.last().unwrap())).unwrap().x as i32;

                    window.mv(y + 1 + border.bottom as i32, final_x);
                    window.addch('+' /* '\u{2510}' */);
                }

                queue.extend(children);
            }
        })
    );

    window.refresh();
    window.getch();
    pancurses::endwin();
}
