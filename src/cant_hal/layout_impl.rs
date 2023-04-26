#[macro_export]
macro_rules! pin_layout {
    ($($pin: tt $alias: ident)+) => {
        $(
            #[allow(unused)]
            pub type $alias = Output<teensy4_bsp::pins::t40::$pin>;
        )+
    };
}