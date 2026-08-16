use relm4::{gtk::prelude::WidgetExt, prelude::*};

use crate::weather_rec::TimedRecommendation;

pub struct WeatherRecommendationWidget {
    pub recommendation: TimedRecommendation,
}

#[relm4::factory(pub)]
impl FactoryComponent for WeatherRecommendationWidget {
    type Init = TimedRecommendation;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        gtk::Box{
            gtk::Label {
                set_label: &self.recommendation.get_text(),
                set_wrap: true,
                set_hexpand: true,
                set_justify: gtk::Justification::Center,
                set_margin_horizontal: 10,
            },
        }
    }

    fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            recommendation: init,
        }
    }
}
