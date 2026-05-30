use std::cell::OnceCell;

use windows_core::Interface;

use crate::bindings::*;

const SHARED_TEMPLATE_XAML: &str = "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
    <ContentControl HorizontalContentAlignment='Stretch' VerticalContentAlignment='Stretch'/>\
    </DataTemplate>";

thread_local! {

    static SHARED_TEMPLATE: OnceCell<DataTemplate> = const { OnceCell::new() };
}

pub fn shared_content_control_template() -> DataTemplate {
    SHARED_TEMPLATE.with(|cell| cell.get_or_init(parse_template).clone())
}

fn parse_template() -> DataTemplate {
    let inspectable = XamlReader::Load(SHARED_TEMPLATE_XAML).unwrap();
    inspectable.cast::<DataTemplate>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires WinUI 3 ASTA thread — run as a ui-tests integration test"]
    fn second_call_returns_same_com_pointer_as_first() {
        let a = shared_content_control_template();
        let b = shared_content_control_template();

        assert_eq!(a, b);
    }
}
