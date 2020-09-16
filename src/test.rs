use rocket::request::{FromForm, FormItems};

struct Item {
    field: String
}

impl<'f> FromForm<'f> for Item {
    // In practice, we'd use a more descriptive error type.
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<Item, ()> {
        let mut field = None;

        for item in items {
            match item.key.as_str() {
                "balloon" | "space" if field.is_none() => {
                    let decoded = item.value.url_decode().map_err(|_| ())?;
                    field = Some(decoded);
                }
                _ if strict => return Err(()),
                _ => { /* allow extra value when not strict */ }
            }
        }

        field.map(|field| Item { field }).ok_or(())
    }
}

trait Dyno <T> {
    fn (Request: T) -> result(Response: T, httpError)
}