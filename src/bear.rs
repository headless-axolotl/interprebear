#[derive(Debug)]
pub struct Bear {
    pub value: i32,
    pub swap: i32,
    pub basket: Vec<i32>,
    pub selected: usize,
    pub position: usize,
    pub collect_mode: bool,
}

impl Bear {
    pub fn new(position: usize) -> Self {
        Bear {
            value: 0,
            swap: 0,
            basket: vec![0],
            selected: 0,
            position,
            collect_mode: false,
        }
    }

    pub fn selected_value(&self) -> i32 {
        if let Some(val) = self.basket.get(self.selected) {
            *val
        } else {
            0
        }
    }

    pub fn selected_char(&self) -> Option<char> {
        char::from_u32(self.selected_value() as u32)
    }

    pub fn is_equal(&self) -> bool {
        self.selected_value() == self.value
    }

    pub fn toggle(&mut self) {
        self.collect_mode = !self.collect_mode;
    }

    pub fn shift(&mut self) {
        let l = self.basket.len();
        self.selected += if !self.collect_mode { 1 } else { l - 1 };
        self.selected %= l;
    }

    pub fn add(&mut self) {
        let mut sel = self.selected_value();
        if self.collect_mode {
            sel *= -1;
        }
        self.value = self.value.wrapping_add(sel);
    }

    pub fn mul(&mut self) {
        let sel = self.selected_value();
        self.value = if !self.collect_mode {
            self.value.wrapping_mul(sel)
        } else {
            self.value.wrapping_div(sel)
        };
    }

    pub fn and(&mut self) {
        let sel = self.selected_value();
        self.value = if !self.collect_mode {
            self.value & sel
        } else {
            self.value | sel
        }
    }

    pub fn not(&mut self) {
        self.value = !self.value;
    }

    pub fn retrieve(&mut self) {
        if !self.collect_mode {
            self.value = self.selected_value()
        } else if let Some(selected_value) = self.basket.get_mut(self.selected) {
            *selected_value = self.value;
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.value, &mut self.swap);
    }

    pub fn append(&mut self) {
        if !self.collect_mode {
            self.basket.push(self.value);
        } else if self.basket.len() > 1 {
            self.basket.pop();
            self.selected %= self.basket.len();
        }
    }

    pub fn food(&mut self, many: bool) {
        if !self.collect_mode {
            return;
        }
        self.value = self.value.wrapping_shl(1);
        self.value = self.value.wrapping_add(if many { 1 } else { 0 });
    }
}
