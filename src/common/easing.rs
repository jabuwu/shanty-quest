use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Easing {
    SineIn,
    SineOut,
    SineInOut,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    BackIn,
    BackOut,
    BackInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C1 + 1.;
const C4: f32 = (2. * PI) / 3.;
const C5: f32 = (2. * PI) / 4.5;
const N1: f32 = 7.5625;
const D1: f32 = 2.75;
pub fn ease(func: Easing, x: f32) -> f32 {
    match func {
        Easing::SineIn => 1. - ((x * PI) / 2.).cos(),
        Easing::SineOut => ((x * PI) / 2.).sin(),
        Easing::SineInOut => -((PI * x).cos() - 1.) / 2.,
        Easing::QuadIn => x * x,
        Easing::QuadOut => 1. - (1. - x) * (1. - x),
        Easing::QuadInOut => {
            if x < 0.5 {
                2. * x * x
            } else {
                1. - (-2. * x + 2.).powf(2.) / 2.
            }
        }
        Easing::CubicIn => x * x * x,
        Easing::CubicOut => 1. - (1. - x).powf(3.),
        Easing::CubicInOut => {
            if x < 0.5 {
                4. * x * x * x
            } else {
                1. - (-2. * x + 2.).powf(3.) / 2.
            }
        }
        Easing::QuartIn => x * x * x * x,
        Easing::QuartOut => 1. - (1. - x).powf(4.),
        Easing::QuartInOut => {
            if x < 0.5 {
                8. * x * x * x * x
            } else {
                1. - (-2. * x + 2.).powf(4.) / 2.
            }
        }
        Easing::QuintIn => x * x * x * x * x,
        Easing::QuintOut => 1. - (1. - x).powf(5.),
        Easing::QuintInOut => {
            if x < 0.5 {
                16. * x * x * x * x * x
            } else {
                1. - (-2. * x + 2.).powf(5.) / 2.
            }
        }
        Easing::ExpoIn => {
            if x == 0. {
                0.
            } else {
                2.0_f32.powf(10. * x - 10.)
            }
        }
        Easing::ExpoOut => {
            if x == 1. {
                1.
            } else {
                1. - 2.0_f32.powf(-10. * x)
            }
        }
        Easing::ExpoInOut => {
            if x == 0. {
                0.
            } else {
                if x == 1. {
                    1.
                } else {
                    if x < 0.5 {
                        2.0_f32.powf(20. * x - 10.) / 2.
                    } else {
                        (2. - 2.0_f32.powf(-20. * x + 10.)) / 2.
                    }
                }
            }
        }
        Easing::CircIn => 1. - (1. - x.powf(2.)).sqrt(),
        Easing::CircOut => (1. - (x - 1.).powf(2.)).sqrt(),
        Easing::CircInOut => {
            if x < 0.5 {
                (1. - (1. - (2. * x).powf(2.)).sqrt()) / 2.
            } else {
                ((1. - (-2. * x + 2.).powf(2.)).sqrt() + 1.) / 2.
            }
        }
        Easing::BackIn => C3 * x * x * x - C1 * x * x,
        Easing::BackOut => 1. + C3 * (x - 1.).powf(3.) + C1 * (x - 1.).powf(2.),
        Easing::BackInOut => {
            if x < 0.5 {
                ((2. * x).powf(2.) * ((C2 + 1.) * 2. * x - C2)) / 2.
            } else {
                ((2. * x - 2.).powf(2.) * ((C2 + 1.) * (x * 2. - 2.) + C2) + 2.) / 2.
            }
        }
        Easing::ElasticIn => {
            if x == 0. {
                0.
            } else {
                if x == 1. {
                    1.
                } else {
                    -2.0_f32.powf(10. * x - 10.) * ((x * 10. - 10.75) * C4).sin()
                }
            }
        }
        Easing::ElasticOut => {
            if x == 0. {
                0.
            } else {
                if x == 1. {
                    1.
                } else {
                    2.0_f32.powf(-10. * x) * ((x * 10. - 0.75) * C4).sin() + 1.
                }
            }
        }
        Easing::ElasticInOut => {
            if x == 0. {
                0.
            } else {
                if x == 1. {
                    1.
                } else {
                    if x < 0.5 {
                        -(2.0_f32.powf(20. * x - 10.) * ((20. * x - 11.125) * C5).sin()) / 2.
                    } else {
                        (2.0_f32.powf(-20. * x + 10.) * ((20. * x - 11.125) * C5).sin()) / 2. + 1.
                    }
                }
            }
        }
        Easing::BounceIn => 1. - ease(Easing::BounceOut, 1. - x),
        Easing::BounceOut => {
            if x < 1. / D1 {
                N1 * x * x
            } else if x < 2. / D1 {
                N1 * (x - (1.5 / D1)) * (x - 1.5 / D1) + 0.75
            } else if x < 2.5 / D1 {
                N1 * (x - (2.25 / D1)) * (x - 2.25 / D1) + 0.9375
            } else {
                N1 * (x - (2.625 / D1)) * (x - 2.625 / D1) + 0.984375
            }
        }
        Easing::BounceInOut => {
            if x < 0.5 {
                (1. - ease(Easing::BounceOut, 1. - 2. * x)) / 2.
            } else {
                (1. + ease(Easing::BounceOut, 2. * x - 1.)) / 2.
            }
        }
    }
}
