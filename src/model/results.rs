use crate::model::Submit;

#[derive(Debug, PartialEq, Clone)]
pub struct Results {
    pub submits: Vec<Submit>,
}

impl Results {
    pub fn print(&self, amount: usize) {
        self.submits.iter().take(amount).for_each(|s| s.print());
    }

    // todo: filter results
}
