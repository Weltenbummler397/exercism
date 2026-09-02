#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T> {
    elements: Vec<T>
}

impl<T: std::cmp::PartialEq + std::cmp::Ord + std::clone::Clone> CustomSet<T> {
    pub fn new(_input: &[T]) -> Self {
        let mut i = _input.to_vec();
        i.sort();
        i.dedup();
        CustomSet{elements: i}
    }

    pub fn contains(&self, _element: &T) -> bool {
        self.elements.contains(_element)
    }

    pub fn add(&mut self, _element: T) {
        if !self.contains(&_element) {
            self.elements.push(_element);
            self.elements.sort()
        }
    }

    pub fn is_subset(&self, _other: &Self) -> bool {
        self.elements.iter().all(|item| _other.contains(item))
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        self.elements.iter().all(|item| !_other.contains(item))
    }

    #[must_use]
    pub fn intersection(&self, _other: &Self) -> Self {
        let common_elements: Vec<T> = self.elements
            .iter()
            .filter(|item| _other.contains(item))
            .cloned()
            .collect();
        CustomSet { elements: common_elements }
    }

    #[must_use]
    pub fn difference(&self, _other: &Self) -> Self {
        let common_elements: Vec<T> = self.elements
            .iter()
            .filter(|item| !_other.contains(item))
            .cloned()
            .collect();
        CustomSet { elements: common_elements }
    }

    #[must_use]
    pub fn union(&self, _other: &Self) -> Self {
        let mut union_elements: Vec<T> = self.elements
            .iter()
            .chain(_other.elements.iter())
            .cloned()
            .collect();
        union_elements.sort();
        union_elements.dedup();

        CustomSet { elements: union_elements }
    }
}
