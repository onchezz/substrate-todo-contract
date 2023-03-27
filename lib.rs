#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod todo {

    // use ink::prelude::collections::VecDeque;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    // src="https://api-sa.substrate.io/latest.js"
    #[derive(scale::Decode, scale::Encode, Clone, Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    // #[ink::storage_item(derive = false)]
    pub struct Note {
        id: u64,

        todo: String,
    }

    // impl Note {

    //     fn new(note_id: u64, note: String) -> Self {
    //         Self {
    //             id: note_id,
    //             todo: note,
    //         }
    //     }
    // }

    // use ink::primitives::AccountId;

    #[ink(storage)]
    // #[derive(scale::Decode, scale::Encode, Clone, Debug)]

    pub struct Todos {
        // user: AccountId,
        todos: Vec<Note>,
        // todos: VecDeque<Note>,
    }

    impl Todos {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                // user: Self::env().account_id(),
                todos: Default::default(),
            }
        }

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                // user: Self::env().account_id(),
                todos: Vec::new(),
            }
        }

        // #[ink(constructor)]
        // pub fn default() -> Self {
        //     Self::new(Default::default())
        // }

        #[ink(message)]
        pub fn new_note(&mut self, note: String) -> Result<String, String> {
            let id = self.todos.len() + 1;
            let new_note = Note {
                id: id as u64,
                todo: note,
            };
            // self.todos.insert(id, new_note);
            self.todos.push(new_note);
            Ok(String::from("Note successfully added"))
        }

        #[ink(message)]
        pub fn get_notes(&self) -> Vec<Note> {
            let notes = self.todos.clone();
            notes
        }
        #[ink(message)]
        pub fn delete_note(&mut self, id: u64) -> Result<String, String> {
            let notes = &mut self.todos;
            notes.remove(id as usize);
            let msg = format!("Note successfully deleted at index {}", id);
            Ok(String::from(msg))
        }
        #[ink(message)]
        pub fn edit_note(&mut self, id: u64, todo: String) -> Result<String, String> {
            let notes = &mut self.todos;
            for i in notes.iter_mut() {
                if i.id == id {
                    i.todo = todo.clone();
                }
            }
            Ok(String::from("Note successfully Edited "))
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn new_note() {
            let mut todo = Todos {
                // user: Self::env().account_id(),
                todos: Vec::new(),
            };
            let note = String::from(" hello test information");
            todo.new_note(note).unwrap();

            assert_eq!(todo.get_notes().len(), 1)
        }

        #[ink::test]
        fn delete_note() {
            let mut todo = Todos { todos: Vec::new() };
            let note = String::from(" hello test information");
            todo.new_note(note).unwrap();
            let note2 = String::from(" hello test 2");
            todo.new_note(note2).unwrap();
            assert_eq!(todo.get_notes().len(), 2);
            todo.delete_note(1).unwrap();
            assert_eq!(todo.get_notes().len(), 1)
        }

        #[ink::test]
        fn get_notes() {
            let mut todo = Todos { todos: Vec::new() };
            let note = String::from(" hello test information");
            todo.new_note(note).unwrap();
            let note2 = String::from(" hello test 2");
            todo.new_note(note2).unwrap();
            let note3 = String::from(" test info 3 ");
            todo.new_note(note3).unwrap();

            assert_eq!(todo.get_notes().len(), 3)
        }
        #[ink::test]
        fn edit_note() {
            let mut todo = Todos { todos: Vec::new() };
            let note = String::from(" hello test information");
            todo.new_note(note).unwrap();
            let note2 = String::from(" hello test 2");
            todo.edit_note(1, note2).unwrap();

            assert_eq!(todo.get_notes().len(), 1)
        }
    }
}
