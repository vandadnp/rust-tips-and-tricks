// Want to support my work 🤝? https://buymeacoffee.com/vandad

// 👇🏻 instead of this
use std::io;
use std::io::Result as IOResult;
use std::io::Seek;

// 👇🏻 do this
use std::io::{self, Result as IOResult, Seek};
