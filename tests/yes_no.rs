use glossa::{LocaleContext, traits::ChainProvider};
use glossa_l10n::{
  error::locale_registry::all_locales as all_locales_of_your_program,
  yes_no::matches::map as your_l10n_map,
};

trait GetL10nText: ChainProvider {
  fn try_get<'t>(&self, key: &[u8]) -> Option<&'t str> {
    let lookup = |(language, key)| match your_l10n_map(language, key) {
      "" => None,
      s => Some(s),
    };

    self
      .provide_chain()?
      .iter()
      .map(|id| (id.as_bytes(), key))
      .find_map(lookup)
  }
}

impl GetL10nText for LocaleContext {}

#[ignore]
#[test]
fn test_yes_no() {
  let new_ctx =
    || LocaleContext::default().with_all_locales(all_locales_of_your_program());
  let display = |ctx: &LocaleContext, key: &str| {
    let text = ctx
      .try_get(key.as_bytes())
      .unwrap_or_else(|| panic!("{}", glossa::Error::new_text_not_found(key)));
    println!("{key}: {text}")
  };

  let ctx = new_ctx();
  for key in ["yes", "no", "ok", "cancel"] {
    display(&ctx, key)
  }
}
