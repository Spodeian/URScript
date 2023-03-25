//! Functions for controlling motion

use core::fmt;
use super::traits::PreProcess;

///  13.1.2 encoder_enable_pulse_decode
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct EncoderEnablePulseDecode {
    encoder_inded: u8,
    decoder_type: u8,
    a: u8,
    b: u8,
}

impl EncoderEnablePulseDecode {
    
}

impl Display for EncoderEnablePulseDecode {
    
}

impl PreProcess for EncoderEnablePulseDecode {
    
}
