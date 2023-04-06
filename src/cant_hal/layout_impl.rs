#[macro_export]
macro_rules! pin_layout {
    ($($pin: tt $alias: ident)+) => {
        $(
            pub type $alias = Output<teensy4_bsp::pins::t40::$pin>;
        )+
    };
}