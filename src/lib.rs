use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_pos(elapsed: f64) -> f64 {
    return 100.0 + (elapsed / 50.0).cos() * 100.0;
}

pub struct BomberTeammate {
    pub name: String
}

impl BomberTeammate {

    pub fn add(&mut self) {
        self.name = "Nuevo nombre".to_string();
    }

}

#[wasm_bindgen]
pub fn get_obj(newname: String) -> BomberTeammate {
    return BomberTeammate{name:newname};
}

