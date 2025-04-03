use derive_more::{Deref, DerefMut};
pub use smallvec::SmallVec;
use tap::Pipe;

use crate::MiniStr;

/// Compact string list optimized for small datasets
///
/// Implements stack-allocated storage using `SmallVec`, avoiding heap
/// allocation when containing up to `N` elements. Ideal for small string
/// collections created and destroyed frequently.
///
/// # Generic Parameter
///
/// - `N`: Inline storage capacity determining maximum elements stored on stack
///
/// # Example
///
/// ```
/// use glossa_shared::{MiniStr, small_list::SmallList};
///
/// // Create from string literals (stack-allocated)
/// let list: SmallList<4> = ["a", "b", "c"].into_iter().collect();
/// assert_eq!(list.len(), 3);
///
/// // Create from dynamic strings (auto heap allocation when exceeding N)
/// let strings = vec!["hello".to_string(); 5];
/// let list: SmallList<3> = strings.into_iter().collect();
/// assert_eq!(list.len(), 5);
/// ```
#[derive(Default, Debug, Clone, Deref, DerefMut)]
pub struct SmallList<const N: usize>(pub SmallVec<MiniStr, N>);

impl<const N: usize, S> FromIterator<S> for SmallList<N>
where
  S: Into<MiniStr>,
{
  /// Creates a `SmallList` from an iterator, performing automatic type
  /// conversion
  ///
  /// # Parameter
  ///
  /// - `iter`: Any type implementing `IntoIterator` with elements convertible
  ///   to `MiniStr`
  ///
  /// # Conversion Flow
  /// 1. Iterate through elements
  /// 2. Convert each element to `MiniStr` via `Into::into`
  /// 3. Collect into `SmallVec` with stack optimization
  /// 4. Wrap in `SmallList` container
  ///
  /// # Example
  ///
  /// ```
  /// use glossa_shared::{MiniStr, small_list::SmallList};
  ///
  /// let list: SmallList<1> = [
  ///     "Hello",
  /// ].into_iter().collect();
  /// ```
  fn from_iter<I: IntoIterator<Item = S>>(iter: I) -> Self {
    iter
      .into_iter()
      .map(Into::into)
      .collect::<SmallVec<_, N>>()
      .pipe(SmallList)
  }
}
