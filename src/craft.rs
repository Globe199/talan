use crate::macros;
use crate::task::Task;
use crate::ui;
use std::io::{stdout, Write};
// Runs through the set of tasks
// TODO: make it actually run more than one task
pub fn craft_items(window: ui::WinHandle, tasks: &[Task]) {
    for task in tasks {
        clear_windows(window);
        if task.collectable {
            toggle_collectable(window);
        }

        // Bring up the crafting window itself and give it time to appear
        ui::open_craft_window(window);
        ui::wait_secs(1);

        // Navigate to the correct recipe based on the index provided
        select_recipe(window, &task);
        // Time to craft the items
        execute_task(window, &task);

        clear_windows(window);
        if task.collectable {
            toggle_collectable(window);
        }
    }
}

fn clear_windows(window: ui::WinHandle) {
    println!("clearing window...");
    // Hitting escape closes one window each. 10 is excessive, but conservative
    for _ in 0..10 {
        ui::escape(window);
    }

    // Cancelling twice will close the System menu if it is open
    ui::cancel(window);
    ui::cancel(window);
    ui::wait_secs(1);
}

fn select_recipe(window: ui::WinHandle, task: &Task) {
    println!("selecting recipe...");
    // Loop backward through the UI 9 times to ensure we hit the text box
    // no matter what crafting class we are. The text input boxes are strangely
    // modal so that if we select them at any point they will hold on to focus
    // for characters.
    for _ in 0..9 {
        ui::move_backward(window);
    }

    ui::confirm(window);
    send_string(window, &task.item_name);
    ui::wait_ms(200);
    ui::enter(window);

    // It takse up to a second for results to populate
    ui::wait_secs(1);

    // Navigate to the offset we need
    for _ in 0..task.index {
        ui::cursor_down(window);
    }

    // Select the recipe to get to components / craft
    ui::confirm(window);
}

fn execute_task(window: ui::WinHandle, task: &Task) {
    for task_index in 1..=task.count {
        println!("crafting {} {}/{}", task.item_name, task_index, task.count);
        // Hit the Synthesize button and wait for the window to pop up. We spam
        // it a bit here because the timing can vary a bit depending on framerate
        // and background status after finishing a craft.
        for _ in 0..4 {
            ui::confirm(window);
        }

        // Wait for the craft dialog to pop up
        ui::wait_secs(2);
        // and now execute the actions
        execute_actions(window, &task.actions);
        // If the item is collectable we'll have an additional dialog
        if task.collectable {
            ui::wait_secs(1);
            ui::confirm(window);
        }

        // Wait to get back to the crafting window
        if task.collectable {
            ui::wait_secs(1);
            ui::confirm(window);
            ui::wait_secs(4);
        } else {
            ui::wait_secs(4);
        };
    }
}

fn execute_actions(window: ui::WinHandle, actions: &Vec<macros::Action>) {
    for action in actions {
        print!(".");
        stdout().flush();
        // Each character has a 20ms wait and the shortest action string
        // we can make (observe or reclaim) is 240 ms, along with 50ms
        // from send_action. That reduces how much time is needed to wait
        // here for the GCD to finish. Although macros always wait in 2 or
        // 3 second periods, the actual wait period is 2.0 and 2.5 seconds,
        // so that's adjusted here.
        send_action(window, &action.name);
        if action.wait == 2 {
            ui::wait_ms(1700);
        } else {
            ui::wait_ms(2200);
        };
    }
    println!("");
}

pub fn send_string(window: ui::WinHandle, s: &str) {
    for c in s.chars() {
        ui::send_char(window, c);
    }
}

pub fn send_action(window: ui::WinHandle, action: &str) {
    ui::enter(window);
    send_string(window, &format!("/ac \"{}\"\n", action));
    ui::wait_ms(50);
    ui::enter(window);
}

fn toggle_collectable(window: ui::WinHandle) {
    send_action(window, &"collectable synthesis");
}
