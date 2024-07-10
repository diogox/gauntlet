use iced::Border;
use iced::widget::container;
use crate::theme::{BACKGROUND_LIGHT, BACKGROUND_LIGHTER, BACKGROUND_LIGHTEST, GauntletSettingsTheme, TEXT};

#[derive(Default, Clone)]
pub enum TableStyle {
    #[default]
    Default
}

impl iced_table::StyleSheet for GauntletSettingsTheme {
    type Style = TableStyle;

    fn header(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(TEXT.to_iced()),
            background: Some(BACKGROUND_LIGHT.to_iced().into()),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn footer(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(TEXT.to_iced()),
            background: Some(BACKGROUND_LIGHT.to_iced().into()),
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    // TODO selected and hovered upstream
    fn row(&self, _: &Self::Style, index: usize) -> container::Appearance {
        let background = if index % 2 == 0 {
            None
        } else {
            Some(BACKGROUND_LIGHTEST.to_iced().into())
        };

        container::Appearance {
            background,
            border: Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn divider(&self, _: &Self::Style, hovered: bool) -> container::Appearance {
        container::Appearance {
            ..Default::default()
        }
    }
}
