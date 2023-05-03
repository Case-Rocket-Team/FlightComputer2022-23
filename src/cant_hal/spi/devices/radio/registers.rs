
#[macro_export]
macro_rules! radio_registers {
    ($trait: ident; $(
        $addr: expr, 
        $fsk_osk_ident: ident,
        $lora_ident: ident,
        $reset: expr,
        $default: expr,
        $fsk_osk_descr: expr,
        $lora_descr: expr
    )+) => {
        trait $trait {
            /// Address of the register.
            const ADDR: u8;

            /// Value of the register after chip reset.
            const RESET: Option<u8>;

            /// Default value of the register.
            const DEFAULT: Option<u8>;
        }

        $(crate::_radio_reg!($trait, $addr, $fsk_osk_ident, $lora_ident, $reset, $default, $fsk_osk_descr, $lora_descr);)+
    }
}

#[macro_export]
macro_rules! _radio_reg {
    (
        $trait: ident,
        $addr: expr, 
        $fsk_osk_ident: ident,
        $lora_ident: ident,
        $reset: expr,
        $default: expr,
        $fsk_osk_descr: expr,
        $lora_descr: expr
    ) => {
        paste::paste! {
            crate::_lora_reg!($trait, $addr, [< LoRa $lora_ident >], $reset, $default, $lora_descr);
        }
    }
}

#[macro_export]
macro_rules! _lora_reg {
    (
        $trait: ident,
        $addr: expr, 
        LoRaUNUSED,
        $reset: expr,
        $default: expr,
        $lora_descr: expr
    ) => {};

    (
        $trait: ident,
        $addr: expr, 
        LoRaRESERVED,
        $reset: expr,
        $default: expr,
        $lora_descr: expr
    ) => {};

    (
        $trait: ident,
        $addr: expr, 
        $lora_ident: ident,
        $reset: expr,
        $default: expr,
        $lora_descr: expr
    ) => {
        #[doc = $lora_descr]
        pub struct $lora_ident;

        impl $lora_ident {
            /// Address of the register.
            pub const ADDR: u8 = $addr;

            /// Value of the register after chip reset.
            crate::optional_const!(RESET, u8, $reset);
            
            /// Default value of the register.
            crate::optional_const!(DEFAULT, u8, $default);
        }

        impl $trait for $lora_ident {
            const ADDR: u8 = $addr;
            const RESET: Option<u8> = $crate::optional!($reset);
            const DEFAULT: Option<u8> = $crate::optional!($default);
        }
    }
}

#[macro_export]
macro_rules! optional_const {
    ($name: ident, $type: ty, $val: literal) => {
        pub const $name: $type = $val;
    };

    ($name: ident, $type: ty, $none: expr) => {};
}

#[macro_export]
macro_rules! optional {
    ($lit: literal) => {
        Some($lit)
    };

    ($none: expr) => {
        None
    };    
}