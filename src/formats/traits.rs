use std::io::Read;

use formats::errors::Result;
use pattern::pattern::Pattern;

pub trait PatternLoader {
    /// Returns true when the file is able to be loaded by this `PatternLoader`.
    /// Ideally this should inspect the file's magic number, or some metadata; this shouldn't load
    /// the entire file, nor should it perform a check that the contents is valid(unless of course
    /// that is the easiest way, for example when checking that a JSON document is actually a
    /// pattern).
    fn is_loadable(&self, item: &mut Read) -> Result<bool>;

    /// Read the pattern from the file and return it.
    fn read_pattern(&self, item: &mut Read) -> Result<Pattern>;
}