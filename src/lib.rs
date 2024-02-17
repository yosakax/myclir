/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// SQL executor
pub mod query;

// lazy_static! {
//     pub static ref ENV: Environment =  {
//         // Enable connection pooling. Let driver decide wether the attributes of two connection
//         // are similar enough to change the attributes of a pooled one, to fit the requested
//         // connection, or if it is cheaper to create a new Connection from scratch.
//         // See <https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/driver-aware-connection-pooling>
//         Environment::new().unwrap();
//         let  env = Environment::new().unwrap();
//         env
//     };
// }

/// Completion for editor
pub mod completion;
