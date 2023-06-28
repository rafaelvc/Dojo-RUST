#![allow(dead_code)]

pub mod experiment_traits { 

    pub fn turn_on_any_enginable( eng: &mut dyn Enginable ) {
        eng.turn_on();
    }
    
    #[derive(Debug)]
    pub struct V6 {
        on: Box<bool>,
        on_: bool,

    }

    impl V6 {
        pub fn new() -> Self {
            V6 { on: Box::new(false), on_ : false } 
        }
        pub fn get_on(&self) -> bool { *self.on }
    }


    impl Enginable for V6 {
        fn turn_on(&mut self) -> bool {
            *self.on = true;
            *self.on
        }

        fn turn_off(&mut self) -> bool {
            *self.on = false;
            *self.on
        }

    }

    //trait Engine  {
    //    fn turn_on(&self) {
    //        self.on = true;
    //    }
    //    fn turn_off(&self);
    //    fn set_power(&self, percent: f32);
    //}

    pub trait Enginable {
        fn turn_on(&mut self) -> bool; 
        fn turn_off(&mut self) -> bool; 
    }

    pub enum Enginies{
        V8,
        Inline4,
        Inline6,
        Flat6,
    }

    pub struct EngineType  {
        eng: Enginies,
        nr_cilinders: usize,
        rpm: u32,
        current_power: f32, // 0 - 100%
        on: bool,
    }

    impl EngineType {
        pub fn turn_on(&mut self) {
            self.on = true;
        }
        fn turn_off(&mut self) {
            self.on = false;
        }
        fn set_power(&mut self, percent: f32) {
            self.current_power = percent;
        }
    }

    pub struct Car {  
        model: String,
        pub engine: EngineType
    }

    impl Car {
        pub fn new(model: String, engine: Enginies) -> Car {
            Car { 
                model, 
                engine: match engine {
                    Enginies::V8 => EngineType{ eng: Enginies::V8, nr_cilinders:8, rpm:9000, current_power:0.0, on:false },
                    Enginies::Inline4 => EngineType{ eng: Enginies::Inline4, nr_cilinders:8, rpm:7500, current_power:0.0, on:false },
                    Enginies::Inline6 => EngineType{ eng: Enginies::Inline6, nr_cilinders:6, rpm:8000, current_power:0.0, on:false },
                    Enginies::Flat6 => EngineType{ eng: Enginies::Inline6, nr_cilinders:6, rpm:8000, current_power:0.0, on:false }
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::experiment_traits::*;
    #[test]
    fn test_tratis() {
        let mut v6_3l = V6::new();
        //let mut v6_3l = V6{ };
//        let v6_3l: V6;
//        print!("{:?}", v6_3l);
        turn_on_any_enginable(&mut v6_3l);
        assert_eq!(true, v6_3l.get_on());


        let mut car = Car::new("AMG GT3".to_string(), Enginies::V8);
        car.engine.turn_on();
        assert_eq!(true, true);
    }
}
