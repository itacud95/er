use crate::autocomplete::autocomplete;

pub mod autocomplete;

/**Todo: 
 * 
 * - Income & outcome calculater
 *  - Save to local file
 *  - Save to firebase
 * 
 */

fn main() {
    let operation = autocomplete();
    if operation.is_none() {
        // tab-completion 
        return;
    }

    let operation = operation.unwrap();
    operation();
}
