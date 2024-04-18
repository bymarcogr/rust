pub mod theme {
    use iced::{application,color, Border};
    use iced::widget::{container};

    #[derive(Debug,Clone,Copy,Default)]
    pub struct Theme;

    #[derive(Debug,Clone,Copy,Default)]
    pub enum Container{
        #[default]
        Default,
        Bordered
    }

    impl container::StyleSheet for Theme{
        type Style = Container;
        
        fn appearance(&self, _style: &Self::Style) -> container::Appearance {
            match _style{
                Container::Default => container::Appearance::default(),
                Container::Bordered => {
                    container::Appearance {
                        border: Border { color:color!(0x45,085,0x88), width:2.0, radius: {5.0.into()}},
                        ..Default::default()
                    }
                 }
            }
        }
    }
        
       
}
